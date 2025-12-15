use sqlx::{MySql, Pool, mysql::{MySqlConnectOptions, MySqlPool}};
use std::sync::{OnceLock, Arc, Mutex};
use std::path::Path;
use crate::config::{load_database_config, DatabaseConfig, parse_database_url};
// Eliminamos imports de ssh_tunnel
use std::time::Duration;
use tokio::time::sleep;

pub type DbPool = Pool<MySql>;

static DB_POOL: OnceLock<Arc<Mutex<Option<Arc<DbPool>>>>> = OnceLock::new();
static DB_CONNECTION_STATUS: OnceLock<Arc<Mutex<DatabaseStatus>>> = OnceLock::new();
// Eliminamos static SSH_TUNNEL

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

/// Función auxiliar para verificar existencia de certificados SSL
fn verify_ssl_certificates(config: &DatabaseConfig) -> Result<(), sqlx::Error> {
    if let Some(ssl_ca_path) = &config.ssl_ca {
        if !Path::new(ssl_ca_path).exists() {
            return Err(sqlx::Error::Configuration(
                format!("SSL CA certificate file not found: {}", ssl_ca_path).into()
            ));
        }
    }
    
    if let Some(ssl_cert_path) = &config.ssl_cert {
        if !Path::new(ssl_cert_path).exists() {
            return Err(sqlx::Error::Configuration(
                format!("SSL client certificate file not found: {}", ssl_cert_path).into()
            ));
        }
    }
    
    if let Some(ssl_key_path) = &config.ssl_key {
        if !Path::new(ssl_key_path).exists() {
            return Err(sqlx::Error::Configuration(
                format!("SSL client key file not found: {}", ssl_key_path).into()
            ));
        }
    }
    
    Ok(())
}

/// Construye opciones de conexión MySQL directas (sin SSH)
async fn build_mysql_connect_options(config: &DatabaseConfig, include_database: bool) -> Result<MySqlConnectOptions, sqlx::Error> {
    use sqlx::mysql::MySqlSslMode;
    
    println!("[DB] Construyendo opciones de conexión MySQL (Directa)");
    
    // Conexión directa usando la configuración
    let mut options = MySqlConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.username)
        .statement_cache_capacity(0); 
    
    // Solo agregar password si NO está vacía
    if !config.password.is_empty() {
        options = options.password(&config.password);
    } else {
        println!("[DB] Conectando sin contraseña (configuración default)");
    }
    
    if include_database {
        options = options.database(&config.database);
    }
    
    // Configuración SSL (mantenemos esto por si usas SSL localmente o en prod sin SSH)
    let ssl_mode_env = std::env::var("MYSQL_SSL_MODE").ok();
    
    match ssl_mode_env.as_deref() {
        Some("DISABLED") | Some("disabled") => {
            options = options.ssl_mode(MySqlSslMode::Disabled);
        }
        Some("PREFERRED") | Some("preferred") => {
            options = options.ssl_mode(MySqlSslMode::Preferred);
        }
        Some("REQUIRED") | Some("required") => {
            options = options.ssl_mode(MySqlSslMode::Required);
            verify_ssl_certificates(config)?;
        }
        Some("VERIFY_CA") | Some("verify_ca") => {
            options = options.ssl_mode(MySqlSslMode::VerifyCa);
            verify_ssl_certificates(config)?;
        }
        Some("VERIFY_IDENTITY") | Some("verify_identity") => {
            options = options.ssl_mode(MySqlSslMode::VerifyIdentity);
            verify_ssl_certificates(config)?;
        }
        _ => {
            // Por defecto DISABLED para desarrollo local simple, a menos que haya certs
            if config.ssl_ca.is_some() {
                options = options.ssl_mode(MySqlSslMode::Preferred);
            } else {
                options = options.ssl_mode(MySqlSslMode::Disabled);
            }
        }
    }
    
    Ok(options)
}

// Eliminamos ensure_ssh_tunnel y close_ssh_tunnel

/// Conecta a MySQL directamente
async fn connect_mysql_pool(config: &DatabaseConfig, include_database: bool) -> Result<Pool<MySql>, sqlx::Error> {
    println!("[DB] Iniciando conexión a MySQL en {}:{}", config.host, config.port);
    
    let options = build_mysql_connect_options(config, include_database).await?;
    
    match MySqlPool::connect_with(options).await {
        Ok(pool) => {
            println!("[DB] ✓ Conexión a MySQL exitosa");
            Ok(pool)
        }
        Err(e) => {
            println!("[DB] ❌ ERROR conectando a MySQL: {}", e);
            Err(e)
        }
    }
}

