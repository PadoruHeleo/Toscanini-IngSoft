use crate::infrastructure::api::client::get_http_client;
use crate::models::cotizacion::{
    Cotizacion,
    CotizacionDetallada,
    Pieza,
    PiezaCotizacion,
    CreateCotizacionRequest,
    UpdateCotizacionRequest,
    CreatePiezaRequest,
    UpdatePiezaRequest,
    PiezaCotizacionRequest,
    InventarioEquipo,
    InventarioEquipoRequest,
    SalidaEquipo,
    RegistrarSalidaRequest
};
use serde_json::json;
use serde::Deserialize;

// Helper para obtener URLs base por recurso
fn get_base_url(resource: &str) -> Result<(reqwest::Client, String), String> {
    let (client, api_url) = get_http_client()?;
    Ok((client, format!("{}/{}", api_url, resource)))
}

pub async fn get_cotizaciones() -> Result<Vec<Cotizacion>, String> {
    let (client, url) = get_base_url("cotizaciones")?;
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Vec<Cotizacion>>().await.map_err(|e| e.to_string())
}

pub async fn get_cotizaciones_detalladas() -> Result<Vec<CotizacionDetallada>, String> {
    let (client, base_url) = get_base_url("cotizaciones")?;
    let url = format!("{}/detalladas", base_url); // Endpoint sugerido
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        // Fallback: Si no existe el endpoint detallado, obtenemos las normales y mapeamos (básico)
        // O retornamos error si es estricto.
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Vec<CotizacionDetallada>>().await.map_err(|e| e.to_string())
}

pub async fn get_cotizacion_by_id(cotizacion_id: i32) -> Result<Option<Cotizacion>, String> {
    let (client, base_url) = get_base_url("cotizaciones")?;
    let url = format!("{}/{}", base_url, cotizacion_id);
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Option<Cotizacion>>().await.map_err(|e| e.to_string())
}

pub async fn get_cotizacion_by_codigo(cotizacion_codigo: String) -> Result<Option<Cotizacion>, String> {
    let (client, base_url) = get_base_url("cotizaciones")?;
    let url = format!("{}/codigo/{}", base_url, urlencoding::encode(&cotizacion_codigo));
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Option<Cotizacion>>().await.map_err(|e| e.to_string())
}

pub async fn create_cotizacion(request: CreateCotizacionRequest) -> Result<Cotizacion, String> {
    let (client, url) = get_base_url("cotizaciones")?;
    
    let response = client.post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !response.status().is_success() {
        let error_msg = response.text().await.unwrap_or_default();
        return Err(error_msg);
    }
    
    response.json::<Cotizacion>().await.map_err(|e| e.to_string())
}

pub async fn update_cotizacion(cotizacion_id: i32, request: UpdateCotizacionRequest, updated_by: i32) -> Result<Option<Cotizacion>, String> {
    let (client, base_url) = get_base_url("cotizaciones")?;
    let url = format!("{}/{}", base_url, cotizacion_id);
    
    // Inject updated_by into body
    let mut body_json = serde_json::to_value(&request).map_err(|e| e.to_string())?;
    if let Some(obj) = body_json.as_object_mut() {
        obj.insert("updated_by".to_string(), json!(updated_by));
    }
    
    let response = client.put(&url)
        .json(&body_json)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !response.status().is_success() {
        return Err(response.text().await.unwrap_or_default());
    }
    
    // API retorna el objeto actualizado
    response.json::<Option<Cotizacion>>().await.map_err(|e| e.to_string())
}

pub async fn delete_cotizacion(cotizacion_id: i32, deleted_by: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url("cotizaciones")?;
    let url = format!("{}/delete", base_url);
    
    let body = json!({
        "cotizacion_id": cotizacion_id,
        "deleted_by": deleted_by
    });
    
    let response = client.post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !response.status().is_success() {
        return Err(response.text().await.unwrap_or_default());
    }
    
    Ok(true)
}

pub async fn search_cotizaciones(search_term: String) -> Result<Vec<CotizacionDetallada>, String> {
    let (client, base_url) = get_base_url("cotizaciones")?;
    let url = format!("{}/search/query", base_url);
    
    let response = client.get(&url)
        .query(&[("term", search_term)])
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Vec<CotizacionDetallada>>().await.map_err(|e| e.to_string())
}

pub async fn count_cotizaciones() -> Result<i64, String> {
    let cotizaciones = get_cotizaciones().await?;
    Ok(cotizaciones.len() as i64)
}

