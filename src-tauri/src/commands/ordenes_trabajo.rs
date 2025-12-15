use std::sync::RwLock;
use tauri::State;
use crate::config::AppConfig;
use crate::models::ordenes_trabajo::{
    OrdenTrabajo, CreateOrdenTrabajoRequest, UpdateOrdenTrabajoRequest,
    OrdenTrabajoDetallada, Filtros
};
use crate::infrastructure::db::ordenes_trabajo as db_impl;
use crate::infrastructure::api::ordenes_trabajo as api_impl;

#[tauri::command]
pub async fn get_ordenes_trabajo(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<OrdenTrabajo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_ordenes_trabajo().await } else { db_impl::get_ordenes_trabajo().await }
}

#[tauri::command]
pub async fn get_orden_trabajo_by_id(state: State<'_, RwLock<AppConfig>>, orden_id: i32) -> Result<Option<OrdenTrabajo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_orden_trabajo_by_id(orden_id).await } else { db_impl::get_orden_trabajo_by_id(orden_id).await }
}

#[tauri::command]
pub async fn get_orden_trabajo_by_codigo(state: State<'_, RwLock<AppConfig>>, orden_codigo: String) -> Result<Option<OrdenTrabajo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_orden_trabajo_by_codigo(orden_codigo).await } else { db_impl::get_orden_trabajo_by_codigo(orden_codigo).await }
}

#[tauri::command]
pub async fn get_ordenes_trabajo_by_equipo(state: State<'_, RwLock<AppConfig>>, equipo_id: i32) -> Result<Vec<OrdenTrabajo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_ordenes_trabajo_by_equipo(equipo_id).await } else { db_impl::get_ordenes_trabajo_by_equipo(equipo_id).await }
}

#[tauri::command]
pub async fn get_ordenes_trabajo_by_estado(state: State<'_, RwLock<AppConfig>>, estado: String) -> Result<Vec<OrdenTrabajo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_ordenes_trabajo_by_estado(estado).await } else { db_impl::get_ordenes_trabajo_by_estado(estado).await }
}

#[tauri::command]
pub async fn get_ordenes_trabajo_by_prioridad(state: State<'_, RwLock<AppConfig>>, prioridad: String) -> Result<Vec<OrdenTrabajo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_ordenes_trabajo_by_prioridad(prioridad).await } else { db_impl::get_ordenes_trabajo_by_prioridad(prioridad).await }
}

#[tauri::command]
pub async fn get_ordenes_trabajo_by_usuario(state: State<'_, RwLock<AppConfig>>, usuario_id: i32) -> Result<Vec<OrdenTrabajo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_ordenes_trabajo_by_usuario(usuario_id).await } else { db_impl::get_ordenes_trabajo_by_usuario(usuario_id).await }
}

#[tauri::command]
pub async fn get_ordenes_trabajo_detalladas(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<OrdenTrabajoDetallada>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_ordenes_trabajo_detalladas().await } else { db_impl::get_ordenes_trabajo_detalladas().await }
}

#[tauri::command]
pub async fn get_orden_trabajo_detallada_by_id(state: State<'_, RwLock<AppConfig>>, orden_id: i32) -> Result<Option<OrdenTrabajoDetallada>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_orden_trabajo_detallada_by_id(orden_id).await } else { db_impl::get_orden_trabajo_detallada_by_id(orden_id).await }
}

#[tauri::command]
pub async fn create_orden_trabajo(state: State<'_, RwLock<AppConfig>>, request: CreateOrdenTrabajoRequest) -> Result<OrdenTrabajo, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::create_orden_trabajo(request).await } else { db_impl::create_orden_trabajo(request).await }
}

#[tauri::command]
pub async fn update_orden_trabajo(state: State<'_, RwLock<AppConfig>>, orden_id: i32, request: UpdateOrdenTrabajoRequest, updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::update_orden_trabajo(orden_id, request, updated_by).await } else { db_impl::update_orden_trabajo(orden_id, request, updated_by).await }
}

#[tauri::command]
pub async fn cambiar_estado_orden_trabajo(state: State<'_, RwLock<AppConfig>>, orden_id: i32, nuevo_estado: String, updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::cambiar_estado_orden_trabajo(orden_id, nuevo_estado, updated_by).await } else { db_impl::cambiar_estado_orden_trabajo(orden_id, nuevo_estado, updated_by).await }
}

