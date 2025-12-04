

use crate::models::terminos_condiciones::{
    TerminoCondicion, CreateTerminoCondicionRequest, UpdateTerminoCondicionRequest,
    TerminoInforme, TerminoCotizacion, TerminoInformeRequest, TerminoCotizacionRequest
};
use crate::infrastructure::api::client::get_http_client;
use serde_json::json;

fn get_base_url() -> Result<(reqwest::Client, String), String> {
    let (client, api_url) = get_http_client()?;
    // Corregido según API_MAPPING_TERMINOS.md: Prefijo es /terminos
    Ok((client, format!("{}/terminos", api_url)))
}

pub async fn get_terminos_condiciones() -> Result<Vec<TerminoCondicion>, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.get(&base_url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoCondicion>>().await.map_err(|e| e.to_string())
}

pub async fn get_terminos_condiciones_activos() -> Result<Vec<TerminoCondicion>, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.get(&base_url).send().await.map_err(|e| e.to_string())?;
    let terminos = response.json::<Vec<TerminoCondicion>>().await.map_err(|e| e.to_string())?;
    Ok(terminos.into_iter().filter(|t| t.is_active.unwrap_or(false)).collect())
}

pub async fn get_terminos_condiciones_by_tipo(tipo: String) -> Result<Vec<TerminoCondicion>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}?tipo={}", base_url, tipo);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoCondicion>>().await.map_err(|e| e.to_string())
}

pub async fn get_terminos_condiciones_default(tipo: String) -> Result<Vec<TerminoCondicion>, String> {
    let terminos = get_terminos_condiciones_by_tipo(tipo).await?;
    Ok(terminos.into_iter().filter(|t| t.is_default.unwrap_or(false)).collect())
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

pub async fn create_termino_condicion(request: CreateTerminoCondicionRequest, created_by: i32) -> Result<i32, String> {
    let (client, base_url) = get_base_url()?;
    let mut body = serde_json::to_value(&request).map_err(|e| e.to_string())?;
    if let Some(obj) = body.as_object_mut() {
        obj.insert("created_by".to_string(), json!(created_by));
    }

    let response = client.post(&base_url).json(&body).send().await.map_err(|e| e.to_string())?;
    
    let termino: TerminoCondicion = response.json().await.map_err(|e| e.to_string())?;
    Ok(termino.termino_id)
}

pub async fn update_termino_condicion(termino_id: i32, request: UpdateTerminoCondicionRequest, updated_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, termino_id);
    
    let mut body = serde_json::to_value(&request).map_err(|e| e.to_string())?;
    if let Some(obj) = body.as_object_mut() {
        obj.insert("updated_by".to_string(), json!(updated_by));
    }

    let response = client.put(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error updating termino: {}", response.status()))
    }
}

