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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateEquipoRequest {
    pub numero_serie: String,
    pub equipo_marca: String,
    pub equipo_modelo: String,
    pub equipo_tipo: String, // 'radio', 'antena', 'repetidor', 'otro'
    pub equipo_precio: Option<i32>,
    pub equipo_ubicacion: Option<String>,
    pub cliente_id: i32,
    pub created_by: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateEquipoRequest {
    pub numero_serie: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_ubicacion: Option<String>,
    pub cliente_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrarSalidaRequest {
    pub equipo_id: i32,
    pub orden_trabajo_id: Option<i32>,
    pub motivo_salida: String, // 'entregado_cliente', 'retirado_sin_reparacion', 'abandonado', 'baja_definitiva'
    pub observaciones: Option<String>,
    pub usuario_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SalidaEquipoResponse {
    pub success: bool,
    pub message: String,
    pub nuevo_estado: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EquipoConEstado {
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
    // Información del cliente
    pub cliente_nombre: Option<String>,
    // Estado de la última orden de trabajo
    pub ultimo_estado_orden: Option<String>,
    pub ultimo_codigo_orden: Option<String>,
    pub fecha_ultima_orden: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FiltrosEquipos {
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
    pub marcas: Option<Vec<String>>,
    pub modelos: Option<Vec<String>>,
    pub tipos: Option<Vec<String>>,
    pub clientes: Option<Vec<String>>,
    pub ubicaciones: Option<Vec<String>>,
    pub estados_orden: Option<Vec<String>>, // Estados de órdenes de trabajo
    pub search: Option<String>,
    pub ordenamiento: Option<String>,
    pub precio_min: Option<i32>,
    pub precio_max: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteEquipoRequest {
    pub equipo_id: i32,
    pub deleted_by: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EquipoWithCliente {
    pub equipo_id: i32,
    pub numero_serie: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_ubicacion: Option<String>,
    pub cliente_id: Option<i32>,
    pub cliente_nombre: Option<String>,
    pub cliente_correo: Option<String>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}
