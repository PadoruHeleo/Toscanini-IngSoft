use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cotizacion {
    pub cotizacion_id: i32,
    pub cotizacion_codigo: Option<String>,
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub costo_total: Option<i32>,
    pub is_aprobada: Option<bool>,
    pub is_borrador: Option<bool>,
    pub informe: String,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Pieza {
    pub pieza_id: i32,
    pub pieza_nombre: Option<String>,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
    pub pieza_stock: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PiezaCotizacion {
    pub pieza_id: i32,
    pub cotizacion_id: i32,
    pub cantidad: Option<i32>,
    // Campos adicionales para JOINs
    pub pieza_nombre: Option<String>,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CotizacionDetallada {
    pub cotizacion_id: i32,
    pub cotizacion_codigo: Option<String>,
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub costo_total: Option<i32>,
    pub is_aprobada: Option<bool>,
    pub is_borrador: Option<bool>,
    pub informe: String,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub created_by_nombre: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCotizacionRequest {
    // pub cotizacion_codigo: String, // Eliminar este campo
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub costo_total: Option<i32>,
    pub is_aprobada: Option<bool>,
    pub is_borrador: Option<bool>,
    pub informe: String,
    pub created_by: i32,
    pub piezas: Option<Vec<PiezaCotizacionRequest>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCotizacionRequest {
    pub cotizacion_codigo: Option<String>,
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub costo_total: Option<i32>,
    pub is_aprobada: Option<bool>,
    pub is_borrador: Option<bool>,
    pub informe: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePiezaRequest {
    pub pieza_nombre: String,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PiezaCotizacionRequest {
    pub pieza_id: i32,
    pub cantidad: i32,
}

#[derive(Debug, Deserialize)]
pub struct OrdenInfoRow {
    pub orden_codigo: Option<String>,
    pub estado: String,
}