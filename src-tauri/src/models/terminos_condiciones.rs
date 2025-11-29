use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TerminoCondicion {
    pub termino_id: i32,
    pub termino_nombre: String,
    pub termino_descripcion: String,
    pub is_active: Option<bool>,
    pub tipo_referencia: String, // 'informe', 'cotizacion', 'ambos'
    pub is_default: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TerminoInforme {
    pub termino_id: i32,
    pub informe_id: i32,
    pub aplicado: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    // Campos adicionales para JOINs
    pub termino_nombre: Option<String>,
    pub termino_descripcion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TerminoCotizacion {
    pub termino_id: i32,
    pub cotizacion_id: i32,
    pub aplicado: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    // Campos adicionales para JOINs
    pub termino_nombre: Option<String>,
    pub termino_descripcion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTerminoCondicionRequest {
    pub termino_nombre: String,
    pub termino_descripcion: String,
    pub tipo_referencia: String,
    pub is_default: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTerminoCondicionRequest {
    pub termino_nombre: Option<String>,
    pub termino_descripcion: Option<String>,
    pub is_active: Option<bool>,
    pub tipo_referencia: Option<String>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TerminoInformeRequest {
    pub termino_id: i32,
    pub aplicado: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TerminoCotizacionRequest {
    pub termino_id: i32,
    pub aplicado: Option<bool>,
}