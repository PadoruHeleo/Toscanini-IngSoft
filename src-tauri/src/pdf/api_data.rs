use crate::pdf::{CotizacionPdfData, InformePdfData, OrdenTrabajoPdfData};
use crate::infrastructure::api::client::get_http_client;
use crate::infrastructure::api::cotizacion::get_cotizacion_pdf_data as api_get_cotizacion_pdf_data;

pub async fn get_cotizacion_pdf_data(cotizacion_id: i32) -> Result<CotizacionPdfData, String> {
    let json_value = api_get_cotizacion_pdf_data(cotizacion_id).await?;
    serde_json::from_value(json_value).map_err(|e| format!("Error deserializing PDF data: {}", e))
}

pub async fn get_informe_pdf_data(informe_id: i32) -> Result<InformePdfData, String> {
    let (client, api_url) = get_http_client()?;
    let url = format!("{}/informes/{}/pdf-data", api_url, informe_id);
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<InformePdfData>().await.map_err(|e| e.to_string())
}

pub async fn get_orden_trabajo_pdf_data(orden_id: i32) -> Result<OrdenTrabajoPdfData, String> {
    let (client, api_url) = get_http_client()?;
    let url = format!("{}/ordenes-trabajo/{}/pdf-data", api_url, orden_id);
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<OrdenTrabajoPdfData>().await.map_err(|e| e.to_string())
}
