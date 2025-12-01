use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub log_id: i32,
    pub log_accion: Option<String>,
    pub log_usuario_id: Option<i32>,
    pub log_entidad_tabla: Option<String>,
    pub log_entidad_id: Option<i32>,
    pub log_prev_v: Option<String>,
    pub log_new_v: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AuditLogWithUser {
    pub log_id: i32,
    pub log_accion: Option<String>,
    pub log_usuario_id: Option<i32>,
    pub log_entidad_tabla: Option<String>,
    pub log_entidad_id: Option<i32>,
    pub log_prev_v: Option<String>,
    pub log_new_v: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub usuario_nombre: Option<String>,
    pub usuario_correo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAuditLogRequest {
    pub log_accion: String,
    pub log_usuario_id: Option<i32>,
    pub log_entidad_tabla: String,
    pub log_entidad_id: Option<i32>,
    pub log_prev_v: Option<String>,
    pub log_new_v: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogFilters {
    pub usuario_id: Option<i32>,
    pub entidad_tabla: Option<Vec<String>>,  
    pub accion: Option<Vec<String>>,          
    pub search: Option<String>,               
    pub fecha_desde: Option<String>,
    pub fecha_hasta: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// Alias for compatibility if needed, or just use LogFilters
pub type FiltrosLogs = LogFilters;
