use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_safe;
use crate::commands::logs::log_action;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Equipo {
    pub equipo_id: i32,
    pub numero_serie: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_ubicacion: Option<String>,
    pub cliente_id: Option<i32>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEquipoRequest {
    pub numero_serie: String,
    pub equipo_marca: String,
    pub equipo_modelo: String,
    pub equipo_tipo: String, // 'radio', 'antena', 'repetidor', 'otro'
    pub equipo_precio: Option<i32>,
    pub equipo_ubicacion: Option<String>,
    pub cliente_id: i32,
    pub created_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEquipoRequest {
    pub numero_serie: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_ubicacion: Option<String>,
    pub cliente_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrarSalidaRequest {
    pub equipo_id: i32,
    pub orden_trabajo_id: Option<i32>,
    pub motivo_salida: String, // 'entregado_cliente', 'retirado_sin_reparacion', 'abandonado', 'baja_definitiva'
    pub observaciones: Option<String>,
    pub usuario_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SalidaEquipoResponse {
    pub success: bool,
    pub message: String,
    pub nuevo_estado: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EquipoConEstado {
    pub equipo_id: i32,
    pub numero_serie: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_ubicacion: Option<String>,
    pub cliente_id: Option<i32>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    // Información del cliente
    pub cliente_nombre: Option<String>,
    // Estado de la última orden de trabajo
    pub ultimo_estado_orden: Option<String>,
    pub ultimo_codigo_orden: Option<String>,
    pub fecha_ultima_orden: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct FiltrosEquipos {
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
    pub marcas: Option<Vec<String>>,
    pub modelos: Option<Vec<String>>,
    pub tipos: Option<Vec<String>>,
    pub clientes: Option<Vec<String>>,
    pub ubicaciones: Option<Vec<String>>,
    pub estados_orden: Option<Vec<String>>, // Estados de órdenes de trabajo
    pub search: Option<String>,
    pub ordenamiento: Option<String>,
    pub precio_min: Option<i32>,
    pub precio_max: Option<i32>,
}

/// Obtener todos los equipos
#[tauri::command]
pub async fn get_equipos() -> Result<Vec<Equipo>, String> {
    let pool = get_db_pool_safe()?;
    let equipos = sqlx::query_as::<_, Equipo>(
        "SELECT equipo_id, numero_serie, equipo_marca, equipo_modelo, equipo_tipo, equipo_precio, equipo_ubicacion, cliente_id, created_by, created_at 
         FROM EQUIPO 
         ORDER BY equipo_marca, equipo_modelo"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(equipos)
}

/// Obtener un equipo por ID
#[tauri::command]
pub async fn get_equipo_by_id(equipo_id: i32) -> Result<Option<Equipo>, String> {
    let pool = get_db_pool_safe()?;
    let equipo = sqlx::query_as::<_, Equipo>(
        "SELECT equipo_id, numero_serie, equipo_marca, equipo_modelo, equipo_tipo, equipo_precio, equipo_ubicacion, cliente_id, created_by, created_at 
         FROM EQUIPO 
         WHERE equipo_id = ?"
    )
    .bind(equipo_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(equipo)
}

/// Obtener un equipo por número de serie
#[tauri::command]
pub async fn get_equipo_by_numero_serie(numero_serie: String) -> Result<Option<Equipo>, String> {
    let pool = get_db_pool_safe()?;
    let equipo = sqlx::query_as::<_, Equipo>(
        "SELECT equipo_id, numero_serie, equipo_marca, equipo_modelo, equipo_tipo, equipo_precio, equipo_ubicacion, cliente_id, created_by, created_at 
         FROM EQUIPO 
         WHERE numero_serie = ?"
    )
    .bind(numero_serie)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(equipo)
}

/// Obtener equipos por cliente
#[tauri::command]
pub async fn get_equipos_by_cliente(cliente_id: i32) -> Result<Vec<Equipo>, String> {
    let pool = get_db_pool_safe()?;
    let equipos = sqlx::query_as::<_, Equipo>(
        "SELECT equipo_id, numero_serie, equipo_marca, equipo_modelo, equipo_tipo, equipo_precio, equipo_ubicacion, cliente_id, created_by, created_at 
         FROM EQUIPO 
         WHERE cliente_id = ? 
         ORDER BY equipo_marca, equipo_modelo"
    )
    .bind(cliente_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(equipos)
}

/// Obtener equipos por tipo
#[tauri::command]
pub async fn get_equipos_by_tipo(equipo_tipo: String) -> Result<Vec<Equipo>, String> {
    let pool = get_db_pool_safe()?;
    let equipos = sqlx::query_as::<_, Equipo>(
        "SELECT equipo_id, numero_serie, equipo_marca, equipo_modelo, equipo_tipo, equipo_precio, equipo_ubicacion, cliente_id, created_by, created_at 
         FROM EQUIPO 
         WHERE equipo_tipo = ? 
         ORDER BY equipo_marca, equipo_modelo"
    )
    .bind(equipo_tipo)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(equipos)
}

/// Obtener equipos por usuario que los creó
#[tauri::command]
pub async fn get_equipos_by_created_by(created_by: i32) -> Result<Vec<Equipo>, String> {
    let pool = get_db_pool_safe()?;
    let equipos = sqlx::query_as::<_, Equipo>(
        "SELECT equipo_id, numero_serie, equipo_marca, equipo_modelo, equipo_tipo, equipo_precio, equipo_ubicacion, cliente_id, created_by, created_at 
         FROM EQUIPO 
         WHERE created_by = ? 
         ORDER BY created_at DESC"
    )
    .bind(created_by)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(equipos)
}

/// Buscar equipos por término de búsqueda
#[tauri::command]
pub async fn search_equipos(search_term: String) -> Result<Vec<Equipo>, String> {
    let pool = get_db_pool_safe()?;
    let search_pattern = format!("%{}%", search_term);
    
    let equipos = sqlx::query_as::<_, Equipo>(
        "SELECT equipo_id, numero_serie, equipo_marca, equipo_modelo, equipo_tipo, equipo_precio, equipo_ubicacion, cliente_id, created_by, created_at 
         FROM EQUIPO 
         WHERE numero_serie LIKE ? 
         OR equipo_marca LIKE ? 
         OR equipo_modelo LIKE ? 
         OR equipo_ubicacion LIKE ?
         ORDER BY equipo_marca, equipo_modelo"
    )
    .bind(&search_pattern)
    .bind(&search_pattern)
    .bind(&search_pattern)
    .bind(&search_pattern)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(equipos)
}

/// Crear un nuevo equipo
#[tauri::command]
pub async fn create_equipo(request: CreateEquipoRequest) -> Result<Equipo, String> {
    let pool = get_db_pool_safe()?;
    
    // Verificar que el número de serie no existe ya
    if let Some(_) = get_equipo_by_numero_serie(request.numero_serie.clone()).await? {
        return Err("Ya existe un equipo con este número de serie".to_string());
    }
    
    // Verificar que el cliente existe
    let cliente_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM CLIENTE WHERE cliente_id = ?"
    )
    .bind(request.cliente_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if cliente_exists == 0 {
        return Err("El cliente especificado no existe".to_string());
    }
    
    let result = sqlx::query(
        "INSERT INTO EQUIPO (numero_serie, equipo_marca, equipo_modelo, equipo_tipo, equipo_precio, equipo_ubicacion, cliente_id, created_by) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&request.numero_serie)
    .bind(&request.equipo_marca)
    .bind(&request.equipo_modelo)
    .bind(&request.equipo_tipo)
    .bind(&request.equipo_precio)
    .bind(&request.equipo_ubicacion)
    .bind(&request.cliente_id)
    .bind(&request.created_by)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let equipo_id = result.last_insert_id() as i32;
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "CREATE_EQUIPO",
        Some(request.created_by),
        "EQUIPO",
        Some(equipo_id),
        None,
        Some(&format!("Equipo creado: {} {} (S/N: {})", 
            request.equipo_marca, 
            request.equipo_modelo,
            request.numero_serie
        ))
    ).await;
    
    // Obtener el equipo recién creado
    get_equipo_by_id(equipo_id)
        .await?
        .ok_or_else(|| "Failed to retrieve created equipo".to_string())
}

/// Actualizar un equipo existente
#[tauri::command]
pub async fn update_equipo(equipo_id: i32, request: UpdateEquipoRequest, updated_by: i32) -> Result<Option<Equipo>, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener el equipo actual para logging
    let current_equipo = get_equipo_by_id(equipo_id).await?;
    
    // Verificar que el número de serie no está en uso por otro equipo (si se está actualizando)
    if let Some(ref new_numero_serie) = request.numero_serie {
        if let Some(existing_equipo) = get_equipo_by_numero_serie(new_numero_serie.clone()).await? {
            if existing_equipo.equipo_id != equipo_id {
                return Err("Ya existe otro equipo con este número de serie".to_string());
            }
        }
    }
    
    // Verificar que el cliente existe (si se está actualizando)
    if let Some(cliente_id) = request.cliente_id {
        let cliente_exists = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM CLIENTE WHERE cliente_id = ?"
        )
        .bind(cliente_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
        
        if cliente_exists == 0 {
            return Err("El cliente especificado no existe".to_string());
        }
    }
    
    let result = sqlx::query(
        "UPDATE EQUIPO SET 
         numero_serie = COALESCE(?, numero_serie),
         equipo_marca = COALESCE(?, equipo_marca),
         equipo_modelo = COALESCE(?, equipo_modelo),
         equipo_tipo = COALESCE(?, equipo_tipo),
         equipo_precio = COALESCE(?, equipo_precio),
         equipo_ubicacion = COALESCE(?, equipo_ubicacion),
         cliente_id = COALESCE(?, cliente_id)
         WHERE equipo_id = ?"
    )
    .bind(&request.numero_serie)
    .bind(&request.equipo_marca)
    .bind(&request.equipo_modelo)
    .bind(&request.equipo_tipo)
    .bind(&request.equipo_precio)
    .bind(&request.equipo_ubicacion)
    .bind(&request.cliente_id)
    .bind(equipo_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Ok(None);
    }
    
    // Registrar la acción en el log de auditoría
    if let Some(ref equipo) = current_equipo {
        let prev_data = format!("{}|{}|{}|{}|{}|{}", 
            equipo.numero_serie.as_deref().unwrap_or(""), 
            equipo.equipo_marca.as_deref().unwrap_or(""),
            equipo.equipo_modelo.as_deref().unwrap_or(""),
            equipo.equipo_tipo.as_deref().unwrap_or(""),
            equipo.equipo_precio.map_or("".to_string(), |p| p.to_string()),
            equipo.equipo_ubicacion.as_deref().unwrap_or("")
        );
        let new_data = format!("{}|{}|{}|{}|{}|{}", 
            request.numero_serie.as_deref().unwrap_or(equipo.numero_serie.as_deref().unwrap_or("")),
            request.equipo_marca.as_deref().unwrap_or(equipo.equipo_marca.as_deref().unwrap_or("")),
            request.equipo_modelo.as_deref().unwrap_or(equipo.equipo_modelo.as_deref().unwrap_or("")),
            request.equipo_tipo.as_deref().unwrap_or(equipo.equipo_tipo.as_deref().unwrap_or("")),
            request.equipo_precio
                .or(equipo.equipo_precio)
                .map_or("".to_string(), |p| p.to_string()),
            request.equipo_ubicacion.as_deref().unwrap_or(equipo.equipo_ubicacion.as_deref().unwrap_or(""))
        );
        
        let _ = log_action(
            "UPDATE_EQUIPO",
            Some(updated_by),
            "EQUIPO",
            Some(equipo_id),
            Some(&prev_data),
            Some(&new_data)
        ).await;
    }
    
    get_equipo_by_id(equipo_id).await
}

/// Eliminar un equipo
#[tauri::command]
pub async fn delete_equipo(equipo_id: i32, deleted_by: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener el equipo antes de eliminarlo para logging
    let equipo_to_delete = get_equipo_by_id(equipo_id).await?;
    
    // Verificar si el equipo tiene órdenes de trabajo asociadas
    let has_dependencies = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM ORDEN_TRABAJO WHERE equipo_id = ?"
    )
    .bind(equipo_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Database error checking dependencies: {}", e))?;
    
    if has_dependencies > 0 {
        return Err("No se puede eliminar el equipo porque tiene órdenes de trabajo asociadas".to_string());
    }
    
    let result = sqlx::query("DELETE FROM EQUIPO WHERE equipo_id = ?")
        .bind(equipo_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    let was_deleted = result.rows_affected() > 0;
    
    // Registrar la acción en el log de auditoría
    if was_deleted {
        if let Some(ref equipo) = equipo_to_delete {
            let _ = log_action(
                "DELETE_EQUIPO",
                Some(deleted_by),
                "EQUIPO",
                Some(equipo_id),
                Some(&format!("Equipo eliminado: {} {} (S/N: {})", 
                    equipo.equipo_marca.as_deref().unwrap_or("N/A"),
                    equipo.equipo_modelo.as_deref().unwrap_or("N/A"),
                    equipo.numero_serie.as_deref().unwrap_or("N/A")
                )),
                None
            ).await;
        }
    }
    
    Ok(was_deleted)
}

/// Contar total de equipos
#[tauri::command]
pub async fn count_equipos() -> Result<i64, String> {
    let pool = get_db_pool_safe()?;
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM EQUIPO")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(count)
}

/// Obtener equipos con paginación
#[tauri::command]
pub async fn get_equipos_with_pagination(offset: i64, limit: i64) -> Result<Vec<Equipo>, String> {
    let pool = get_db_pool_safe()?;
    let equipos = sqlx::query_as::<_, Equipo>(
        "SELECT equipo_id, numero_serie, equipo_marca, equipo_modelo, equipo_tipo, equipo_precio, equipo_ubicacion, cliente_id, created_by, created_at 
         FROM EQUIPO 
         ORDER BY equipo_marca, equipo_modelo 
         LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(equipos)
}

/// Obtener estadísticas de equipos por tipo
#[tauri::command]
pub async fn get_equipos_stats_by_tipo() -> Result<Vec<(String, i64)>, String> {
    let pool = get_db_pool_safe()?;
    let stats = sqlx::query_as::<_, (String, i64)>(
        "SELECT equipo_tipo, COUNT(*) as count 
         FROM EQUIPO 
         GROUP BY equipo_tipo 
         ORDER BY count DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(stats)
}

/// Obtener equipos por rango de precios
#[tauri::command]
pub async fn get_equipos_by_price_range(min_price: Option<i32>, max_price: Option<i32>) -> Result<Vec<Equipo>, String> {
    let pool = get_db_pool_safe()?;
    
    let mut query = "SELECT equipo_id, numero_serie, equipo_marca, equipo_modelo, equipo_tipo, equipo_precio, equipo_ubicacion, cliente_id, created_by, created_at FROM EQUIPO WHERE 1=1".to_string();
    let mut bind_values: Vec<Option<i32>> = Vec::new();
    
    if let Some(min) = min_price {
        query.push_str(" AND equipo_precio >= ?");
        bind_values.push(Some(min));
    }
    
    if let Some(max) = max_price {
        query.push_str(" AND equipo_precio <= ?");
        bind_values.push(Some(max));
    }
    
    query.push_str(" ORDER BY equipo_precio");
    
    let mut sql_query = sqlx::query_as::<_, Equipo>(&query);
    
    for value in bind_values {
        sql_query = sql_query.bind(value);
    }
    
    let equipos = sql_query
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(equipos)
}

/// Obtener resumen de un equipo con información del cliente
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EquipoWithCliente {
    pub equipo_id: i32,
    pub numero_serie: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_ubicacion: Option<String>,
    pub cliente_id: Option<i32>,
    pub cliente_nombre: Option<String>,
    pub cliente_correo: Option<String>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

/// Obtener equipos con información del cliente
#[tauri::command]
pub async fn get_equipos_with_cliente() -> Result<Vec<EquipoWithCliente>, String> {
    let pool = get_db_pool_safe()?;
    let equipos = sqlx::query_as::<_, EquipoWithCliente>(
        "SELECT e.equipo_id, e.numero_serie, e.equipo_marca, e.equipo_modelo, e.equipo_tipo, 
                e.equipo_precio, e.equipo_ubicacion, e.cliente_id, c.cliente_nombre, c.cliente_correo,
                e.created_by, e.created_at
         FROM EQUIPO e
         LEFT JOIN CLIENTE c ON e.cliente_id = c.cliente_id
         ORDER BY e.equipo_marca, e.equipo_modelo"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(equipos)
}

/// Cambiar el cliente de un equipo
#[tauri::command]
pub async fn transfer_equipo_to_cliente(equipo_id: i32, new_cliente_id: i32, updated_by: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Verificar que el equipo existe
    let equipo = get_equipo_by_id(equipo_id).await?;
    if equipo.is_none() {
        return Err("Equipo no encontrado".to_string());
    }
    
    // Verificar que el cliente existe
    let cliente_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM CLIENTE WHERE cliente_id = ?"
    )
    .bind(new_cliente_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if cliente_exists == 0 {
        return Err("El cliente especificado no existe".to_string());
    }
    
    let result = sqlx::query(
        "UPDATE EQUIPO SET cliente_id = ? WHERE equipo_id = ?"
    )
    .bind(new_cliente_id)
    .bind(equipo_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let was_updated = result.rows_affected() > 0;
    
    // Registrar la acción en el log de auditoría
    if was_updated {
        let equipo_info = equipo.unwrap();
        let _ = log_action(
            "TRANSFER_EQUIPO",
            Some(updated_by),
            "EQUIPO",
            Some(equipo_id),
            Some(&format!("Cliente anterior: {}", equipo_info.cliente_id.map_or("N/A".to_string(), |id| id.to_string()))),
            Some(&format!("Nuevo cliente: {}", new_cliente_id))
        ).await;
    }
    
    Ok(was_updated)
}

/// Obtener marcas únicas de equipos
#[tauri::command]
pub async fn get_equipos_marcas() -> Result<Vec<String>, String> {
    let pool = get_db_pool_safe()?;
    let marcas = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT equipo_marca 
         FROM EQUIPO 
         WHERE equipo_marca IS NOT NULL AND equipo_marca != ''
         ORDER BY equipo_marca"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(marcas)
}

/// Obtener modelos únicos por marca
#[tauri::command]
pub async fn get_equipos_modelos_by_marca(marca: String) -> Result<Vec<String>, String> {
    let pool = get_db_pool_safe()?;
    let modelos = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT equipo_modelo 
         FROM EQUIPO 
         WHERE equipo_marca = ? AND equipo_modelo IS NOT NULL AND equipo_modelo != ''
         ORDER BY equipo_modelo"
    )
    .bind(marca)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(modelos)
}

/// Obtener ubicaciones únicas de equipos
#[tauri::command]
pub async fn get_equipos_ubicaciones() -> Result<Vec<String>, String> {
    let pool = get_db_pool_safe()?;
    let ubicaciones = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT equipo_ubicacion 
         FROM EQUIPO 
         WHERE equipo_ubicacion IS NOT NULL AND equipo_ubicacion != ''
         ORDER BY equipo_ubicacion"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(ubicaciones)
}

/// Registrar salida de equipo del inventario
#[tauri::command]
pub async fn registrar_salida_equipo(request: RegistrarSalidaRequest) -> Result<SalidaEquipoResponse, String> {
    
    // Validar que el equipo existe
    let equipo = get_equipo_by_id(request.equipo_id).await?;
    if equipo.is_none() {
        return Err("El equipo especificado no existe".to_string());
    }
    let equipo = equipo.unwrap();
    
    // Validar motivo de salida
    let motivos_validos = vec![
        "entregado_cliente",
        "retirado_sin_reparacion", 
        "abandonado",
        "baja_definitiva"
    ];
    
    if !motivos_validos.contains(&request.motivo_salida.as_str()) {
        return Err("Motivo de salida no válido".to_string());
    }
    
    // Obtener orden de trabajo asociada si existe
    let orden_trabajo = if let Some(orden_id) = request.orden_trabajo_id {
        crate::commands::ordenes_trabajo::get_orden_trabajo_by_id(orden_id).await?
    } else {
        // Buscar orden de trabajo por equipo_id
        let ordenes = crate::commands::ordenes_trabajo::get_ordenes_trabajo_by_equipo(request.equipo_id).await?;
        ordenes.into_iter().find(|o| {
            matches!(o.estado.as_deref(), Some("espera_de_retiro") | Some("entregado") | Some("abandonado") | Some("equipo_no_reparable"))
        })
    };
    
    // Validar estado compatible para salida
    if let Some(ref orden) = orden_trabajo {
        // Estados que permiten registro de salida (equipo AÚN en sistema)
        let estados_en_sistema = vec![
            "recibido",
            "cotizacion_enviada", 
            "aprobacion_pendiente",
            "en_reparacion",
            "espera_de_retiro"
        ];
        
        // Estados donde ya se registró salida (equipo FUERA del sistema)
        let estados_fuera_sistema = vec![
            "entregado",
            "abandonado", 
            "equipo_no_reparable"
        ];
        
        if let Some(estado_actual) = &orden.estado {
            if estados_fuera_sistema.contains(&estado_actual.as_str()) {
                return Err(format!("Ya se registró salida del equipo - Estado actual: '{}'", estado_actual));
            }
            if !estados_en_sistema.contains(&estado_actual.as_str()) {
                return Err(format!("Estado '{}' no permite registro de salida", estado_actual));
            }
        }
    }
    
    // Determinar nuevo estado según motivo
    let nuevo_estado = match request.motivo_salida.as_str() {
        "entregado_cliente" => "entregado",
        "retirado_sin_reparacion" | "abandonado" => "abandonado",
        "baja_definitiva" => "equipo_no_reparable",
        _ => return Err("Motivo de salida no válido".to_string()),
    };
    
    // Actualizar estado de la orden de trabajo si existe
    if let Some(orden) = orden_trabajo {
        let estado_anterior = orden.estado.clone().unwrap_or_default();
        
        // Solo actualizar si el estado es diferente
        if estado_anterior != nuevo_estado {
            let _ = crate::commands::ordenes_trabajo::cambiar_estado_orden_trabajo(
                orden.orden_id,
                nuevo_estado.to_string(),
                request.usuario_id
            ).await?;
        }
        
        // Registrar la salida en el log de auditoría siguiendo el patrón de otros comandos
        
        // Registrar en audit log con mensajes muy cortos para VARCHAR(32)
        if let Err(e) = log_action(
            "REGISTRAR_SALIDA_EQUIPO",
            Some(request.usuario_id),
            "EQUIPO",
            Some(request.equipo_id),
            Some(&format!("{}_en", &estado_anterior[..std::cmp::min(estado_anterior.len(), 10)])),
            Some(&format!("{}_out", &nuevo_estado[..std::cmp::min(nuevo_estado.len(), 10)]))
        ).await {
            eprintln!("Error al guardar log de auditoría: {}", e);
        }
        
        Ok(SalidaEquipoResponse {
            success: true,
            message: format!("Salida registrada exitosamente. Equipo {} - {}", 
                           equipo.numero_serie.clone().unwrap_or_else(|| "Sin serie".to_string()),
                           get_motivo_display(&request.motivo_salida)),
            nuevo_estado: Some(nuevo_estado.to_string()),
        })
    } else {
        // Si no hay orden de trabajo, solo registrar en auditoría
        
        // Registrar en audit log con mensajes muy cortos para VARCHAR(32)
        if let Err(e) = log_action(
            "REGISTRAR_SALIDA_EQUIPO_DIRECTO",
            Some(request.usuario_id),
            "EQUIPO",
            Some(request.equipo_id),
            Some("en_sistema"),
            Some("salida_directa")
        ).await {
            eprintln!("Error al guardar log de auditoría: {}", e);
        }
        
        Ok(SalidaEquipoResponse {
            success: true,
            message: format!("Salida registrada exitosamente. Equipo {} - {}", 
                           equipo.numero_serie.clone().unwrap_or_else(|| "Sin serie".to_string()),
                           get_motivo_display(&request.motivo_salida)),
            nuevo_estado: Some(nuevo_estado.to_string()),
        })
    }
}

/// Verificar si un equipo puede registrar salida
#[tauri::command]
pub async fn puede_registrar_salida_equipo(equipo_id: i32) -> Result<(bool, String), String> {
    // Verificar que el equipo existe
    let equipo = get_equipo_by_id(equipo_id).await?;
    if equipo.is_none() {
        return Ok((false, "El equipo no existe".to_string()));
    }
    
    // Buscar órdenes de trabajo asociadas
    let ordenes = crate::commands::ordenes_trabajo::get_ordenes_trabajo_by_equipo(equipo_id).await?;
    
    // Si no hay órdenes, se puede registrar salida directa
    if ordenes.is_empty() {
        return Ok((true, "Puede registrar salida directa - equipo sin órdenes".to_string()));
    }
    
    // Verificar el estado de la orden más reciente
    let orden_reciente = ordenes.iter()
        .max_by_key(|o| &o.created_at);
    
    if let Some(orden) = orden_reciente {
        // Estados que permiten registro de salida (equipo AÚN en sistema)
        let estados_en_sistema = vec![
            "recibido",
            "cotizacion_enviada", 
            "aprobacion_pendiente",
            "en_reparacion",
            "espera_de_retiro"
        ];
        
        // Estados donde ya se registró salida (equipo FUERA del sistema)
        let estados_fuera_sistema = vec![
            "entregado",
            "abandonado", 
            "equipo_no_reparable"
        ];
        
        if let Some(estado) = &orden.estado {
            if estados_en_sistema.contains(&estado.as_str()) {
                Ok((true, format!("Puede registrar salida - Orden {} en estado: {}", 
                                 orden.orden_codigo.as_ref().unwrap_or(&"N/A".to_string()),
                                 estado)))
            } else if estados_fuera_sistema.contains(&estado.as_str()) {
                Ok((false, format!("Ya se registró salida - Orden {} en estado: {}", 
                                  orden.orden_codigo.as_ref().unwrap_or(&"N/A".to_string()),
                                  estado)))
            } else {
                Ok((false, format!("Estado no válido para salida: {}", estado)))
            }
        } else {
            Ok((true, "Puede registrar salida - estado no definido".to_string()))
        }
    } else {
        Ok((true, "Puede registrar salida directa - sin órdenes válidas".to_string()))
    }
}

/// Verificar si un equipo está actualmente en el sistema (no ha salido)
#[tauri::command]
pub async fn equipo_esta_en_sistema(equipo_id: i32) -> Result<(bool, String), String> {
    // Verificar que el equipo existe
    let equipo = get_equipo_by_id(equipo_id).await?;
    if equipo.is_none() {
        return Ok((false, "El equipo no existe".to_string()));
    }
    
    // Buscar órdenes de trabajo asociadas
    let ordenes = crate::commands::ordenes_trabajo::get_ordenes_trabajo_by_equipo(equipo_id).await?;
    
    // Si no hay órdenes, el equipo está en el sistema
    if ordenes.is_empty() {
        return Ok((true, "Equipo en sistema - sin órdenes de trabajo".to_string()));
    }
    
    // Verificar el estado de la orden más reciente
    let orden_reciente = ordenes.iter()
        .max_by_key(|o| &o.created_at);
    
    if let Some(orden) = orden_reciente {
        // Estados que indican equipo EN sistema
        let estados_en_sistema = vec![
            "recibido",
            "cotizacion_enviada", 
            "aprobacion_pendiente",
            "en_reparacion",
            "espera_de_retiro"
        ];
        
        // Estados que indican equipo FUERA del sistema
        let estados_fuera_sistema = vec![
            "entregado", 
            "abandonado", 
            "equipo_no_reparable"
        ];
        
        if let Some(estado) = &orden.estado {
            if estados_fuera_sistema.contains(&estado.as_str()) {
                Ok((false, format!("Equipo FUERA del sistema - Estado: {}", estado)))
            } else if estados_en_sistema.contains(&estado.as_str()) {
                Ok((true, format!("Equipo EN sistema - Estado: {}", estado)))
            } else {
                // Estado desconocido, asumir en sistema por seguridad
                Ok((true, format!("Equipo EN sistema - Estado desconocido: {}", estado)))
            }
        } else {
            Ok((true, "Equipo en sistema - estado no definido".to_string()))
        }
    } else {
        Ok((true, "Equipo en sistema - sin órdenes válidas".to_string()))
    }
}

/// Obtener equipos que están actualmente en el sistema
#[tauri::command]
pub async fn get_equipos_en_sistema() -> Result<Vec<Equipo>, String> {
    let todos_equipos = get_equipos().await?;
    let mut equipos_en_sistema = Vec::new();
    
    for equipo in todos_equipos {
        let (esta_en_sistema, _) = equipo_esta_en_sistema(equipo.equipo_id).await?;
        if esta_en_sistema {
            equipos_en_sistema.push(equipo);
        }
    }
    
    Ok(equipos_en_sistema)
}

/// Obtener equipos que han salido del sistema
#[tauri::command]
pub async fn get_equipos_fuera_sistema() -> Result<Vec<Equipo>, String> {
    let todos_equipos = get_equipos().await?;
    let mut equipos_fuera_sistema = Vec::new();
    
    for equipo in todos_equipos {
        let (esta_en_sistema, _) = equipo_esta_en_sistema(equipo.equipo_id).await?;
        if !esta_en_sistema {
            equipos_fuera_sistema.push(equipo);
        }
    }
    
    Ok(equipos_fuera_sistema)
}

/// Obtener estadísticas de equipos en/fuera del sistema
#[tauri::command]
pub async fn get_estadisticas_equipos_sistema() -> Result<serde_json::Value, String> {
    let todos_equipos = get_equipos().await?;
    let mut en_sistema = 0;
    let mut fuera_sistema = 0;
    let mut detalles_fuera = std::collections::HashMap::new();
    
    for equipo in todos_equipos {
        let (esta_en_sistema, mensaje) = equipo_esta_en_sistema(equipo.equipo_id).await?;
        if esta_en_sistema {
            en_sistema += 1;
        } else {
            fuera_sistema += 1;
            // Extraer el estado del mensaje para estadísticas
            if mensaje.contains("entregado") {
                *detalles_fuera.entry("entregado".to_string()).or_insert(0) += 1;
            } else if mensaje.contains("abandonado") {
                *detalles_fuera.entry("abandonado".to_string()).or_insert(0) += 1;
            } else if mensaje.contains("equipo_no_reparable") {
                *detalles_fuera.entry("no_reparable".to_string()).or_insert(0) += 1;
            }
        }
    }
    
    let estadisticas = serde_json::json!({
        "total_equipos": en_sistema + fuera_sistema,
        "en_sistema": en_sistema,
        "fuera_sistema": fuera_sistema,
        "porcentaje_en_sistema": if (en_sistema + fuera_sistema) > 0 {
            (en_sistema as f64 / (en_sistema + fuera_sistema) as f64) * 100.0
        } else { 0.0 },
        "detalles_fuera_sistema": detalles_fuera
    });
    
    Ok(estadisticas)
}

// Función auxiliar para obtener texto descriptivo del motivo
fn get_motivo_display(motivo: &str) -> &str {
    match motivo {
        "entregado_cliente" => "Entregado al cliente",
        "retirado_sin_reparacion" => "Retirado sin reparación",
        "abandonado" => "Equipo abandonado",
        "baja_definitiva" => "Baja definitiva del inventario",
        _ => "Motivo desconocido"
    }
}

/// Obtener equipos con estado de última orden de trabajo y filtros avanzados
#[tauri::command]
pub async fn get_equipos_filtrados(filtros: FiltrosEquipos) -> Result<Vec<EquipoConEstado>, String> {
    let pool = get_db_pool_safe()?;

    let mut query = String::from(
        "SELECT 
            e.equipo_id,
            e.numero_serie,
            e.equipo_marca,
            e.equipo_modelo,
            e.equipo_tipo,
            e.equipo_precio,
            e.equipo_ubicacion,
            e.cliente_id,
            e.created_by,
            e.created_at,
            c.cliente_nombre,
            ot_ultima.estado as ultimo_estado_orden,
            ot_ultima.orden_codigo as ultimo_codigo_orden,
            ot_ultima.created_at as fecha_ultima_orden
        FROM EQUIPO e
        LEFT JOIN CLIENTE c ON e.cliente_id = c.cliente_id
        LEFT JOIN (
            SELECT 
                ot1.equipo_id,
                ot1.estado,
                ot1.orden_codigo,
                ot1.created_at,
                ROW_NUMBER() OVER (PARTITION BY ot1.equipo_id ORDER BY ot1.created_at DESC) as rn
            FROM ORDEN_TRABAJO ot1
        ) ot_ultima ON e.equipo_id = ot_ultima.equipo_id AND ot_ultima.rn = 1
        WHERE 1=1"
    );

    let mut params: Vec<String> = Vec::new();

    // Filtro por rango de fechas de creación del equipo
    if let Some(fecha_inicio) = filtros.fecha_inicio {
        if !fecha_inicio.is_empty() {
            query.push_str(" AND DATE(e.created_at) >= ?");
            params.push(fecha_inicio);
        }
    }

    if let Some(fecha_fin) = filtros.fecha_fin {
        if !fecha_fin.is_empty() {
            query.push_str(" AND DATE(e.created_at) <= ?");
            params.push(fecha_fin);
        }
    }

    // Filtro por marcas
    if let Some(marcas) = filtros.marcas {
        if !marcas.is_empty() {
            let placeholders = vec!["?"; marcas.len()].join(",");
            query.push_str(&format!(" AND e.equipo_marca IN ({})", placeholders));
            params.extend(marcas);
        }
    }

    // Filtro por modelos
    if let Some(modelos) = filtros.modelos {
        if !modelos.is_empty() {
            let placeholders = vec!["?"; modelos.len()].join(",");
            query.push_str(&format!(" AND e.equipo_modelo IN ({})", placeholders));
            params.extend(modelos);
        }
    }

    // Filtro por tipos
    if let Some(tipos) = filtros.tipos {
        if !tipos.is_empty() {
            let placeholders = vec!["?"; tipos.len()].join(",");
            query.push_str(&format!(" AND e.equipo_tipo IN ({})", placeholders));
            params.extend(tipos);
        }
    }

    // Filtro por clientes
    if let Some(clientes) = filtros.clientes {
        if !clientes.is_empty() {
            let placeholders = vec!["?"; clientes.len()].join(",");
            query.push_str(&format!(" AND c.cliente_nombre IN ({})", placeholders));
            params.extend(clientes);
        }
    }

    // Filtro por ubicaciones
    if let Some(ubicaciones) = filtros.ubicaciones {
        if !ubicaciones.is_empty() {
            let placeholders = vec!["?"; ubicaciones.len()].join(",");
            query.push_str(&format!(" AND e.equipo_ubicacion IN ({})", placeholders));
            params.extend(ubicaciones);
        }
    }

    // Filtro por estados de orden de trabajo
    if let Some(estados_orden) = filtros.estados_orden {
        if !estados_orden.is_empty() {
            let placeholders = vec!["?"; estados_orden.len()].join(",");
            query.push_str(&format!(" AND ot_ultima.estado IN ({})", placeholders));
            params.extend(estados_orden);
        }
    }

    // Filtro por rango de precios
    if let Some(precio_min) = filtros.precio_min {
        query.push_str(" AND e.equipo_precio >= ?");
        params.push(precio_min.to_string());
    }

    if let Some(precio_max) = filtros.precio_max {
        query.push_str(" AND e.equipo_precio <= ?");
        params.push(precio_max.to_string());
    }

    // Filtro por búsqueda de texto
    if let Some(search_term) = filtros.search {
        if !search_term.trim().is_empty() {
            let search_pattern = format!("%{}%", search_term.trim());
            query.push_str(" AND (e.numero_serie LIKE ? OR e.equipo_marca LIKE ? OR e.equipo_modelo LIKE ? OR c.cliente_nombre LIKE ?)");
            params.push(search_pattern.clone());
            params.push(search_pattern.clone());
            params.push(search_pattern.clone());
            params.push(search_pattern);
        }
    }

    // Ordenamiento
    match filtros.ordenamiento.as_deref() {
        Some("marca_asc") => query.push_str(" ORDER BY e.equipo_marca ASC"),
        Some("marca_desc") => query.push_str(" ORDER BY e.equipo_marca DESC"),
        Some("modelo_asc") => query.push_str(" ORDER BY e.equipo_modelo ASC"),
        Some("modelo_desc") => query.push_str(" ORDER BY e.equipo_modelo DESC"),
        Some("fecha_asc") => query.push_str(" ORDER BY e.created_at ASC"),
        Some("fecha_desc") => query.push_str(" ORDER BY e.created_at DESC"),
        Some("precio_asc") => query.push_str(" ORDER BY e.equipo_precio ASC"),
        Some("precio_desc") => query.push_str(" ORDER BY e.equipo_precio DESC"),
        Some("cliente_asc") => query.push_str(" ORDER BY c.cliente_nombre ASC"),
        Some("cliente_desc") => query.push_str(" ORDER BY c.cliente_nombre DESC"),
        Some("estado_asc") => query.push_str(" ORDER BY ot_ultima.estado ASC"),
        Some("estado_desc") => query.push_str(" ORDER BY ot_ultima.estado DESC"),
        _ => query.push_str(" ORDER BY e.created_at DESC"), // Por defecto, más recientes primero
    }

    // Ejecutar la consulta con parámetros dinámicos
    let mut query_builder = sqlx::query_as::<_, EquipoConEstado>(&query);
    
    for param in params {
        query_builder = query_builder.bind(param);
    }
    
    let equipos = query_builder
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Database error en get_equipos_filtrados: {}", e))?;

    Ok(equipos)
}

/// Obtener equipos con estado (versión simplificada para compatibilidad)
#[tauri::command]
pub async fn get_equipos_con_estado() -> Result<Vec<EquipoConEstado>, String> {
    let filtros = FiltrosEquipos {
        fecha_inicio: None,
        fecha_fin: None,
        marcas: None,
        modelos: None,
        tipos: None,
        clientes: None,
        ubicaciones: None,
        estados_orden: None,
        search: None,
        ordenamiento: Some("fecha_desc".to_string()),
        precio_min: None,
        precio_max: None,
    };
    
    get_equipos_filtrados(filtros).await
}

/// Obtener todos los clientes únicos que tienen equipos
#[tauri::command]
pub async fn get_clientes_con_equipos() -> Result<Vec<String>, String> {
    let pool = get_db_pool_safe()?;
    
    let clientes = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT c.cliente_nombre 
         FROM CLIENTE c 
         INNER JOIN EQUIPO e ON c.cliente_id = e.cliente_id 
         WHERE c.cliente_nombre IS NOT NULL 
         ORDER BY c.cliente_nombre"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(clientes)
}

/// Obtener todos los tipos únicos de equipos
#[tauri::command]
pub async fn get_tipos_equipos() -> Result<Vec<String>, String> {
    let pool = get_db_pool_safe()?;
    
    let tipos = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT equipo_tipo 
         FROM EQUIPO 
         WHERE equipo_tipo IS NOT NULL 
         ORDER BY equipo_tipo"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(tipos)
}

/// Obtener todos los estados únicos de órdenes de trabajo
#[tauri::command]
pub async fn get_estados_ordenes_trabajo() -> Result<Vec<String>, String> {
    let pool = get_db_pool_safe()?;
    
    let estados = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT estado 
         FROM ORDEN_TRABAJO 
         WHERE estado IS NOT NULL 
         ORDER BY estado"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(estados)
}

/// Obtener estadísticas de equipos por estado de orden de trabajo
#[tauri::command]
pub async fn get_estadisticas_equipos_por_estado() -> Result<Vec<(String, i64)>, String> {
    let pool = get_db_pool_safe()?;
    
    let estadisticas = sqlx::query_as::<_, (String, i64)>(
        "SELECT 
            COALESCE(ot_ultima.estado, 'Sin orden de trabajo') as estado,
            COUNT(*) as cantidad
        FROM EQUIPO e
        LEFT JOIN (
            SELECT 
                ot1.equipo_id,
                ot1.estado,
                ROW_NUMBER() OVER (PARTITION BY ot1.equipo_id ORDER BY ot1.created_at DESC) as rn
            FROM ORDEN_TRABAJO ot1
        ) ot_ultima ON e.equipo_id = ot_ultima.equipo_id AND ot_ultima.rn = 1
        GROUP BY ot_ultima.estado
        ORDER BY cantidad DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(estadisticas)
}
