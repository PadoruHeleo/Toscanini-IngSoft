use sqlx::{MySql, Pool};
use std::sync::{OnceLock, Arc, Mutex};
use std::path::Path;
use crate::config::load_database_config;
use std::time::Duration;
use tokio::time::sleep;

pub type DbPool = Pool<MySql>;

// Cambiar el tipo almacenado para usar Arc directamente
static DB_POOL: OnceLock<Arc<Mutex<Option<Arc<DbPool>>>>> = OnceLock::new();
static DB_CONNECTION_STATUS: OnceLock<Arc<Mutex<DatabaseStatus>>> = OnceLock::new();

// Macro para funciones que requieren base de datos
#[macro_export]
macro_rules! require_db {
    () => {
        match crate::database::get_db_pool_safe() {
            Ok(pool) => pool,
            Err(e) => return Err(e),
        }
    };
}

#[derive(Clone, Debug)]
pub struct DatabaseStatus {
    pub is_connected: bool,
    pub error_message: Option<String>,
    pub last_check: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for DatabaseStatus {
    fn default() -> Self {
        Self {
            is_connected: false,
            error_message: None,
            last_check: None,
        }
    }
}

pub async fn init_database() -> Result<(), sqlx::Error> {
    // Intentar cargar configuración segura primero
    let database_url = match load_database_config() {
        Ok(config) => {
            log::info!("Configuración segura de base de datos cargada");
            config.to_connection_string()
        }
        Err(e) => {
            log::warn!("No se pudo cargar la configuración segura ({}), intentando fallback con .env", e);
            // Fallback a la carga tradicional de .env
            load_env_file();
            
            std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| {
                    log::warn!("DATABASE_URL no encontrado. Usando configuración por defecto.");
                    "mysql://root:@localhost:3306/toscanini_db".to_string()
                })
        }
    };
    
    log::info!("Intentando conectar a la base de datos con URL: {}", 
        database_url.split('@').next().unwrap_or("***").to_string() + "@***");
    
    // Inicializar DB_POOL si no existe
    if DB_POOL.get().is_none() {
        let _ = DB_POOL.set(Arc::new(Mutex::new(None)));
    }
    
    // Inicializar el estado de conexión si no existe
    if DB_CONNECTION_STATUS.get().is_none() {
        let _ = DB_CONNECTION_STATUS.set(Arc::new(Mutex::new(DatabaseStatus::default())));
    }
      match sqlx::MySqlPool::connect(&database_url).await {
        Ok(pool) => {
            // Ejecutar migraciones si es necesario
            if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
                let error_msg = format!("Migration failed: {}", e);
                update_database_status(false, Some(error_msg.clone()));
                return Err(sqlx::Error::Configuration(error_msg.into()));
            }
            
            // Guardar el pool envuelto en Arc
            if let Some(pool_arc) = DB_POOL.get() {
                if let Ok(mut pool_guard) = pool_arc.lock() {
                    *pool_guard = Some(Arc::new(pool));
                }
            }
            
            update_database_status(true, None);
            Ok(())
        }
        Err(e) => {
            update_database_status(false, Some(e.to_string()));
            Err(e)
        }
    }
}

pub fn get_db_pool() -> Option<Arc<DbPool>> {
    if let Some(pool_arc) = DB_POOL.get() {
        if let Ok(pool_guard) = pool_arc.lock() {
            return pool_guard.as_ref().cloned();
        }
    }
    None
}

pub fn get_db_pool_unchecked() -> Arc<DbPool> {
    get_db_pool().expect("Database pool not initialized")
}

// Nueva función segura que no hace panic
pub fn get_db_pool_safe() -> Result<Arc<DbPool>, String> {
    get_db_pool().ok_or_else(|| "Database not connected".to_string())
}

pub fn update_database_status(is_connected: bool, error_message: Option<String>) {
    if let Some(status_arc) = DB_CONNECTION_STATUS.get() {
        if let Ok(mut status) = status_arc.lock() {
            status.is_connected = is_connected;
            status.error_message = error_message;
            status.last_check = Some(chrono::Utc::now());
        }
    }
}

pub fn get_database_status() -> DatabaseStatus {
    if let Some(status_arc) = DB_CONNECTION_STATUS.get() {
        if let Ok(status) = status_arc.lock() {
            status.clone()
        } else {
            DatabaseStatus::default()
        }
    } else {
        DatabaseStatus::default()
    }
}

pub async fn check_database_connection() -> bool {
    if let Some(pool) = get_db_pool() {
        match sqlx::query("SELECT 1").execute(&*pool).await {
            Ok(_) => {
                update_database_status(true, None);
                true
            }
            Err(e) => {
                update_database_status(false, Some(e.to_string()));
                false
            }
        }
    } else {
        update_database_status(false, Some("Database pool not initialized".to_string()));
        false
    }
}

