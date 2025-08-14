use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_safe;
use crate::commands::logs::log_action;
use chrono::{DateTime, Utc};
use chrono::Datelike;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrdenTrabajo {
    pub orden_id: i32,
    pub orden_codigo: Option<String>,
    pub orden_desc: Option<String>,
    pub prioridad: Option<String>,
    pub estado: Option<String>,
    pub has_garantia: Option<bool>,
    pub equipo_id: Option<i32>,
    pub created_by: Option<i32>,
    pub cotizacion_id: Option<i32>,
    pub informe_id: Option<i32>,
    pub pre_informe: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrdenTrabajoRequest {
    // orden_codigo se genera automáticamente
    pub orden_desc: String,
    pub prioridad: String, // 'baja', 'media', 'alta'
    pub estado: String, // 'pendiente', 'en_proceso', 'completado', 'cancelado'
    pub has_garantia: bool,
    pub equipo_id: i32,
    pub created_by: i32,
    pub pre_informe: String,
    pub cotizacion_id: Option<i32>,
    pub informe_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrdenTrabajoRequest {
    pub orden_codigo: Option<String>,
    pub orden_desc: Option<String>,
    pub prioridad: Option<String>,
    pub estado: Option<String>,
    pub has_garantia: Option<bool>,
    pub equipo_id: Option<i32>,
    pub cotizacion_id: Option<i32>,
    pub informe_id: Option<i32>,
    pub pre_informe: Option<String>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrdenTrabajoDetallada {
    pub orden_id: i32,
    pub orden_codigo: Option<String>,
    pub orden_desc: Option<String>,
    pub prioridad: Option<String>,
    pub estado: Option<String>,
    pub has_garantia: Option<bool>,
    pub equipo_id: Option<i32>,
    pub created_by: Option<i32>,
    pub cotizacion_id: Option<i32>,
    pub informe_id: Option<i32>,
    pub pre_informe: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    // Información del equipo
    pub numero_serie: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: Option<String>,
    // Información del cliente (a través del equipo)
    pub cliente_id: Option<i32>,
    pub cliente_nombre: Option<String>,
    // Información del usuario que creó la orden
    pub creador_nombre: Option<String>,
    // Información de cotización
    pub cotizacion_codigo: Option<String>,
    pub costo_total: Option<i32>,
    // Información de informe
    pub informe_codigo: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Filtros {
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
    pub marcas: Option<Vec<String>>,
    pub modelos: Option<Vec<String>>, 
    pub prioridades: Option<Vec<String>>,
}

/// Obtener todas las órdenes de trabajo
#[tauri::command]
pub async fn get_ordenes_trabajo() -> Result<Vec<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;

    let ordenes = sqlx::query_as::<_, OrdenTrabajo>(
        "SELECT orden_id, orden_codigo, orden_desc, prioridad, estado, has_garantia, 
                equipo_id, created_by, cotizacion_id, informe_id, pre_informe, created_at, finished_at 
         FROM ORDEN_TRABAJO 
         ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(ordenes)
}

/// Obtener una orden de trabajo por ID
#[tauri::command]
pub async fn get_orden_trabajo_by_id(orden_id: i32) -> Result<Option<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;    let orden = sqlx::query_as::<_, OrdenTrabajo>(
        "SELECT orden_id, orden_codigo, orden_desc, prioridad, estado, has_garantia, 
                equipo_id, created_by, cotizacion_id, informe_id, pre_informe, created_at, finished_at 
         FROM ORDEN_TRABAJO 
         WHERE orden_id = ?"
    )
    .bind(orden_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(orden)
}

/// Obtener una orden de trabajo por código
#[tauri::command]
pub async fn get_orden_trabajo_by_codigo(orden_codigo: String) -> Result<Option<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;    let orden = sqlx::query_as::<_, OrdenTrabajo>(
        "SELECT orden_id, orden_codigo, orden_desc, prioridad, estado, has_garantia, 
                equipo_id, created_by, cotizacion_id, informe_id, pre_informe, created_at, finished_at 
         FROM ORDEN_TRABAJO 
         WHERE orden_codigo = ?"
    )
    .bind(orden_codigo)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(orden)
}

/// Obtener órdenes de trabajo por equipo
#[tauri::command]
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
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(ordenes)
}

/// Obtener órdenes de trabajo por estado
#[tauri::command]
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
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(ordenes)
}

/// Obtener órdenes de trabajo por prioridad
#[tauri::command]
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
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(ordenes)
}

/// Obtener órdenes de trabajo creadas por un usuario específico
#[tauri::command]
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
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(ordenes)
}

/// Obtener órdenes de trabajo con información detallada (con JOINs)
#[tauri::command]
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
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(ordenes)
}

/// Obtener orden de trabajo detallada por ID
#[tauri::command]
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
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(orden)
}

