use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Equipo {
    pub equipo_id: i32,
    pub numero_serie: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_ubicacion: Option<String>,
    pub cliente_id: Option<i32>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateEquipoRequest {
    pub numero_serie: String,
    pub equipo_marca: String,
    pub equipo_modelo: String,
    pub equipo_tipo: String,
    pub equipo_precio: Option<i32>,
    pub equipo_ubicacion: Option<String>,
    pub cliente_id: i32,
    pub created_by: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateEquipoRequest {
    pub numero_serie: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_ubicacion: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FiltrosEquipos {
    pub cliente_id: Option<Vec<i32>>,
    pub tipo: Option<Vec<String>>,
    pub marca: Option<Vec<String>>,
    pub ubicacion: Option<Vec<String>>,
    pub estado: Option<Vec<bool>>, // activo/inactivo
    pub search: Option<String>,
    pub ordenamiento: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteEquipoRequest {
    pub equipo_id: i32,
    pub deleted_by: i32,
    pub motivo: Option<String>,
}