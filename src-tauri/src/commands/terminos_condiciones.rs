use std::sync::RwLock;
use tauri::State;
use crate::config::AppConfig;
use crate::models::terminos_condiciones::{
    TerminoCondicion, TerminoInforme, TerminoCotizacion,
    CreateTerminoCondicionRequest, UpdateTerminoCondicionRequest,
    TerminoInformeRequest, TerminoCotizacionRequest
};
use crate::infrastructure::db::terminos_condiciones as db_impl;
use crate::infrastructure::api::terminos_condiciones as api_impl;

#[tauri::command]
pub async fn get_terminos_condiciones(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<TerminoCondicion>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_terminos_condiciones().await } else { db_impl::get_terminos_condiciones().await }
}

#[tauri::command]
pub async fn get_terminos_condiciones_activos(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<TerminoCondicion>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_terminos_condiciones_activos().await } else { db_impl::get_terminos_condiciones_activos().await }
}

#[tauri::command]
pub async fn get_terminos_condiciones_by_tipo(state: State<'_, RwLock<AppConfig>>, tipo: String) -> Result<Vec<TerminoCondicion>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_terminos_condiciones_by_tipo(tipo).await } else { db_impl::get_terminos_condiciones_by_tipo(tipo).await }
}

#[tauri::command]
pub async fn get_terminos_condiciones_default(state: State<'_, RwLock<AppConfig>>, tipo: String) -> Result<Vec<TerminoCondicion>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_terminos_condiciones_default(tipo).await } else { db_impl::get_terminos_condiciones_default(tipo).await }
}

#[tauri::command]
pub async fn get_termino_condicion_by_id(state: State<'_, RwLock<AppConfig>>, termino_id: i32) -> Result<Option<TerminoCondicion>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_termino_condicion_by_id(termino_id).await } else { db_impl::get_termino_condicion_by_id(termino_id).await }
}

#[tauri::command]
pub async fn create_termino_condicion(state: State<'_, RwLock<AppConfig>>, request: CreateTerminoCondicionRequest, created_by: i32) -> Result<i32, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::create_termino_condicion(request, created_by).await } else { db_impl::create_termino_condicion(request, created_by).await }
}

#[tauri::command]
pub async fn update_termino_condicion(state: State<'_, RwLock<AppConfig>>, termino_id: i32, request: UpdateTerminoCondicionRequest, updated_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::update_termino_condicion(termino_id, request, updated_by).await } else { db_impl::update_termino_condicion(termino_id, request, updated_by).await }
}

#[tauri::command]
pub async fn delete_termino_condicion(state: State<'_, RwLock<AppConfig>>, termino_id: i32, deleted_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::delete_termino_condicion(termino_id, deleted_by).await } else { db_impl::delete_termino_condicion(termino_id, deleted_by).await }
}

#[tauri::command]
pub async fn get_terminos_by_informe(state: State<'_, RwLock<AppConfig>>, informe_id: i32) -> Result<Vec<TerminoInforme>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_terminos_by_informe(informe_id).await } else { db_impl::get_terminos_by_informe(informe_id).await }
}

#[tauri::command]
pub async fn get_terminos_by_cotizacion(state: State<'_, RwLock<AppConfig>>, cotizacion_id: i32) -> Result<Vec<TerminoCotizacion>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_terminos_by_cotizacion(cotizacion_id).await } else { db_impl::get_terminos_by_cotizacion(cotizacion_id).await }
}

#[tauri::command]
pub async fn apply_terminos_to_informe(state: State<'_, RwLock<AppConfig>>, informe_id: i32, terminos: Vec<TerminoInformeRequest>, applied_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::apply_terminos_to_informe(informe_id, terminos, applied_by).await } else { db_impl::apply_terminos_to_informe(informe_id, terminos, applied_by).await }
}

#[tauri::command]
pub async fn apply_terminos_to_cotizacion(state: State<'_, RwLock<AppConfig>>, cotizacion_id: i32, terminos: Vec<TerminoCotizacionRequest>, applied_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::apply_terminos_to_cotizacion(cotizacion_id, terminos, applied_by).await } else { db_impl::apply_terminos_to_cotizacion(cotizacion_id, terminos, applied_by).await }
}

#[tauri::command]
pub async fn apply_default_terminos_to_informe(state: State<'_, RwLock<AppConfig>>, informe_id: i32, applied_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::apply_default_terminos_to_informe(informe_id, applied_by).await } else { db_impl::apply_default_terminos_to_informe(informe_id, applied_by).await }
}

