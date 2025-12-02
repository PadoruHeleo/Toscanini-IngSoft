use crate::models::equipos::{
    Equipo, CreateEquipoRequest, UpdateEquipoRequest, RegistrarSalidaRequest, 
    SalidaEquipoResponse, EquipoWithCliente, FiltrosEquipos, EquipoConEstado
};
use crate::infrastructure::api::client::get_http_client;

fn get_base_url() -> Result<(reqwest::Client, String), String> {
    let (client, api_url) = get_http_client()?;
    Ok((client, format!("{}/equipos", api_url)))
}

pub async fn get_equipos() -> Result<Vec<Equipo>, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.get(&base_url).send().await.map_err(|e| e.to_string())?;
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

pub async fn search_equipos(search_term: String) -> Result<Vec<Equipo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/search/query", base_url);
    let response = client.get(&url).query(&[("term", search_term)]).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<Equipo>>().await.map_err(|e| e.to_string())
}

pub async fn create_equipo(request: CreateEquipoRequest) -> Result<Equipo, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.post(&base_url).json(&request).send().await.map_err(|e| e.to_string())?;
    response.json::<Equipo>().await.map_err(|e| e.to_string())
}

pub async fn update_equipo(equipo_id: i32, request: UpdateEquipoRequest, _updated_by: i32) -> Result<Option<Equipo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, equipo_id);
    let response = client.put(&url).json(&request).send().await.map_err(|e| e.to_string())?;
    if response.status() == reqwest::StatusCode::NOT_FOUND { return Ok(None); }
    response.json::<Option<Equipo>>().await.map_err(|e| e.to_string())
}

pub async fn delete_equipo(equipo_id: i32, deleted_by: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/delete", base_url);
    let body = serde_json::json!({
        "equipo_id": equipo_id,
        "deleted_by": deleted_by,
        "motivo": "Eliminado desde aplicación de escritorio"
    });
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    Ok(response.status().is_success())
}

// Implementations for missing functions

pub async fn get_equipo_by_numero_serie(_numero_serie: String) -> Result<Option<Equipo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_equipos_by_tipo(_equipo_tipo: String) -> Result<Vec<Equipo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_equipos_by_created_by(_created_by: i32) -> Result<Vec<Equipo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn count_equipos() -> Result<i64, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_equipos_with_pagination(_offset: i64, _limit: i64) -> Result<Vec<Equipo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_equipos_stats_by_tipo() -> Result<Vec<(String, i64)>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_equipos_by_price_range(_min_price: Option<i32>, _max_price: Option<i32>) -> Result<Vec<Equipo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_equipos_with_cliente() -> Result<Vec<EquipoWithCliente>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn transfer_equipo_to_cliente(_equipo_id: i32, _new_cliente_id: i32, _updated_by: i32) -> Result<bool, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_equipos_marcas() -> Result<Vec<String>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/list/marcas", base_url);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<String>>().await.map_err(|e| e.to_string())
}

pub async fn get_equipos_modelos_by_marca(_marca: String) -> Result<Vec<String>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_equipos_ubicaciones() -> Result<Vec<String>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/list/ubicaciones", base_url);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<String>>().await.map_err(|e| e.to_string())
}

pub async fn registrar_salida_equipo(_request: RegistrarSalidaRequest) -> Result<SalidaEquipoResponse, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn puede_registrar_salida_equipo(_equipo_id: i32) -> Result<(bool, String), String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn equipo_esta_en_sistema(_equipo_id: i32) -> Result<(bool, String), String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_equipos_filtrados(filtros: FiltrosEquipos) -> Result<Vec<EquipoConEstado>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/filter", base_url);
    let response = client.post(&url).json(&filtros).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<EquipoConEstado>>().await.map_err(|e| e.to_string())
}

pub async fn get_equipos_en_sistema() -> Result<Vec<Equipo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_equipos_fuera_sistema() -> Result<Vec<Equipo>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_estadisticas_equipos_sistema() -> Result<serde_json::Value, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_equipos_con_estado() -> Result<Vec<EquipoConEstado>, String> {
    let filtros = FiltrosEquipos {
        fecha_inicio: None,
        fecha_fin: None,
        marcas: None,
        modelos: None,
        tipos: None,
        clientes: None,
        ubicaciones: None,
        estados_orden: None,
        search: None,
        ordenamiento: Some("fecha_desc".to_string()),
        precio_min: None,
        precio_max: None,
    };
    get_equipos_filtrados(filtros).await
}

pub async fn get_clientes_con_equipos() -> Result<Vec<String>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_tipos_equipos() -> Result<Vec<String>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/list/tipos", base_url);
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
    response.json::<Vec<(String, i64)>>().await.map_err(|e| e.to_string())
}