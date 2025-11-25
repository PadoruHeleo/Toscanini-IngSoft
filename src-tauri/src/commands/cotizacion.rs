use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_safe;
use crate::commands::logs::log_action;
use crate::commands::terminos_condiciones::apply_default_terminos_to_cotizacion;
use chrono::{DateTime, Utc};
use chrono::Datelike;
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
    OrdenInfoRow
};

/// Obtener todas las cotizaciones
#[tauri::command]
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
#[tauri::command]
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
#[tauri::command]
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
#[tauri::command]
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
#[tauri::command]
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
            
            println!("  ✅ Pieza {} insertada correctamente", idx + 1);
        }
        println!("✅ create_cotizacion: Todas las piezas insertadas correctamente");
    } else {
        println!("⚠️ create_cotizacion: No se proporcionaron piezas");
    }
    
    // Confirmar transacción
    tx.commit().await.map_err(|e| format!("Database error committing transaction: {}", e))?;
    println!("✅ create_cotizacion: Transacción confirmada");
    
    // Aplicar términos y condiciones por defecto automáticamente
    let _ = apply_default_terminos_to_cotizacion(cotizacion_id, request.created_by).await;
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "CREATE_COTIZACION",
        Some(request.created_by),
        "COTIZACION",
        Some(cotizacion_id),
        None,
        Some(&format!("Cotización creada: {}", codigo))
    ).await;
    
    // Obtener la cotización recién creada
    get_cotizacion_by_id(cotizacion_id)
        .await?
        .ok_or_else(|| "Failed to retrieve created cotización".to_string())
}

