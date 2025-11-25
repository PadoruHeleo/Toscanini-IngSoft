use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cliente {
    pub cliente_id: i32,
    pub cliente_rut: Option<String>,
    pub cliente_nombre: Option<String>,
    pub cliente_correo: Option<String>,
    pub cliente_telefono: Option<String>,
    pub cliente_direccion: Option<String>,
    pub is_active: Option<bool>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateClienteRequest {
    pub cliente_rut: String,
    pub cliente_nombre: String,
    pub cliente_correo: String,
    pub cliente_telefono: Option<String>,
    pub cliente_direccion: Option<String>,
    pub created_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClienteRequest {
    pub cliente_rut: Option<String>,
    pub cliente_nombre: Option<String>,
    pub cliente_correo: Option<String>,
    pub cliente_telefono: Option<String>,
    pub cliente_direccion: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FiltrosClientes {
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
    pub correo: Option<Vec<String>>,
    pub rut: Option<Vec<String>>,    
    pub ciudad: Option<Vec<String>>,
    pub search: Option<String>,
    pub estado: Option<Vec<bool>>,    
    pub ordenamiento: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cotizacion {
    pub cotizacion_id: i32,
    pub cliente_id: i32,
    pub fecha: Option<DateTime<Utc>>,
    pub total: Option<f64>,
    pub estado: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrdenTrabajo {
    pub orden_id: i32,
    pub cliente_id: i32,
    pub fecha_inicio: Option<DateTime<Utc>>,
    pub fecha_fin: Option<DateTime<Utc>>,
    pub estado: Option<String>,
    pub descripcion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteClienteRequest {
    pub cliente_id: i32,           // ID del cliente a inactivar
    pub deleted_by: i32,           // ID del usuario que elimina
    pub motivo: Option<String>,    // Motivo de inactivación
}