/// Crear una nueva orden de trabajo
#[tauri::command]
pub async fn create_orden_trabajo(request: CreateOrdenTrabajoRequest) -> Result<OrdenTrabajo, String> {
    let pool = get_db_pool_safe()?;
    
    // Generar código automático: OT-YYYY-XXX
    let year = chrono::Utc::now().year();
    
    // Buscar el mayor número correlativo existente para el año actual
    let last_codigo: Option<String> = sqlx::query_scalar(
        "SELECT orden_codigo FROM ORDEN_TRABAJO WHERE orden_codigo LIKE ? ORDER BY orden_id DESC LIMIT 1"
    )
    .bind(format!("OT-{}-%", year))
    .fetch_one(pool)
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
    .execute(pool)
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
#[tauri::command]
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
        .execute(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "UPDATE_ORDEN_TRABAJO",
        Some(updated_by),
        "ORDEN_TRABAJO",
        Some(orden_id),
        current_orden.as_ref().and_then(|o| o.orden_codigo.as_deref()),
        request.orden_codigo.as_deref()
    ).await;
    
    // Obtener la orden actualizada
    get_orden_trabajo_by_id(orden_id).await
}

/// Cambiar el estado de una orden de trabajo
#[tauri::command]
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
        "equipo_no_reparable"
    ];
    if !estados_validos.contains(&nuevo_estado.as_str()) {
        return Err("Estado no válido".to_string());
    }    let current_orden = get_orden_trabajo_by_id(orden_id).await?;
    
    // Si el estado es 'entregado', actualizar finished_at
    let query_builder = if nuevo_estado == "entregado" {
        sqlx::query("UPDATE ORDEN_TRABAJO SET estado = ?, finished_at = CURRENT_TIMESTAMP WHERE orden_id = ?")
            .bind(&nuevo_estado)
            .bind(orden_id)
    } else {
        sqlx::query("UPDATE ORDEN_TRABAJO SET estado = ? WHERE orden_id = ?")
            .bind(&nuevo_estado)
            .bind(orden_id)
    };
    
    query_builder
        .execute(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "CHANGE_ORDER_STATUS",
        Some(updated_by),
        "ORDEN_TRABAJO",
        Some(orden_id),
        current_orden.as_ref().and_then(|o| o.estado.as_deref()),
        Some(&nuevo_estado)
    ).await;
    
    get_orden_trabajo_by_id(orden_id).await
}

/// Asignar cotización a una orden de trabajo
#[tauri::command]
pub async fn asignar_cotizacion_orden_trabajo(orden_id: i32, cotizacion_id: i32, updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;
    
    sqlx::query("UPDATE ORDEN_TRABAJO SET cotizacion_id = ? WHERE orden_id = ?")
        .bind(cotizacion_id)
        .bind(orden_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "ASSIGN_COTIZACION",
        Some(updated_by),
        "ORDEN_TRABAJO",
        Some(orden_id),
        None,
        Some(&format!("Cotización {} asignada", cotizacion_id))
    ).await;
    
    get_orden_trabajo_by_id(orden_id).await
}

/// Asignar informe a una orden de trabajo
#[tauri::command]
pub async fn asignar_informe_orden_trabajo(orden_id: i32, informe_id: i32, updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;
    
    sqlx::query("UPDATE ORDEN_TRABAJO SET informe_id = ? WHERE orden_id = ?")
        .bind(informe_id)
        .bind(orden_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "ASSIGN_INFORME",
        Some(updated_by),
        "ORDEN_TRABAJO",
        Some(orden_id),
        None,
        Some(&format!("Informe {} asignado", informe_id))
    ).await;
    
    get_orden_trabajo_by_id(orden_id).await
}

