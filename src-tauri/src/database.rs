use sqlx::{MySql, Pool};
use std::sync::OnceLock;

pub type DbPool = Pool<MySql>;

static DB_POOL: OnceLock<DbPool> = OnceLock::new();

pub async fn init_database() -> Result<(), sqlx::Error> {
    // Cargar variables de entorno desde .env
    dotenv::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://user:password@localhost:3306/database_name".to_string());
    
    let pool = sqlx::MySqlPool::connect(&database_url).await?;
    
    // Ejecutar migraciones si es necesario
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    DB_POOL.set(pool).map_err(|_| {
        sqlx::Error::Configuration("Failed to set database pool".into())
    })?;
    
    Ok(())
}

pub fn get_db_pool() -> Option<&'static DbPool> {
    DB_POOL.get()
}

pub fn get_db_pool_unchecked() -> &'static DbPool {
    DB_POOL.get().expect("Database pool not initialized")
}
