use tauri::State;
use crate::config::AppConfig;
use crate::infrastructure::db::email as db_impl;
use crate::infrastructure::api::email as api_impl;

#[tauri::command]
pub async fn test_email_send(state: State<'_, AppConfig>, to_email: String) -> Result<String, String> {
    if state.use_api {
        api_impl::send_test_email(to_email).await
    } else {
        let config = state.email_config.as_ref().ok_or("Configuración de email no encontrada")?;
        db_impl::send_test_email(config, to_email).await
    }
}

#[tauri::command]
pub async fn send_orden_trabajo_cliente(state: State<'_, AppConfig>, orden_id: i32, _sent_by: i32) -> Result<String, String> {
    if state.use_api {
        api_impl::send_orden_trabajo_cliente(orden_id).await
    } else {
        let config = state.email_config.as_ref().ok_or("Configuración de email no encontrada")?;
        db_impl::send_orden_trabajo_cliente(config, orden_id).await
    }
}

#[tauri::command]
pub async fn send_cotizacion_email(state: State<'_, AppConfig>, cotizacion_id: i32, sent_by: i32) -> Result<String, String> {
    if state.use_api {
        api_impl::send_cotizacion_email(cotizacion_id, sent_by).await
    } else {
        let config = state.email_config.as_ref().ok_or("Configuración de email no encontrada")?;
        db_impl::send_cotizacion_email(config, cotizacion_id, sent_by).await
    }
}

#[tauri::command]
pub async fn send_informe_email(state: State<'_, AppConfig>, orden_id: i32, sent_by: i32) -> Result<String, String> {
    if state.use_api {
        api_impl::send_informe_email(orden_id, sent_by).await
    } else {
        let config = state.email_config.as_ref().ok_or("Configuración de email no encontrada")?;
        db_impl::send_informe_email(config, orden_id, sent_by).await
    }
}
