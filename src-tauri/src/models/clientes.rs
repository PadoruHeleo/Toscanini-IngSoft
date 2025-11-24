use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateClienteRequest {
    pub cliente_rut: String,
    pub cliente_nombre: String,
    pub cliente_correo: String,
    pub cliente_telefono: Option<String>,
    pub cliente_direccion: Option<String>,
    pub created_by: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateClienteRequest {
    pub cliente_rut: Option<String>,
    pub cliente_nombre: Option<String>,
    pub cliente_correo: Option<String>,
    pub cliente_telefono: Option<String>,
    pub cliente_direccion: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteClienteRequest {
    pub cliente_id: i32,
    pub deleted_by: i32,
    pub motivo: Option<String>,
}

// Structs auxiliares para resultados de queries (Distincts)
#[derive(Debug, FromRow)]
pub struct RutResult {
    pub cliente_rut: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct CorreoResult {
    pub cliente_correo: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct CiudadResult {
    pub cliente_direccion: Option<String>,
}