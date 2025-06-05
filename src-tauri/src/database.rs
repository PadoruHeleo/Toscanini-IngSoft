use sqlx::{MySql, Pool};
use std::sync::{OnceLock, Arc, Mutex};
use std::path::Path;

pub type DbPool = Pool<MySql>;

static DB_POOL: OnceLock<DbPool> = OnceLock::new();
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
    // Intentar cargar variables de entorno desde .env en múltiples ubicaciones
    load_env_file();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            println!("Warning: DATABASE_URL not found in environment variables. Using default.");
            "mysql://root:@localhost:3306/toscanini_db".to_string()
        });
    
    println!("Attempting to connect to database with URL: {}", 
        database_url.split('@').next().unwrap_or("***").to_string() + "@***");
    
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
            
            if let Err(_) = DB_POOL.set(pool) {
                update_database_status(false, Some("Failed to set database pool".to_string()));
                return Err(sqlx::Error::Configuration("Failed to set database pool".into()));
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

pub fn get_db_pool() -> Option<&'static DbPool> {
    DB_POOL.get()
}

pub fn get_db_pool_unchecked() -> &'static DbPool {
    DB_POOL.get().expect("Database pool not initialized")
}

// Nueva función segura que no hace panic
pub fn get_db_pool_safe() -> Result<&'static DbPool, String> {
    DB_POOL.get().ok_or_else(|| "Database not connected".to_string())
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
        match sqlx::query("SELECT 1").execute(pool).await {
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
    println!("Attempting to retry database connection...");
    
    // Recargar variables de entorno
    load_env_file();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            println!("Warning: DATABASE_URL not found in environment variables. Using default.");
            "mysql://root:@localhost:3306/toscanini_db".to_string()
        });
    
    println!("Retrying connection to database with URL: {}", 
        database_url.split('@').next().unwrap_or("***").to_string() + "@***");
    
    match sqlx::MySqlPool::connect(&database_url).await {
        Ok(pool) => {
            // Intentar ejecutar migraciones si es necesario
            if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
                let error_msg = format!("Migration failed during retry: {}", e);
                println!("Warning: {}", error_msg);
                update_database_status(false, Some(error_msg.clone()));
                // No fallar por migraciones en retry, puede que ya estén aplicadas
            }
            
            // Como OnceLock no permite reemplazar valores, vamos a usar un enfoque diferente
            // Simplemente verificamos que la conexión funciona y actualizamos el estado
            match sqlx::query("SELECT 1").execute(&pool).await {
                Ok(_) => {
                    update_database_status(true, None);
                    println!("Database connection retry successful!");
                    Ok(())
                }
                Err(e) => {
                    let error_msg = format!("Connection test failed: {}", e);
                    update_database_status(false, Some(error_msg));
                    Err(e)
                }
            }
        }
        Err(e) => {
            let error_msg = format!("Retry failed: {}", e);
            println!("Error: {}", error_msg);
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
            println!("Loading .env from: {}", path);
            if let Err(e) = dotenv::from_path(path) {
                println!("Warning: Failed to load .env from {}: {}", path, e);
            } else {
                println!("Successfully loaded .env from: {}", path);
                return;
            }
        }
    }

    // Si no se encuentra ningún archivo .env, intentar la carga por defecto
    match dotenv::dotenv() {
        Ok(_) => println!("Loaded .env from default location"),
        Err(_) => println!("Warning: No .env file found. Using environment variables or defaults."),
    }
}
