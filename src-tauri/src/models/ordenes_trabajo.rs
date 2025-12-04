use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct OrdenTrabajo {
    #[serde(alias = "id")]
    pub orden_id: i32,
    pub orden_codigo: Option<String>,
    pub orden_desc: Option<String>,
    pub prioridad: Option<String>,
    pub estado: Option<String>,
    pub has_garantia: Option<bool>,
    pub equipo_id: Option<i32>,
    pub cliente_id: Option<i32>, // A veces útil tenerlo directo
    pub created_by: Option<i32>,
    pub cotizacion_id: Option<i32>,
    pub informe_id: Option<i32>,
    pub pre_informe: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateOrdenTrabajoRequest {
    pub orden_desc: String,
    pub prioridad: String,
    pub estado: String,
    pub has_garantia: bool,
    pub equipo_id: i32,
    pub created_by: i32,
    pub pre_informe: Option<String>,
    pub cotizacion_id: Option<i32>,
    pub informe_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateOrdenTrabajoRequest {
    pub orden_codigo: Option<String>,
    pub orden_desc: Option<String>,
    pub prioridad: Option<String>,
    pub estado: Option<String>,
    pub has_garantia: Option<bool>,
    pub equipo_id: Option<i32>,
    pub cotizacion_id: Option<i32>,
    pub informe_id: Option<i32>,
    pub pre_informe: Option<String>,
    pub finished_at: Option<DateTime<Utc>>,
    pub updated_by: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct OrdenTrabajoDetallada {
    pub orden_id: i32,
    pub orden_codigo: Option<String>,
    pub orden_desc: Option<String>,
    pub prioridad: Option<String>,
    pub estado: Option<String>,
    pub has_garantia: Option<bool>,
    pub equipo_id: Option<i32>,
    pub created_by: Option<i32>,
    pub cotizacion_id: Option<i32>,
    pub informe_id: Option<i32>,
    pub pre_informe: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    // Información del equipo
    pub numero_serie: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: Option<String>,
    // Información del cliente (a través del equipo)
    pub cliente_id: Option<i32>,
    pub cliente_nombre: Option<String>,
    // Información del usuario que creó la orden
    pub creador_nombre: Option<String>,
    // Información de cotización
    pub cotizacion_codigo: Option<String>,
    #[sqlx(skip)]
    pub estados: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Filtros {
    pub search: Option<String>,
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
    pub marcas: Option<Vec<String>>,
    pub modelos: Option<Vec<String>>, 
    pub prioridades: Option<Vec<String>>,
    pub clientes: Option<Vec<String>>,
    pub estados: Option<Vec<String>>,
}