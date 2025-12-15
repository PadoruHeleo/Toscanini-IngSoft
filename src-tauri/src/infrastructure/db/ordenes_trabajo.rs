use chrono::{Datelike};

use crate::database::get_db_pool_safe;
use crate::infrastructure::db::logs::log_action;


// --- IMPORTACIÓN DE MODELOS CENTRALIZADOS ---
// Aquí es donde ocurre la magia. Usamos el modelo compartido.
use crate::models::ordenes_trabajo::{
    OrdenTrabajo,
    CreateOrdenTrabajoRequest,
    UpdateOrdenTrabajoRequest,
    OrdenTrabajoDetallada,
    Filtros
};

/// Obtener todas las órdenes de trabajo

pub async fn get_ordenes_trabajo() -> Result<Vec<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;

    let ordenes = sqlx::query_as::<_, OrdenTrabajo>(
        "SELECT orden_id, orden_codigo, orden_desc, prioridad, estado, has_garantia, 
                equipo_id, created_by, cotizacion_id, informe_id, pre_informe, created_at, finished_at,
                NULL as cliente_id
         FROM ORDEN_TRABAJO 
         ORDER BY created_at DESC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(ordenes)
}

/// Obtener una orden de trabajo por ID

pub async fn get_orden_trabajo_by_id(orden_id: i32) -> Result<Option<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;    let orden = sqlx::query_as::<_, OrdenTrabajo>(
        "SELECT orden_id, orden_codigo, orden_desc, prioridad, estado, has_garantia, 
                equipo_id, created_by, cotizacion_id, informe_id, pre_informe, created_at, finished_at 
         FROM ORDEN_TRABAJO 
         WHERE orden_id = ?"
    )
    .bind(orden_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(orden)
}

/// Obtener una orden de trabajo por código

pub async fn get_orden_trabajo_by_codigo(orden_codigo: String) -> Result<Option<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;    let orden = sqlx::query_as::<_, OrdenTrabajo>(
        "SELECT orden_id, orden_codigo, orden_desc, prioridad, estado, has_garantia, 
                equipo_id, created_by, cotizacion_id, informe_id, pre_informe, created_at, finished_at 
         FROM ORDEN_TRABAJO 
         WHERE orden_codigo = ?"
    )
    .bind(orden_codigo)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(orden)
}

/// Obtener órdenes de trabajo por equipo

pub async fn get_ordenes_trabajo_by_equipo(equipo_id: i32) -> Result<Vec<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;
    let ordenes = sqlx::query_as::<_, OrdenTrabajo>(
        "SELECT orden_id, orden_codigo, orden_desc, prioridad, estado, has_garantia, 
                equipo_id, created_by, cotizacion_id, informe_id, pre_informe, created_at, finished_at 
         FROM ORDEN_TRABAJO 
         WHERE equipo_id = ?
         ORDER BY created_at DESC"
    )
    .bind(equipo_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(ordenes)
}

/// Obtener órdenes de trabajo por estado

pub async fn get_ordenes_trabajo_by_estado(estado: String) -> Result<Vec<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;
    let ordenes = sqlx::query_as::<_, OrdenTrabajo>(
        "SELECT orden_id, orden_codigo, orden_desc, prioridad, estado, has_garantia, 
                equipo_id, created_by, cotizacion_id, informe_id, pre_informe, created_at, finished_at 
         FROM ORDEN_TRABAJO 
         WHERE estado = ?
         ORDER BY created_at DESC"
    )
    .bind(estado)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(ordenes)
}

/// Obtener órdenes de trabajo por prioridad

pub async fn get_ordenes_trabajo_by_prioridad(prioridad: String) -> Result<Vec<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;
    let ordenes = sqlx::query_as::<_, OrdenTrabajo>(
        "SELECT orden_id, orden_codigo, orden_desc, prioridad, estado, has_garantia, 
                equipo_id, created_by, cotizacion_id, informe_id, pre_informe, created_at, finished_at 
         FROM ORDEN_TRABAJO 
         WHERE prioridad = ?
         ORDER BY created_at DESC"
    )
    .bind(prioridad)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(ordenes)
}

