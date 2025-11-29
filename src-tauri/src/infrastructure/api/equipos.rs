use crate::models::equipos::{
    Equipo, CreateEquipoRequest, UpdateEquipoRequest, FiltrosEquipos, DeleteEquipoRequest
};
use crate::infrastructure::api::client::get_http_client;
use serde_json::json;

fn get_base_url() -> Result<(reqwest::Client, String), String> {
    let (client, api_url) = get_http_client()?;
    Ok((client, format!("{}/equipos", api_url)))
}

pub async fn get_equipos() -> Result<Vec<Equipo>, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.get(&base_url).send().await.map_err(|e| e.to_string())?;
    if !response.status().is_success() { return Err(format!("Error API: {}", response.status())); }
    response.json::<Vec<Equipo>>().await.map_err(|e| e.to_string())
}

pub async fn get_equipo_by_id(equipo_id: i32) -> Result<Option<Equipo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, equipo_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if response.status() == reqwest::StatusCode::NOT_FOUND { return Ok(None); }
    response.json::<Option<Equipo>>().await.map_err(|e| e.to_string())
}

pub async fn get_equipos_by_cliente(cliente_id: i32) -> Result<Vec<Equipo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/cliente/{}", base_url, cliente_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<Equipo>>().await.map_err(|e| e.to_string())
}

pub async fn create_equipo(request: CreateEquipoRequest) -> Result<Equipo, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.post(&base_url).json(&request).send().await.map_err(|e| e.to_string())?;
    if !response.status().is_success() { return Err(response.text().await.unwrap_or_default()); }
    response.json::<Equipo>().await.map_err(|e| e.to_string())
}

pub async fn update_equipo(equipo_id: i32, request: UpdateEquipoRequest, updated_by: i32) -> Result<Option<Equipo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, equipo_id);
    
    let mut body_json = serde_json::to_value(&request).map_err(|e| e.to_string())?;
    if let Some(obj) = body_json.as_object_mut() {
        obj.insert("updated_by".to_string(), json!(updated_by));
    }

    let response = client.put(&url).json(&body_json).send().await.map_err(|e| e.to_string())?;
    if response.status() == reqwest::StatusCode::NOT_FOUND { return Ok(None); }
    response.json::<Option<Equipo>>().await.map_err(|e| e.to_string())
}

pub async fn delete_equipo(request: DeleteEquipoRequest) -> Result<bool, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/delete", base_url);
    let response = client.post(&url).json(&request).send().await.map_err(|e| e.to_string())?;
    Ok(response.status().is_success())
}

pub async fn search_equipos(search_term: String) -> Result<Vec<Equipo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/search/query", base_url);
    let response = client.get(&url).query(&[("term", search_term)]).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<Equipo>>().await.map_err(|e| e.to_string())
}

pub async fn get_equipos_filtrados(filtros: FiltrosEquipos) -> Result<Vec<Equipo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/filter", base_url);
    let response = client.post(&url).json(&filtros).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<Equipo>>().await.map_err(|e| e.to_string())
}

// Listas auxiliares
pub async fn get_tipos_equipos() -> Result<Vec<String>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/list/tipos", base_url);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<String>>().await.map_err(|e| e.to_string())
}

pub async fn get_marcas_equipos() -> Result<Vec<String>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/list/marcas", base_url);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<String>>().await.map_err(|e| e.to_string())
}

pub async fn get_ubicaciones_equipos() -> Result<Vec<String>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/list/ubicaciones", base_url);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<String>>().await.map_err(|e| e.to_string())
}

pub async fn get_estados_ordenes_trabajo() -> Result<Vec<String>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/list/estados-ot", base_url);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<String>>().await.map_err(|e| e.to_string())
}

pub async fn get_estadisticas_equipos_por_estado() -> Result<Vec<(String, i64)>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/stats/por-estado", base_url);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    // La API devuelve [{estado: "X", cantidad: 5}], Rust espera Vec<(String, i64)>
    // Necesitamos un struct temporal para mapear
    #[derive(serde::Deserialize)]
    struct StatRow { estado: String, cantidad: i64 }
    
    let rows: Vec<StatRow> = response.json().await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|r| (r.estado, r.cantidad)).collect())
}