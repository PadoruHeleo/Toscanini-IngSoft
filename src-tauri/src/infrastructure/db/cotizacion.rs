use crate::database::get_db_pool_safe;
use crate::commands::logs::log_action;
use crate::infrastructure::db::terminos_condiciones::apply_default_terminos_to_cotizacion;
use chrono::Datelike;
use crate::pdf::{CotizacionPdfData, EmpresaInfo, ClienteInfo, EquipoInfo, PiezaPdf, TerminoPdf};
use sqlx::Row;

use crate::models::cotizacion::{
    Cotizacion, 
    Pieza, 
    PiezaCotizacion, 
    CotizacionDetallada, 
    CreateCotizacionRequest, 
    UpdateCotizacionRequest, 
    CreatePiezaRequest, 
    PiezaCotizacionRequest,
    SalidaEquipo,
    RegistrarSalidaRequest,
    UpdatePiezaRequest,
    InventarioEquipo,
    InventarioEquipoRequest
};

/// Obtener todas las cotizaciones
pub async fn get_cotizaciones() -> Result<Vec<Cotizacion>, String> {
    let pool = get_db_pool_safe()?;
    
    let cotizaciones = sqlx::query_as::<_, Cotizacion>(
        "SELECT cotizacion_id, cotizacion_codigo, costo_revision, costo_reparacion, \
                costo_total, is_aprobada, is_borrador, informe, created_by, created_at, deleted_at \
         FROM COTIZACION \
         WHERE deleted_at IS NULL \
         ORDER BY created_at DESC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizaciones)
}

/// Obtener cotizaciones con información detallada
pub async fn get_cotizaciones_detalladas() -> Result<Vec<CotizacionDetallada>, String> {
    let pool = get_db_pool_safe()?;
    
    let cotizaciones = sqlx::query_as::<_, CotizacionDetallada>(
        "SELECT c.cotizacion_id, c.cotizacion_codigo, c.costo_revision, c.costo_reparacion,
                c.costo_total, c.is_aprobada, c.is_borrador, c.created_by, c.created_at,
                u.usuario_nombre as created_by_nombre
         FROM COTIZACION c
         LEFT JOIN USUARIO u ON c.created_by = u.usuario_id
         WHERE c.deleted_at IS NULL
         ORDER BY c.created_at DESC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizaciones)
}

/// Obtener una cotización por ID
pub async fn get_cotizacion_by_id(cotizacion_id: i32) -> Result<Option<Cotizacion>, String> {
    let pool = get_db_pool_safe()?;
    
    let cotizacion = sqlx::query_as::<_, Cotizacion>(
        "SELECT cotizacion_id, cotizacion_codigo, costo_revision, costo_reparacion,\
                costo_total, is_aprobada, is_borrador, informe, created_by, created_at, deleted_at \
         FROM COTIZACION \
         WHERE cotizacion_id = ? AND deleted_at IS NULL"
    )
    .bind(cotizacion_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizacion)
}

/// Obtener una cotización por código
pub async fn get_cotizacion_by_codigo(cotizacion_codigo: String) -> Result<Option<Cotizacion>, String> {
    let pool = get_db_pool_safe()?;
    
    let cotizacion = sqlx::query_as::<_, Cotizacion>(
        "SELECT cotizacion_id, cotizacion_codigo, costo_revision, costo_reparacion,\
                costo_total, is_aprobada, is_borrador, informe, created_by, created_at, deleted_at \
         FROM COTIZACION \
         WHERE cotizacion_codigo = ? AND deleted_at IS NULL"
    )
    .bind(&cotizacion_codigo)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizacion)
}

