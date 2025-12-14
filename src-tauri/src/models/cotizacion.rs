use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cotizacion {
    #[serde(alias = "id")]
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
    pub cotizacion_id: Option<i32>,
    pub cantidad: Option<i32>,
    // Campos adicionales para JOINs
    pub pieza_nombre: Option<String>,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
    pub pieza_stock: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCotizacionRequest {
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub costo_total: Option<i32>,
    pub is_aprobada: Option<bool>,
    pub is_borrador: Option<bool>,
    pub informe: String,
    pub created_by: i32,
    pub piezas: Option<Vec<PiezaCotizacionRequest>>,
    pub orden_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCotizacionRequest {
    pub cotizacion_codigo: Option<String>,
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub costo_total: Option<i32>,
    pub is_aprobada: Option<bool>,
    pub is_borrador: Option<bool>,
    pub informe: Option<String>,
    pub piezas: Option<Vec<PiezaCotizacionRequest>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePiezaRequest {
    pub pieza_nombre: String,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
    pub pieza_stock: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PiezaCotizacionRequest {
    pub pieza_id: i32,
    pub cantidad: i32,
}

#[derive(Debug, Deserialize)]
pub struct OrdenInfoRow {
    pub orden_codigo: Option<String>,
    pub estado: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePiezaRequest {
    pub pieza_nombre: Option<String>,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
    pub pieza_stock: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InventarioEquipo {
    #[serde(alias = "inventario_equipo_id")]
    pub equipo_id: i32,
    pub equipo_codigo: Option<String>,
    pub equipo_nombre: Option<String>,
    pub equipo_tipo: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_descripcion: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_stock: Option<i32>,
    pub numero_serie: Option<String>,
    pub equipo_estado: Option<String>,
    pub equipo_ubicacion: Option<String>,
    pub fecha_adquisicion: Option<DateTime<Utc>>,
    pub proveedor: Option<String>,
    pub garantia_vencimiento: Option<DateTime<Utc>>,
    pub observaciones: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventarioEquipoRequest {
    pub equipo_codigo: Option<String>,
    pub equipo_nombre: Option<String>,
    pub equipo_tipo: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_descripcion: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_stock: Option<i32>,
    pub numero_serie: Option<String>,
    pub equipo_estado: Option<String>,
    pub equipo_ubicacion: Option<String>,
    pub fecha_adquisicion: Option<DateTime<Utc>>,
    pub proveedor: Option<String>,
    pub garantia_vencimiento: Option<DateTime<Utc>>,
    pub observaciones: Option<String>,
    pub created_by: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SalidaEquipo {
    pub salida_id: i32,
    pub orden_trabajo_id: i32,
    pub motivo_salida: String,
    pub fecha_salida: Option<DateTime<Utc>>,
    pub usuario_id: Option<i32>,
    pub observaciones: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    // Campos adicionales para JOINs
    pub orden_codigo: Option<String>,
    pub equipo_nombre: Option<String>,
    pub cliente_nombre: Option<String>,
    pub usuario_nombre: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrarSalidaRequest {
    pub orden_trabajo_id: i32,
    pub motivo_salida: String,
    pub observaciones: Option<String>,
    pub usuario_id: i32,
}