#[tauri::command]
pub async fn apply_default_terminos_to_cotizacion(state: State<'_, RwLock<AppConfig>>, cotizacion_id: i32, applied_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::apply_default_terminos_to_cotizacion(cotizacion_id, applied_by).await } else { db_impl::apply_default_terminos_to_cotizacion(cotizacion_id, applied_by).await }
}

#[tauri::command]
pub async fn reactivate_termino_condicion(state: State<'_, RwLock<AppConfig>>, termino_id: i32, reactivated_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::reactivate_termino_condicion(termino_id, reactivated_by).await } else { db_impl::reactivate_termino_condicion(termino_id, reactivated_by).await }
}

#[tauri::command]
pub async fn toggle_termino_default(state: State<'_, RwLock<AppConfig>>, termino_id: i32, is_default: bool, updated_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::toggle_termino_default(termino_id, is_default, updated_by).await } else { db_impl::toggle_termino_default(termino_id, is_default, updated_by).await }
}

#[tauri::command]
pub async fn create_termino_informe_relation(state: State<'_, RwLock<AppConfig>>, termino_id: i32, informe_id: i32, aplicado: Option<bool>, created_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::create_termino_informe_relation(termino_id, informe_id, aplicado, created_by).await } else { db_impl::create_termino_informe_relation(termino_id, informe_id, aplicado, created_by).await }
}

#[tauri::command]
pub async fn create_termino_cotizacion_relation(state: State<'_, RwLock<AppConfig>>, termino_id: i32, cotizacion_id: i32, aplicado: Option<bool>, created_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::create_termino_cotizacion_relation(termino_id, cotizacion_id, aplicado, created_by).await } else { db_impl::create_termino_cotizacion_relation(termino_id, cotizacion_id, aplicado, created_by).await }
}

#[tauri::command]
pub async fn update_termino_informe_relation(state: State<'_, RwLock<AppConfig>>, termino_id: i32, informe_id: i32, aplicado: bool, updated_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::update_termino_informe_relation(termino_id, informe_id, aplicado, updated_by).await } else { db_impl::update_termino_informe_relation(termino_id, informe_id, aplicado, updated_by).await }
}

#[tauri::command]
pub async fn update_termino_cotizacion_relation(state: State<'_, RwLock<AppConfig>>, termino_id: i32, cotizacion_id: i32, aplicado: bool, updated_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::update_termino_cotizacion_relation(termino_id, cotizacion_id, aplicado, updated_by).await } else { db_impl::update_termino_cotizacion_relation(termino_id, cotizacion_id, aplicado, updated_by).await }
}

#[tauri::command]
pub async fn delete_termino_informe_relation(state: State<'_, RwLock<AppConfig>>, termino_id: i32, informe_id: i32, deleted_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::delete_termino_informe_relation(termino_id, informe_id, deleted_by).await } else { db_impl::delete_termino_informe_relation(termino_id, informe_id, deleted_by).await }
}

#[tauri::command]
pub async fn delete_termino_cotizacion_relation(state: State<'_, RwLock<AppConfig>>, termino_id: i32, cotizacion_id: i32, deleted_by: i32) -> Result<(), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::delete_termino_cotizacion_relation(termino_id, cotizacion_id, deleted_by).await } else { db_impl::delete_termino_cotizacion_relation(termino_id, cotizacion_id, deleted_by).await }
}

#[tauri::command]
pub async fn get_informes_by_termino(state: State<'_, RwLock<AppConfig>>, termino_id: i32) -> Result<Vec<TerminoInforme>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_informes_by_termino(termino_id).await } else { db_impl::get_informes_by_termino(termino_id).await }
}

#[tauri::command]
pub async fn get_cotizaciones_by_termino(state: State<'_, RwLock<AppConfig>>, termino_id: i32) -> Result<Vec<TerminoCotizacion>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_cotizaciones_by_termino(termino_id).await } else { db_impl::get_cotizaciones_by_termino(termino_id).await }
}

#[tauri::command]
pub async fn check_termino_in_informe(state: State<'_, RwLock<AppConfig>>, termino_id: i32, informe_id: i32) -> Result<Option<bool>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::check_termino_in_informe(termino_id, informe_id).await } else { db_impl::check_termino_in_informe(termino_id, informe_id).await }
}

#[tauri::command]
pub async fn check_termino_in_cotizacion(state: State<'_, RwLock<AppConfig>>, termino_id: i32, cotizacion_id: i32) -> Result<Option<bool>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::check_termino_in_cotizacion(termino_id, cotizacion_id).await } else { db_impl::check_termino_in_cotizacion(termino_id, cotizacion_id).await }
}
