use crate::database::{get_database_status as get_db_status, check_database_connection as check_db_connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStatusResponse {
    pub is_connected: bool,
    pub error_message: Option<String>,
    pub last_check: Option<String>,
}

#[tauri::command]
pub async fn get_database_status() -> DatabaseStatusResponse {
    let status = get_db_status();
    DatabaseStatusResponse {
        is_connected: status.is_connected,
        error_message: status.error_message,
        last_check: status.last_check.map(|dt| dt.to_rfc3339()),
    }
}

#[tauri::command]
pub async fn check_database_connection() -> DatabaseStatusResponse {
    let is_connected = check_db_connection().await;
    let status = get_db_status();
    DatabaseStatusResponse {
        is_connected,
        error_message: status.error_message,
        last_check: status.last_check.map(|dt| dt.to_rfc3339()),
    }
}

#[tauri::command]
pub async fn retry_database_connection() -> DatabaseStatusResponse {
    match crate::database::init_database().await {        Ok(_) => {
            let status = get_db_status();
            DatabaseStatusResponse {
                is_connected: true,
                error_message: None,
                last_check: status.last_check.map(|dt| dt.to_rfc3339()),
            }
        }
        Err(e) => {
            DatabaseStatusResponse {
                is_connected: false,
                error_message: Some(e.to_string()),
                last_check: Some(chrono::Utc::now().to_rfc3339()),
            }
        }
    }
}

#[tauri::command]
pub async fn force_run_migrations() -> Result<String, String> {
    let pool = crate::database::get_db_pool_safe()?;
    
    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => Ok("Migraciones ejecutadas exitosamente".to_string()),
        Err(e) => Err(format!("Error ejecutando migraciones: {}", e)),
    }
}

#[tauri::command]  
pub async fn insert_test_data() -> Result<String, String> {
    let pool = crate::database::get_db_pool_safe()?;
    
    // Verificar conteos de tablas
    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM USUARIO")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error verificando usuarios: {}", e))?;
    
    let cliente_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM CLIENTE")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error verificando clientes: {}", e))?;
        
    let equipo_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM EQUIPO")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error verificando equipos: {}", e))?;
        
    let orden_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ORDEN_TRABAJO")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Error verificando ordenes: {}", e))?;
    
    Ok(format!("Conteos actuales - Usuarios: {}, Clientes: {}, Equipos: {}, Ordenes: {}", 
        user_count, cliente_count, equipo_count, orden_count))
}

#[tauri::command]  
pub async fn check_equipo_ids() -> Result<String, String> {
    let pool = crate::database::get_db_pool_safe()?;
    
    let equipos: Vec<(i32, String)> = sqlx::query_as("SELECT equipo_id, numero_serie FROM EQUIPO ORDER BY equipo_id LIMIT 10")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Error consultando equipos: {}", e))?;
    
    let mut result = String::from("Equipos disponibles:\n");
    for (id, serie) in equipos {
        result.push_str(&format!("ID: {}, Serie: {}\n", id, serie));
    }
    
    Ok(result)
}