pub async fn delete_termino_condicion(termino_id: i32, deleted_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/delete", base_url);
    let body = json!({
        "termino_id": termino_id,
        "deleted_by": deleted_by
    });
    
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
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

pub async fn apply_terminos_to_informe(informe_id: i32, terminos: Vec<TerminoInformeRequest>, applied_by: i32) -> Result<(), String> {
    for termino in terminos {
        create_termino_informe_relation(termino.termino_id, informe_id, Some(true), applied_by).await?;
    }
    Ok(())
}

pub async fn apply_terminos_to_cotizacion(cotizacion_id: i32, terminos: Vec<TerminoCotizacionRequest>, applied_by: i32) -> Result<(), String> {
    for termino in terminos {
        create_termino_cotizacion_relation(termino.termino_id, cotizacion_id, Some(true), applied_by).await?;
    }
    Ok(())
}

pub async fn apply_default_terminos_to_informe(informe_id: i32, applied_by: i32) -> Result<(), String> {
    let defaults = get_terminos_condiciones_default("informe".to_string()).await?;
    for term in defaults {
        create_termino_informe_relation(term.termino_id, informe_id, Some(true), applied_by).await?;
    }
    Ok(())
}

pub async fn apply_default_terminos_to_cotizacion(cotizacion_id: i32, applied_by: i32) -> Result<(), String> {
    let defaults = get_terminos_condiciones_default("cotizacion".to_string()).await?;
    println!("🔍 Aplicando {} términos por defecto a cotización {}", defaults.len(), cotizacion_id);
    
    for term in defaults {
        if let Err(e) = create_termino_cotizacion_relation(term.termino_id, cotizacion_id, Some(true), applied_by).await {
            println!("❌ Error aplicando término {} a cotización {}: {}", term.termino_id, cotizacion_id, e);
            // No retornamos error para intentar aplicar los siguientes
        }
    }
    Ok(())
}

pub async fn reactivate_termino_condicion(termino_id: i32, updated_by: i32) -> Result<(), String> {
    let request = UpdateTerminoCondicionRequest {
        termino_nombre: None,
        termino_descripcion: None,
        tipo_referencia: None,
        is_default: None,
        is_active: Some(true),
    };
    update_termino_condicion(termino_id, request, updated_by).await
}

pub async fn toggle_termino_default(termino_id: i32, is_default: bool, updated_by: i32) -> Result<(), String> {
    let request = UpdateTerminoCondicionRequest {
        termino_nombre: None,
        termino_descripcion: None,
        tipo_referencia: None,
        is_default: Some(is_default),
        is_active: None,
    };
    update_termino_condicion(termino_id, request, updated_by).await
}

pub async fn create_termino_informe_relation(termino_id: i32, informe_id: i32, _aplicado: Option<bool>, created_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/informe", base_url);
    let body = json!({
        "informe_id": informe_id,
        "termino_id": termino_id,
        "added_by": created_by
    });
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error adding term to informe: {}", response.status()))
    }
}

pub async fn create_termino_cotizacion_relation(termino_id: i32, cotizacion_id: i32, _aplicado: Option<bool>, created_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/cotizacion", base_url);
    let body = json!({
        "cotizacion_id": cotizacion_id,
        "termino_id": termino_id,
        "added_by": created_by
    });
    
    println!("🔗 Asociando término {} a cotización {} en URL: {}", termino_id, cotizacion_id, url);
    
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_msg = response.text().await.unwrap_or_default();
        // Si el error es por duplicado, lo ignoramos y retornamos Ok
        if error_msg.contains("Duplicate") || error_msg.contains("ER_DUP_ENTRY") || status == reqwest::StatusCode::CONFLICT {
            println!("⚠️ Término {} ya existe en cotización {}, ignorando error.", termino_id, cotizacion_id);
            return Ok(());
        }
        
        println!("❌ Error API términos: {}", error_msg);
        Err(format!("Error adding term to cotizacion: {}", error_msg))
    }
}

pub async fn update_termino_informe_relation(_termino_id: i32, _informe_id: i32, _aplicado: bool, _updated_by: i32) -> Result<(), String> {
    Err("Update relation not supported directly via API".to_string())
}

pub async fn update_termino_cotizacion_relation(_termino_id: i32, _cotizacion_id: i32, _aplicado: bool, _updated_by: i32) -> Result<(), String> {
    Err("Update relation not supported directly via API".to_string())
}

pub async fn delete_termino_informe_relation(termino_id: i32, informe_id: i32, _deleted_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/informe/remove", base_url);
    let body = json!({
        "informe_id": informe_id,
        "termino_id": termino_id
    });
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error removing term from informe: {}", response.status()))
    }
}

pub async fn delete_termino_cotizacion_relation(termino_id: i32, cotizacion_id: i32, _deleted_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/cotizacion/remove", base_url);
    let body = json!({
        "cotizacion_id": cotizacion_id,
        "termino_id": termino_id
    });
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error removing term from cotizacion: {}", response.status()))
    }
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