#[tauri::command]
pub async fn asignar_cotizacion_orden_trabajo(state: State<'_, RwLock<AppConfig>>, orden_id: i32, cotizacion_id: i32, updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::asignar_cotizacion_orden_trabajo(orden_id, cotizacion_id, updated_by).await } else { db_impl::asignar_cotizacion_orden_trabajo(orden_id, cotizacion_id, updated_by).await }
}

#[tauri::command]
pub async fn asignar_informe_orden_trabajo(state: State<'_, RwLock<AppConfig>>, orden_id: i32, informe_id: i32, updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::asignar_informe_orden_trabajo(orden_id, informe_id, updated_by).await } else { db_impl::asignar_informe_orden_trabajo(orden_id, informe_id, updated_by).await }
}

#[tauri::command]
pub async fn delete_orden_trabajo(state: State<'_, RwLock<AppConfig>>, orden_id: i32, deleted_by: i32) -> Result<bool, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::delete_orden_trabajo(orden_id, deleted_by).await } else { db_impl::delete_orden_trabajo(orden_id, deleted_by).await }
}

#[tauri::command]
pub async fn get_ordenes_trabajo_stats(state: State<'_, RwLock<AppConfig>>) -> Result<serde_json::Value, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_ordenes_trabajo_stats().await } else { db_impl::get_ordenes_trabajo_stats().await }
}

#[tauri::command]
pub async fn search_ordenes_trabajo(state: State<'_, RwLock<AppConfig>>, search_term: String) -> Result<Vec<OrdenTrabajoDetallada>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::search_ordenes_trabajo(search_term).await } else { db_impl::search_ordenes_trabajo(search_term).await }
}

#[tauri::command]
pub async fn send_orden_trabajo_notification(state: State<'_, RwLock<AppConfig>>, orden_id: i32, sent_by: i32) -> Result<bool, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::send_orden_trabajo_notification(orden_id, sent_by).await } else { db_impl::send_orden_trabajo_notification(orden_id, sent_by).await }
}

#[tauri::command]
pub async fn get_orden_trabajo_by_informe_id(state: State<'_, RwLock<AppConfig>>, informe_id: i32) -> Result<Option<OrdenTrabajo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_orden_trabajo_by_informe_id(informe_id).await } else { db_impl::get_orden_trabajo_by_informe_id(informe_id).await }
}

#[tauri::command]
pub async fn get_ordenes_trabajo_filtradas(state: State<'_, RwLock<AppConfig>>, filtros: Filtros) -> Result<Vec<OrdenTrabajoDetallada>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_ordenes_trabajo_filtradas(filtros).await } else { db_impl::get_ordenes_trabajo_filtradas(filtros).await }
}

#[tauri::command]
pub async fn get_modelos_disponibles(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<String>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_modelos_disponibles().await } else { db_impl::get_modelos_disponibles().await }
}

#[tauri::command]
pub async fn get_marcas_disponibles(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<String>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_marcas_disponibles().await } else { db_impl::get_marcas_disponibles().await }
}

#[tauri::command]
pub async fn get_clientes_disponibles(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<String>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_clientes_disponibles().await } else { db_impl::get_clientes_disponibles().await }
}

#[tauri::command]
pub async fn get_ordenes_trabajo_by_cliente(state: State<'_, RwLock<AppConfig>>, cliente_id: i32) -> Result<Vec<OrdenTrabajo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_ordenes_trabajo_by_cliente(cliente_id).await } else { db_impl::get_ordenes_trabajo_by_cliente(cliente_id).await }
}

#[tauri::command]
pub async fn remove_cotizacion_from_ordenes(state: State<'_, RwLock<AppConfig>>, cotizacion_id: i32, updated_by: i32) -> Result<bool, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::remove_cotizacion_from_ordenes(cotizacion_id, updated_by).await } else { db_impl::remove_cotizacion_from_ordenes(cotizacion_id, updated_by).await }
}

#[tauri::command]
pub async fn remove_informe_from_ordenes(state: State<'_, RwLock<AppConfig>>, informe_id: i32, updated_by: i32) -> Result<bool, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::remove_informe_from_ordenes(informe_id, updated_by).await } else { db_impl::remove_informe_from_ordenes(informe_id, updated_by).await }
}