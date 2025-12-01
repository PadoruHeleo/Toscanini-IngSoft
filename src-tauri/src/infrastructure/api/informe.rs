use crate::models::informe::{Informe, InformeDetallado, CreateInformeRequest, UpdateInformeRequest, PiezaInforme};

use crate::infrastructure::api::client::get_http_client;

fn get_base_url() -> Result<(reqwest::Client, String), String> {
    let (client, api_url) = get_http_client()?;
    Ok((client, format!("{}/informes", api_url)))
}

pub async fn get_informes() -> Result<Vec<Informe>, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.get(&base_url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<Informe>>().await.map_err(|e| e.to_string())
}

pub async fn get_informe_by_id(informe_id: i32) -> Result<Option<Informe>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, informe_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if response.status() == reqwest::StatusCode::NOT_FOUND { return Ok(None); }
    response.json::<Option<Informe>>().await.map_err(|e| e.to_string())
}

pub async fn create_informe(request: CreateInformeRequest) -> Result<Informe, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.post(&base_url).json(&request).send().await.map_err(|e| e.to_string())?;
    response.json::<Informe>().await.map_err(|e| e.to_string())
}

pub async fn update_informe(informe_id: i32, request: UpdateInformeRequest, _updated_by: i32) -> Result<Option<Informe>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, informe_id);
    let response = client.put(&url).json(&request).send().await.map_err(|e| e.to_string())?;
    if response.status() == reqwest::StatusCode::NOT_FOUND { return Ok(None); }
    response.json::<Option<Informe>>().await.map_err(|e| e.to_string())
}

pub async fn delete_informe(informe_id: i32, _deleted_by: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, informe_id);
    let response = client.delete(&url).send().await.map_err(|e| e.to_string())?;
    Ok(response.status().is_success())
}

pub async fn get_informes_by_orden(orden_id: i32) -> Result<Vec<Informe>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/orden/{}", base_url, orden_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<Informe>>().await.map_err(|e| e.to_string())
}

// Stubs for missing functions

pub async fn get_informes_detallados() -> Result<Vec<InformeDetallado>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_informe_by_codigo(_informe_codigo: String) -> Result<Option<Informe>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn rechazar_informe_borrador(_informe_id: i32, _motivo: String, _updated_by: i32) -> Result<bool, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn search_informes(_search_term: String) -> Result<Vec<InformeDetallado>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn count_informes() -> Result<i64, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_informes_with_pagination(_offset: i64, _limit: i64) -> Result<Vec<InformeDetallado>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_piezas_informe(_informe_id: i32) -> Result<Vec<PiezaInforme>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn send_informe_to_client(_informe_id: i32, _sent_by: i32) -> Result<bool, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_informes_by_cliente(_cliente_id: i32) -> Result<Vec<Informe>, String> {
    Err("Not implemented via API yet".to_string())
}