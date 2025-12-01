use tauri::State;
use crate::config::AppConfig;
use crate::models::clientes::{
    Cliente, CreateClienteRequest, UpdateClienteRequest, FiltrosClientes, DeleteClienteRequest
};
use crate::infrastructure::db::clientes as db_impl;
use crate::infrastructure::api::clientes as api_impl;

#[tauri::command]
pub async fn get_clientes(state: State<'_, AppConfig>) -> Result<Vec<Cliente>, String> {
    if state.use_api { api_impl::get_clientes().await } else { db_impl::get_clientes().await }
}

#[tauri::command]
pub async fn get_cliente_by_id(state: State<'_, AppConfig>, cliente_id: i32) -> Result<Option<Cliente>, String> {
    if state.use_api { api_impl::get_cliente_by_id(cliente_id).await } else { db_impl::get_cliente_by_id(cliente_id).await }
}

#[tauri::command]
pub async fn get_cliente_by_rut(state: State<'_, AppConfig>, cliente_rut: String) -> Result<Option<Cliente>, String> {
    if state.use_api { api_impl::get_cliente_by_rut(cliente_rut).await } else { db_impl::get_cliente_by_rut(cliente_rut).await }
}

#[tauri::command]
pub async fn get_clientes_by_created_by(state: State<'_, AppConfig>, created_by: i32) -> Result<Vec<Cliente>, String> {
    if state.use_api { api_impl::get_clientes_by_created_by(created_by).await } else { db_impl::get_clientes_by_created_by(created_by).await }
}

#[tauri::command]
pub async fn search_clientes(state: State<'_, AppConfig>, search_term: String) -> Result<Vec<Cliente>, String> {
    if state.use_api { api_impl::search_clientes(search_term).await } else { db_impl::search_clientes(search_term).await }
}

#[tauri::command]
pub async fn create_cliente(state: State<'_, AppConfig>, request: CreateClienteRequest) -> Result<Cliente, String> {
    if state.use_api { api_impl::create_cliente(request).await } else { db_impl::create_cliente(request).await }
}

#[tauri::command]
pub async fn update_cliente(state: State<'_, AppConfig>, cliente_id: i32, request: UpdateClienteRequest, updated_by: i32) -> Result<Option<Cliente>, String> {
    if state.use_api { api_impl::update_cliente(cliente_id, request, updated_by).await } else { db_impl::update_cliente(cliente_id, request, updated_by).await }
}

#[tauri::command]
pub async fn delete_cliente(state: State<'_, AppConfig>, request: DeleteClienteRequest) -> Result<bool, String> {
    if state.use_api { api_impl::delete_cliente(request).await } else { db_impl::delete_cliente(request).await }
}

#[tauri::command]
pub async fn reactivate_cliente(state: State<'_, AppConfig>, cliente_id: i32, reactivated_by: i32) -> Result<bool, String> {
    if state.use_api { api_impl::reactivate_cliente(cliente_id, reactivated_by).await } else { db_impl::reactivate_cliente(cliente_id, reactivated_by).await }
}

#[tauri::command]
pub async fn count_clientes(state: State<'_, AppConfig>) -> Result<i64, String> {
    if state.use_api { api_impl::count_clientes().await } else { db_impl::count_clientes().await }
}

#[tauri::command]
pub async fn get_clientes_with_pagination(state: State<'_, AppConfig>, offset: i64, limit: i64) -> Result<Vec<Cliente>, String> {
    if state.use_api { api_impl::get_clientes_with_pagination(offset, limit).await } else { db_impl::get_clientes_with_pagination(offset, limit).await }
}

#[tauri::command]
pub async fn get_clientes_filtrados(state: State<'_, AppConfig>, filtros: FiltrosClientes) -> Result<Vec<Cliente>, String> {
    if state.use_api { api_impl::get_clientes_filtrados(filtros).await } else { db_impl::get_clientes_filtrados(filtros).await }
}

#[tauri::command]
pub async fn get_ruts_clientes(state: State<'_, AppConfig>) -> Result<Vec<String>, String> {
    if state.use_api { api_impl::get_ruts_clientes().await } else { db_impl::get_ruts_clientes().await }
}

#[tauri::command]
pub async fn get_correos_clientes(state: State<'_, AppConfig>) -> Result<Vec<String>, String> {
    if state.use_api { api_impl::get_correos_clientes().await } else { db_impl::get_correos_clientes().await }
}

#[tauri::command]
pub async fn get_ciudades_clientes(state: State<'_, AppConfig>) -> Result<Vec<String>, String> {
    if state.use_api { api_impl::get_ciudades_clientes().await } else { db_impl::get_ciudades_clientes().await }
}
