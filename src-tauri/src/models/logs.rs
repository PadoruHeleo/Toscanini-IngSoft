use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Option<i32>,
    pub log_accion: String,
    pub log_usuario_id: Option<i32>,
    pub log_entidad_tabla: String,
    pub log_entidad_id: Option<i32>,
    pub log_prev_v: Option<String>,
    pub log_new_v: Option<String>,
    pub created_at: Option<String>,
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
pub struct FiltrosLogs {
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
    pub usuario_id: Option<i32>,
    pub accion: Option<String>,
    pub entidad_tabla: Option<String>,
}
