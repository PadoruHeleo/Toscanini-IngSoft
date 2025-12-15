use std::sync::RwLock;
use tauri::State;
use crate::config::AppConfig;
use crate::models::equipos::{
    Equipo, CreateEquipoRequest, UpdateEquipoRequest, RegistrarSalidaRequest, 
    SalidaEquipoResponse, EquipoWithCliente, FiltrosEquipos, EquipoConEstado
};
use crate::infrastructure::db::equipos as db_impl;
use crate::infrastructure::api::equipos as api_impl;

#[tauri::command]
pub async fn get_equipos(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<Equipo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos().await } else { db_impl::get_equipos().await }
}

#[tauri::command]
pub async fn get_equipo_by_id(state: State<'_, RwLock<AppConfig>>, equipo_id: i32) -> Result<Option<Equipo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipo_by_id(equipo_id).await } else { db_impl::get_equipo_by_id(equipo_id).await }
}

#[tauri::command]
pub async fn get_equipo_by_numero_serie(state: State<'_, RwLock<AppConfig>>, numero_serie: String) -> Result<Option<Equipo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipo_by_numero_serie(numero_serie).await } else { db_impl::get_equipo_by_numero_serie(numero_serie).await }
}

#[tauri::command]
pub async fn get_equipos_by_cliente(state: State<'_, RwLock<AppConfig>>, cliente_id: i32) -> Result<Vec<Equipo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_by_cliente(cliente_id).await } else { db_impl::get_equipos_by_cliente(cliente_id).await }
}

#[tauri::command]
pub async fn get_equipos_by_tipo(state: State<'_, RwLock<AppConfig>>, equipo_tipo: String) -> Result<Vec<Equipo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_by_tipo(equipo_tipo).await } else { db_impl::get_equipos_by_tipo(equipo_tipo).await }
}

#[tauri::command]
pub async fn get_equipos_by_created_by(state: State<'_, RwLock<AppConfig>>, created_by: i32) -> Result<Vec<Equipo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_by_created_by(created_by).await } else { db_impl::get_equipos_by_created_by(created_by).await }
}

#[tauri::command]
pub async fn search_equipos(state: State<'_, RwLock<AppConfig>>, search_term: String) -> Result<Vec<Equipo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::search_equipos(search_term).await } else { db_impl::search_equipos(search_term).await }
}

#[tauri::command]
pub async fn create_equipo(state: State<'_, RwLock<AppConfig>>, request: CreateEquipoRequest) -> Result<Equipo, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::create_equipo(request).await } else { db_impl::create_equipo(request).await }
}

#[tauri::command]
pub async fn update_equipo(state: State<'_, RwLock<AppConfig>>, equipo_id: i32, request: UpdateEquipoRequest, updated_by: i32) -> Result<Option<Equipo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::update_equipo(equipo_id, request, updated_by).await } else { db_impl::update_equipo(equipo_id, request, updated_by).await }
}

#[tauri::command]
pub async fn delete_equipo(state: State<'_, RwLock<AppConfig>>, equipo_id: i32, deleted_by: i32) -> Result<bool, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::delete_equipo(equipo_id, deleted_by).await } else { db_impl::delete_equipo(equipo_id, deleted_by).await }
}

#[tauri::command]
pub async fn count_equipos(state: State<'_, RwLock<AppConfig>>) -> Result<i64, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::count_equipos().await } else { db_impl::count_equipos().await }
}

#[tauri::command]
pub async fn get_equipos_with_pagination(state: State<'_, RwLock<AppConfig>>, offset: i64, limit: i64) -> Result<Vec<Equipo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_with_pagination(offset, limit).await } else { db_impl::get_equipos_with_pagination(offset, limit).await }
}

#[tauri::command]
pub async fn get_equipos_stats_by_tipo(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<(String, i64)>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_stats_by_tipo().await } else { db_impl::get_equipos_stats_by_tipo().await }
}

#[tauri::command]
pub async fn get_equipos_by_price_range(state: State<'_, RwLock<AppConfig>>, min_price: Option<i32>, max_price: Option<i32>) -> Result<Vec<Equipo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_by_price_range(min_price, max_price).await } else { db_impl::get_equipos_by_price_range(min_price, max_price).await }
}

