
use serde::Deserialize;
use crate::models::terminos_condiciones::{
    TerminoCondicion, CreateTerminoCondicionRequest, UpdateTerminoCondicionRequest,
    TerminoInforme, TerminoCotizacion, TerminoInformeRequest, TerminoCotizacionRequest
};
use crate::infrastructure::api::client::get_http_client;

fn get_base_url() -> Result<(reqwest::Client, String), String> {
    let (client, api_url) = get_http_client()?;
    Ok((client, format!("{}/terminos-condiciones", api_url)))
}

pub async fn get_terminos_condiciones() -> Result<Vec<TerminoCondicion>, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.get(&base_url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoCondicion>>().await.map_err(|e| e.to_string())
}

pub async fn get_terminos_condiciones_activos() -> Result<Vec<TerminoCondicion>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/activos", base_url);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoCondicion>>().await.map_err(|e| e.to_string())
}

pub async fn get_terminos_condiciones_by_tipo(tipo: String) -> Result<Vec<TerminoCondicion>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/tipo/{}", base_url, tipo);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoCondicion>>().await.map_err(|e| e.to_string())
}

pub async fn get_terminos_condiciones_default(tipo: String) -> Result<Vec<TerminoCondicion>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/default/{}", base_url, tipo);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoCondicion>>().await.map_err(|e| e.to_string())
}

pub async fn get_termino_condicion_by_id(termino_id: i32) -> Result<Option<TerminoCondicion>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, termino_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    let termino = response.json::<TerminoCondicion>().await.map_err(|e| e.to_string())?;
    Ok(Some(termino))
}

pub async fn create_termino_condicion(request: CreateTerminoCondicionRequest, _created_by: i32) -> Result<i32, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.post(&base_url).json(&request).send().await.map_err(|e| e.to_string())?;
    
    #[derive(Deserialize)]
    struct CreateResponse { id: i32 }
    let res: CreateResponse = response.json().await.map_err(|e| e.to_string())?;
    Ok(res.id)
}

pub async fn update_termino_condicion(termino_id: i32, request: UpdateTerminoCondicionRequest, _updated_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, termino_id);
    let response = client.put(&url).json(&request).send().await.map_err(|e| e.to_string())?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error updating termino: {}", response.status()))
    }
}

pub async fn delete_termino_condicion(termino_id: i32, _deleted_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, termino_id);
    let response = client.delete(&url).send().await.map_err(|e| e.to_string())?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error deleting termino: {}", response.status()))
    }
}

pub async fn get_terminos_by_informe(informe_id: i32) -> Result<Vec<TerminoInforme>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/informe/{}", base_url, informe_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoInforme>>().await.map_err(|e| e.to_string())
}

pub async fn get_terminos_by_cotizacion(cotizacion_id: i32) -> Result<Vec<TerminoCotizacion>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/cotizacion/{}", base_url, cotizacion_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoCotizacion>>().await.map_err(|e| e.to_string())
}

pub async fn apply_terminos_to_informe(informe_id: i32, terminos: Vec<TerminoInformeRequest>, _applied_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/informe/{}/apply", base_url, informe_id);
    let response = client.post(&url).json(&terminos).send().await.map_err(|e| e.to_string())?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error applying terms to informe: {}", response.status()))
    }
}

pub async fn apply_terminos_to_cotizacion(cotizacion_id: i32, terminos: Vec<TerminoCotizacionRequest>, _applied_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/cotizacion/{}/apply", base_url, cotizacion_id);
    let response = client.post(&url).json(&terminos).send().await.map_err(|e| e.to_string())?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error applying terms to cotizacion: {}", response.status()))
    }
}

pub async fn apply_default_terminos_to_informe(informe_id: i32, _applied_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/informe/{}/apply-default", base_url, informe_id);
    let response = client.post(&url).send().await.map_err(|e| e.to_string())?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error applying default terms to informe: {}", response.status()))
    }
}

pub async fn apply_default_terminos_to_cotizacion(cotizacion_id: i32, _applied_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/cotizacion/{}/apply-default", base_url, cotizacion_id);
    let response = client.post(&url).send().await.map_err(|e| e.to_string())?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error applying default terms to cotizacion: {}", response.status()))
    }
}

pub async fn reactivate_termino_condicion(termino_id: i32, _reactivated_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}/reactivate", base_url, termino_id);
    let response = client.post(&url).send().await.map_err(|e| e.to_string())?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error reactivating termino: {}", response.status()))
    }
}

pub async fn toggle_termino_default(termino_id: i32, is_default: bool, _updated_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}/default", base_url, termino_id);
    let body = serde_json::json!({ "is_default": is_default });
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error toggling default status: {}", response.status()))
    }
}

pub async fn create_termino_informe_relation(_termino_id: i32, _informe_id: i32, _aplicado: Option<bool>, _created_by: i32) -> Result<(), String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn create_termino_cotizacion_relation(_termino_id: i32, _cotizacion_id: i32, _aplicado: Option<bool>, _created_by: i32) -> Result<(), String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn update_termino_informe_relation(_termino_id: i32, _informe_id: i32, _aplicado: bool, _updated_by: i32) -> Result<(), String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn update_termino_cotizacion_relation(_termino_id: i32, _cotizacion_id: i32, _aplicado: bool, _updated_by: i32) -> Result<(), String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn delete_termino_informe_relation(_termino_id: i32, _informe_id: i32, _deleted_by: i32) -> Result<(), String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn delete_termino_cotizacion_relation(_termino_id: i32, _cotizacion_id: i32, _deleted_by: i32) -> Result<(), String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_informes_by_termino(_termino_id: i32) -> Result<Vec<TerminoInforme>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn get_cotizaciones_by_termino(_termino_id: i32) -> Result<Vec<TerminoCotizacion>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn check_termino_in_informe(_termino_id: i32, _informe_id: i32) -> Result<Option<bool>, String> {
    Err("Not implemented via API yet".to_string())
}

pub async fn check_termino_in_cotizacion(_termino_id: i32, _cotizacion_id: i32) -> Result<Option<bool>, String> {
    Err("Not implemented via API yet".to_string())
}