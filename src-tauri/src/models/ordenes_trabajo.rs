use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct OrdenTrabajo {
    pub orden_id: i32,
    pub orden_codigo: Option<String>,
    pub orden_desc: Option<String>,
    pub prioridad: Option<String>,
    pub estado: Option<String>,
    pub has_garantia: Option<bool>,
    pub equipo_id: Option<i32>,
    pub cliente_id: Option<i32>, // A veces útil tenerlo directo
    pub created_by: Option<i32>,
    pub cotizacion_id: Option<i32>,
    pub informe_id: Option<i32>,
    pub pre_informe: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateOrdenTrabajoRequest {
    pub orden_desc: String,
    pub prioridad: String,
    pub estado: String,
    pub has_garantia: bool,
    pub equipo_id: i32,
    pub created_by: i32,
    pub pre_informe: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateOrdenTrabajoRequest {
    pub orden_desc: Option<String>,
    pub prioridad: Option<String>,
    pub has_garantia: Option<bool>,
    pub pre_informe: Option<String>,
    pub updated_by: i32,
}