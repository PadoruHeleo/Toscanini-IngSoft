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
