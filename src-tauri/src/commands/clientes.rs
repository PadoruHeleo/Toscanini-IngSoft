use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_safe;
use crate::commands::logs::log_action;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cliente {
    pub cliente_id: i32,
    pub cliente_rut: Option<String>,
    pub cliente_nombre: Option<String>,
    pub cliente_correo: Option<String>,
    pub cliente_telefono: Option<String>,
    pub cliente_direccion: Option<String>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateClienteRequest {
    pub cliente_rut: String,
    pub cliente_nombre: String,
    pub cliente_correo: String,
    pub cliente_telefono: Option<String>,
    pub cliente_direccion: Option<String>,
    pub created_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClienteRequest {
    pub cliente_rut: Option<String>,
    pub cliente_nombre: Option<String>,
    pub cliente_correo: Option<String>,
    pub cliente_telefono: Option<String>,
    pub cliente_direccion: Option<String>,
}

#[tauri::command]
pub async fn get_clientes() -> Result<Vec<Cliente>, String> {
    let pool = get_db_pool_safe()?;
    let clientes = sqlx::query_as::<_, Cliente>(
        "SELECT cliente_id, cliente_rut, cliente_nombre, cliente_correo, cliente_telefono, cliente_direccion, created_by, created_at FROM CLIENTE ORDER BY cliente_nombre"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(clientes)
}

#[tauri::command]
pub async fn get_cliente_by_id(cliente_id: i32) -> Result<Option<Cliente>, String> {
    let pool = get_db_pool_safe()?;
    let cliente = sqlx::query_as::<_, Cliente>(
        "SELECT cliente_id, cliente_rut, cliente_nombre, cliente_correo, cliente_telefono, cliente_direccion, created_by, created_at FROM CLIENTE WHERE cliente_id = ?"
    )
    .bind(cliente_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cliente)
}

#[tauri::command]
pub async fn get_cliente_by_rut(cliente_rut: String) -> Result<Option<Cliente>, String> {
    let pool = get_db_pool_safe()?;
    let cliente = sqlx::query_as::<_, Cliente>(
        "SELECT cliente_id, cliente_rut, cliente_nombre, cliente_correo, cliente_telefono, cliente_direccion, created_by, created_at FROM CLIENTE WHERE cliente_rut = ?"
    )
    .bind(cliente_rut)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cliente)
}

#[tauri::command]
pub async fn get_clientes_by_created_by(created_by: i32) -> Result<Vec<Cliente>, String> {
    let pool = get_db_pool_safe()?;
    let clientes = sqlx::query_as::<_, Cliente>(
        "SELECT cliente_id, cliente_rut, cliente_nombre, cliente_correo, cliente_telefono, cliente_direccion, created_by, created_at FROM CLIENTE WHERE created_by = ? ORDER BY cliente_nombre"
    )
    .bind(created_by)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(clientes)
}

#[tauri::command]
pub async fn search_clientes(search_term: String) -> Result<Vec<Cliente>, String> {
    let pool = get_db_pool_safe()?;
    let search_pattern = format!("%{}%", search_term);
    
    let clientes = sqlx::query_as::<_, Cliente>(
        "SELECT cliente_id, cliente_rut, cliente_nombre, cliente_correo, cliente_telefono, cliente_direccion, created_by, created_at 
         FROM CLIENTE 
         WHERE cliente_nombre LIKE ? OR cliente_rut LIKE ? OR cliente_correo LIKE ?
         ORDER BY cliente_nombre"
    )
    .bind(&search_pattern)
    .bind(&search_pattern)
    .bind(&search_pattern)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(clientes)
}

#[tauri::command]
pub async fn create_cliente(request: CreateClienteRequest) -> Result<Cliente, String> {
    let pool = get_db_pool_safe()?;
    
    // Verificar que el RUT no existe ya
    if let Some(_) = get_cliente_by_rut(request.cliente_rut.clone()).await? {
        return Err("Ya existe un cliente con este RUT".to_string());
    }
    
    let result = sqlx::query(
        "INSERT INTO CLIENTE (cliente_rut, cliente_nombre, cliente_correo, cliente_telefono, cliente_direccion, created_by) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&request.cliente_rut)
    .bind(&request.cliente_nombre)
    .bind(&request.cliente_correo)
    .bind(&request.cliente_telefono)
    .bind(&request.cliente_direccion)
    .bind(&request.created_by)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let cliente_id = result.last_insert_id() as i32;
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "CREATE_CLIENTE",
        Some(request.created_by),
        "CLIENTE",
        Some(cliente_id),
        None,
        Some(&format!("Cliente creado: {} ({})", request.cliente_nombre, request.cliente_correo))
    ).await;
    
    // Obtener el cliente recién creado
    get_cliente_by_id(cliente_id)
        .await?
        .ok_or_else(|| "Failed to retrieve created cliente".to_string())
}

#[tauri::command]
pub async fn update_cliente(cliente_id: i32, request: UpdateClienteRequest, updated_by: i32) -> Result<Option<Cliente>, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener el cliente actual para logging
    let current_cliente = get_cliente_by_id(cliente_id).await?;
    
    // Verificar que el RUT no está en uso por otro cliente (si se está actualizando)
    if let Some(ref new_rut) = request.cliente_rut {
        if let Some(existing_cliente) = get_cliente_by_rut(new_rut.clone()).await? {
            if existing_cliente.cliente_id != cliente_id {
                return Err("Ya existe otro cliente con este RUT".to_string());
            }
        }
    }
    
    let result = sqlx::query(
        "UPDATE CLIENTE SET 
         cliente_rut = COALESCE(?, cliente_rut),
         cliente_nombre = COALESCE(?, cliente_nombre),
         cliente_correo = COALESCE(?, cliente_correo),
         cliente_telefono = COALESCE(?, cliente_telefono),
         cliente_direccion = COALESCE(?, cliente_direccion)
         WHERE cliente_id = ?"
    )
    .bind(&request.cliente_rut)
    .bind(&request.cliente_nombre)
    .bind(&request.cliente_correo)
    .bind(&request.cliente_telefono)
    .bind(&request.cliente_direccion)
    .bind(cliente_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Ok(None);
    }
    
    // Registrar la acción en el log de auditoría
    if let Some(ref cliente) = current_cliente {
        let prev_data = format!("{}|{}|{}|{}", 
            cliente.cliente_nombre.as_deref().unwrap_or(""), 
            cliente.cliente_correo.as_deref().unwrap_or(""),
            cliente.cliente_telefono.as_deref().unwrap_or(""),
            cliente.cliente_direccion.as_deref().unwrap_or("")
        );
        let new_data = format!("{}|{}|{}|{}", 
            request.cliente_nombre.as_deref().unwrap_or(cliente.cliente_nombre.as_deref().unwrap_or("")),
            request.cliente_correo.as_deref().unwrap_or(cliente.cliente_correo.as_deref().unwrap_or("")),
            request.cliente_telefono.as_deref().unwrap_or(cliente.cliente_telefono.as_deref().unwrap_or("")),
            request.cliente_direccion.as_deref().unwrap_or(cliente.cliente_direccion.as_deref().unwrap_or(""))
        );
        
        let _ = log_action(
            "UPDATE_CLIENTE",
            Some(updated_by),
            "CLIENTE",
            Some(cliente_id),
            Some(&prev_data),
            Some(&new_data)
        ).await;
    }
    
    get_cliente_by_id(cliente_id).await
}

#[tauri::command]
pub async fn delete_cliente(cliente_id: i32, deleted_by: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener el cliente antes de eliminarlo para logging
    let cliente_to_delete = get_cliente_by_id(cliente_id).await?;
    
    // Verificar si el cliente tiene cotizaciones, informes u órdenes de trabajo asociadas
    let has_dependencies = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM (
            SELECT 1 FROM COTIZACION WHERE cliente_id = ?
            UNION ALL
            SELECT 1 FROM INFORME WHERE cliente_id = ?
            UNION ALL
            SELECT 1 FROM ORDEN_TRABAJO WHERE cliente_id = ?
        ) as dependencies"
    )
    .bind(cliente_id)
    .bind(cliente_id)
    .bind(cliente_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Database error checking dependencies: {}", e))?;
    
    if has_dependencies > 0 {
        return Err("No se puede eliminar el cliente porque tiene cotizaciones, informes u órdenes de trabajo asociadas".to_string());
    }
    
    let result = sqlx::query("DELETE FROM CLIENTE WHERE cliente_id = ?")
        .bind(cliente_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    let was_deleted = result.rows_affected() > 0;
    
    // Registrar la acción en el log de auditoría
    if was_deleted {
        if let Some(ref cliente) = cliente_to_delete {
            let _ = log_action(
                "DELETE_CLIENTE",
                Some(deleted_by),
                "CLIENTE",
                Some(cliente_id),
                Some(&format!("Cliente eliminado: {} ({})", 
                    cliente.cliente_nombre.as_deref().unwrap_or("N/A"),
                    cliente.cliente_correo.as_deref().unwrap_or("N/A")
                )),
                None
            ).await;
        }
    }
    
    Ok(was_deleted)
}

#[tauri::command]
pub async fn count_clientes() -> Result<i64, String> {
    let pool = get_db_pool_safe()?;
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM CLIENTE")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(count)
}

#[tauri::command]
pub async fn get_clientes_with_pagination(offset: i64, limit: i64) -> Result<Vec<Cliente>, String> {
    let pool = get_db_pool_safe()?;
    let clientes = sqlx::query_as::<_, Cliente>(
        "SELECT cliente_id, cliente_rut, cliente_nombre, cliente_correo, cliente_telefono, cliente_direccion, created_by, created_at 
         FROM CLIENTE 
         ORDER BY cliente_nombre 
         LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(clientes)
}
