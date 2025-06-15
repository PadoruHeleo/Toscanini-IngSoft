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