/// Obtener órdenes de trabajo creadas por un usuario específico

pub async fn get_ordenes_trabajo_by_usuario(usuario_id: i32) -> Result<Vec<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;
    let ordenes = sqlx::query_as::<_, OrdenTrabajo>(
        "SELECT orden_id, orden_codigo, orden_desc, prioridad, estado, has_garantia, 
                equipo_id, created_by, cotizacion_id, informe_id, pre_informe, created_at, finished_at 
         FROM ORDEN_TRABAJO 
         WHERE created_by = ?
         ORDER BY created_at DESC"
    )
    .bind(usuario_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(ordenes)
}

/// Obtener órdenes de trabajo con información detallada (con JOINs)

pub async fn get_ordenes_trabajo_detalladas() -> Result<Vec<OrdenTrabajoDetallada>, String> {
    let pool = get_db_pool_safe()?;

    let ordenes = sqlx::query_as::<_, OrdenTrabajoDetallada>(
        "SELECT 
            ot.orden_id, ot.orden_codigo, ot.orden_desc, ot.prioridad, ot.estado, 
            ot.has_garantia, ot.equipo_id, ot.created_by, ot.cotizacion_id, ot.informe_id, 
            ot.pre_informe, ot.created_at, ot.finished_at,
            e.numero_serie, e.equipo_marca, e.equipo_modelo, e.equipo_tipo,
            c.cliente_id, c.cliente_nombre,
            u.usuario_nombre as creador_nombre,
            cot.cotizacion_codigo, cot.costo_total,
            inf.informe_codigo
         FROM ORDEN_TRABAJO ot
         LEFT JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         LEFT JOIN CLIENTE c ON e.cliente_id = c.cliente_id
         LEFT JOIN USUARIO u ON ot.created_by = u.usuario_id
         LEFT JOIN COTIZACION cot ON ot.cotizacion_id = cot.cotizacion_id
         LEFT JOIN INFORME inf ON ot.informe_id = inf.informe_id
         ORDER BY ot.created_at DESC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(ordenes)
}

/// Obtener orden de trabajo detallada por ID

pub async fn get_orden_trabajo_detallada_by_id(orden_id: i32) -> Result<Option<OrdenTrabajoDetallada>, String> {
    let pool = get_db_pool_safe()?;    let orden = sqlx::query_as::<_, OrdenTrabajoDetallada>(
        "SELECT 
            ot.orden_id, ot.orden_codigo, ot.orden_desc, ot.prioridad, ot.estado, 
            ot.has_garantia, ot.equipo_id, ot.created_by, ot.cotizacion_id, ot.informe_id, 
            ot.pre_informe, ot.created_at, ot.finished_at,
            e.numero_serie, e.equipo_marca, e.equipo_modelo, e.equipo_tipo,
            c.cliente_id, c.cliente_nombre,
            u.usuario_nombre as creador_nombre,
            cot.cotizacion_codigo, cot.costo_total,
            inf.informe_codigo
         FROM ORDEN_TRABAJO ot
         LEFT JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         LEFT JOIN CLIENTE c ON e.cliente_id = c.cliente_id
         LEFT JOIN USUARIO u ON ot.created_by = u.usuario_id
         LEFT JOIN COTIZACION cot ON ot.cotizacion_id = cot.cotizacion_id
         LEFT JOIN INFORME inf ON ot.informe_id = inf.informe_id
         WHERE ot.orden_id = ?"
    )
    .bind(orden_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(orden)
}

/// Crear una nueva orden de trabajo

pub async fn create_orden_trabajo(request: CreateOrdenTrabajoRequest) -> Result<OrdenTrabajo, String> {
    let pool = get_db_pool_safe()?;
    
    // Generar código automático: OT-YYYY-XXX
    let year = chrono::Utc::now().year();
    
    // Buscar el mayor número correlativo existente para el año actual
    let last_codigo: Option<String> = sqlx::query_scalar(
        "SELECT orden_codigo FROM ORDEN_TRABAJO WHERE orden_codigo LIKE ? ORDER BY orden_id DESC LIMIT 1"
    )
    .bind(format!("OT-{}-%", year))
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
    
    let codigo = format!("OT-{}-{:03}", year, next_number);
    
    let result = sqlx::query(
        "INSERT INTO ORDEN_TRABAJO (orden_codigo, orden_desc, prioridad, estado, has_garantia, 
                                   equipo_id, created_by, cotizacion_id, informe_id, pre_informe) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&codigo)
    .bind(&request.orden_desc)
    .bind(&request.prioridad)
    .bind(&request.estado)
    .bind(request.has_garantia)
    .bind(request.equipo_id)
    .bind(request.created_by)
    .bind(request.cotizacion_id)
    .bind(request.informe_id)
    .bind(&request.pre_informe)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let orden_id = result.last_insert_id() as i32;
      // Registrar la acción en el log de auditoría
    let _ = log_action(
        "CREATE_ORDEN_TRABAJO",
        Some(request.created_by),
        "ORDEN_TRABAJO",
        Some(orden_id),
        None,
        Some(&format!("Orden de trabajo creada: {}", codigo))
    ).await;
    
    // Enviar notificación automática por email
    let _ = send_orden_trabajo_notification(orden_id, request.created_by).await;
    
    // Obtener la orden recién creada
    get_orden_trabajo_by_id(orden_id)
        .await?
        .ok_or_else(|| "Failed to retrieve created orden de trabajo".to_string())
}

/// Actualizar una orden de trabajo

pub async fn update_orden_trabajo(orden_id: i32, request: UpdateOrdenTrabajoRequest, updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener la orden actual para logging
    let current_orden = get_orden_trabajo_by_id(orden_id).await?;
    
    let mut query_parts = Vec::new();
    let mut bindings = Vec::new();
    
    if let Some(orden_codigo) = &request.orden_codigo {
        query_parts.push("orden_codigo = ?");
        bindings.push(orden_codigo.clone());
    }
    
    if let Some(orden_desc) = &request.orden_desc {
        query_parts.push("orden_desc = ?");
        bindings.push(orden_desc.clone());
    }
    
    if let Some(prioridad) = &request.prioridad {
        query_parts.push("prioridad = ?");
        bindings.push(prioridad.clone());
    }
      if let Some(estado) = &request.estado {
        query_parts.push("estado = ?");
        bindings.push(estado.clone());
        
        // Si el estado es 'entregado', actualizar finished_at
        if estado == "entregado" {
            query_parts.push("finished_at = CURRENT_TIMESTAMP");
        }
    }
    
    if let Some(has_garantia) = request.has_garantia {
        query_parts.push("has_garantia = ?");
        bindings.push(has_garantia.to_string());
    }
    
    if let Some(equipo_id) = request.equipo_id {
        query_parts.push("equipo_id = ?");
        bindings.push(equipo_id.to_string());
    }
    
    if let Some(cotizacion_id) = request.cotizacion_id {
        query_parts.push("cotizacion_id = ?");
        bindings.push(cotizacion_id.to_string());
    }
      if let Some(informe_id) = request.informe_id {
        query_parts.push("informe_id = ?");
        bindings.push(informe_id.to_string());
    }
    
    if let Some(pre_informe) = &request.pre_informe {
        query_parts.push("pre_informe = ?");
        bindings.push(pre_informe.clone());
    }
    
    if let Some(finished_at) = request.finished_at {
        query_parts.push("finished_at = ?");
        bindings.push(finished_at.to_string());
    }
    
    if query_parts.is_empty() {
        return Ok(current_orden);
    }
    
    let query = format!("UPDATE ORDEN_TRABAJO SET {} WHERE orden_id = ?", query_parts.join(", "));
    
    let mut query_builder = sqlx::query(&query);
    for binding in bindings {
        query_builder = query_builder.bind(binding);
    }
    query_builder = query_builder.bind(orden_id);
    
    query_builder
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    // Crear mensaje descriptivo de los cambios realizados
    let mut cambios = Vec::new();
    
    if let Some(prioridad) = &request.prioridad {
        cambios.push(format!("prioridad: {}", prioridad));
    }
    if let Some(estado) = &request.estado {
        cambios.push(format!("estado: {}", estado));
    }
    if let Some(orden_desc) = &request.orden_desc {
        cambios.push(format!("descripción: {}", orden_desc));
    }
    if let Some(has_garantia) = request.has_garantia {
        cambios.push(format!("garantía: {}", if has_garantia { "sí" } else { "no" }));
    }
    if let Some(pre_informe) = &request.pre_informe {
        cambios.push(format!("pre-informe: {}", pre_informe));
    }
    
    let mensaje_cambios = if cambios.is_empty() {
        "Sin cambios específicos".to_string()
    } else {
        cambios.join(", ")
    };

    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "UPDATE_ORDEN_TRABAJO",
        Some(updated_by),
        "ORDEN_TRABAJO",
        Some(orden_id),
        current_orden.as_ref().and_then(|o| o.orden_codigo.as_deref()),
        Some(&mensaje_cambios)
    ).await;

    // Obtener la orden actualizada
    get_orden_trabajo_by_id(orden_id).await
}