pub async fn get_cotizaciones_with_pagination(offset: i64, limit: i64) -> Result<Vec<CotizacionDetallada>, String> {
    // Si la API no soporta paginación nativa en GET /, usamos una estrategia de fetch all temporal
    // O asumimos un endpoint /filter o /page
    let todas = get_cotizaciones_detalladas().await?;
    
    let start = offset as usize;
    if start >= todas.len() {
        return Ok(Vec::new());
    }
    let end = std::cmp::min(start + limit as usize, todas.len());
    
    Ok(todas[start..end].to_vec())
}

pub async fn get_cotizaciones_by_cliente(cliente_id: i32) -> Result<Vec<Cotizacion>, String> {
    let (client, base_url) = get_base_url("cotizaciones")?;
    let url = format!("{}/cliente/{}", base_url, cliente_id);
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Vec<Cotizacion>>().await.map_err(|e| e.to_string())
}

pub async fn get_piezas() -> Result<Vec<Pieza>, String> {
    let (client, url) = get_base_url("piezas")?; // Endpoint: /api/piezas
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Vec<Pieza>>().await.map_err(|e| e.to_string())
}

pub async fn get_pieza_by_id(pieza_id: i32) -> Result<Option<Pieza>, String> {
    let (client, base_url) = get_base_url("piezas")?;
    let url = format!("{}/{}", base_url, pieza_id);
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if response.status() == reqwest::StatusCode::NOT_FOUND { return Ok(None); }
    if !response.status().is_success() { return Err(format!("Error API: {}", response.status())); }
    
    response.json::<Option<Pieza>>().await.map_err(|e| e.to_string())
}

pub async fn create_pieza(request: CreatePiezaRequest) -> Result<Pieza, String> {
    let (client, url) = get_base_url("piezas")?;
    
    let response = client.post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !response.status().is_success() {
        let error_msg = response.text().await.unwrap_or_default();
        return Err(error_msg);
    }
    
    response.json::<Pieza>().await.map_err(|e| e.to_string())
}