/// Crear una nueva cotización
pub async fn create_cotizacion(request: CreateCotizacionRequest) -> Result<Cotizacion, String> {
    let pool = get_db_pool_safe()?;
    // Generar código automático: COT-YYYY-XXXX
    let year = chrono::Utc::now().year();
    // Buscar el mayor número correlativo existente para el año actual
    let last_codigo: Option<String> = sqlx::query_scalar(
        "SELECT cotizacion_codigo FROM COTIZACION WHERE cotizacion_codigo LIKE ? AND deleted_at IS NULL ORDER BY cotizacion_id DESC LIMIT 1"
    )
    .bind(format!("COT-{}-%", year))
    .fetch_one(&*pool)
    .await
    .ok();
    let next_number = if let Some(codigo) = last_codigo {
        // Extraer el número correlativo actual y sumarle 1
        let parts: Vec<&str> = codigo.split('-').collect();
        if parts.len() == 3 {
            parts[2].parse::<u32>().unwrap_or(0) + 1
        } else {
            1
        }
    } else {
        1
    };
    let codigo = format!("COT-{}-{:03}", year, next_number);
    
    println!("🔍 create_cotizacion: Creando cotización con código {}", codigo);
    println!("🔍 create_cotizacion: Piezas recibidas: {:?}", request.piezas);
    
    // Iniciar transacción
    let mut tx = pool.begin().await.map_err(|e| format!("Database error: {}", e))?;
    
    // Crear la cotización
    let result = sqlx::query(
        "INSERT INTO COTIZACION (cotizacion_codigo, costo_revision, costo_reparacion, \
                                costo_total, is_aprobada, is_borrador, informe, created_by) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&codigo)
    .bind(request.costo_revision)
    .bind(request.costo_reparacion)
    .bind(request.costo_total)
    .bind(request.is_aprobada.unwrap_or(false))
    .bind(request.is_borrador.unwrap_or(true))
    .bind(&request.informe)
    .bind(request.created_by)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let cotizacion_id = result.last_insert_id() as i32;
    println!("✅ create_cotizacion: Cotización creada con ID {}", cotizacion_id);
    
    // Agregar piezas si se proporcionaron
    if let Some(ref piezas) = request.piezas {
        println!("📦 create_cotizacion: Insertando {} piezas", piezas.len());
        for (idx, pieza) in piezas.iter().enumerate() {
            // Asegurar que la cantidad sea al menos 1
            let cantidad = if pieza.cantidad <= 0 { 1 } else { pieza.cantidad };
            println!("  Pieza {}: pieza_id={}, cantidad={}", idx + 1, pieza.pieza_id, cantidad);
            
            sqlx::query(
                "INSERT INTO PIEZAS_COTIZACION (pieza_id, cotizacion_id, cantidad) VALUES (?, ?, ?)"
            )
            .bind(pieza.pieza_id)
            .bind(cotizacion_id)
            .bind(cantidad)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                let error_msg = format!("Database error adding part {} (pieza_id={}, cantidad={}): {}", 
                    idx + 1, pieza.pieza_id, cantidad, e);
                println!("❌ {}", error_msg);
                error_msg
            })?;
        }
        println!("✅ Todas las piezas insertadas correctamente");
    } else {
        println!("⚠️ No se proporcionaron piezas, solo se eliminaron las existentes");
    }

    // Vincular con orden de trabajo si se proporciona orden_id
    if let Some(orden_id) = request.orden_id {
        println!("🔗 create_cotizacion: Vinculando cotización {} a orden {}", cotizacion_id, orden_id);
        sqlx::query("UPDATE ORDEN_TRABAJO SET cotizacion_id = ? WHERE orden_id = ?")
            .bind(cotizacion_id)
            .bind(orden_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Database error linking order: {}", e))?;
    }
    
    // Confirmar transacción
    tx.commit().await.map_err(|e| format!("Database error: {}", e))?;

    // Aplicar términos y condiciones por defecto
    if let Err(e) = apply_default_terminos_to_cotizacion(cotizacion_id, request.created_by).await {
        println!("⚠️ Error aplicando términos por defecto: {}", e);
        // No fallamos la creación por esto, pero lo logueamos
    }
    
    // Obtener la cotización creada
    let cotizacion = sqlx::query_as::<_, Cotizacion>(
        "SELECT cotizacion_id, cotizacion_codigo, costo_revision, costo_reparacion,\
                costo_total, is_aprobada, is_borrador, informe, created_by, created_at, deleted_at \
         FROM COTIZACION \
         WHERE cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "CREATE_COTIZACION",
        Some(request.created_by),
        "COTIZACION",
        Some(cotizacion_id),
        None,
        Some(&format!("Cotización creada: {}", codigo))
    ).await;
    
    Ok(cotizacion)
}

/// Actualizar una cotización existente
pub async fn update_cotizacion(cotizacion_id: i32, request: UpdateCotizacionRequest, updated_by: i32) -> Result<Option<Cotizacion>, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener la cotización actual antes de actualizar para logging
    let current_cotizacion = get_cotizacion_by_id(cotizacion_id).await?;
    
    if let Some(ref cotizacion) = current_cotizacion {
        // Si estaba en borrador y ahora se rechaza
        if cotizacion.is_borrador == Some(true) && request.is_aprobada == Some(false) {
            // Desvincular la cotización de la orden de trabajo
            sqlx::query("UPDATE ORDEN_TRABAJO SET cotizacion_id = NULL WHERE cotizacion_id = ?")
                .bind(cotizacion_id)
                .execute(&*pool)
                .await
                .map_err(|e| format!("Database error al desvincular cotización: {}", e))?;

            // AuditLog para rechazo de borrador
            let motivo = request.informe.as_deref().unwrap_or(""); 
            let _ = log_action(
                "ELIMINAR_BORRADOR_COTIZACION",
                Some(updated_by),
                "COTIZACION",
                Some(cotizacion_id),
                Some("Cotización en borrador rechazada"),
                Some(motivo)
            ).await;
        }
    }
    
    // Verificar que el código no está en uso por otra cotización (si se está actualizando)
    if let Some(ref new_codigo) = request.cotizacion_codigo {
        if let Some(existing_cotizacion) = get_cotizacion_by_codigo(new_codigo.clone()).await? {
            if existing_cotizacion.cotizacion_id != cotizacion_id {
                return Err("Ya existe otra cotización con este código".to_string());
            }
        }
    }
    
    let _ = sqlx::query(
        "UPDATE COTIZACION SET \
         cotizacion_codigo = COALESCE(?, cotizacion_codigo),\
         costo_revision = COALESCE(?, costo_revision),\
         costo_reparacion = COALESCE(?, costo_reparacion),\
         costo_total = COALESCE(?, costo_total),\
         is_aprobada = COALESCE(?, is_aprobada),\
         is_borrador = COALESCE(?, is_borrador),\
         informe = COALESCE(?, informe)\
         WHERE cotizacion_id = ?"
    )
    .bind(&request.cotizacion_codigo)
    .bind(request.costo_revision)
    .bind(request.costo_reparacion)
    .bind(request.costo_total)
    .bind(request.is_aprobada)
    .bind(request.is_borrador)
    .bind(&request.informe)
    .bind(cotizacion_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    // Registrar cambios en el log de auditoría
    if let Some(ref cotizacion) = current_cotizacion {
        let mut cambios = Vec::new();
        
        if let Some(ref new_codigo) = request.cotizacion_codigo {
            if cotizacion.cotizacion_codigo.as_deref() != Some(new_codigo.as_str()) {
                cambios.push(format!("Código: '{}' → '{}'", 
                    cotizacion.cotizacion_codigo.as_deref().unwrap_or("(vacío)"),
                    new_codigo
                ));
            }
        }
        
        let descripcion = if cambios.is_empty() {
            "Sin cambios detectados".to_string()
        } else {
            format!("Campos modificados: {}", cambios.join(", "))
        };
        
        let _ = log_action(
            "UPDATE_COTIZACION",
            Some(updated_by),
            "COTIZACION",
            Some(cotizacion_id),
            None,
            Some(&descripcion)
        ).await;
    }
    
    get_cotizacion_by_id(cotizacion_id).await
}