#[tauri::command]
pub async fn get_equipos_with_cliente(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<EquipoWithCliente>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_with_cliente().await } else { db_impl::get_equipos_with_cliente().await }
}

#[tauri::command]
pub async fn transfer_equipo_to_cliente(state: State<'_, RwLock<AppConfig>>, equipo_id: i32, new_cliente_id: i32, updated_by: i32) -> Result<bool, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::transfer_equipo_to_cliente(equipo_id, new_cliente_id, updated_by).await } else { db_impl::transfer_equipo_to_cliente(equipo_id, new_cliente_id, updated_by).await }
}

#[tauri::command]
pub async fn get_equipos_marcas(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<String>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_marcas().await } else { db_impl::get_equipos_marcas().await }
}

#[tauri::command]
pub async fn get_equipos_modelos_by_marca(state: State<'_, RwLock<AppConfig>>, marca: String) -> Result<Vec<String>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_modelos_by_marca(marca).await } else { db_impl::get_equipos_modelos_by_marca(marca).await }
}

#[tauri::command]
pub async fn get_equipos_ubicaciones(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<String>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_ubicaciones().await } else { db_impl::get_equipos_ubicaciones().await }
}

#[tauri::command]
pub async fn registrar_salida_equipo(state: State<'_, RwLock<AppConfig>>, request: RegistrarSalidaRequest) -> Result<SalidaEquipoResponse, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::registrar_salida_equipo(request).await } else { db_impl::registrar_salida_equipo(request).await }
}

#[tauri::command]
pub async fn puede_registrar_salida_equipo(state: State<'_, RwLock<AppConfig>>, equipo_id: i32) -> Result<(bool, String), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::puede_registrar_salida_equipo(equipo_id).await } else { db_impl::puede_registrar_salida_equipo(equipo_id).await }
}

#[tauri::command]
pub async fn equipo_esta_en_sistema(state: State<'_, RwLock<AppConfig>>, equipo_id: i32) -> Result<(bool, String), String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::equipo_esta_en_sistema(equipo_id).await } else { db_impl::equipo_esta_en_sistema(equipo_id).await }
}

#[tauri::command]
pub async fn get_equipos_filtrados(state: State<'_, RwLock<AppConfig>>, filtros: FiltrosEquipos) -> Result<Vec<EquipoConEstado>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_filtrados(filtros).await } else { db_impl::get_equipos_filtrados(filtros).await }
}

#[tauri::command]
pub async fn get_equipos_en_sistema(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<Equipo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_en_sistema().await } else { db_impl::get_equipos_en_sistema().await }
}

#[tauri::command]
pub async fn get_equipos_fuera_sistema(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<Equipo>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_fuera_sistema().await } else { db_impl::get_equipos_fuera_sistema().await }
}

#[tauri::command]
pub async fn get_estadisticas_equipos_sistema(state: State<'_, RwLock<AppConfig>>) -> Result<serde_json::Value, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_estadisticas_equipos_sistema().await } else { db_impl::get_estadisticas_equipos_sistema().await }
}

#[tauri::command]
pub async fn get_equipos_con_estado(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<EquipoConEstado>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_equipos_con_estado().await } else { db_impl::get_equipos_con_estado().await }
}

#[tauri::command]
pub async fn get_clientes_con_equipos(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<String>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_clientes_con_equipos().await } else { db_impl::get_clientes_con_equipos().await }
}

#[tauri::command]
pub async fn get_tipos_equipos(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<String>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_tipos_equipos().await } else { db_impl::get_tipos_equipos().await }
}

#[tauri::command]
pub async fn get_estados_ordenes_trabajo(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<String>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_estados_ordenes_trabajo().await } else { db_impl::get_estados_ordenes_trabajo().await }
}

#[tauri::command]
pub async fn get_estadisticas_equipos_por_estado(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<(String, i64)>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_estadisticas_equipos_por_estado().await } else { db_impl::get_estadisticas_equipos_por_estado().await }
}
