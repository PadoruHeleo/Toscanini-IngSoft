
use serde::Deserialize;
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
    // Asumiendo que el filtro se pasa por query param ?is_active=true o similar, 
    // pero el mapping dice "Soporta filtro ?tipo=...". 
    // Si no hay endpoint específico /activos en el mapping, usamos el filtro del GET /
    // Sin embargo, mantendremos la lógica existente si el backend lo soporta, 
    // o nos ajustamos al mapping estricto. El mapping no menciona /activos explícitamente como ruta,
    // pero GET / dice "Obtiene términos activos".
    // Intentaremos usar GET / con query params si es necesario, pero por ahora asumimos que el endpoint base devuelve activos por defecto o todos.
    // Si el mapping dice "Obtiene términos activos", entonces GET / debería ser suficiente.
    let response = client.get(&base_url).send().await.map_err(|e| e.to_string())?;
    let terminos = response.json::<Vec<TerminoCondicion>>().await.map_err(|e| e.to_string())?;
    // Filtramos en cliente si la API devuelve todos, o confiamos en la API.
    // Para seguridad, filtramos aquí también.
    Ok(terminos.into_iter().filter(|t| t.is_active.unwrap_or(false)).collect())
}

pub async fn get_terminos_condiciones_by_tipo(tipo: String) -> Result<Vec<TerminoCondicion>, String> {
    let (client, base_url) = get_base_url()?;
    // Mapping dice: GET /?tipo=...
    let url = format!("{}?tipo={}", base_url, tipo);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoCondicion>>().await.map_err(|e| e.to_string())
}

pub async fn get_terminos_condiciones_default(tipo: String) -> Result<Vec<TerminoCondicion>, String> {
    // No está explícito en el mapping, pero podemos filtrar los del tipo.
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
    // El mapping dice que el body incluye created_by.
    // La struct request ya debería tenerlo o lo inyectamos.
    // Si CreateTerminoCondicionRequest no tiene created_by, deberíamos crear un objeto nuevo.
    // Asumiremos que el backend lo espera en el body.
    let mut body = serde_json::to_value(&request).map_err(|e| e.to_string())?;
    if let Some(obj) = body.as_object_mut() {
        obj.insert("created_by".to_string(), json!(created_by));
    }

    let response = client.post(&base_url).json(&body).send().await.map_err(|e| e.to_string())?;
    
    #[derive(Deserialize)]
    struct CreateResponse { termino_id: i32 } // Mapping dice que devuelve Termino, extraemos ID
    // O si devuelve el objeto completo:
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
    // Mapping: POST /delete con body { termino_id, deleted_by }
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
    // Mapping: GET /informe/:informeId
    let url = format!("{}/informe/{}", base_url, informe_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoInforme>>().await.map_err(|e| e.to_string())
}

pub async fn get_terminos_by_cotizacion(cotizacion_id: i32) -> Result<Vec<TerminoCotizacion>, String> {
    let (client, base_url) = get_base_url()?;
    // Mapping: GET /cotizacion/:cotizacionId
    let url = format!("{}/cotizacion/{}", base_url, cotizacion_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<TerminoCotizacion>>().await.map_err(|e| e.to_string())
}

// Funciones de "apply" masivo no están en el mapping, pero las mantenemos por compatibilidad si el backend las soporta.
// Si no, deberían usar las funciones individuales en un loop.
pub async fn apply_terminos_to_informe(informe_id: i32, terminos: Vec<TerminoInformeRequest>, applied_by: i32) -> Result<(), String> {
    // Si el backend no soporta bulk, iteramos.
    // Pero intentaremos usar la implementación individual según mapping.
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
    // Lógica de negocio: obtener defaults y aplicarlos
    let defaults = get_terminos_condiciones_default("informe".to_string()).await?;
    for term in defaults {
        create_termino_informe_relation(term.termino_id, informe_id, Some(true), applied_by).await?;
    }
    Ok(())
}

pub async fn apply_default_terminos_to_cotizacion(cotizacion_id: i32, applied_by: i32) -> Result<(), String> {
    let defaults = get_terminos_condiciones_default("cotizacion".to_string()).await?;
    for term in defaults {
        create_termino_cotizacion_relation(term.termino_id, cotizacion_id, Some(true), applied_by).await?;
    }
    Ok(())
}

pub async fn reactivate_termino_condicion(termino_id: i32, updated_by: i32) -> Result<(), String> {
    // No está en mapping, usamos update para activar
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
    // Mapping: POST /informe
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
    // Mapping: POST /cotizacion
    let url = format!("{}/cotizacion", base_url);
    let body = json!({
        "cotizacion_id": cotizacion_id,
        "termino_id": termino_id,
        "added_by": created_by
    });
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error adding term to cotizacion: {}", response.status()))
    }
}

pub async fn update_termino_informe_relation(_termino_id: i32, _informe_id: i32, _aplicado: bool, _updated_by: i32) -> Result<(), String> {
    // No soportado explícitamente en mapping (solo add/remove), pero podría ser un remove + add
    Err("Update relation not supported directly via API".to_string())
}

pub async fn update_termino_cotizacion_relation(_termino_id: i32, _cotizacion_id: i32, _aplicado: bool, _updated_by: i32) -> Result<(), String> {
    // No soportado explícitamente en mapping (solo add/remove), pero podría ser un remove + add
    Err("Update relation not supported directly via API".to_string())
}

pub async fn delete_termino_informe_relation(termino_id: i32, informe_id: i32, _deleted_by: i32) -> Result<(), String> {
    let (client, base_url) = get_base_url()?;
    // Mapping: POST /informe/remove
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
    // Mapping: POST /cotizacion/remove
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