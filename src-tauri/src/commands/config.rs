use crate::config::{DatabaseConfig, SecureConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigResponse {
    pub exists: bool,
    pub config: Option<DatabaseConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigTestResult {
    pub success: bool,
    pub error: Option<String>,
}

/// Verifica si existe una configuración de base de datos guardada
#[tauri::command]
pub async fn check_database_config() -> Result<ConfigResponse, String> {
    let secure_config = SecureConfig::new()
        .map_err(|e| format!("Failed to initialize secure config: {}", e))?;
    
    let exists = secure_config.config_exists();
    let config = if exists {
        match secure_config.load_config() {
            Ok(cfg) => Some(cfg),
            Err(_) => None,
        }
    } else {
        None
    };

    Ok(ConfigResponse { exists, config })
}

/// Guarda la configuración de la base de datos de forma segura
#[tauri::command]
pub async fn save_database_config(config: DatabaseConfig) -> Result<String, String> {
    let secure_config = SecureConfig::new()
        .map_err(|e| format!("Failed to initialize secure config: {}", e))?;
    
    secure_config.save_config(&config)
        .map_err(|e| format!("Failed to save config: {}", e))?;

    Ok("Configuration saved successfully".to_string())
}

/// Carga la configuración de la base de datos
#[tauri::command]
pub async fn load_database_config() -> Result<DatabaseConfig, String> {
    crate::config::load_database_config()
        .map_err(|e| format!("Failed to load config: {}", e))
}

/// Prueba la conexión con la configuración proporcionada
#[tauri::command]
pub async fn test_database_connection(config: DatabaseConfig) -> Result<ConfigTestResult, String> {
    let connection_string = config.to_connection_string();
    
    match sqlx::MySqlPool::connect(&connection_string).await {
        Ok(pool) => {
            // Probar una consulta simple
            match sqlx::query("SELECT 1").execute(&pool).await {
                Ok(_) => Ok(ConfigTestResult {
                    success: true,
                    error: None,
                }),
                Err(e) => Ok(ConfigTestResult {
                    success: false,
                    error: Some(format!("Query test failed: {}", e)),
                }),
            }
        }
        Err(e) => Ok(ConfigTestResult {
            success: false,
            error: Some(format!("Connection failed: {}", e)),
        }),
    }
}

/// Elimina la configuración guardada
#[tauri::command]
pub async fn delete_database_config() -> Result<String, String> {
    let secure_config = SecureConfig::new()
        .map_err(|e| format!("Failed to initialize secure config: {}", e))?;
    
    secure_config.delete_config()
        .map_err(|e| format!("Failed to delete config: {}", e))?;

    Ok("Configuration deleted successfully".to_string())
}

/// Genera una configuración de ejemplo con valores por defecto
#[tauri::command]
pub async fn get_default_database_config() -> Result<DatabaseConfig, String> {
    Ok(DatabaseConfig::default())
}

/// Genera una configuración específica para GCP con bastion host
#[tauri::command]
pub async fn get_gcp_database_config() -> Result<DatabaseConfig, String> {
    Ok(DatabaseConfig::with_gcp_bastion())
}

/// Verifica si el túnel SSH está activo
#[tauri::command]
pub async fn check_ssh_tunnel_status() -> Result<bool, String> {
    Ok(crate::ssh_tunnel::is_ssh_tunnel_active())
}

/// Inicia el túnel SSH manualmente
#[tauri::command]
pub async fn start_ssh_tunnel(config: DatabaseConfig) -> Result<String, String> {
    if let Some(bastion_config) = &config.bastion {
        crate::ssh_tunnel::ensure_ssh_tunnel(bastion_config)
            .await
            .map_err(|e| format!("Failed to start SSH tunnel: {}", e))?;
        Ok("SSH tunnel started successfully".to_string())
    } else {
        Err("No bastion configuration found".to_string())
    }
}

/// Detiene el túnel SSH
#[tauri::command]
pub async fn stop_ssh_tunnel() -> Result<String, String> {
    crate::ssh_tunnel::stop_ssh_tunnel()
        .map_err(|e| format!("Failed to stop SSH tunnel: {}", e))?;
    Ok("SSH tunnel stopped successfully".to_string())
}
