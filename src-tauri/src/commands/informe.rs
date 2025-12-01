use tauri::State;
use crate::config::AppConfig;
use crate::models::informe::{
    Informe, InformeDetallado, CreateInformeRequest, UpdateInformeRequest, PiezaInforme
};
use crate::infrastructure::db::informe as db_impl;
use crate::infrastructure::api::informe as api_impl;

#[tauri::command]
pub async fn get_informes(state: State<'_, AppConfig>) -> Result<Vec<Informe>, String> {
    if state.use_api { api_impl::get_informes().await } else { db_impl::get_informes().await }
}

#[tauri::command]
pub async fn get_informes_detallados(state: State<'_, AppConfig>) -> Result<Vec<InformeDetallado>, String> {
    if state.use_api { api_impl::get_informes_detallados().await } else { db_impl::get_informes_detallados().await }
}

#[tauri::command]
pub async fn get_informe_by_id(state: State<'_, AppConfig>, informe_id: i32) -> Result<Option<Informe>, String> {
    if state.use_api { api_impl::get_informe_by_id(informe_id).await } else { db_impl::get_informe_by_id(informe_id).await }
}

#[tauri::command]
pub async fn get_informe_by_codigo(state: State<'_, AppConfig>, informe_codigo: String) -> Result<Option<Informe>, String> {
    if state.use_api { api_impl::get_informe_by_codigo(informe_codigo).await } else { db_impl::get_informe_by_codigo(informe_codigo).await }
}

#[tauri::command]
pub async fn create_informe(state: State<'_, AppConfig>, request: CreateInformeRequest) -> Result<Informe, String> {
    if state.use_api { api_impl::create_informe(request).await } else { db_impl::create_informe(request).await }
}

#[tauri::command]
pub async fn rechazar_informe_borrador(state: State<'_, AppConfig>, informe_id: i32, motivo_eliminacion: String, updated_by: i32) -> Result<bool, String> {
    if state.use_api { api_impl::rechazar_informe_borrador(informe_id, motivo_eliminacion, updated_by).await } else { db_impl::rechazar_informe_borrador(informe_id, motivo_eliminacion, updated_by).await }
}

#[tauri::command]
pub async fn update_informe(state: State<'_, AppConfig>, informe_id: i32, request: UpdateInformeRequest, updated_by: i32) -> Result<Option<Informe>, String> {
    if state.use_api { api_impl::update_informe(informe_id, request, updated_by).await } else { db_impl::update_informe(informe_id, request, updated_by).await }
}

#[tauri::command]
pub async fn delete_informe(state: State<'_, AppConfig>, informe_id: i32, deleted_by: i32) -> Result<bool, String> {
    if state.use_api { api_impl::delete_informe(informe_id, deleted_by).await } else { db_impl::delete_informe(informe_id, deleted_by).await }
}

#[tauri::command]
pub async fn search_informes(state: State<'_, AppConfig>, search_term: String) -> Result<Vec<InformeDetallado>, String> {
    if state.use_api { api_impl::search_informes(search_term).await } else { db_impl::search_informes(search_term).await }
}

#[tauri::command]
pub async fn count_informes(state: State<'_, AppConfig>) -> Result<i64, String> {
    if state.use_api { api_impl::count_informes().await } else { db_impl::count_informes().await }
}

#[tauri::command]
pub async fn get_informes_with_pagination(state: State<'_, AppConfig>, offset: i64, limit: i64) -> Result<Vec<InformeDetallado>, String> {
    if state.use_api { api_impl::get_informes_with_pagination(offset, limit).await } else { db_impl::get_informes_with_pagination(offset, limit).await }
}

#[tauri::command]
pub async fn get_piezas_informe(state: State<'_, AppConfig>, informe_id: i32) -> Result<Vec<PiezaInforme>, String> {
    if state.use_api { api_impl::get_piezas_informe(informe_id).await } else { db_impl::get_piezas_informe(informe_id).await }
}

#[tauri::command]
pub async fn send_informe_to_client(state: State<'_, AppConfig>, informe_id: i32, sent_by: i32) -> Result<bool, String> {
    if state.use_api { api_impl::send_informe_to_client(informe_id, sent_by).await } else { db_impl::send_informe_to_client(informe_id, sent_by).await }
}

#[tauri::command]
pub async fn get_informes_by_cliente(state: State<'_, AppConfig>, cliente_id: i32) -> Result<Vec<Informe>, String> {
    if state.use_api { api_impl::get_informes_by_cliente(cliente_id).await } else { db_impl::get_informes_by_cliente(cliente_id).await }
}