pub async fn cambiar_estado_orden_trabajo(orden_id: i32, nuevo_estado: String, updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;
    
    // Validar que el estado sea válido
    let estados_validos = vec![
        "recibido",
        "cotizacion_enviada", 
        "aprobacion_pendiente",
        "en_reparacion",
        "espera_de_retiro",
        "entregado",
        "abandonado",
        "equipo_no_reparable",
        "cotizacion_rechazada"
    ];
    if !estados_validos.contains(&nuevo_estado.as_str()) {
        return Err("Estado no válido".to_string());
    }
    
    let current_orden = get_orden_trabajo_by_id(orden_id).await?;
    
    // NUEVA VALIDACIÓN: Si se intenta cambiar a "en_reparacion", validar que la cotización esté aprobada
    if nuevo_estado == "en_reparacion" {
        if let Some(ref orden) = current_orden {
            // Verificar que el estado actual permita este cambio
            let estado_actual = orden.estado.as_deref().unwrap_or("");
            if estado_actual != "cotizacion_enviada" && estado_actual != "aprobacion_pendiente" {
                return Err(format!(
                    "No se puede cambiar a 'en_reparacion' desde el estado '{}'. Solo se permite desde 'cotizacion_enviada' o 'aprobacion_pendiente'.",
                    estado_actual
                ));
            }
            
            // Verificar que existe una cotización asociada
            if let Some(cotizacion_id) = orden.cotizacion_id {
                // Verificar que la cotización esté aprobada
                let cotizacion_aprobada: Option<bool> = sqlx::query_scalar(
                    "SELECT is_aprobada FROM COTIZACION WHERE cotizacion_id = ?"
                )
                .bind(cotizacion_id)
                .fetch_optional(&*pool)
                .await
                .map_err(|e| format!("Database error: {}", e))?;

                if !cotizacion_aprobada.unwrap_or(false) {
                     return Err("La cotización asociada no está aprobada. No se puede iniciar la reparación.".to_string());
                }
            }
        }
    }

    let mut query = "UPDATE ORDEN_TRABAJO SET estado = ?".to_string();
    if nuevo_estado == "entregado" {
        query.push_str(", finished_at = CURRENT_TIMESTAMP");
    }
    query.push_str(" WHERE orden_id = ?");

    sqlx::query(&query)
        .bind(&nuevo_estado)
        .bind(orden_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "CHANGE_STATUS_ORDEN_TRABAJO",
        Some(updated_by),
        "ORDEN_TRABAJO",
        Some(orden_id),
        current_orden.as_ref().and_then(|o| o.orden_codigo.as_deref()),
        Some(&format!("Estado cambiado a: {}", nuevo_estado))
    ).await;

    get_orden_trabajo_by_id(orden_id).await
}

