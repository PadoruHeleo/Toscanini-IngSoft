use std::sync::RwLock;
use tauri::State;
use crate::config::AppConfig;
use crate::models::users::{
    Usuario, CreateUsuarioRequest, UpdateUsuarioRequest, 
    RequestPasswordResetRequest, ResetPasswordRequest, 
    ChangePasswordRequest, ChangeEmailRequest, ChangePhoneRequest
};

// Importamos las implementaciones
use crate::infrastructure::db::users as db_impl;
use crate::infrastructure::api::users as api_impl;

// CRUD Básico (Híbrido)
#[tauri::command]
pub async fn get_usuarios(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<Usuario>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_usuarios().await } else { db_impl::get_usuarios().await }
}

#[tauri::command]
pub async fn get_usuario_by_id(state: State<'_, RwLock<AppConfig>>, usuario_id: i32) -> Result<Option<Usuario>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_usuario_by_id(usuario_id).await } else { db_impl::get_usuario_by_id(usuario_id).await }
}

#[tauri::command]
pub async fn get_usuario_by_rut(state: State<'_, RwLock<AppConfig>>, usuario_rut: String) -> Result<Option<Usuario>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_usuario_by_rut(usuario_rut).await } else { db_impl::get_usuario_by_rut(usuario_rut).await }
}

#[tauri::command]
pub async fn create_usuario(state: State<'_, RwLock<AppConfig>>, request: CreateUsuarioRequest) -> Result<Usuario, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::create_usuario(request).await } else { db_impl::create_usuario(request).await }
}

#[tauri::command]
pub async fn update_usuario(state: State<'_, RwLock<AppConfig>>, usuario_id: i32, request: UpdateUsuarioRequest) -> Result<Option<Usuario>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::update_usuario(usuario_id, request).await } else { db_impl::update_usuario(usuario_id, request).await }
}

#[tauri::command]
pub async fn delete_usuario(state: State<'_, RwLock<AppConfig>>, usuario_id: i32) -> Result<bool, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::delete_usuario(usuario_id).await } else { db_impl::delete_usuario(usuario_id).await }
}

// Autenticación (Híbrido)
#[tauri::command]
pub async fn authenticate_usuario(state: State<'_, RwLock<AppConfig>>, usuario_correo: String, usuario_contrasena: String) -> Result<Option<Usuario>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::authenticate_usuario(usuario_correo, usuario_contrasena).await } else { db_impl::authenticate_usuario(usuario_correo, usuario_contrasena).await }
}

#[tauri::command]
pub async fn validate_session(state: State<'_, RwLock<AppConfig>>, session_token: String) -> Result<Option<Usuario>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::validate_session(session_token).await } else { db_impl::validate_session(session_token).await }
}

#[tauri::command]
pub async fn logout_user(state: State<'_, RwLock<AppConfig>>, session_token: String) -> Result<bool, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::logout_user(session_token).await } else { db_impl::logout_user(session_token).await }
}

// Gestión de Cuenta (Híbrido)
#[tauri::command]
pub async fn change_user_password(state: State<'_, RwLock<AppConfig>>, usuario_id: i32, request: ChangePasswordRequest) -> Result<bool, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::change_user_password(usuario_id, request).await } else { db_impl::change_user_password(usuario_id, request).await }
}

#[tauri::command]
pub async fn change_user_email(state: State<'_, RwLock<AppConfig>>, usuario_id: i32, request: ChangeEmailRequest) -> Result<Option<Usuario>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::change_user_email(usuario_id, request).await } else { db_impl::change_user_email(usuario_id, request).await }
}

#[tauri::command]
pub async fn change_user_phone(state: State<'_, RwLock<AppConfig>>, usuario_id: i32, request: ChangePhoneRequest) -> Result<Option<Usuario>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::change_user_phone(usuario_id, request).await } else { db_impl::change_user_phone(usuario_id, request).await }
}

// Recuperación de Contraseña (Híbrido)
#[tauri::command]
pub async fn request_password_reset(state: State<'_, RwLock<AppConfig>>, request: RequestPasswordResetRequest) -> Result<String, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::request_password_reset(request).await } else { db_impl::request_password_reset(request).await }
}

#[tauri::command]
pub async fn verify_reset_code(state: State<'_, RwLock<AppConfig>>, reset_code: String) -> Result<bool, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::verify_reset_code(reset_code).await } else { db_impl::verify_reset_code(reset_code).await }
}

#[tauri::command]
pub async fn reset_password_with_code(state: State<'_, RwLock<AppConfig>>, request: ResetPasswordRequest) -> Result<String, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::reset_password_with_code(request).await } else { db_impl::reset_password_with_code(request).await }
}

// Mantenimiento (Híbrido)
#[tauri::command]
pub async fn cleanup_expired_reset_codes(state: State<'_, RwLock<AppConfig>>) -> Result<u64, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::cleanup_expired_reset_codes().await } else { db_impl::cleanup_expired_reset_codes().await }
}

#[tauri::command]
pub async fn cleanup_expired_sessions(state: State<'_, RwLock<AppConfig>>) -> Result<u64, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::cleanup_expired_sessions().await } else { db_impl::cleanup_expired_sessions().await }
}

#[tauri::command]
pub async fn create_admin_user(state: State<'_, RwLock<AppConfig>>) -> Result<Usuario, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::create_admin_user().await } else { db_impl::create_admin_user().await }
}

#[tauri::command]
pub async fn get_admin_and_tech_emails(state: State<'_, RwLock<AppConfig>>) -> Result<Vec<String>, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { api_impl::get_admin_and_tech_emails().await } else { db_impl::get_admin_and_tech_emails().await }
}

// === FUNCIONES DE LÓGICA PURA O UTILITARIA (Sin duplicar en API) ===

#[tauri::command]
pub async fn verify_phone(_state: State<'_, RwLock<AppConfig>>, phone: String) -> Result<bool, String> {
    // Es una validación de formato (Regex). Usamos siempre la implementación local.
    // En db/users.rs la función no es async, así que la envolvemos en Ok()
    Ok(db_impl::verify_phone(phone))
}

#[tauri::command]
pub async fn send_password_email(_state: State<'_, RwLock<AppConfig>>, to_email: &str, user_name: &str, temp_password: &str) -> Result<(), String> {
    // Usamos el servicio de email local (SMTP) en ambos casos.
    // Esto evita tener que crear un endpoint especial en la API solo para "enviar un correo arbitrario".
    db_impl::send_password_email(to_email, user_name, temp_password).await
}

// === VALIDACIONES QUE REQUIEREN BASE DE DATOS (Siguen siendo híbridas) ===

#[tauri::command]
pub async fn verify_rut_in_use(state: State<'_, RwLock<AppConfig>>, rut: String) -> Result<bool, String> {
    // Tauri exige Result<T, E>, convertimos el bool directo a Ok(bool)
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { 
        Ok(api_impl::verify_rut_in_use(rut).await) 
    } else { 
        Ok(db_impl::verify_rut_in_use(rut).await) 
    }
}

#[tauri::command]
pub async fn verify_email_in_use(state: State<'_, RwLock<AppConfig>>, correo: String) -> Result<bool, String> {
    let use_api = state.read().map_err(|_| "Error de lectura de configuración")?.use_api;
    if use_api { 
        Ok(api_impl::verify_email_in_use(correo).await) 
    } else { 
        Ok(db_impl::verify_email_in_use(correo).await) 
    }
}