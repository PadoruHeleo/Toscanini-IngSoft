use crate::models::terminos_condiciones::{
    TerminoCondicion, TerminoInforme, TerminoCotizacion, CreateTerminoCondicionRequest
};
use crate::infrastructure::api::client::get_http_client;
use serde_json::json;

fn get_base_url() -> Result<(reqwest::Client, String), String> {
    let (client, api_url) = get_http_client()?;
    Ok((client, format!("{}/terminos", api_url)))
}

pub async fn get_terminos_condiciones(tipo: Option<String>) -> Result<Vec<TerminoCondicion>, String> {
    let (client, base_url) = get_base_url()?;
    let mut req = client.get(&base_url);
    if let Some(t) = tipo {
        req = req.query(&[("tipo", t)]);
    }
    let response = req.send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoCondicion>>().await.map_err(|e| e.to_string())
}

pub async fn get_termino_by_id(termino_id: i32) -> Result<Option<TerminoCondicion>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, termino_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if response.status() == reqwest::StatusCode::NOT_FOUND { return Ok(None); }
    response.json::<Option<TerminoCondicion>>().await.map_err(|e| e.to_string())
}

pub async fn create_termino(request: CreateTerminoCondicionRequest) -> Result<TerminoCondicion, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.post(&base_url).json(&request).send().await.map_err(|e| e.to_string())?;
    if !response.status().is_success() { return Err(response.text().await.unwrap_or_default()); }
    response.json::<TerminoCondicion>().await.map_err(|e| e.to_string())
}

// ... update y delete similares ...

// Asociaciones (Informe)
pub async fn get_terminos_by_informe(informe_id: i32) -> Result<Vec<TerminoInforme>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/informe/{}", base_url, informe_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoInforme>>().await.map_err(|e| e.to_string())
}

pub async fn add_termino_to_informe(informe_id: i32, termino_desc: String, added_by: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/informe/add", base_url);
    let body = json!({ "informe_id": informe_id, "termino_desc": termino_desc, "added_by": added_by });
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    Ok(response.status().is_success())
}

// Asociaciones (Cotización)
pub async fn get_terminos_by_cotizacion(cotizacion_id: i32) -> Result<Vec<TerminoCotizacion>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/cotizacion/{}", base_url, cotizacion_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoCotizacion>>().await.map_err(|e| e.to_string())
}

pub async fn add_termino_to_cotizacion(cotizacion_id: i32, termino_desc: String, added_by: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/cotizacion/add", base_url);
    let body = json!({ "cotizacion_id": cotizacion_id, "termino_desc": termino_desc, "added_by": added_by });
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    Ok(response.status().is_success())
}

// Estos métodos se pueden dejar vacíos o implementar un "apply default" en la API si se desea
pub async fn apply_default_terminos_to_informe(_informe_id: i32, _created_by: i32) -> Result<(), String> { Ok(()) }
pub async fn apply_default_terminos_to_cotizacion(_cotizacion_id: i32, _created_by: i32) -> Result<(), String> { Ok(()) }