pub async fn delete_orden_trabajo(orden_id: i32, deleted_by: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener información de la orden antes de eliminarla
    let orden = get_orden_trabajo_by_id(orden_id).await?;
    
    let result = sqlx::query("DELETE FROM ORDEN_TRABAJO WHERE orden_id = ?")
        .bind(orden_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() > 0 {
         let _ = log_action(
            "DELETE_ORDEN_TRABAJO",
            Some(deleted_by),
            "ORDEN_TRABAJO",
            Some(orden_id),
            orden.as_ref().and_then(|o| o.orden_codigo.as_deref()),
            Some("Orden de trabajo eliminada")
        ).await;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn get_ordenes_trabajo_stats() -> Result<serde_json::Value, String> {
    let pool = get_db_pool_safe()?;

    // Estructura para mapear resultados
    #[derive(Debug, sqlx::FromRow)]
    struct CountByField {
        estado: Option<String>,
        prioridad: Option<String>,
        count: i64,
    }
    
    #[derive(Debug, sqlx::FromRow)]
    struct CountResult {
        count: i64,
    }
    
    // Contar órdenes por estado
    let stats_estado: Vec<CountByField> = sqlx::query_as(
        "SELECT estado, NULL as prioridad, COUNT(*) as count FROM ORDEN_TRABAJO GROUP BY estado"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    // Contar órdenes por prioridad
    let stats_prioridad: Vec<CountByField> = sqlx::query_as(
        "SELECT NULL as estado, prioridad, COUNT(*) as count FROM ORDEN_TRABAJO GROUP BY prioridad"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    // Total de órdenes
    let total: CountResult = sqlx::query_as("SELECT COUNT(*) as count FROM ORDEN_TRABAJO")
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Órdenes con garantía
    let con_garantia: CountResult = sqlx::query_as("SELECT COUNT(*) as count FROM ORDEN_TRABAJO WHERE has_garantia = TRUE")
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
        
    let stats = serde_json::json!({
        "total": total.count,
        "con_garantia": con_garantia.count,
        "por_estado": stats_estado.into_iter().map(|r| serde_json::json!({
            "estado": r.estado,
            "count": r.count
        })).collect::<Vec<_>>(),
        "por_prioridad": stats_prioridad.into_iter().map(|r| serde_json::json!({
            "prioridad": r.prioridad,
            "count": r.count
        })).collect::<Vec<_>>()
    });
    
    Ok(stats)
}

pub async fn get_ordenes_trabajo_filtradas(filtros: Filtros) -> Result<Vec<OrdenTrabajoDetallada>, String> {
    let pool = get_db_pool_safe()?;
    
    let mut query = String::from(
        "SELECT 
            ot.orden_id, ot.orden_codigo, ot.orden_desc, ot.prioridad, ot.estado, 
            ot.has_garantia, ot.equipo_id, ot.created_by, ot.cotizacion_id, ot.informe_id, 
            ot.pre_informe, ot.created_at, ot.finished_at,
            e.numero_serie, e.equipo_marca, e.equipo_modelo, e.equipo_tipo,
            c.cliente_id, c.cliente_nombre,
            u.usuario_nombre as creador_nombre,
            cot.cotizacion_codigo, cot.costo_total,
            inf.informe_codigo
         FROM ORDEN_TRABAJO ot
         LEFT JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         LEFT JOIN CLIENTE c ON e.cliente_id = c.cliente_id
         LEFT JOIN USUARIO u ON ot.created_by = u.usuario_id
         LEFT JOIN COTIZACION cot ON ot.cotizacion_id = cot.cotizacion_id
         LEFT JOIN INFORME inf ON ot.informe_id = inf.informe_id
         WHERE 1=1"
    );
    
    let mut params: Vec<String> = Vec::new();
    
    if let Some(search) = filtros.search {
        if !search.is_empty() {
            query.push_str(" AND (ot.orden_codigo LIKE ? OR c.cliente_nombre LIKE ? OR e.equipo_modelo LIKE ?)");
            let search_term = format!("%{}%", search);
            params.push(search_term.clone());
            params.push(search_term.clone());
            params.push(search_term.clone());
        }
    }
    
    if let Some(marcas) = filtros.marcas {
        if !marcas.is_empty() {
            let placeholders = vec!["?"; marcas.len()].join(",");
            query.push_str(&format!(" AND e.equipo_marca IN ({})", placeholders));
            for marca in marcas {
                params.push(marca);
            }
        }
    }
    
    if let Some(modelos) = filtros.modelos {
        if !modelos.is_empty() {
            let placeholders = vec!["?"; modelos.len()].join(",");
            query.push_str(&format!(" AND e.equipo_modelo IN ({})", placeholders));
            for modelo in modelos {
                params.push(modelo);
            }
        }
    }
    
    if let Some(prioridades) = filtros.prioridades {
        if !prioridades.is_empty() {
            let placeholders = vec!["?"; prioridades.len()].join(",");
            query.push_str(&format!(" AND LOWER(ot.prioridad) IN ({})", placeholders));
            for p in prioridades {
                params.push(p.to_lowercase());
            }
        }
    }
    
    // Nuevo filtro por clientes
    if let Some(clientes) = filtros.clientes {
        if !clientes.is_empty() {
            let placeholders = vec!["?"; clientes.len()].join(",");
            query.push_str(&format!(" AND c.cliente_nombre IN ({})", placeholders));
            for cliente in clientes {
                params.push(cliente);
            }
        }
    }
    
    // Nuevo filtro por estados
    if let Some(estados) = filtros.estados {
        if !estados.is_empty() {
            let placeholders = vec!["?"; estados.len()].join(",");
            query.push_str(&format!(" AND LOWER(ot.estado) IN ({})", placeholders));
            for estado in estados {
                params.push(estado.to_lowercase());
            }
        }
    }
    
    query.push_str(" ORDER BY ot.created_at DESC");

    let mut sqlx_query = sqlx::query_as::<_, OrdenTrabajoDetallada>(&query);

    for param in &params {
        sqlx_query = sqlx_query.bind(param);
    }

    sqlx_query
        .fetch_all(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
}

pub async fn get_modelos_disponibles() -> Result<Vec<String>, String> {
    let pool = get_db_pool_safe()?;
    
    let modelos = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT equipo_modelo FROM EQUIPO WHERE equipo_modelo IS NOT NULL AND equipo_modelo != '' ORDER BY equipo_modelo"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(modelos)
}

pub async fn get_marcas_disponibles() -> Result<Vec<String>, String> {
    let pool = get_db_pool_safe()?;
    
    let marcas = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT equipo_marca FROM EQUIPO WHERE equipo_marca IS NOT NULL AND equipo_marca != '' ORDER BY equipo_marca"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(marcas)
}

pub async fn get_clientes_disponibles() -> Result<Vec<String>, String> {
    let pool = get_db_pool_safe()?;
    
    let clientes = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT c.cliente_nombre 
         FROM CLIENTE c
         INNER JOIN EQUIPO e ON c.cliente_id = e.cliente_id
         INNER JOIN ORDEN_TRABAJO ot ON e.equipo_id = ot.equipo_id
         WHERE c.cliente_nombre IS NOT NULL AND c.cliente_nombre != '' 
         ORDER BY c.cliente_nombre"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(clientes)
}


pub async fn get_ordenes_trabajo_by_cliente(cliente_id: i32) -> Result<Vec<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;
    let ordenes = sqlx::query_as::<_, OrdenTrabajo>(
        "SELECT orden_id, orden_codigo, orden_desc, prioridad, estado, has_garantia, 
                equipo_id, created_by, cotizacion_id, informe_id, pre_informe, created_at, finished_at
         FROM ORDEN_TRABAJO
         WHERE equipo_id IN (SELECT equipo_id FROM EQUIPO WHERE cliente_id = ?)
         ORDER BY created_at DESC"
    )
    .bind(cliente_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(ordenes)
}


pub async fn remove_cotizacion_from_ordenes(cotizacion_id: i32, updated_by: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query("UPDATE ORDEN_TRABAJO SET cotizacion_id = NULL WHERE cotizacion_id = ?")
        .bind(cotizacion_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() > 0 {
        // Registrar la acción en el log de auditoría
        let _ = log_action(
            "REMOVE_COTIZACION_FROM_ORDENES",
            Some(updated_by),
            "ORDEN_TRABAJO",
            None,
            Some(&format!("Cotización {} removida de órdenes de trabajo", cotizacion_id)),
            None
        ).await;
        
        Ok(true)
    } else {
        Ok(false)
    }
}


pub async fn remove_informe_from_ordenes(informe_id: i32, updated_by: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query("UPDATE ORDEN_TRABAJO SET informe_id = NULL WHERE informe_id = ?")
        .bind(informe_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() > 0 {
        // Registrar la acción en el log de auditoría
        let _ = log_action(
            "REMOVE_INFORME_FROM_ORDENES",
            Some(updated_by),
            "ORDEN_TRABAJO",
            None,
            Some(&format!("Informe {} removido de órdenes de trabajo", informe_id)),
            None
        ).await;
        
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn send_orden_trabajo_notification(_orden_id: i32, _sent_by: i32) -> Result<bool, String> {
    // TODO: Implementar envío de notificaciones
    Ok(true)
}

pub async fn get_orden_trabajo_by_informe_id(informe_id: i32) -> Result<Option<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;
    let orden = sqlx::query_as::<_, OrdenTrabajo>(
        "SELECT orden_id, orden_codigo, orden_desc, prioridad, estado, has_garantia, 
                equipo_id, created_by, cotizacion_id, informe_id, pre_informe, created_at, finished_at 
         FROM ORDEN_TRABAJO 
         WHERE informe_id = ?"
    )
    .bind(informe_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(orden)
}
pub async fn asignar_cotizacion_orden_trabajo(orden_id: i32, cotizacion_id: i32, updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;
    
    // Verificar si la orden existe
    let current_orden = get_orden_trabajo_by_id(orden_id).await?;
    if current_orden.is_none() {
        return Ok(None);
    }

    let result = sqlx::query("UPDATE ORDEN_TRABAJO SET cotizacion_id = ? WHERE orden_id = ?")
        .bind(cotizacion_id)
        .bind(orden_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    if result.rows_affected() > 0 {
        let _ = log_action(
            "ASSIGN_COTIZACION_ORDEN_TRABAJO",
            Some(updated_by),
            "ORDEN_TRABAJO",
            Some(orden_id),
            current_orden.as_ref().and_then(|o| o.orden_codigo.as_deref()),
            Some(&format!("Cotización {} asignada a la orden", cotizacion_id))
        ).await;
        
        get_orden_trabajo_by_id(orden_id).await
    } else {
        Ok(None)
    }
}

pub async fn asignar_informe_orden_trabajo(orden_id: i32, informe_id: i32, updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;
    
    // Verificar si la orden existe
    let current_orden = get_orden_trabajo_by_id(orden_id).await?;
    if current_orden.is_none() {
        return Ok(None);
    }

    let result = sqlx::query("UPDATE ORDEN_TRABAJO SET informe_id = ? WHERE orden_id = ?")
        .bind(informe_id)
        .bind(orden_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    if result.rows_affected() > 0 {
        let _ = log_action(
            "ASSIGN_INFORME_ORDEN_TRABAJO",
            Some(updated_by),
            "ORDEN_TRABAJO",
            Some(orden_id),
            current_orden.as_ref().and_then(|o| o.orden_codigo.as_deref()),
            Some(&format!("Informe {} asignado a la orden", informe_id))
        ).await;
        
        get_orden_trabajo_by_id(orden_id).await
    } else {
        Ok(None)
    }
}

pub async fn search_ordenes_trabajo(search_term: String) -> Result<Vec<OrdenTrabajoDetallada>, String> {
    let filtros = Filtros {
        search: Some(search_term),
        fecha_inicio: None,
        fecha_fin: None,
        marcas: None,
        modelos: None,
        prioridades: None,
        clientes: None,
        estados: None,
    };
    
    get_ordenes_trabajo_filtradas(filtros).await
}
