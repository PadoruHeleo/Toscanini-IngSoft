use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Cotizacion {
    pub cotizacion_id: i32,
    pub cotizacion_codigo: Option<String>,
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub costo_total: Option<i32>,
    pub is_aprobada: Option<bool>,
    pub is_borrador: Option<bool>,
    pub informe: Option<i32>, // ID del informe (int en BD)
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Pieza {
    pub pieza_id: i32,
    pub pieza_nombre: Option<String>,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
    pub pieza_stock: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PiezaCotizacion {
    pub pieza_id: i32,
    pub cotizacion_id: i32,
    pub cantidad: i32,
    // Campos de join
    pub pieza_nombre: Option<String>,
    pub pieza_marca: Option<String>,
    pub pieza_precio: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateCotizacionRequest {
    pub cotizacion_codigo: String,
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub informe_id: i32,
    pub created_by: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateCotizacionRequest {
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub updated_by: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdatePiezasCotizacionRequest {
    pub piezas: Vec<PiezaInput>,
    pub updated_by: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PiezaInput {
    pub pieza_id: i32,
    pub cantidad: i32,
}