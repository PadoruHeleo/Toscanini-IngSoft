use tauri::{command, State};
use std::sync::RwLock; // Importar RwLock
use crate::config::AppConfig;
use crate::pdf::CotizacionPdfGenerator;
use crate::pdf::InformePdfGenerator;
use crate::pdf::OrdenTrabajoPdfGenerator;
use crate::pdf::api_data;
use crate::pdf::db_data;

#[command]
pub async fn generate_cotizacion_pdf_command(
    state: State<'_, RwLock<AppConfig>>,
    cotizacion_id: i32
) -> Result<Vec<u8>, String> {
    // Extraer el valor booleano y soltar el lock inmediatamente
    let use_api = {
        let config = state.read().map_err(|_| "Error al leer configuración de estado")?;
        config.use_api
    };
    
    println!("Printing Cotizacion PDF for ID: {}, use_api: {}", cotizacion_id, use_api);
    
    // Usar la variable local 'use_api'
    let pdf_data = if use_api {
        api_data::get_cotizacion_pdf_data(cotizacion_id).await?
    } else {
        db_data::get_cotizacion_pdf_data(cotizacion_id).await?
    };

    // Generar PDF
    let generator = CotizacionPdfGenerator::new();
    generator.generate_cotizacion_pdf(pdf_data).await
}

#[command]
pub async fn generate_informe_pdf_command(
    state: State<'_, RwLock<AppConfig>>,
    informe_id: i32
) -> Result<Vec<u8>, String> {
    // Extraer el valor booleano y soltar el lock inmediatamente
    let use_api = {
        let config = state.read().map_err(|_| "Error al leer configuración de estado")?;
        config.use_api
    };
    
    println!("Printing Informe PDF for ID: {}, use_api: {}", informe_id, use_api);
    
    // Usar la variable local 'use_api'
    let pdf_data = if use_api {
        api_data::get_informe_pdf_data(informe_id).await?
    } else {
        db_data::get_informe_pdf_data(informe_id).await?
    };

    // Generar PDF
    let generator = InformePdfGenerator::new();
    generator.generate_informe_pdf(pdf_data).await
}

#[command]
pub async fn generate_orden_trabajo_pdf_command(
    state: State<'_, RwLock<AppConfig>>,
    orden_id: i32
) -> Result<Vec<u8>, String> {
    // Extraer el valor booleano y soltar el lock inmediatamente
    let use_api = {
        let config = state.read().map_err(|_| "Error al leer configuración de estado")?;
        config.use_api
    };
    
    println!("Printing Orden Trabajo PDF for ID: {}, use_api: {}", orden_id, use_api);
    
    // Usar la variable local 'use_api'
    let pdf_data = if use_api {
        api_data::get_orden_trabajo_pdf_data(orden_id).await?
    } else {
        db_data::get_orden_trabajo_pdf_data(orden_id).await?
    };

    // Generar PDF
    let generator = OrdenTrabajoPdfGenerator::new();
    generator.generate_orden_trabajo_pdf(pdf_data).await
}
