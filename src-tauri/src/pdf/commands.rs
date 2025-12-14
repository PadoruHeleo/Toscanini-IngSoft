// Comandos de Tauri para generar PDFs
use tauri::{command, State};
use crate::config::AppConfig;
use crate::pdf::CotizacionPdfGenerator;
use crate::pdf::InformePdfGenerator;
use crate::pdf::OrdenTrabajoPdfGenerator;
use crate::pdf::api_data;
use crate::pdf::db_data;

#[command]
pub async fn generate_cotizacion_pdf_command(
    state: State<'_, AppConfig>,
    cotizacion_id: i32
) -> Result<Vec<u8>, String> {
    println!("Printing Cotizacion PDF for ID: {}, use_api: {}", cotizacion_id, state.use_api);
    let pdf_data = if state.use_api {
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
    state: State<'_, AppConfig>,
    informe_id: i32
) -> Result<Vec<u8>, String> {
    println!("Printing Informe PDF for ID: {}, use_api: {}", informe_id, state.use_api);
    let pdf_data = if state.use_api {
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
    state: State<'_, AppConfig>,
    orden_id: i32
) -> Result<Vec<u8>, String> {
    println!("Printing Orden Trabajo PDF for ID: {}, use_api: {}", orden_id, state.use_api);
    let pdf_data = if state.use_api {
        api_data::get_orden_trabajo_pdf_data(orden_id).await?
    } else {
        db_data::get_orden_trabajo_pdf_data(orden_id).await?
    };

    // Generar PDF
    let generator = OrdenTrabajoPdfGenerator::new();
    generator.generate_orden_trabajo_pdf(pdf_data).await
}
