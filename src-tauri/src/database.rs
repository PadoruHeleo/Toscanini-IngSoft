use sqlx::{MySql, Pool, mysql::{MySqlConnectOptions, MySqlPool}};
use std::sync::{OnceLock, Arc, Mutex};
use std::path::Path;
use crate::config::{load_database_config, DatabaseConfig, parse_database_url};
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

/// Construye opciones de conexión MySQL con soporte SSL si está configurado
async fn build_mysql_connect_options(config: &DatabaseConfig, include_database: bool) -> Result<MySqlConnectOptions, sqlx::Error> {
    let mut options = MySqlConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.username)
        .password(&config.password);
    
    if include_database {
        options = options.database(&config.database);
    }
    
    // Configurar SSL si los certificados están disponibles
    if config.ssl_ca.is_some() || config.ssl_cert.is_some() || config.ssl_key.is_some() {
        use sqlx::mysql::MySqlSslMode;
        options = options.ssl_mode(MySqlSslMode::Required);
        
        // Verificar que los certificados existen antes de intentar conectarse
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
        
        println!("SSL enabled for database connection");
    }
    
    Ok(options)
}

/// Conecta a MySQL usando las opciones configuradas (con soporte SSL)
async fn connect_mysql_pool(config: &DatabaseConfig, include_database: bool) -> Result<Pool<MySql>, sqlx::Error> {
    let options = build_mysql_connect_options(config, include_database).await?;
    MySqlPool::connect_with(options).await
}