/// Actualizar una cotización existente
#[tauri::command]
pub async fn update_cotizacion(cotizacion_id: i32, request: UpdateCotizacionRequest, updated_by: i32) -> Result<Option<Cotizacion>, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener la cotización actual antes de actualizar para logging
    let current_cotizacion = get_cotizacion_by_id(cotizacion_id).await?;
    
    if let Some(ref cotizacion) = current_cotizacion {
        // Si estaba en borrador y ahora se rechaza
        if cotizacion.is_borrador == Some(true) && request.is_aprobada == Some(false) {
            // Guardar el motivo si corresponde (ya lo haces)
            // Desvincular la cotización de la orden de trabajo
            sqlx::query("UPDATE ORDEN_TRABAJO SET cotizacion_id = NULL WHERE cotizacion_id = ?")
                .bind(cotizacion_id)
                .execute(&*pool)
                .await
                .map_err(|e| format!("Database error al desvincular cotización: {}", e))?;

            // AuditLog para rechazo de borrador
            let motivo = request.informe.as_deref().unwrap_or(""); // O usa request.motivo_rechazo si tienes ese campo
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
    
    let result = sqlx::query(
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
    
    if result.rows_affected() == 0 {
        return Ok(None);
    }
    
    // Registrar la acción en el log de auditoría
    if let Some(ref cotizacion) = current_cotizacion {
        let mut cambios = Vec::new();
        
        // Comparar cada campo y agregar a la lista de cambios si fue modificado
        if let Some(ref new_codigo) = request.cotizacion_codigo {
            if cotizacion.cotizacion_codigo.as_deref() != Some(new_codigo.as_str()) {
                cambios.push(format!("Código: '{}' → '{}'", 
                    cotizacion.cotizacion_codigo.as_deref().unwrap_or("(vacío)"),
                    new_codigo
                ));
            }
        }
        
        if let Some(new_costo_revision) = request.costo_revision {
            if cotizacion.costo_revision != Some(new_costo_revision) {
                cambios.push(format!("Costo revisión: ${} → ${}", 
                    cotizacion.costo_revision.map_or(0, |c| c),
                    new_costo_revision
                ));
            }
        }
        
        if let Some(new_costo_reparacion) = request.costo_reparacion {
            if cotizacion.costo_reparacion != Some(new_costo_reparacion) {
                cambios.push(format!("Costo reparación: ${} → ${}", 
                    cotizacion.costo_reparacion.map_or(0, |c| c),
                    new_costo_reparacion
                ));
            }
        }
        
        if let Some(new_costo_total) = request.costo_total {
            if cotizacion.costo_total != Some(new_costo_total) {
                cambios.push(format!("Costo total: ${} → ${}", 
                    cotizacion.costo_total.map_or(0, |c| c),
                    new_costo_total
                ));
            }
        }
        
        if let Some(new_aprobada) = request.is_aprobada {
            if cotizacion.is_aprobada != Some(new_aprobada) {
                cambios.push(format!("Aprobada: {} → {}", 
                    cotizacion.is_aprobada.map_or("false".to_string(), |a| a.to_string()),
                    new_aprobada
                ));
            }
        }
        
        if let Some(new_borrador) = request.is_borrador {
            if cotizacion.is_borrador != Some(new_borrador) {
                cambios.push(format!("Borrador: {} → {}", 
                    cotizacion.is_borrador.map_or("false".to_string(), |b| b.to_string()),
                    new_borrador
                ));
            }
        }
        
        if let Some(ref new_informe) = request.informe {
            if cotizacion.informe != *new_informe {
                cambios.push(format!("Informe: modificado"));
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
#[tauri::command]
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
#[tauri::command]
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
#[tauri::command]
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
#[tauri::command]
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

#[derive(Debug, Deserialize)]
pub struct UpdatePiezaRequest {
    pub pieza_nombre: Option<String>,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
}

/// Actualizar una pieza existente
#[tauri::command]
pub async fn update_pieza(pieza_id: i32, request: UpdatePiezaRequest) -> Result<Option<Pieza>, String> {
    let pool = get_db_pool_safe()?;
    // Obtener datos previos para el log
    let prev_pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    let result = sqlx::query(
        "UPDATE PIEZA SET \
            pieza_nombre = COALESCE(?, pieza_nombre),\
            pieza_marca = COALESCE(?, pieza_marca),\
            pieza_desc = COALESCE(?, pieza_desc),\
            pieza_precio = COALESCE(?, pieza_precio)\
         WHERE pieza_id = ?"
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
        return Ok(None);
    }
    let pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    // Log de actualización de pieza
    let prev = prev_pieza.as_ref().map(|p| format!("{}|{}|{}|{}", p.pieza_nombre.as_deref().unwrap_or(""), p.pieza_marca.as_deref().unwrap_or(""), p.pieza_desc.as_deref().unwrap_or(""), p.pieza_precio.map_or("".to_string(), |v| v.to_string())));
    let newv = format!("{}|{}|{}|{}", pieza.pieza_nombre.as_deref().unwrap_or(""), pieza.pieza_marca.as_deref().unwrap_or(""), pieza.pieza_desc.as_deref().unwrap_or(""), pieza.pieza_precio.map_or("".to_string(), |v| v.to_string()));
    let _ = log_action(
        "UPDATE_PIEZA",
        None,
        "PIEZA",
        Some(pieza_id),
        prev.as_deref(),
        Some(&newv)
    ).await;
    Ok(Some(pieza))
}

/// Eliminar una pieza
#[tauri::command]
pub async fn delete_pieza(pieza_id: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    // Obtener datos previos para el log
    let prev_pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    let result = sqlx::query("DELETE FROM PIEZA WHERE pieza_id = ?")
        .bind(pieza_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    let deleted = result.rows_affected() > 0;
    // Log de eliminación de pieza
    if deleted {
        if let Some(p) = prev_pieza {
            let prev = format!("{}|{}|{}|{}", p.pieza_nombre.as_deref().unwrap_or(""), p.pieza_marca.as_deref().unwrap_or(""), p.pieza_desc.as_deref().unwrap_or(""), p.pieza_precio.map_or("".to_string(), |v| v.to_string()));
            let _ = log_action(
                "DELETE_PIEZA",
                None,
                "PIEZA",
                Some(pieza_id),
                Some(&prev),
                None
            ).await;
        }
    }
    Ok(deleted)
}

/// Buscar cotizaciones por texto
#[tauri::command]
pub async fn search_cotizaciones(search_term: String) -> Result<Vec<CotizacionDetallada>, String> {
    let pool = get_db_pool_safe()?;
    
    let search_pattern = format!("%{}%", search_term);
    
    let cotizaciones = sqlx::query_as::<_, CotizacionDetallada>(
        "SELECT c.cotizacion_id, c.cotizacion_codigo, c.costo_revision, c.costo_reparacion,\
                c.costo_total, c.is_aprobada, c.is_borrador, c.informe, c.created_by, c.created_at,\
                u.usuario_nombre as created_by_nombre\
         FROM COTIZACION c\
         LEFT JOIN USUARIO u ON c.created_by = u.usuario_id\
         WHERE c.cotizacion_codigo LIKE ? AND c.deleted_at IS NULL \n         ORDER BY c.created_at DESC"
    )
    .bind(&search_pattern)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizaciones)
}

/// Contar total de cotizaciones
#[tauri::command]
pub async fn count_cotizaciones() -> Result<i64, String> {
    let pool = get_db_pool_safe()?;
    
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM COTIZACION WHERE deleted_at IS NULL")
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(count)
}

/// Obtener cotizaciones con paginación
#[tauri::command]
pub async fn get_cotizaciones_with_pagination(offset: i64, limit: i64) -> Result<Vec<CotizacionDetallada>, String> {
    let pool = get_db_pool_safe()?;
    
    let cotizaciones = sqlx::query_as::<_, CotizacionDetallada>(
        "SELECT c.cotizacion_id, c.cotizacion_codigo, c.costo_revision, c.costo_reparacion,\
                c.costo_total, c.is_aprobada, c.is_borrador, c.informe, c.created_by, c.created_at,\
                u.usuario_nombre as created_by_nombre\
         FROM COTIZACION c\
         LEFT JOIN USUARIO u ON c.created_by = u.usuario_id\
         WHERE c.deleted_at IS NULL\
         ORDER BY c.created_at DESC\n         LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizaciones)
}

/// Obtener las piezas asociadas a una cotización
#[tauri::command]
pub async fn get_piezas_cotizacion(cotizacion_id: i32) -> Result<Vec<PiezaCotizacion>, String> {
    let pool = get_db_pool_safe()?;
    let piezas = sqlx::query_as::<_, PiezaCotizacion>(
        "SELECT pc.pieza_id, pc.cotizacion_id, pc.cantidad, \
                p.pieza_nombre, p.pieza_marca, p.pieza_desc, p.pieza_precio \
         FROM PIEZAS_COTIZACION pc \
         LEFT JOIN PIEZA p ON pc.pieza_id = p.pieza_id \
         WHERE pc.cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    println!("🔍 get_piezas_cotizacion: Encontradas {} piezas para cotización_id {}", piezas.len(), cotizacion_id);
    for (idx, pieza) in piezas.iter().enumerate() {
        println!("  Pieza {}: pieza_id={}, cantidad={:?}, nombre={:?}", 
            idx + 1, 
            pieza.pieza_id, 
            pieza.cantidad,
            pieza.pieza_nombre
        );
    }
    
    Ok(piezas)
}

#[tauri::command]
pub async fn get_cotizaciones_by_cliente(cliente_id: i32) -> Result<Vec<Cotizacion>, String> {
    let pool = get_db_pool_safe()?;
    let cotizaciones = sqlx::query_as::<_, Cotizacion>(
        "SELECT c.cotizacion_id, c.cotizacion_codigo, c.costo_revision, c.costo_reparacion, \
                c.costo_total, c.is_aprobada, c.is_borrador, c.informe, c.created_by, c.created_at, c.deleted_at \
         FROM COTIZACION c \
         INNER JOIN ORDEN_TRABAJO ot ON c.cotizacion_id = ot.cotizacion_id \
         INNER JOIN EQUIPO e ON ot.equipo_id = e.equipo_id \
         WHERE e.cliente_id = ? AND c.deleted_at IS NULL \
         GROUP BY c.cotizacion_id \
         ORDER BY c.created_at DESC"
    )
    .bind(cliente_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error al obtener cotizaciones del cliente: {}", e))?;
    Ok(cotizaciones)
}

/// Obtener todas las piezas con información de inventario
#[tauri::command]
pub async fn get_piezas_inventario() -> Result<Vec<Pieza>, String> {
    let pool = get_db_pool_safe()?;
    let piezas = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at 
         FROM PIEZA 
         ORDER BY pieza_nombre ASC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(piezas)
}

/// Actualizar el stock de una pieza
#[tauri::command]
pub async fn update_pieza_stock(pieza_id: i32, cantidad: i32, tipo: String) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener el stock actual
    let current_stock = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT pieza_stock FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Database error getting current stock: {}", e))?
    .unwrap_or(0);
    
    // Calcular el nuevo stock
    let new_stock = match tipo.as_str() {
        "entrada" => current_stock + cantidad,
        "salida" => std::cmp::max(0, current_stock - cantidad),
        _ => return Err("Tipo de operación inválido. Use 'entrada' o 'salida'".to_string()),
    };
    
    // Actualizar el stock
    let result = sqlx::query(
        "UPDATE PIEZA SET pieza_stock = ? WHERE pieza_id = ?"
    )
    .bind(new_stock)
    .bind(pieza_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error updating stock: {}", e))?;
    
    if result.rows_affected() > 0 {
        // Log de la operación
        let _ = log_action(
            "UPDATE_STOCK",
            None,
            "PIEZA",
            Some(pieza_id),
            Some(&format!("Stock anterior: {}", current_stock)),
            Some(&format!("Stock nuevo: {} ({})", new_stock, if tipo == "entrada" { format!("+{}", cantidad) } else { format!("-{}", cantidad) }))
        ).await;
        Ok(true)
    } else {
        Ok(false)
    }
}

// Estructuras para inventario de equipos
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InventarioEquipo {
    pub inventario_equipo_id: i32,
    pub equipo_codigo: Option<String>,
    pub equipo_nombre: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: Option<String>,
    pub equipo_descripcion: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_stock: Option<i32>,
    pub equipo_estado: Option<String>,
    pub equipo_ubicacion: Option<String>,
    pub fecha_adquisicion: Option<String>,
    pub proveedor: Option<String>,
    pub numero_serie: Option<String>,
    pub garantia_vencimiento: Option<String>,
    pub observaciones: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventarioEquipoRequest {
    pub equipo_codigo: String,
    pub equipo_nombre: String,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: String,
    pub equipo_descripcion: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_stock: Option<i32>,
    pub equipo_ubicacion: Option<String>,
    pub proveedor: Option<String>,
    pub numero_serie: Option<String>,
    pub observaciones: Option<String>,
}

// Funciones para inventario de equipos
#[tauri::command]
pub async fn get_inventario_equipos() -> Result<Vec<InventarioEquipo>, String> {
    let pool = get_db_pool_safe()?;
    
    println!("Ejecutando query para obtener inventario de equipos...");
    
    let equipos = sqlx::query_as::<_, InventarioEquipo>(
        "SELECT * FROM INVENTARIO_EQUIPO ORDER BY equipo_nombre"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| {
        println!("Error en query INVENTARIO_EQUIPO: {}", e);
        format!("Database error: {}", e)
    })?;
    
    println!("Equipos encontrados: {}", equipos.len());
    Ok(equipos)
}

#[tauri::command]
pub async fn create_inventario_equipo(request: InventarioEquipoRequest) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query(
        "INSERT INTO INVENTARIO_EQUIPO (
            equipo_codigo, equipo_nombre, equipo_marca, equipo_modelo, 
            equipo_tipo, equipo_descripcion, equipo_precio, equipo_stock,
            equipo_estado, equipo_ubicacion, proveedor, numero_serie, observaciones
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'disponible', ?, ?, ?, ?)"
    )
    .bind(&request.equipo_codigo)
    .bind(&request.equipo_nombre)
    .bind(&request.equipo_marca)
    .bind(&request.equipo_modelo)
    .bind(&request.equipo_tipo)
    .bind(&request.equipo_descripcion)
    .bind(&request.equipo_precio)
    .bind(&request.equipo_stock.unwrap_or(0))
    .bind(&request.equipo_ubicacion)
    .bind(&request.proveedor)
    .bind(&request.numero_serie)
    .bind(&request.observaciones)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() > 0 {
        let _ = log_action(
            "CREATE",
            None,
            "INVENTARIO_EQUIPO",
            None,
            None,
            Some(&format!("Creado equipo: {} - {}", request.equipo_codigo, request.equipo_nombre))
        ).await;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn update_inventario_equipo(equipo_id: i32, request: InventarioEquipoRequest) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query(
        "UPDATE INVENTARIO_EQUIPO SET 
            equipo_codigo = ?, equipo_nombre = ?, equipo_marca = ?, equipo_modelo = ?,
            equipo_tipo = ?, equipo_descripcion = ?, equipo_precio = ?, equipo_stock = ?,
            equipo_ubicacion = ?, proveedor = ?, numero_serie = ?, observaciones = ?
        WHERE inventario_equipo_id = ?"
    )
    .bind(&request.equipo_codigo)
    .bind(&request.equipo_nombre)
    .bind(&request.equipo_marca)
    .bind(&request.equipo_modelo)
    .bind(&request.equipo_tipo)
    .bind(&request.equipo_descripcion)
    .bind(&request.equipo_precio)
    .bind(&request.equipo_stock.unwrap_or(0))
    .bind(&request.equipo_ubicacion)
    .bind(&request.proveedor)
    .bind(&request.numero_serie)
    .bind(&request.observaciones)
    .bind(equipo_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() > 0 {
        let _ = log_action(
            "UPDATE",
            None,
            "INVENTARIO_EQUIPO",
            Some(equipo_id),
            None,
            Some(&format!("Actualizado equipo: {} - {}", request.equipo_codigo, request.equipo_nombre))
        ).await;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn delete_inventario_equipo(equipo_id: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Primero obtenemos la información del equipo para el log
    let equipo_info = sqlx::query_as::<_, InventarioEquipo>(
        "SELECT * FROM INVENTARIO_EQUIPO WHERE inventario_equipo_id = ?"
    )
    .bind(equipo_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let result = sqlx::query(
        "DELETE FROM INVENTARIO_EQUIPO WHERE inventario_equipo_id = ?"
    )
    .bind(equipo_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() > 0 {
        if let Some(equipo) = equipo_info {
            let _ = log_action(
                "DELETE",
                None,
                "INVENTARIO_EQUIPO",
                Some(equipo_id),
                None,
                Some(&format!("Eliminado equipo: {} - {}", 
                    equipo.equipo_codigo.unwrap_or_default(), 
                    equipo.equipo_nombre.unwrap_or_default()))
            ).await;
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn update_inventario_equipo_stock(equipo_id: i32, cantidad: i32, tipo: String) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Primero obtenemos el stock actual
    let current_stock: i32 = sqlx::query_scalar(
        "SELECT equipo_stock FROM INVENTARIO_EQUIPO WHERE inventario_equipo_id = ?"
    )
    .bind(equipo_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?
    .unwrap_or(0);
    
    let new_stock = match tipo.as_str() {
        "entrada" => current_stock + cantidad,
        "salida" => std::cmp::max(0, current_stock - cantidad),
        _ => return Err("Tipo de operación no válido. Use 'entrada' o 'salida'".to_string()),
    };
    
    let result = sqlx::query(
        "UPDATE INVENTARIO_EQUIPO SET equipo_stock = ? WHERE inventario_equipo_id = ?"
    )
    .bind(new_stock)
    .bind(equipo_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error updating stock: {}", e))?;
    
    if result.rows_affected() > 0 {
        // Log de la operación
        let _ = log_action(
            "UPDATE_STOCK",
            None,
            "INVENTARIO_EQUIPO",
            Some(equipo_id),
            Some(&format!("Stock anterior: {}", current_stock)),
            Some(&format!("Stock nuevo: {} ({})", new_stock, if tipo == "entrada" { format!("+{}", cantidad) } else { format!("-{}", cantidad) }))
        ).await;
        Ok(true)
    } else {
        Ok(false)
    }
}

// ===============================
// STRUCTS Y COMANDOS PARA SALIDAS DE EQUIPOS
// ===============================

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SalidaEquipo {
    pub salida_id: i32,
    pub orden_trabajo_id: i32,
    pub motivo_salida: String,
    pub fecha_salida: Option<DateTime<Utc>>,
    pub usuario_id: Option<i32>,
    pub observaciones: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    // Campos adicionales para JOINs
    pub orden_codigo: Option<String>,
    pub equipo_nombre: Option<String>,
    pub cliente_nombre: Option<String>,
    pub usuario_nombre: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrarSalidaRequest {
    pub orden_trabajo_id: i32,
    pub motivo_salida: String,
    pub observaciones: Option<String>,
    pub usuario_id: i32,
}

/// Registrar salida de equipo en tabla específica (NUEVA IMPLEMENTACIÓN)
#[tauri::command]
pub async fn registrar_salida_equipo_v2(request: RegistrarSalidaRequest) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Verificar que la orden existe y está en estado válido
    let orden_info = sqlx::query(
        "SELECT ot.orden_id, ot.orden_codigo, ot.estado, 
                CONCAT(e.equipo_marca, ' ', e.equipo_modelo) as equipo_nombre, 
                c.cliente_nombre
         FROM ORDEN_TRABAJO ot
         JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         JOIN CLIENTE c ON e.cliente_id = c.cliente_id
         WHERE ot.orden_id = ?"
    )
    .bind(request.orden_trabajo_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error al verificar orden: {}", e))?;
    
    let orden_row = orden_info.ok_or("Orden de trabajo no encontrada")?;
    
    // Extraer los campos usando try_get
    // let orden_id: i32 = orden_row.try_get("orden_id")  // Campo no utilizado
    //     .map_err(|e| format!("Error obteniendo orden_id: {}", e))?;
    let orden_codigo: Option<String> = orden_row.try_get("orden_codigo").ok().flatten();
    let estado: String = orden_row.try_get("estado")
        .map_err(|e| format!("Error obteniendo estado: {}", e))?;
    // let equipo_nombre: Option<String> = orden_row.try_get("equipo_nombre").ok().flatten();  // Campo no utilizado
    // let cliente_nombre: Option<String> = orden_row.try_get("cliente_nombre").ok().flatten();  // Campo no utilizado

    // Crear una estructura para usar en el resto del código
    let orden = OrdenInfoRow {
        orden_codigo,
        estado,
    };
    
    // Estados que permiten salida
    let estados_validos = vec![
        "recibido", "cotizacion_enviada", "aprobacion_pendiente",
        "en_reparacion", "espera_de_retiro", "cotizacion_rechazada"
    ];
    
    if !estados_validos.contains(&orden.estado.as_str()) {
        return Err(format!("No se puede registrar salida. Estado actual: {}", orden.estado));
    }
    
    // Verificar que no hay salida previa registrada
    let salida_existente = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM SALIDA_EQUIPO WHERE orden_trabajo_id = ?"
    )
    .bind(request.orden_trabajo_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al verificar salida existente: {}", e))?;
    
    if salida_existente > 0 {
        return Err("Ya se registró una salida para esta orden de trabajo".to_string());
    }
    
    // Determinar nuevo estado según motivo
    let nuevo_estado = match request.motivo_salida.as_str() {
        "entregado_cliente" => "entregado",
        "retirado_sin_reparacion" | "abandonado" => "abandonado",
        "baja_definitiva" => "equipo_no_reparable",
        _ => return Err("Motivo de salida no válido".to_string()),
    };
    
    // Iniciar transacción
    let mut tx = pool.begin().await.map_err(|e| format!("Error iniciando transacción: {}", e))?;
    
    // Registrar la salida
    let result = sqlx::query(
        "INSERT INTO SALIDA_EQUIPO (orden_trabajo_id, motivo_salida, usuario_id, observaciones)
         VALUES (?, ?, ?, ?)"
    )
    .bind(request.orden_trabajo_id)
    .bind(&request.motivo_salida)
    .bind(request.usuario_id)
    .bind(&request.observaciones)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Error registrando salida: {}", e))?;
    
    let salida_id = result.last_insert_id() as i32;
    
    // Actualizar estado de la orden
    sqlx::query("UPDATE ORDEN_TRABAJO SET estado = ? WHERE orden_id = ?")
        .bind(&nuevo_estado)
        .bind(request.orden_trabajo_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Error actualizando estado orden: {}", e))?;
    
    // Confirmar transacción
    tx.commit().await.map_err(|e| format!("Error confirmando transacción: {}", e))?;
    
    // Log de auditoría
    let _ = log_action(
        "REGISTRAR_SALIDA_EQUIPO",
        Some(request.usuario_id),
        "SALIDA_EQUIPO",
        Some(salida_id),
        None,
        Some(&format!("Salida registrada para orden {} - Motivo: {}", 
            orden.orden_codigo.unwrap_or_default(), 
            request.motivo_salida))
    ).await;
    
    Ok(true)
}

/// Obtener historial completo de salidas
#[tauri::command]
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
#[tauri::command]
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
#[tauri::command]
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
#[tauri::command]
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