pub async fn init_database() -> Result<(), sqlx::Error> {
    // Cargar configuración
    let config = match load_database_config() {
        Ok(config) => config,
        Err(e) => {
            println!("Warning: Could not load secure config ({}), trying .env fallback", e);
            load_env_file();
            // Fallback a variables de entorno básicas
            if let Ok(database_url) = std::env::var("DATABASE_URL") {
                parse_database_url(&database_url).unwrap_or_else(|_| DatabaseConfig::default())
            } else {
                DatabaseConfig::default()
            }
        }
    };
    
    let database_name = config.database.clone();
    
    println!("Connecting to database: {}@{}:{}/{}", 
        config.username, config.host, config.port, config.database);
    
    // Inicializar Singletons si no existen
    if DB_POOL.get().is_none() { let _ = DB_POOL.set(Arc::new(Mutex::new(None))); }
    if DB_CONNECTION_STATUS.get().is_none() { let _ = DB_CONNECTION_STATUS.set(Arc::new(Mutex::new(DatabaseStatus::default()))); }
    
    // Intento de conexión
    match connect_mysql_pool(&config, true).await {
        Ok(pool) => {
            // Migraciones automáticas
            if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
                let error_msg = format!("Migration failed: {}", e);
                update_database_status(false, Some(error_msg.clone()));
                return Err(sqlx::Error::Configuration(error_msg.into()));
            }
            
            // Guardar pool
            if let Some(pool_arc) = DB_POOL.get() {
                if let Ok(mut pool_guard) = pool_arc.lock() {
                    *pool_guard = Some(Arc::new(pool));
                }
            }
            
            update_database_status(true, None);
            Ok(())
        }
        Err(e) => {
            // Lógica de creación de DB si no existe (Error 1049)
            if e.to_string().contains("Unknown database") || e.to_string().contains("1049") {
                println!("Database '{}' does not exist. Creating...", database_name);
                
                // Conectar sin DB para crearla
                match connect_mysql_pool(&config, false).await {
                    Ok(admin_pool) => {
                        let create_query = format!("CREATE DATABASE IF NOT EXISTS `{}`", database_name);
                        sqlx::query(&create_query).execute(&admin_pool).await?;
                        admin_pool.close().await;
                        
                        // Reintentar conexión completa
                        let pool = connect_mysql_pool(&config, true).await?;
                        
                        // Migraciones
                        if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
                            return Err(sqlx::Error::Configuration(format!("Migration failed: {}", e).into()));
                        }
                        
                        if let Some(pool_arc) = DB_POOL.get() {
                            if let Ok(mut pool_guard) = pool_arc.lock() {
                                *pool_guard = Some(Arc::new(pool));
                            }
                        }
                        update_database_status(true, None);
                        Ok(())
                    }
                    Err(e) => Err(e)
                }
            } else {
                update_database_status(false, Some(e.to_string()));
                Err(e)
            }
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
    println!("Reintentando conexión a DB...");
    
    // Eliminada llamada a close_ssh_tunnel()
    
    let config = match load_database_config() {
        Ok(c) => c,
        Err(_) => {
            load_env_file();
            if let Ok(url) = std::env::var("DATABASE_URL") {
                parse_database_url(&url).unwrap_or_else(|_| DatabaseConfig::default())
            } else {
                DatabaseConfig::default()
            }
        }
    };
    
    match connect_mysql_pool(&config, true).await {
        Ok(pool) => {
            // Verificar conexión
            if sqlx::query("SELECT 1").execute(&pool).await.is_ok() {
                if let Some(pool_arc) = DB_POOL.get() {
                    if let Ok(mut pool_guard) = pool_arc.lock() {
                        *pool_guard = Some(Arc::new(pool));
                        update_database_status(true, None);
                        println!("Reconexión exitosa.");
                        return Ok(());
                    }
                }
            }
            Err(sqlx::Error::Configuration("Failed to verify reconnection".into()))
        }
        Err(e) => {
            update_database_status(false, Some(e.to_string()));
            Err(e)
        }
    }
}

fn load_env_file() {
    let exe_env_path = std::env::current_exe()
        .ok()
        .and_then(|exe_path| exe_path.parent().map(|p| p.join(".env")));
    
    let possible_paths = vec![
        ".env",
        "../.env",
        "src-tauri/.env",
        exe_env_path.as_deref().and_then(|p| p.to_str()).unwrap_or(""),
    ];

    for path in possible_paths {
        if !path.is_empty() && Path::new(path).exists() {
            println!("Cargando .env desde: {}", path);
            dotenv::from_path(path).ok();
            return;
        }
    }
    dotenv::dotenv().ok();
}

pub fn start_auto_reconnect_task() {
    tokio::spawn(async move {
        let mut retry_interval = Duration::from_secs(10);
        
        loop {
            sleep(retry_interval).await;
            
            let status = get_database_status();
            
            if !status.is_connected {
                println!("Auto-reconnect: Intentando...");
                // Eliminada llamada a close_ssh_tunnel()
                match retry_database_connection().await {
                    Ok(_) => retry_interval = Duration::from_secs(30),
                    Err(_) => retry_interval = Duration::from_secs(10),
                }
            }
        }
    });
}

pub fn start_periodic_connection_check(interval_seconds: u64) {
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(interval_seconds)).await;
            if !check_database_connection().await {
                println!("Chequeo periódico falló. Iniciando reintento...");
                // Eliminada llamada a close_ssh_tunnel()
                let _ = retry_database_connection().await;
            }
        }
    });
}