/// Eliminar una orden de trabajo
#[tauri::command]
pub async fn delete_orden_trabajo(orden_id: i32, deleted_by: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener información de la orden antes de eliminarla
    let orden = get_orden_trabajo_by_id(orden_id).await?;
    
    let result = sqlx::query("DELETE FROM ORDEN_TRABAJO WHERE orden_id = ?")
        .bind(orden_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() > 0 {
        // Registrar la acción en el log de auditoría
        let orden_info = orden.as_ref()
            .map(|o| format!("{} - {}", 
                o.orden_codigo.as_deref().unwrap_or("N/A"), 
                o.orden_desc.as_deref().unwrap_or("N/A")))
            .unwrap_or_else(|| "Orden eliminada".to_string());
            
        let _ = log_action(
            "DELETE_ORDEN_TRABAJO",
            Some(deleted_by),
            "ORDEN_TRABAJO",
            Some(orden_id),
            Some(&orden_info),
            None
        ).await;
        
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Obtener estadísticas de órdenes de trabajo
#[tauri::command]
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
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    // Contar órdenes por prioridad
    let stats_prioridad: Vec<CountByField> = sqlx::query_as(
        "SELECT NULL as estado, prioridad, COUNT(*) as count FROM ORDEN_TRABAJO GROUP BY prioridad"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    // Total de órdenes
    let total: CountResult = sqlx::query_as("SELECT COUNT(*) as count FROM ORDEN_TRABAJO")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Órdenes con garantía
    let con_garantia: CountResult = sqlx::query_as("SELECT COUNT(*) as count FROM ORDEN_TRABAJO WHERE has_garantia = TRUE")
        .fetch_one(pool)
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

/// Buscar órdenes de trabajo por texto
#[tauri::command]
pub async fn search_ordenes_trabajo(search_term: String) -> Result<Vec<OrdenTrabajoDetallada>, String> {
    let pool = get_db_pool_safe()?;
    let search_pattern = format!("%{}%", search_term);
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
         WHERE ot.orden_codigo LIKE ? 
            OR ot.orden_desc LIKE ? 
            OR e.numero_serie LIKE ?
            OR c.cliente_nombre LIKE ?
         ORDER BY ot.created_at DESC"
    )
    .bind(&search_pattern)
    .bind(&search_pattern)
    .bind(&search_pattern)
    .bind(&search_pattern)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(ordenes)
}

/// Enviar notificación de orden de trabajo por email
#[tauri::command]
pub async fn send_orden_trabajo_notification(orden_id: i32, sent_by: i32) -> Result<bool, String> {
    use crate::email::EmailService;
    use crate::commands::equipos::get_equipo_by_id;
    
    // Obtener la orden de trabajo
    let orden_trabajo = get_orden_trabajo_by_id(orden_id).await?
        .ok_or_else(|| "Orden de trabajo no encontrada".to_string())?;
    
    // Obtener información del equipo
    let equipo = get_equipo_by_id(orden_trabajo.equipo_id.unwrap_or(0)).await?
        .ok_or_else(|| "Equipo no encontrado".to_string())?;
    
    // Obtener información del cliente
    let pool = get_db_pool_safe()?;
    let cliente_nombre = sqlx::query_scalar::<_, String>(
        "SELECT cliente_nombre FROM CLIENTE WHERE cliente_id = ?"
    )
    .bind(equipo.cliente_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?
    .unwrap_or_else(|| "Cliente no encontrado".to_string());
    
    // Crear el servicio de email
    let email_service = EmailService::new()
        .map_err(|e| format!("Error inicializando servicio de email: {}", e))?;
    
    // Enviar el email
    email_service.send_orden_trabajo_notification(
        &orden_trabajo,
        &equipo,
        &cliente_nombre,
    ).await
    .map_err(|e| format!("Error enviando email: {}", e))?;
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "SEND_ORDEN_NOTIFICATION",
        Some(sent_by),
        "ORDEN_TRABAJO",
        Some(orden_id),
        None,
        Some(&format!("Notificación de orden {} enviada a benitez.basti0@gmail.com", 
            orden_trabajo.orden_codigo.as_deref().unwrap_or("N/A")
        ))
    ).await;
    
    Ok(true)
}

/// Obtener orden de trabajo por informe_id
#[tauri::command]
pub async fn get_orden_trabajo_by_informe_id(informe_id: i32) -> Result<Option<OrdenTrabajo>, String> {
    let pool = get_db_pool_safe()?;
    
    let orden = sqlx::query_as::<_, OrdenTrabajo>(
        "SELECT orden_id, orden_codigo, orden_desc, prioridad, estado, 
                has_garantia, equipo_id, cotizacion_id, informe_id, 
                created_by, created_at 
         FROM ORDEN_TRABAJO 
         WHERE informe_id = ?"
    )
    .bind(informe_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(orden)
}
//Fltros Unificados
/// Obtener órdenes de trabajo con filtros unificados
#[tauri::command]
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

    if let Some(fecha_inicio) = filtros.fecha_inicio {
        query.push_str(" AND date(ot.created_at) >= date(?)");
        params.push(fecha_inicio);
    }
    
    if let Some(fecha_fin) = filtros.fecha_fin {
        query.push_str(" AND date(ot.created_at) <= date(?)");
        params.push(fecha_fin);
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
    
    query.push_str(" ORDER BY ot.created_at DESC");

    let mut sqlx_query = sqlx::query_as::<_, OrdenTrabajoDetallada>(&query);

    for param in &params {
        sqlx_query = sqlx_query.bind(param);
    }

    sqlx_query
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
}
#[tauri::command]
pub async fn get_modelos_disponibles() -> Result<Vec<String>, String> {
    let pool = get_db_pool_safe()?;
    
    let modelos = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT equipo_modelo FROM EQUIPO WHERE equipo_modelo IS NOT NULL AND equipo_modelo != '' ORDER BY equipo_modelo"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(modelos)
}
#[tauri::command]
pub async fn get_marcas_disponibles() -> Result<Vec<String>, String> {
    let pool = get_db_pool_safe()?;
    
    let marcas = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT equipo_marca FROM EQUIPO WHERE equipo_marca IS NOT NULL AND equipo_marca != '' ORDER BY equipo_marca"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(marcas)
}
