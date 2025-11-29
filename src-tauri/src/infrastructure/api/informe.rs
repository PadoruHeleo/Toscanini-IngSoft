use crate::models::informe::{
    Informe, PiezaInforme, CreateInformeRequest, UpdateInformeRequest
};
use crate::infrastructure::api::client::get_http_client;
use serde_json::json;

fn get_base_url() -> Result<(reqwest::Client, String), String> {
    let (client, api_url) = get_http_client()?;
    Ok((client, format!("{}/informes", api_url)))
}

pub async fn get_informes() -> Result<Vec<Informe>, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.get(&base_url).send().await.map_err(|e| e.to_string())?;
    if !response.status().is_success() { return Err(format!("Error API: {}", response.status())); }
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
    if !response.status().is_success() { return Err(response.text().await.unwrap_or_default()); }
    response.json::<Informe>().await.map_err(|e| e.to_string())
}

pub async fn update_informe(informe_id: i32, request: UpdateInformeRequest, updated_by: i32) -> Result<Option<Informe>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, informe_id);
    
    let mut body_json = serde_json::to_value(&request).map_err(|e| e.to_string())?;
    if let Some(obj) = body_json.as_object_mut() {
        obj.insert("updated_by".to_string(), json!(updated_by));
    }

    let response = client.put(&url).json(&body_json).send().await.map_err(|e| e.to_string())?;
    if response.status() == reqwest::StatusCode::NOT_FOUND { return Ok(None); }
    response.json::<Option<Informe>>().await.map_err(|e| e.to_string())
}

pub async fn delete_informe(informe_id: i32, deleted_by: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/delete", base_url);
    let body = json!({ "informe_id": informe_id, "deleted_by": deleted_by });
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    Ok(response.status().is_success())
}

pub async fn get_piezas_informe(informe_id: i32) -> Result<Vec<PiezaInforme>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}/piezas", base_url, informe_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<PiezaInforme>>().await.map_err(|e| e.to_string())
}

// Nota: update_piezas_informe no estaba en tu código original DB pero es útil
// Si necesitas implementarlo:
// pub async fn update_piezas_informe(...) -> ...

pub async fn send_informe_to_client(informe_id: i32, sent_by: i32, cliente_email: String) -> Result<bool, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/registrar-envio", base_url);
    let body = json!({ "informe_id": informe_id, "sent_by": sent_by, "destinatario": cliente_email });
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    Ok(response.status().is_success())
}

pub async fn get_informes_by_cliente(cliente_id: i32) -> Result<Vec<Informe>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/cliente/{}", base_url, cliente_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<Informe>>().await.map_err(|e| e.to_string())
}

pub async fn get_informes_by_equipo(equipo_id: i32) -> Result<Vec<Informe>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/equipo/{}", base_url, equipo_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<Informe>>().await.map_err(|e| e.to_string())
}

pub async fn get_informe_pdf_data(informe_id: i32) -> Result<serde_json::Value, String> {
    // Retorna un JSON genérico porque la estructura de PDF es compleja
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}/pdf-data", base_url, informe_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<serde_json::Value>().await.map_err(|e| e.to_string())
}