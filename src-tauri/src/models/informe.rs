use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Informe {
    pub informe_id: i32,
    pub informe_codigo: Option<String>,
    pub orden_id: Option<i32>,
    pub informe_acciones: Option<String>,
    pub informe_obs: Option<String>,
    pub diagnostico: Option<String>,
    pub recomendaciones: Option<String>,
    pub solucion_aplicada: Option<String>,
    pub tecnico_responsable: Option<String>,
    pub is_borrador: Option<bool>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PiezaInforme {
    pub pieza_id: i32,
    pub informe_id: i32,
    pub cantidad: Option<i32>,
    pub pieza_nombre: Option<String>,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateInformeRequest {
    pub informe_codigo: String,
    pub orden_id: i32,
    pub informe_acciones: String,
    pub informe_obs: String,
    pub diagnostico: String,
    pub recomendaciones: String,
    pub solucion_aplicada: String,
    pub tecnico_responsable: String,
    pub created_by: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateInformeRequest {
    pub informe_acciones: Option<String>,
    pub informe_obs: Option<String>,
    pub diagnostico: Option<String>,
    pub recomendaciones: Option<String>,
    pub solucion_aplicada: Option<String>,
    pub tecnico_responsable: Option<String>,
    pub updated_by: i32,
}