pub async fn retry_database_connection() -> Result<(), sqlx::Error> {
    log::info!("Intentando reintentar conexión a la base de datos...");
    
    // Intentar cargar configuración segura primero
    let database_url = match load_database_config() {
        Ok(config) => {
            log::info!("Usando configuración segura de base de datos para reintento");
            config.to_connection_string()
        }
        Err(e) => {
            log::warn!("No se pudo cargar la configuración segura para reintento ({}), intentando fallback con .env", e);
            load_env_file();
            
            std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| {
                    log::warn!("DATABASE_URL no encontrado. Usando configuración por defecto.");
                    "mysql://root:@localhost:3306/toscanini_db".to_string()
                })
        }
    };
    
    log::info!("Reintentando conexión a la base de datos con URL: {}", 
        database_url.split('@').next().unwrap_or("***").to_string() + "@***");
    
    match sqlx::MySqlPool::connect(&database_url).await {
        Ok(pool) => {
            // Intentar ejecutar migraciones si es necesario
            if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
                let error_msg = format!("Migration failed during retry: {}", e);
                log::warn!("{}", error_msg);
                // No fallar por migraciones en retry, puede que ya estén aplicadas
            }
            
            // Verificar que la conexión funciona
            match sqlx::query("SELECT 1").execute(&pool).await {
                Ok(_) => {
                    // IMPORTANTE: Reemplazar el pool envuelto en Arc
                    if let Some(pool_arc) = DB_POOL.get() {
                        if let Ok(mut pool_guard) = pool_arc.lock() {
                            *pool_guard = Some(Arc::new(pool));
                            update_database_status(true, None);
                            log::info!("Reintento de conexión a la base de datos exitoso! Pool reemplazado.");
                            return Ok(());
                        }
                    }
                    // Si no se pudo guardar, aún así actualizar el estado
                    update_database_status(true, None);
                    Err(sqlx::Error::Configuration("Failed to store database pool".into()))
                }
                Err(e) => {
                    let error_msg = format!("Connection test failed: {}", e);
                    update_database_status(false, Some(error_msg.clone()));
                    Err(e)
                }
            }
        }
        Err(e) => {
            let error_msg = format!("Retry failed: {}", e);
            log::error!("Error: {}", error_msg);
            update_database_status(false, Some(error_msg));
            Err(e)
        }
    }
}

fn load_env_file() {
    // Lista de posibles ubicaciones del archivo .env
    
    // Crear binding para el path del ejecutable para evitar el temporary value error
    let exe_env_path = std::env::current_exe()
        .ok()
        .and_then(|exe_path| exe_path.parent().map(|p| p.join(".env")));
    
    let exe_env_str = exe_env_path
        .as_deref()
        .and_then(|p| p.to_str())
        .unwrap_or("");

    let possible_paths = vec![
        ".env",                          // Directorio actual
        "../.env",                       // Directorio padre
        "src-tauri/.env",               // Desde el directorio raíz del proyecto
        "./resources/.env",             // En el directorio de recursos (Tauri)
        exe_env_str,                    // Junto al ejecutable
    ];

    for path in possible_paths {
        if !path.is_empty() && Path::new(path).exists() {
            log::debug!("Cargando .env desde: {}", path);
            if let Err(e) = dotenv::from_path(path) {
                log::warn!("No se pudo cargar .env desde {}: {}", path, e);
            } else {
                log::info!("Archivo .env cargado exitosamente desde: {}", path);
                return;
            }
        }
    }

    // Si no se encuentra ningún archivo .env, intentar la carga por defecto
    match dotenv::dotenv() {
        Ok(_) => log::info!("Archivo .env cargado desde la ubicación por defecto"),
        Err(_) => log::warn!("No se encontró archivo .env. Usando variables de entorno o valores por defecto."),
    }
}

// Nueva función mejorada que verifica periódicamente
pub fn start_auto_reconnect_task() {
    tokio::spawn(async move {
        let mut retry_interval = Duration::from_secs(30);
        let check_interval = Duration::from_secs(60); // Verificar cada 60 segundos
        let mut consecutive_failures = 0u32;
        let mut last_check = std::time::Instant::now();
        
        loop {
            // Verificar periódicamente incluso si está conectada
            if last_check.elapsed() >= check_interval {
                let is_connected = check_database_connection().await;
                last_check = std::time::Instant::now();
                
                if !is_connected {
                    log::warn!("Conexión perdida detectada - iniciando reconexión...");
                }
            }
            
            sleep(retry_interval).await;
            
            // Verificar el estado actual
            let status = get_database_status();
            
            // Solo intentar reconectar si no está conectado
            if !status.is_connected {
                log::info!("Auto-reconnect: Intentando reconectar a la base de datos...");
                
                match retry_database_connection().await {
                    Ok(_) => {
                        log::info!("Auto-reconnect: Reconexión exitosa!");
                        retry_interval = Duration::from_secs(30);
                        consecutive_failures = 0;
                    }
                    Err(e) => {
                        consecutive_failures += 1;
                        log::warn!("Auto-reconnect: Fallo en el intento {}: {}", consecutive_failures, e);
                        
                        retry_interval = Duration::from_secs(
                            (30 * (1 << consecutive_failures.min(4))).min(300)
                        );
                    }
                }
            } else {
                retry_interval = Duration::from_secs(30);
                consecutive_failures = 0;
            }
        }
    });
}

/// Inicia una tarea que verifica periódicamente la conexión a la base de datos
/// independientemente de si está conectada o no
pub fn start_periodic_connection_check(interval_seconds: u64) {
    tokio::spawn(async move {
        let interval = Duration::from_secs(interval_seconds);
        
        loop {
            sleep(interval).await;
            
            let is_connected = check_database_connection().await;
            
            if is_connected {
            } else {
                log::warn!("✗ Conexión fallida - intentando reconectar...");
                // Intentar reconectar automáticamente
                if let Err(e) = retry_database_connection().await {
                    log::error!("Error al intentar reconectar: {}", e);
                }
            }
        }
    });
}
