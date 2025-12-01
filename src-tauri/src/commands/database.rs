use crate::database::{get_database_status as get_db_status, check_database_connection as check_db_connection};
use crate::config::AppConfig;
use tauri::State;
use serde::{Deserialize, Serialize};

use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStatusResponse {
    pub is_connected: bool,
    pub error_message: Option<String>,
    pub last_check: Option<String>,
}

#[tauri::command]
pub async fn get_database_status(state: State<'_, AppConfig>) -> Result<DatabaseStatusResponse, String> {
    if state.use_api {
        return Ok(DatabaseStatusResponse {
            is_connected: false,
            error_message: Some("Base de datos no disponible en modo API".to_string()),
            last_check: Some(chrono::Utc::now().to_rfc3339()),
        });
    }
    
    let status = get_db_status();
    Ok(DatabaseStatusResponse {
        is_connected: status.is_connected,
        error_message: status.error_message,
        last_check: status.last_check.map(|dt| dt.to_rfc3339()),
    })
}

#[tauri::command]
pub async fn check_database_connection(state: State<'_, AppConfig>) -> Result<DatabaseStatusResponse, String> {
    if state.use_api {
        return Ok(DatabaseStatusResponse {
            is_connected: false,
            error_message: Some("Operación no disponible en modo API".to_string()),
            last_check: Some(chrono::Utc::now().to_rfc3339()),
        });
    }
    
    let is_connected = check_db_connection().await;
    let status = get_db_status();
    Ok(DatabaseStatusResponse {
        is_connected,
        error_message: status.error_message,
        last_check: status.last_check.map(|dt| dt.to_rfc3339()),
    })
}

#[tauri::command]
pub async fn retry_database_connection(state: State<'_, AppConfig>) -> Result<DatabaseStatusResponse, String> {
    if state.use_api {
        return Ok(DatabaseStatusResponse {
            is_connected: false,
            error_message: Some("Operación no disponible en modo API".to_string()),
            last_check: Some(chrono::Utc::now().to_rfc3339()),
        });
    }
    
    match crate::database::retry_database_connection().await {
        Ok(_) => {
            let status = get_db_status();
            Ok(DatabaseStatusResponse {
                is_connected: true,
                error_message: None,
                last_check: status.last_check.map(|dt| dt.to_rfc3339()),
            })
        }
        Err(e) => {
            Ok(DatabaseStatusResponse {
                is_connected: false,
                error_message: Some(e.to_string()),
                last_check: Some(chrono::Utc::now().to_rfc3339()),
            })
        }
    }
}

#[tauri::command]
pub async fn force_run_migrations(state: State<'_, AppConfig>) -> Result<String, String> {
    if state.use_api {
        return Err("Operación no disponible en modo API".to_string());
    }
    
    let pool = crate::database::get_db_pool_safe()?;
    
    match sqlx::migrate!("./migrations").run(&*pool).await {
        Ok(_) => Ok("Migraciones ejecutadas exitosamente".to_string()),
        Err(e) => Err(format!("Error ejecutando migraciones: {}", e)),
    }
}

#[tauri::command]  
pub async fn insert_test_data(state: State<'_, AppConfig>) -> Result<String, String> {
    if state.use_api {
        return Err("Operación no disponible en modo API".to_string());
    }
    
    let pool = crate::database::get_db_pool_safe()?;
    
    // Verificar conteos de tablas
    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM USUARIO")
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Error verificando usuarios: {}", e))?;
    
    let cliente_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM CLIENTE")
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Error verificando clientes: {}", e))?;
        
    let equipo_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM EQUIPO")
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Error verificando equipos: {}", e))?;
        
    let orden_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ORDEN_TRABAJO")
        .fetch_one(&*pool)
        .await
        .map_err(|e| format!("Error verificando ordenes: {}", e))?;
    
    Ok(format!("Conteos actuales - Usuarios: {}, Clientes: {}, Equipos: {}, Ordenes: {}", 
        user_count, cliente_count, equipo_count, orden_count))
}

#[tauri::command]  
pub async fn check_equipo_ids(state: State<'_, AppConfig>) -> Result<String, String> {
    if state.use_api {
        return Err("Operación no disponible en modo API".to_string());
    }
    
    let pool = crate::database::get_db_pool_safe()?;
    
    let equipos: Vec<(i32, String)> = sqlx::query_as("SELECT equipo_id, numero_serie FROM EQUIPO ORDER BY equipo_id LIMIT 10")
        .fetch_all(&*pool)
        .await
        .map_err(|e| format!("Error consultando equipos: {}", e))?;
    
    let mut result = String::from("Equipos disponibles:\\n");
    for (id, serie) in equipos {
        result.push_str(&format!("ID: {}, Serie: {}\\n", id, serie));
    }
    
    Ok(result)
}

// Nueva función para iniciar el sistema de reconexión automática
pub fn start_auto_reconnect_task() {
    tokio::spawn(async move {
        let mut retry_interval = Duration::from_secs(30); // Intervalo inicial de 30 segundos
        let mut consecutive_failures = 0u32;
        
        loop {
            sleep(retry_interval).await;
            
            // Verificar el estado actual - usar get_db_status() que es síncrona
            let status = get_db_status();
            
            // Solo intentar reconectar si no está conectado
            if !status.is_connected {
                println!("Auto-reconnect: Intentando reconectar a la base de datos...");
                
                match crate::database::retry_database_connection().await {
                    Ok(_) => {
                        println!("Auto-reconnect: Reconexión exitosa!");
                        // Resetear el intervalo y contador de fallos
                        retry_interval = Duration::from_secs(30);
                        consecutive_failures = 0;
                    }
                    Err(e) => {
                        consecutive_failures += 1;
                        println!("Auto-reconnect: Fallo en el intento {}: {}", consecutive_failures, e);
                        
                        // Backoff exponencial: aumentar el intervalo con cada fallo
                        // pero con un máximo
                        retry_interval = Duration::from_secs(
                            (30 * (1 << consecutive_failures.min(4))).min(300)
                        );
                    }
                }
            } else {
                // Si está conectado, resetear el intervalo
                retry_interval = Duration::from_secs(30);
                consecutive_failures = 0;
            }
        }
    });
}