/// Eliminar una cotización (eliminación lógica solo si fue enviada al cliente)
pub async fn delete_cotizacion(cotizacion_id: i32, deleted_by: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener la cotización antes de eliminarla para logging (sin filtrar por deleted_at)
    let cotizacion_to_delete = sqlx::query_as::<_, Cotizacion>(
        "SELECT cotizacion_id, cotizacion_codigo, costo_revision, costo_reparacion,\
                costo_total, is_aprobada, is_borrador, informe, created_by, created_at, deleted_at \
         FROM COTIZACION \
         WHERE cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?
    .ok_or_else(|| "Cotización no encontrada".to_string())?;
    
    // Si ya está eliminada lógicamente, no hacer nada
    if cotizacion_to_delete.deleted_at.is_some() {
        return Err("La cotización ya fue eliminada".to_string());
    }
    
    // Verificar si la cotización tiene órdenes de trabajo asociadas
    let has_dependencies = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM ORDEN_TRABAJO WHERE cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Database error checking dependencies: {}", e))?;
    
    // Verificar si fue enviada al cliente (is_borrador == false)
    let fue_enviada = cotizacion_to_delete.is_borrador == Some(false);
    
    // Iniciar transacción
    let mut tx = pool.begin().await.map_err(|e| format!("Database error: {}", e))?;
    
    let was_deleted = if fue_enviada {
        // Eliminación lógica: cualquier cotización que fue enviada al cliente
        // Marcar como eliminado y desvincular de órdenes
        sqlx::query("UPDATE COTIZACION SET deleted_at = CURRENT_TIMESTAMP WHERE cotizacion_id = ?")
            .bind(cotizacion_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        
        // Desvincular de órdenes de trabajo si existen
        if has_dependencies > 0 {
            sqlx::query("UPDATE ORDEN_TRABAJO SET cotizacion_id = NULL WHERE cotizacion_id = ?")
                .bind(cotizacion_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| format!("Database error: {}", e))?;
        }
        
        true
    } else {
        // Eliminación física: borradores o sin órdenes asociadas
        // Eliminar primero las relaciones con piezas
        sqlx::query("DELETE FROM PIEZAS_COTIZACION WHERE cotizacion_id = ?")
            .bind(cotizacion_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        
        // Luego eliminar la cotización
        let result = sqlx::query("DELETE FROM COTIZACION WHERE cotizacion_id = ?")
            .bind(cotizacion_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        
        result.rows_affected() > 0
    };
    
    // Confirmar transacción
    tx.commit().await.map_err(|e| format!("Database error: {}", e))?;
    
    // Registrar la acción en el log de auditoría
    if was_deleted {
        let action_type = if fue_enviada {
            "DELETE_COTIZACION_LOGICAL"
        } else {
            "DELETE_COTIZACION"
        };
        let _ = log_action(
            action_type,
            Some(deleted_by),
            "COTIZACION",
            Some(cotizacion_id),
            Some(&format!("Cotización {} eliminada: {}", 
                if fue_enviada { "lógicamente" } else { "físicamente" },
                cotizacion_to_delete.cotizacion_codigo.as_deref().unwrap_or("N/A")
            )),
            None
        ).await;
    }
    
    Ok(was_deleted)
}

/// Obtener todas las piezas
pub async fn get_piezas() -> Result<Vec<Pieza>, String> {
    let pool = get_db_pool_safe()?;
    let piezas = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at FROM PIEZA ORDER BY pieza_nombre ASC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(piezas)
}

/// Obtener una pieza por ID
pub async fn get_pieza_by_id(pieza_id: i32) -> Result<Option<Pieza>, String> {
    let pool = get_db_pool_safe()?;
    let pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(pieza)
}

/// Crear una nueva pieza
pub async fn create_pieza(request: CreatePiezaRequest) -> Result<Pieza, String> {
    let pool = get_db_pool_safe()?;
    let result = sqlx::query(
        "INSERT INTO PIEZA (pieza_nombre, pieza_marca, pieza_desc, pieza_precio) VALUES (?, ?, ?, ?)"
    )
    .bind(&request.pieza_nombre)
    .bind(&request.pieza_marca)
    .bind(&request.pieza_desc)
    .bind(request.pieza_precio)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    let pieza_id = result.last_insert_id() as i32;
    let pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    // Log de creación de pieza
    let _ = log_action(
        "CREATE_PIEZA",
        None,
        "PIEZA",
        Some(pieza_id),
        None,
        Some(&format!("Pieza creada: {}", request.pieza_nombre))
    ).await;
    
    Ok(pieza)
}

/// Obtener historial completo de salidas
pub async fn get_salidas_equipo() -> Result<Vec<SalidaEquipo>, String> {
    let pool = get_db_pool_safe()?;
    
    let salidas = sqlx::query_as::<_, SalidaEquipo>(
        "SELECT s.salida_id, s.orden_trabajo_id, s.motivo_salida, s.fecha_salida,
                s.usuario_id, s.observaciones, s.created_at,
                ot.orden_codigo, 
                CONCAT(e.equipo_marca, ' ', e.equipo_modelo) as equipo_nombre, 
                c.cliente_nombre, u.usuario_nombre
         FROM SALIDA_EQUIPO s
         JOIN ORDEN_TRABAJO ot ON s.orden_trabajo_id = ot.orden_id
         JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         JOIN CLIENTE c ON e.cliente_id = c.cliente_id
         LEFT JOIN USUARIO u ON s.usuario_id = u.usuario_id
         ORDER BY s.fecha_salida DESC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo salidas: {}", e))?;
    
    Ok(salidas)
}

/// Verificar si una orden puede registrar salida (NUEVA VALIDACIÓN)
pub async fn puede_registrar_salida_v2(orden_trabajo_id: i32) -> Result<(bool, String), String> {
    let pool = get_db_pool_safe()?;
    
    // Verificar estado de la orden
    let estado = sqlx::query_scalar::<_, Option<String>>(
        "SELECT estado FROM ORDEN_TRABAJO WHERE orden_id = ?"
    )
    .bind(orden_trabajo_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error verificando estado: {}", e))?
    .flatten();
    
    let estado = estado.ok_or("Orden no encontrada")?;
    
    // Verificar que no hay salida registrada
    let salida_existente = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM SALIDA_EQUIPO WHERE orden_trabajo_id = ?"
    )
    .bind(orden_trabajo_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error verificando salida: {}", e))?;
    
    let estados_validos = vec![
        "recibido", "cotizacion_enviada", "aprobacion_pendiente",
        "en_reparacion", "espera_de_retiro", "cotizacion_rechazada"
    ];
    
    let puede_registrar = estados_validos.contains(&estado.as_str()) && salida_existente == 0;
    
    let mensaje = if salida_existente > 0 {
        format!("Ya se registró salida para esta orden")
    } else if !estados_validos.contains(&estado.as_str()) {
        format!("Estado '{}' no permite registro de salida", estado)
    } else {
        format!("Puede registrar salida - Estado: {}", estado)
    };
    
    Ok((puede_registrar, mensaje))
}

/// Obtener salida específica por orden de trabajo
pub async fn get_salida_by_orden(orden_trabajo_id: i32) -> Result<Option<SalidaEquipo>, String> {
    let pool = get_db_pool_safe()?;
    
    let salida = sqlx::query_as::<_, SalidaEquipo>(
        "SELECT s.salida_id, s.orden_trabajo_id, s.motivo_salida, s.fecha_salida,
                s.usuario_id, s.observaciones, s.created_at,
                ot.orden_codigo, 
                CONCAT(e.equipo_marca, ' ', e.equipo_modelo) as equipo_nombre, 
                c.cliente_nombre, u.usuario_nombre
         FROM SALIDA_EQUIPO s
         JOIN ORDEN_TRABAJO ot ON s.orden_trabajo_id = ot.orden_id
         JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         JOIN CLIENTE c ON e.cliente_id = c.cliente_id
         LEFT JOIN USUARIO u ON s.usuario_id = u.usuario_id
         WHERE s.orden_trabajo_id = ?"
    )
    .bind(orden_trabajo_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo salida: {}", e))?;
    
    Ok(salida)
}

/// Actualizar las piezas de una cotización
pub async fn update_cotizacion_piezas(
    cotizacion_id: i32,
    piezas: Vec<PiezaCotizacionRequest>,
    updated_by: i32,
) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    println!("🔍 update_cotizacion_piezas: Actualizando {} piezas para cotización_id {}", piezas.len(), cotizacion_id);
    
    // Iniciar transacción
    let mut tx = pool.begin().await.map_err(|e| format!("Database error: {}", e))?;
    
    // Eliminar todas las piezas existentes de la cotización
    sqlx::query("DELETE FROM PIEZAS_COTIZACION WHERE cotizacion_id = ?")
        .bind(cotizacion_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Database error deleting existing parts: {}", e))?;
    
    println!("✅ Piezas existentes eliminadas");
    
    // Insertar las nuevas piezas
    if !piezas.is_empty() {
        for (idx, pieza) in piezas.iter().enumerate() {
            let cantidad = if pieza.cantidad <= 0 { 1 } else { pieza.cantidad };
            println!("  Insertando pieza {}: pieza_id={}, cantidad={}", idx + 1, pieza.pieza_id, cantidad);
            
            sqlx::query(
                "INSERT INTO PIEZAS_COTIZACION (pieza_id, cotizacion_id, cantidad) VALUES (?, ?, ?)"
            )
            .bind(pieza.pieza_id)
            .bind(cotizacion_id)
            .bind(cantidad)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                let error_msg = format!("Database error adding part {} (pieza_id={}, cantidad={}): {}", 
                    idx + 1, pieza.pieza_id, cantidad, e);
                println!("❌ {}", error_msg);
                error_msg
            })?;
        }
        println!("✅ Todas las piezas insertadas correctamente");
    } else {
        println!("⚠️ No se proporcionaron piezas, solo se eliminaron las existentes");
    }
    
    // Confirmar transacción
    tx.commit().await.map_err(|e| format!("Database error committing transaction: {}", e))?;
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "UPDATE_COTIZACION_PIEZAS",
        Some(updated_by),
        "COTIZACION",
        Some(cotizacion_id),
        None,
        Some(&format!("Actualizadas {} piezas", piezas.len()))
    ).await;
    
    Ok(true)
}

/// Registrar salida de equipo
pub async fn registrar_salida_equipo(request: RegistrarSalidaRequest) -> Result<SalidaEquipo, String> {
    let pool = get_db_pool_safe()?;
    
    // Verificar si ya existe salida para esta orden
    let salida_existente = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM SALIDA_EQUIPO WHERE orden_trabajo_id = ?"
    )
    .bind(request.orden_trabajo_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if salida_existente > 0 {
        return Err("Ya existe una salida registrada para esta orden de trabajo".to_string());
    }
    
    // Iniciar transacción
    let mut tx = pool.begin().await.map_err(|e| format!("Database error: {}", e))?;
    
    // Insertar salida
    let result = sqlx::query(
        "INSERT INTO SALIDA_EQUIPO (orden_trabajo_id, motivo_salida, usuario_id, observaciones) VALUES (?, ?, ?, ?)"
    )
    .bind(request.orden_trabajo_id)
    .bind(&request.motivo_salida)
    .bind(request.usuario_id)
    .bind(&request.observaciones)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let salida_id = result.last_insert_id() as i32;
    
    // Actualizar estado de la orden a "entregado" si no está ya en un estado final
    sqlx::query("UPDATE ORDEN_TRABAJO SET estado = 'entregado', finished_at = CURRENT_TIMESTAMP WHERE orden_id = ?")
        .bind(request.orden_trabajo_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Database error updating order status: {}", e))?;
        
    // Confirmar transacción
    tx.commit().await.map_err(|e| format!("Database error: {}", e))?;
    
    // Obtener la salida creada
    let salida = sqlx::query_as::<_, SalidaEquipo>(
        "SELECT s.salida_id, s.orden_trabajo_id, s.motivo_salida, s.fecha_salida,
                s.usuario_id, s.observaciones, s.created_at,
                ot.orden_codigo, 
                CONCAT(e.equipo_marca, ' ', e.equipo_modelo) as equipo_nombre, 
                c.cliente_nombre, u.usuario_nombre
         FROM SALIDA_EQUIPO s
         JOIN ORDEN_TRABAJO ot ON s.orden_trabajo_id = ot.orden_id
         JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         JOIN CLIENTE c ON e.cliente_id = c.cliente_id
         LEFT JOIN USUARIO u ON s.usuario_id = u.usuario_id
         WHERE s.salida_id = ?"
    )
    .bind(salida_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    // Log de la acción
    let _ = log_action(
        "REGISTRAR_SALIDA",
        Some(request.usuario_id),
        "ORDEN_TRABAJO",
        Some(request.orden_trabajo_id),
        None,
        Some(&format!("Salida registrada. Motivo: {}", request.motivo_salida))
    ).await;
    
    Ok(salida)
}

/// Obtener piezas de una cotización
pub async fn get_piezas_cotizacion(cotizacion_id: i32) -> Result<Vec<PiezaCotizacion>, String> {
    let pool = get_db_pool_safe()?;
    
    let piezas = sqlx::query_as::<_, PiezaCotizacion>(
        "SELECT pc.cotizacion_id, pc.pieza_id, pc.cantidad,
                p.pieza_nombre, p.pieza_marca, p.pieza_desc, p.pieza_precio, p.pieza_stock
         FROM PIEZAS_COTIZACION pc
         JOIN PIEZA p ON pc.pieza_id = p.pieza_id
         WHERE pc.cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(piezas)
}

/// Eliminar una pieza (lógicamente o físicamente si no tiene uso)
pub async fn delete_pieza(pieza_id: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Verificar si la pieza está en uso en alguna cotización
    let uso_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM PIEZAS_COTIZACION WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if uso_count > 0 {
        return Err("No se puede eliminar la pieza porque está en uso en cotizaciones".to_string());
    }
    
    let result = sqlx::query("DELETE FROM PIEZA WHERE pieza_id = ?")
        .bind(pieza_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
        
    Ok(result.rows_affected() > 0)
}

/// Actualizar una pieza
pub async fn update_pieza(pieza_id: i32, request: UpdatePiezaRequest) -> Result<Pieza, String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query(
        "UPDATE PIEZA SET pieza_nombre = ?, pieza_marca = ?, pieza_desc = ?, pieza_precio = ? WHERE pieza_id = ?"
    )
    .bind(&request.pieza_nombre)
    .bind(&request.pieza_marca)
    .bind(&request.pieza_desc)
    .bind(request.pieza_precio)
    .bind(pieza_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("Pieza no encontrada".to_string());
    }
    
    let pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(pieza)
}

/// Actualizar stock de pieza
pub async fn update_pieza_stock(pieza_id: i32, cantidad: i32, tipo: String) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    let operation = if tipo == "add" { "+" } else { "-" };
    let query = format!("UPDATE PIEZA SET pieza_stock = pieza_stock {} ? WHERE pieza_id = ?", operation);
    
    let result = sqlx::query(&query)
        .bind(cantidad)
        .bind(pieza_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
        
    Ok(result.rows_affected() > 0)
}

/// Obtener inventario de equipos
pub async fn get_inventario_equipos() -> Result<Vec<InventarioEquipo>, String> {
    let pool = get_db_pool_safe()?;
    
    // CORRECCIÓN FINAL: Alineado con 015_create_inventario_equipo_table.sql
    let query = r#"
        SELECT 
            inventario_equipo_id AS equipo_id,
            equipo_codigo,
            equipo_nombre,
            equipo_tipo, 
            equipo_marca, 
            equipo_modelo, 
            numero_serie, 
            equipo_estado, 
            equipo_ubicacion, 
            equipo_stock,
            equipo_precio,
            created_at, 
            updated_at
        FROM INVENTARIO_EQUIPO
        ORDER BY created_at DESC
    "#;
    
    let equipos = sqlx::query_as::<_, InventarioEquipo>(query)
        .fetch_all(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(equipos)
}

/// Crear registro de inventario
pub async fn create_inventario_equipo(request: InventarioEquipoRequest) -> Result<InventarioEquipo, String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query(
        "INSERT INTO INVENTARIO_EQUIPO (equipo_tipo, equipo_marca, equipo_modelo, numero_serie, 
                                       equipo_estado, equipo_ubicacion) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&request.equipo_tipo)
    .bind(&request.equipo_marca)
    .bind(&request.equipo_modelo)
    .bind(&request.numero_serie)
    .bind(&request.equipo_estado)
    .bind(&request.equipo_ubicacion)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    let equipo_id = result.last_insert_id() as i32;

    let equipo = sqlx::query_as::<_, InventarioEquipo>(
        "SELECT inventario_equipo_id AS equipo_id, equipo_tipo, equipo_marca, equipo_modelo, numero_serie,
                equipo_estado, equipo_ubicacion, created_at, updated_at
         FROM INVENTARIO_EQUIPO
         WHERE inventario_equipo_id = ?"
    )
    .bind(equipo_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(equipo)
}

/// Actualizar registro de inventario
pub async fn update_inventario_equipo(equipo_id: i32, request: InventarioEquipoRequest) -> Result<InventarioEquipo, String> {
    let pool = get_db_pool_safe()?;
    let result = sqlx::query(
        "UPDATE INVENTARIO_EQUIPO 
         SET equipo_tipo = ?, equipo_marca = ?, equipo_modelo = ?, numero_serie = ?, 
             equipo_estado = ?, equipo_ubicacion = ?
         WHERE inventario_equipo_id = ?"
    )
    .bind(&request.equipo_tipo)
    .bind(&request.equipo_marca)
    .bind(&request.equipo_modelo)
    .bind(&request.numero_serie)
    .bind(&request.equipo_estado)
    .bind(&request.equipo_ubicacion)
    .bind(equipo_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("Equipo no encontrado".to_string());
    }
    
    let equipo = sqlx::query_as::<_, InventarioEquipo>(
        "SELECT inventario_equipo_id AS equipo_id, equipo_tipo, equipo_marca, equipo_modelo, numero_serie, 
                equipo_estado, equipo_ubicacion, created_at, updated_at
         FROM INVENTARIO_EQUIPO
         WHERE inventario_equipo_id = ?"
    )
    .bind(equipo_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(equipo)
}

/// Eliminar registro de inventario
pub async fn delete_inventario_equipo(equipo_id: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query("DELETE FROM INVENTARIO_EQUIPO WHERE inventario_equipo_id = ?")
        .bind(equipo_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
        
    Ok(result.rows_affected() > 0)
}

pub async fn search_cotizaciones(search_term: String) -> Result<Vec<CotizacionDetallada>, String> {
    let pool = get_db_pool_safe()?;
    let pattern = format!("%{}%", search_term);
    let rows = sqlx::query_as::<_, CotizacionDetallada>(
        "SELECT c.cotizacion_id, c.cotizacion_codigo, c.costo_revision, c.costo_reparacion,
                c.costo_total, c.is_aprobada, c.is_borrador, c.informe, c.created_by, c.created_at,
                u.usuario_nombre as created_by_nombre
         FROM COTIZACION c
         LEFT JOIN USUARIO u ON c.created_by = u.usuario_id
         WHERE (c.cotizacion_codigo LIKE ? OR u.usuario_nombre LIKE ?) AND c.deleted_at IS NULL 
         ORDER BY c.created_at DESC"
    )
    .bind(&pattern)
    .bind(&pattern)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(rows)
}

pub async fn count_cotizaciones() -> Result<i64, String> {
    let pool = get_db_pool_safe()?;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM COTIZACION WHERE deleted_at IS NULL")
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(count)
}

pub async fn get_cotizaciones_with_pagination(offset: i64, limit: i64) -> Result<Vec<CotizacionDetallada>, String> {
    let pool = get_db_pool_safe()?;
    
    let cotizaciones = sqlx::query_as::<_, CotizacionDetallada>(
        "SELECT c.cotizacion_id, c.cotizacion_codigo, c.costo_revision, c.costo_reparacion,
                c.costo_total, c.is_aprobada, c.is_borrador, c.informe, c.created_by, c.created_at,
                u.usuario_nombre as created_by_nombre
         FROM COTIZACION c
         LEFT JOIN USUARIO u ON c.created_by = u.usuario_id
         WHERE c.deleted_at IS NULL
         ORDER BY c.created_at DESC
         LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizaciones)
}

pub async fn get_cotizaciones_by_cliente(cliente_id: i32) -> Result<Vec<Cotizacion>, String> {
    let pool = get_db_pool_safe()?;
    
    // Cotizaciones are linked to OrdenTrabajo, which is linked to Equipo, which is linked to Cliente.
    let cotizaciones = sqlx::query_as::<_, Cotizacion>(
        "SELECT c.cotizacion_id, c.cotizacion_codigo, c.costo_revision, c.costo_reparacion,
                c.costo_total, c.is_aprobada, c.is_borrador, c.informe, c.created_by, c.created_at, c.deleted_at
         FROM COTIZACION c
         JOIN ORDEN_TRABAJO ot ON c.cotizacion_id = ot.cotizacion_id
         JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         WHERE e.cliente_id = ? AND c.deleted_at IS NULL
         ORDER BY c.created_at DESC"
    )
    .bind(cliente_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizaciones)
}

pub async fn get_piezas_inventario() -> Result<Vec<Pieza>, String> {
    let pool = get_db_pool_safe()?;
    
    let piezas = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at 
         FROM PIEZA 
         WHERE pieza_stock > 0
         ORDER BY pieza_nombre ASC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(piezas)
}

pub async fn get_cotizacion_pdf_data(cotizacion_id: i32) -> Result<CotizacionPdfData, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener datos de la cotización
    let cotizacion_row = sqlx::query(
        "SELECT c.cotizacion_codigo, c.costo_revision, c.costo_reparacion, c.costo_total, 
                c.is_aprobada, c.informe, c.created_at,
                ot.orden_codigo,
                cl.cliente_nombre, cl.cliente_correo, cl.cliente_telefono, cl.cliente_direccion,
                e.equipo_marca, e.equipo_modelo, e.equipo_tipo, e.numero_serie, e.equipo_ubicacion
         FROM COTIZACION c
         LEFT JOIN ORDEN_TRABAJO ot ON c.cotizacion_id = ot.cotizacion_id
         LEFT JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         LEFT JOIN CLIENTE cl ON e.cliente_id = cl.cliente_id
         WHERE c.cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo cotización: {}", e))?
    .ok_or_else(|| format!("Cotización con ID {} no encontrada", cotizacion_id))?;

    // Acceder a los campos por nombre
    let cotizacion_codigo: Option<String> = cotizacion_row.try_get("cotizacion_codigo").ok();
    let costo_revision: Option<i32> = cotizacion_row.try_get("costo_revision").ok();
    let costo_reparacion: Option<i32> = cotizacion_row.try_get("costo_reparacion").ok();
    let costo_total: Option<i32> = cotizacion_row.try_get("costo_total").ok();
    let is_aprobada: Option<i32> = cotizacion_row.try_get("is_aprobada").ok();
    let informe: Option<String> = cotizacion_row.try_get("informe").ok();
    let created_at: chrono::DateTime<chrono::Utc> = cotizacion_row.try_get("created_at").ok().unwrap();
    let orden_codigo: Option<String> = cotizacion_row.try_get("orden_codigo").ok();
    let cliente_nombre: Option<String> = cotizacion_row.try_get("cliente_nombre").ok();
    let cliente_correo: Option<String> = cotizacion_row.try_get("cliente_correo").ok();
    let cliente_telefono: Option<String> = cotizacion_row.try_get("cliente_telefono").ok();
    let cliente_direccion: Option<String> = cotizacion_row.try_get("cliente_direccion").ok();
    let equipo_marca: Option<String> = cotizacion_row.try_get("equipo_marca").ok();
    let equipo_modelo: Option<String> = cotizacion_row.try_get("equipo_modelo").ok();
    let equipo_tipo: Option<String> = cotizacion_row.try_get("equipo_tipo").ok();
    let numero_serie: Option<String> = cotizacion_row.try_get("numero_serie").ok();
    let equipo_ubicacion: Option<String> = cotizacion_row.try_get("equipo_ubicacion").ok();

    // Obtener piezas de la cotización
    let piezas_rows = sqlx::query(
        "SELECT p.pieza_nombre, p.pieza_marca, p.pieza_precio, pc.cantidad
         FROM PIEZAS_COTIZACION pc
         INNER JOIN PIEZA p ON pc.pieza_id = p.pieza_id
         WHERE pc.cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo piezas: {}", e))?;

    // Obtener términos y condiciones de la cotización
    let terminos_rows = sqlx::query(
        "SELECT tc.termino_nombre, tc.termino_descripcion
         FROM TERMINOS_COTIZACION tcot
         INNER JOIN TERMINOS_CONDICIONES tc ON tcot.termino_id = tc.termino_id
         WHERE tcot.cotizacion_id = ? AND tcot.aplicado = TRUE AND tc.is_active = TRUE
         ORDER BY tc.termino_nombre"
    )
    .bind(cotizacion_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo términos y condiciones: {}", e))?;
    
    let piezas: Vec<PiezaPdf> = piezas_rows.iter().map(|row| {
        let pieza_nombre: String = row.try_get("pieza_nombre")
            .ok()
            .flatten()
            .unwrap_or_else(|| "Pieza sin nombre".to_string());
        let pieza_marca: Option<String> = row.try_get("pieza_marca").ok().flatten();
        let pieza_precio: Option<i32> = row.try_get("pieza_precio").ok().flatten();
        let cantidad: Option<i32> = row.try_get("cantidad").ok().flatten();
        
        PiezaPdf {
            nombre: pieza_nombre,
            marca: pieza_marca,
            cantidad: cantidad.unwrap_or(1),
            precio_unitario: pieza_precio.unwrap_or(0),
            subtotal: (pieza_precio.unwrap_or(0) * cantidad.unwrap_or(1)),
        }
    }).collect();

    let terminos_condiciones: Vec<TerminoPdf> = terminos_rows.iter().map(|row| {
        let termino_nombre: String = row.try_get("termino_nombre").unwrap();
        let termino_descripcion: String = row.try_get("termino_descripcion").unwrap();
        TerminoPdf {
            nombre: termino_nombre,
            descripcion: termino_descripcion,
        }
    }).collect();

    // Crear estructura de datos para el PDF
    let pdf_data = CotizacionPdfData {
        cotizacion_codigo: cotizacion_codigo.unwrap_or_else(|| "COT-0000".to_string()),
        fecha: created_at,
        empresa: EmpresaInfo {
            nombre: "Toscanini".to_string(),
            direccion: Some("Dirección de la empresa".to_string()),
            telefono: Some("+56 9 1234 5678".to_string()),
            email: Some("contacto@toscanini.cl".to_string()),
            website: Some("www.toscanini.cl".to_string()),
        },
        cliente: ClienteInfo {
            nombre: cliente_nombre.unwrap_or_else(|| "Cliente sin nombre".to_string()),
            email: cliente_correo,
            telefono: cliente_telefono,
            direccion: cliente_direccion,
        },
        equipo: EquipoInfo {
            marca: equipo_marca,
            modelo: equipo_modelo,
            tipo: equipo_tipo,
            numero_serie: numero_serie,
            ubicacion: equipo_ubicacion,
        },
        informe_tecnico: informe.unwrap_or_else(|| "Sin informe técnico".to_string()),
        costo_revision: costo_revision,
        costo_reparacion: costo_reparacion,
        costo_total: costo_total.unwrap_or(0),
        piezas,
        is_aprobada: is_aprobada.unwrap_or(0) == 1,
        orden_codigo: orden_codigo,
        terminos_condiciones,
    };

    Ok(pdf_data)
}

pub async fn update_inventario_equipo_stock(equipo_id: i32, cantidad: i32, tipo: String, updated_by: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Validar tipo
    if tipo != "add" && tipo != "subtract" {
        return Err("Tipo de ajuste inválido (use 'add' o 'subtract')".to_string());
    }

    let operation = if tipo == "add" { "+" } else { "-" };
    // Usamos COALESCE para tratar null como 0
    let query = format!("UPDATE INVENTARIO_EQUIPO SET equipo_stock = COALESCE(equipo_stock, 0) {} ? WHERE inventario_equipo_id = ?", operation);
    
    let result = sqlx::query(&query)
        .bind(cantidad)
        .bind(equipo_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("Equipo no encontrado".to_string());
    }

    // Log action
    let _ = log_action(
        "UPDATE_INVENTARIO_STOCK",
        Some(updated_by),
        "INVENTARIO_EQUIPO",
        Some(equipo_id),
        None,
        Some(&format!("Stock {} por {}", if tipo == "add" { "aumentado" } else { "disminuido" }, cantidad))
    ).await;
        
    Ok(true)
}