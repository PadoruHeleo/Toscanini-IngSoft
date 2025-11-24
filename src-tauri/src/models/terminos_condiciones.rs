use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct TerminoCondicion {
    pub termino_id: i32,
    pub termino_nombre: String,
    pub termino_descripcion: String,
    pub is_active: Option<bool>,
    pub tipo_referencia: String,
    pub is_default: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct TerminoInforme {
    pub id: Option<i32>, // ID autoincremental de la tabla relación
    pub termino_id: Option<i32>, // Puede ser null si es texto libre
    pub informe_id: i32,
    pub termino_desc: String, // El texto snapshot
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct TerminoCotizacion {
    pub id: Option<i32>,
    pub termino_id: Option<i32>,
    pub cotizacion_id: i32,
    pub termino_desc: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateTerminoRequest {
    pub termino_nombre: String,
    pub termino_descripcion: String,
    pub tipo_referencia: String,
    pub is_default: bool,
    pub created_by: i32,
}