pub async fn init_database() -> Result<(), sqlx::Error> {
    // Intentar cargar configuración segura primero
    let config = match load_database_config() {
        Ok(config) => {
            println!("Loaded secure database configuration");
            config
        }
        Err(e) => {
            println!("Warning: Could not load secure config ({}), trying .env fallback", e);
            // Fallback a la carga tradicional de .env
            load_env_file();
            
            // Intentar parsear desde DATABASE_URL o usar valores por defecto
            if let Ok(database_url) = std::env::var("DATABASE_URL") {
                match parse_database_url(&database_url) {
                    Ok(cfg) => cfg,
                    Err(_) => {
                        println!("Warning: Could not parse DATABASE_URL. Using default configuration.");
                        DatabaseConfig::default()
                    }
                }
            } else {
                println!("Warning: DATABASE_URL not found. Using default configuration.");
                DatabaseConfig::default()
            }
        }
    };
    
    let database_name = config.database.clone();
    
    println!("Attempting to connect to database server: {}@{}:{}", 
        config.username, config.host, config.port);
    
    // Inicializar DB_POOL si no existe
    if DB_POOL.get().is_none() {
        let _ = DB_POOL.set(Arc::new(Mutex::new(None)));
    }
    
    // Inicializar el estado de conexión si no existe
    if DB_CONNECTION_STATUS.get().is_none() {
        let _ = DB_CONNECTION_STATUS.set(Arc::new(Mutex::new(DatabaseStatus::default())));
    }
    
    // Primero intentar conectarse con la base de datos especificada
    match connect_mysql_pool(&config, true).await {
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
            // Si el error es que la base de datos no existe, intentar crearla
            if e.to_string().contains("Unknown database") || e.to_string().contains("1049") {
                println!("Database '{}' does not exist. Attempting to create it...", database_name);
                
                // Conectarse sin especificar la base de datos
                match connect_mysql_pool(&config, false).await {
                    Ok(admin_pool) => {
                        // Crear la base de datos
                        let create_db_query = format!("CREATE DATABASE IF NOT EXISTS `{}`", database_name);
                        match sqlx::query(&create_db_query).execute(&admin_pool).await {
                            Ok(_) => {
                                println!("Database '{}' created successfully", database_name);
                                // Cerrar la conexión administrativa
                                admin_pool.close().await;
                                
                                // Ahora intentar conectarse a la base de datos recién creada
                                match connect_mysql_pool(&config, true).await {
                                    Ok(pool) => {
                                        // Ejecutar migraciones
                                        if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
                                            let error_msg = format!("Migration failed: {}", e);
                                            update_database_status(false, Some(error_msg.clone()));
                                            return Err(sqlx::Error::Configuration(error_msg.into()));
                                        }
                                        
                                        // Guardar el pool
                                        if let Some(pool_arc) = DB_POOL.get() {
                                            if let Ok(mut pool_guard) = pool_arc.lock() {
                                                *pool_guard = Some(Arc::new(pool));
                                            }
                                        }
                                        
                                        update_database_status(true, None);
                                        Ok(())
                                    }
                                    Err(e2) => {
                                        let error_msg = format!("Failed to connect to newly created database: {}", e2);
                                        update_database_status(false, Some(error_msg.clone()));
                                        Err(e2)
                                    }
                                }
                            }
                            Err(create_err) => {
                                let error_msg = format!("Failed to create database '{}': {}", database_name, create_err);
                                println!("Error: {}", error_msg);
                                update_database_status(false, Some(error_msg.clone()));
                                Err(create_err)
                            }
                        }
                    }
                    Err(admin_err) => {
                        let error_msg = format!("Failed to connect to database server (cannot create database): {}", admin_err);
                        println!("Error: {}", error_msg);
                        update_database_status(false, Some(error_msg.clone()));
                        Err(admin_err)
                    }
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
    println!("Attempting to retry database connection...");
    
    // Intentar cargar configuración segura primero
    let config = match load_database_config() {
        Ok(config) => {
            println!("Using secure database configuration for retry");
            config
        }
        Err(e) => {
            println!("Warning: Could not load secure config for retry ({}), trying .env fallback", e);
            load_env_file();
            
            // Intentar parsear desde DATABASE_URL o usar valores por defecto
            if let Ok(database_url) = std::env::var("DATABASE_URL") {
                match parse_database_url(&database_url) {
                    Ok(cfg) => cfg,
                    Err(_) => {
                        println!("Warning: Could not parse DATABASE_URL. Using default configuration.");
                        DatabaseConfig::default()
                    }
                }
            } else {
                println!("Warning: DATABASE_URL not found. Using default configuration.");
                DatabaseConfig::default()
            }
        }
    };
    
    let database_name = config.database.clone();
    
    println!("Retrying connection to database server: {}@{}:{}", 
        config.username, config.host, config.port);
    
    match connect_mysql_pool(&config, true).await {
        Ok(pool) => {
            // Intentar ejecutar migraciones si es necesario
            if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
                let error_msg = format!("Migration failed during retry: {}", e);
                println!("Warning: {}", error_msg);
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
                            println!("Database connection retry successful! Pool replaced.");
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
            // Si el error es que la base de datos no existe, intentar crearla
            if e.to_string().contains("Unknown database") || e.to_string().contains("1049") {
                println!("Database '{}' does not exist during retry. Attempting to create it...", database_name);
                
                // Conectarse sin especificar la base de datos
                match connect_mysql_pool(&config, false).await {
                    Ok(admin_pool) => {
                        // Crear la base de datos
                        let create_db_query = format!("CREATE DATABASE IF NOT EXISTS `{}`", database_name);
                        match sqlx::query(&create_db_query).execute(&admin_pool).await {
                            Ok(_) => {
                                println!("Database '{}' created successfully during retry", database_name);
                                // Cerrar la conexión administrativa
                                admin_pool.close().await;
                                
                                // Ahora intentar conectarse a la base de datos recién creada
                                match connect_mysql_pool(&config, true).await {
                                    Ok(pool) => {
                                        // Ejecutar migraciones
                                        if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
                                            let error_msg = format!("Migration failed during retry: {}", e);
                                            println!("Warning: {}", error_msg);
                                        }
                                        
                                        // Verificar que la conexión funciona
                                        match sqlx::query("SELECT 1").execute(&pool).await {
                                            Ok(_) => {
                                                // Guardar el pool
                                                if let Some(pool_arc) = DB_POOL.get() {
                                                    if let Ok(mut pool_guard) = pool_arc.lock() {
                                                        *pool_guard = Some(Arc::new(pool));
                                                        update_database_status(true, None);
                                                        println!("Database connection retry successful after creating database!");
                                                        return Ok(());
                                                    }
                                                }
                                                update_database_status(true, None);
                                                Err(sqlx::Error::Configuration("Failed to store database pool".into()))
                                            }
                                            Err(e2) => {
                                                let error_msg = format!("Connection test failed after creating database: {}", e2);
                                                update_database_status(false, Some(error_msg.clone()));
                                                Err(e2)
                                            }
                                        }
                                    }
                                    Err(e2) => {
                                        let error_msg = format!("Failed to connect to newly created database during retry: {}", e2);
                                        update_database_status(false, Some(error_msg.clone()));
                                        Err(e2)
                                    }
                                }
                            }
                            Err(create_err) => {
                                let error_msg = format!("Failed to create database '{}' during retry: {}", database_name, create_err);
                                println!("Error: {}", error_msg);
                                update_database_status(false, Some(error_msg.clone()));
                                Err(create_err)
                            }
                        }
                    }
                    Err(admin_err) => {
                        let error_msg = format!("Retry failed: {}", admin_err);
                        println!("Error: {}", error_msg);
                        update_database_status(false, Some(error_msg));
                        Err(admin_err)
                    }
                }
            } else {
                let error_msg = format!("Retry failed: {}", e);
                println!("Error: {}", error_msg);
                update_database_status(false, Some(error_msg));
                Err(e)
            }
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
                    println!("Conexión perdida detectada - iniciando reconexión...");
                }
            }
            
            sleep(retry_interval).await;
            
            // Verificar el estado actual
            let status = get_database_status();
            
            // Solo intentar reconectar si no está conectado
            if !status.is_connected {
                println!("Auto-reconnect: Intentando reconectar a la base de datos...");
                
                match retry_database_connection().await {
                    Ok(_) => {
                        println!("Auto-reconnect: Reconexión exitosa!");
                        retry_interval = Duration::from_secs(30);
                        consecutive_failures = 0;
                    }
                    Err(e) => {
                        consecutive_failures += 1;
                        println!("Auto-reconnect: Fallo en el intento {}: {}", consecutive_failures, e);
                        
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
                println!("✗ Conexión fallida - intentando reconectar...");
                // Intentar reconectar automáticamente
                if let Err(e) = retry_database_connection().await {
                    println!("Error al intentar reconectar: {}", e);
                }
            }
        }
    });
}