pub async fn update_pieza(pieza_id: i32, request: UpdatePiezaRequest) -> Result<Option<Pieza>, String> {
    let (client, base_url) = get_base_url("piezas")?;
    let url = format!("{}/{}", base_url, pieza_id);
    
    let response = client.put(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !response.status().is_success() {
        return Err(response.text().await.unwrap_or_default());
    }
    
    response.json::<Option<Pieza>>().await.map_err(|e| e.to_string())
}

pub async fn delete_pieza(pieza_id: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url("piezas")?;
    let url = format!("{}/{}", base_url, pieza_id);
    
    let response = client.delete(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !response.status().is_success() {
        return Err(response.text().await.unwrap_or_default());
    }
    
    Ok(true)
}

pub async fn get_piezas_cotizacion(cotizacion_id: i32) -> Result<Vec<PiezaCotizacion>, String> {
    let (client, base_url) = get_base_url("cotizaciones")?;
    let url = format!("{}/{}/piezas", base_url, cotizacion_id);
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Vec<PiezaCotizacion>>().await.map_err(|e| e.to_string())
}

pub async fn get_inventario_equipos() -> Result<Vec<InventarioEquipo>, String> {
    let (client, url) = get_base_url("inventario-equipos")?;
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Vec<InventarioEquipo>>().await.map_err(|e| e.to_string())
}

pub async fn create_inventario_equipo(request: InventarioEquipoRequest) -> Result<InventarioEquipo, String> {
    let (client, url) = get_base_url("inventario-equipos")?;
    
    let response = client.post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !response.status().is_success() {
        return Err(response.text().await.unwrap_or_default());
    }
    
    response.json::<InventarioEquipo>().await.map_err(|e| e.to_string())
}

pub async fn update_inventario_equipo(equipo_id: i32, request: InventarioEquipoRequest) -> Result<InventarioEquipo, String> {
    let (client, base_url) = get_base_url("inventario-equipos")?;
    let url = format!("{}/{}", base_url, equipo_id);
    
    let response = client.put(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !response.status().is_success() {
        return Err(response.text().await.unwrap_or_default());
    }
    
    response.json::<InventarioEquipo>().await.map_err(|e| e.to_string())
}

pub async fn delete_inventario_equipo(equipo_id: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url("inventario-equipos")?;
    let url = format!("{}/{}", base_url, equipo_id);
    
    let response = client.delete(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !response.status().is_success() {
        return Err(response.text().await.unwrap_or_default());
    }
    
    Ok(true)
}

pub async fn update_inventario_equipo_stock(equipo_id: i32, cantidad: i32, tipo: String, updated_by: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url("inventario-equipos")?;
    let url = format!("{}/{}/stock", base_url, equipo_id);
    
    let body = json!({
        "cantidad": cantidad,
        "tipo": tipo,
        "updated_by": updated_by
    });
    
    let response = client.post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !response.status().is_success() {
        return Err(response.text().await.unwrap_or_default());
    }
    
    Ok(true)
}

pub async fn registrar_salida_equipo_v2(request: RegistrarSalidaRequest) -> Result<bool, String> {
    let (client, url) = get_base_url("salidas-equipos")?;
    
    let response = client.post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !response.status().is_success() {
        return Err(response.text().await.unwrap_or_default());
    }
    
    Ok(true)
}

pub async fn get_salidas_equipo() -> Result<Vec<SalidaEquipo>, String> {
    let (client, url) = get_base_url("salidas-equipos")?;
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Vec<SalidaEquipo>>().await.map_err(|e| e.to_string())
}

pub async fn puede_registrar_salida_v2(orden_trabajo_id: i32) -> Result<(bool, String), String> {
    let (client, base_url) = get_base_url("salidas-equipos")?;
    let url = format!("{}/check/{}", base_url, orden_trabajo_id);
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    let result = response.json::<CheckResponse>().await.map_err(|e| e.to_string())?;
    Ok((result.puede, result.mensaje))
}

#[derive(Deserialize)]
struct CheckResponse {
    puede: bool,
    mensaje: String,
}

pub async fn get_salida_by_orden(orden_trabajo_id: i32) -> Result<Option<SalidaEquipo>, String> {
    let (client, base_url) = get_base_url("salidas-equipos")?;
    let url = format!("{}/orden/{}", base_url, orden_trabajo_id);
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Option<SalidaEquipo>>().await.map_err(|e| e.to_string())
}

pub async fn update_cotizacion_piezas(cotizacion_id: i32, piezas: Vec<PiezaCotizacionRequest>, updated_by: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url("cotizaciones")?;
    let url = format!("{}/{}/piezas", base_url, cotizacion_id);
    
    let body = json!({
        "piezas": piezas,
        "updated_by": updated_by
    });
    
    let response = client.put(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !response.status().is_success() {
        return Err(response.text().await.unwrap_or_default());
    }
    
    Ok(true)
}

pub async fn aprobar_cotizacion(cotizacion_id: i32, approved_by: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url("cotizaciones")?;
    let url = format!("{}/aprobar", base_url);
    
    let body = json!({
        "cotizacion_id": cotizacion_id,
        "approved_by": approved_by
    });
    
    let response = client.post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !response.status().is_success() {
        return Err(response.text().await.unwrap_or_default());
    }
    
    Ok(true)
}

pub async fn duplicate_cotizacion(cotizacion_id: i32, created_by: i32, new_informe_id: Option<i32>) -> Result<Cotizacion, String> {
    let (client, base_url) = get_base_url("cotizaciones")?;
    let url = format!("{}/duplicar", base_url);
    
    let body = json!({
        "cotizacion_id": cotizacion_id,
        "created_by": created_by,
        "new_informe_id": new_informe_id
    });
    
    let response = client.post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    if !response.status().is_success() {
        return Err(response.text().await.unwrap_or_default());
    }
    
    response.json::<Cotizacion>().await.map_err(|e| e.to_string())
}

pub async fn get_cotizacion_pdf_data(cotizacion_id: i32) -> Result<serde_json::Value, String> {
    let (client, base_url) = get_base_url("cotizaciones")?;
    let url = format!("{}/{}/pdf-data", base_url, cotizacion_id);
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<serde_json::Value>().await.map_err(|e| e.to_string())
}

pub async fn get_piezas_inventario() -> Result<Vec<Pieza>, String> {
    // Mapping: /api/piezas (Catálogo)
    get_piezas().await
}

pub async fn update_pieza_stock(_pieza_id: i32, _cantidad: i32, _tipo: String) -> Result<bool, String> {
    // Mapping: No hay endpoint específico para stock de piezas en el catálogo, 
    // solo para inventario de equipos (/api/inventario-equipos/:id/stock).
    // Si se refiere a stock de piezas, se usa update_pieza.
    Err("Not implemented via API yet".to_string())
}