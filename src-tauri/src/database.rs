use sqlx::{MySql, Pool};
use std::sync::{OnceLock, Arc, Mutex};

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
    // Cargar variables de entorno desde .env
    dotenv::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://user:password@localhost:3306/database_name".to_string());
    
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
