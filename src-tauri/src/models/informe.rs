use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Informe {
    pub informe_id: i32,
    pub informe_codigo: Option<String>,
    pub informe_acciones: Option<String>,
    pub informe_obs: Option<String>,
    pub is_borrador: Option<bool>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    // Nuevos campos para compatibilidad con el frontend
    pub diagnostico: Option<String>,
    pub recomendaciones: Option<String>,
    pub solucion_aplicada: Option<String>,
    pub tecnico_responsable: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PiezaInforme {
    pub pieza_id: i32,
    pub informe_id: i32,
    pub cantidad: Option<i32>,
    // Campos adicionales para JOINs
    pub pieza_nombre: Option<String>,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InformeDetallado {
    pub informe_id: i32,
    pub informe_codigo: Option<String>,
    pub informe_acciones: Option<String>,
    pub informe_obs: Option<String>,
    pub is_borrador: Option<bool>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub created_by_nombre: Option<String>,
    // Nuevos campos para compatibilidad con el frontend
    pub diagnostico: Option<String>,
    pub recomendaciones: Option<String>,
    pub solucion_aplicada: Option<String>,
    pub tecnico_responsable: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInformeRequest {
    // informe_codigo se genera automáticamente
    pub informe_acciones: String,
    pub informe_obs: Option<String>,
    pub is_borrador: Option<bool>,
    pub created_by: i32,
    pub piezas: Option<Vec<PiezaInformeRequest>>,
    // Nuevos campos
    pub diagnostico: String,
    pub recomendaciones: Option<String>,
    pub solucion_aplicada: Option<String>,
    pub tecnico_responsable: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateInformeRequest {
    pub informe_codigo: Option<String>,
    pub informe_acciones: Option<String>,
    pub informe_obs: Option<String>,
    pub is_borrador: Option<bool>,
    // Nuevos campos
    pub diagnostico: Option<String>,
    pub recomendaciones: Option<String>,
    pub solucion_aplicada: Option<String>,
    pub tecnico_responsable: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PiezaInformeRequest {
    pub pieza_id: i32,
    pub cantidad: i32,
}