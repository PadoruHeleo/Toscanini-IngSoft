use crate::models::ordenes_trabajo::{
    OrdenTrabajo, CreateOrdenTrabajoRequest, UpdateOrdenTrabajoRequest,
    OrdenTrabajoDetallada, Filtros
};
use crate::infrastructure::api::client::get_http_client;

fn get_base_url() -> Result<(reqwest::Client, String), String> {
    let (client, api_url) = get_http_client()?;
    Ok((client, format!("{}/ordenes-trabajo", api_url)))
}

pub async fn get_ordenes_trabajo() -> Result<Vec<OrdenTrabajo>, String> {
    let (client, base_url) = get_base_url()?;
    println!("🔍 Llamando a: {}", base_url);
    let response = client.get(&base_url).send().await.map_err(|e| {
        let error_msg = format!("Error de conexión: {}", e);
        println!("❌ {}", error_msg);
        error_msg
    })?;
    
    let status = response.status();
    println!("📊 Status code: {}", status);
    
    if !status.is_success() {
        let error_body = response.text().await.unwrap_or_else(|_| "No se pudo leer el cuerpo del error".to_string());
        let error_msg = format!("Error API ({}): {}", status, error_body);
        println!("❌ {}", error_msg);
        return Err(error_msg);
    }
    
    response.json::<Vec<OrdenTrabajo>>().await.map_err(|e| {
        let error_msg = format!("Error parseando JSON: {}", e);
        println!("❌ {}", error_msg);
        error_msg
    })
}

pub async fn get_orden_trabajo_by_id(orden_id: i32) -> Result<Option<OrdenTrabajo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, orden_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if response.status() == reqwest::StatusCode::NOT_FOUND { return Ok(None); }
    response.json::<Option<OrdenTrabajo>>().await.map_err(|e| e.to_string())
}

pub async fn create_orden_trabajo(request: CreateOrdenTrabajoRequest) -> Result<OrdenTrabajo, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.post(&base_url).json(&request).send().await.map_err(|e| e.to_string())?;
    response.json::<OrdenTrabajo>().await.map_err(|e| e.to_string())
}

pub async fn update_orden_trabajo(orden_id: i32, request: UpdateOrdenTrabajoRequest, _updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, orden_id);
    let response = client.put(&url).json(&request).send().await.map_err(|e| e.to_string())?;
    if response.status() == reqwest::StatusCode::NOT_FOUND { return Ok(None); }
    response.json::<Option<OrdenTrabajo>>().await.map_err(|e| e.to_string())
}

pub async fn delete_orden_trabajo(orden_id: i32, _deleted_by: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, orden_id);
    let response = client.delete(&url).send().await.map_err(|e| e.to_string())?;
    Ok(response.status().is_success())
}

pub async fn get_ordenes_trabajo_by_cliente(cliente_id: i32) -> Result<Vec<OrdenTrabajo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/cliente/{}", base_url, cliente_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<OrdenTrabajo>>().await.map_err(|e| e.to_string())
}

pub async fn get_ordenes_trabajo_by_equipo(equipo_id: i32) -> Result<Vec<OrdenTrabajo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/equipo/{}", base_url, equipo_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<OrdenTrabajo>>().await.map_err(|e| e.to_string())
}

// Stubs for missing functions

pub async fn get_orden_trabajo_by_codigo(_orden_codigo: String) -> Result<Option<OrdenTrabajo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_ordenes_trabajo_by_estado(_estado: String) -> Result<Vec<OrdenTrabajo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_ordenes_trabajo_by_prioridad(_prioridad: String) -> Result<Vec<OrdenTrabajo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_ordenes_trabajo_by_usuario(_usuario_id: i32) -> Result<Vec<OrdenTrabajo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_ordenes_trabajo_detalladas() -> Result<Vec<OrdenTrabajoDetallada>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_orden_trabajo_detallada_by_id(_orden_id: i32) -> Result<Option<OrdenTrabajoDetallada>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn cambiar_estado_orden_trabajo(_orden_id: i32, _nuevo_estado: String, _updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn asignar_cotizacion_orden_trabajo(_orden_id: i32, _cotizacion_id: i32, _updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn asignar_informe_orden_trabajo(_orden_id: i32, _informe_id: i32, _updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_ordenes_trabajo_stats() -> Result<serde_json::Value, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn search_ordenes_trabajo(_search_term: String) -> Result<Vec<OrdenTrabajoDetallada>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn send_orden_trabajo_notification(_orden_id: i32, _sent_by: i32) -> Result<bool, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_orden_trabajo_by_informe_id(_informe_id: i32) -> Result<Option<OrdenTrabajo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_ordenes_trabajo_filtradas(_filtros: Filtros) -> Result<Vec<OrdenTrabajoDetallada>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_modelos_disponibles() -> Result<Vec<String>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_marcas_disponibles() -> Result<Vec<String>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_clientes_disponibles() -> Result<Vec<String>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn remove_cotizacion_from_ordenes(_cotizacion_id: i32, _updated_by: i32) -> Result<bool, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn remove_informe_from_ordenes(_informe_id: i32, _updated_by: i32) -> Result<bool, String> {
    Err("Not implemented via API yet".to_string())
}