use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_unchecked;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub usuario_id: i32,
    pub usuario_rut: Option<String>,
    pub usuario_nombre: Option<String>,
    pub usuario_correo: Option<String>,
    pub usuario_contrasena: Option<String>,
    pub usuario_telefono: Option<String>,
    pub usuario_rol: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUsuarioRequest {
    pub usuario_rut: String,
    pub usuario_nombre: String,
    pub usuario_correo: String,
    pub usuario_contrasena: String,
    pub usuario_telefono: Option<String>,
    pub usuario_rol: String, // 'admin', 'tecnico', 'cliente'
}

#[derive(Debug, Deserialize)]
pub struct UpdateUsuarioRequest {
    pub usuario_rut: Option<String>,
    pub usuario_nombre: Option<String>,
    pub usuario_correo: Option<String>,
    pub usuario_contrasena: Option<String>,
    pub usuario_telefono: Option<String>,
    pub usuario_rol: Option<String>,
}

#[tauri::command]
pub async fn get_usuarios() -> Result<Vec<Usuario>, String> {
    let pool = get_db_pool_unchecked();
    
    let usuarios = sqlx::query_as::<_, Usuario>(
        "SELECT usuario_id, usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol FROM USUARIO"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(usuarios)
}

#[tauri::command]
pub async fn get_usuario_by_id(usuario_id: i32) -> Result<Option<Usuario>, String> {
    let pool = get_db_pool_unchecked();
    
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT usuario_id, usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol FROM USUARIO WHERE usuario_id = ?"
    )
    .bind(usuario_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(usuario)
}

#[tauri::command]
pub async fn get_usuario_by_rut(usuario_rut: String) -> Result<Option<Usuario>, String> {
    let pool = get_db_pool_unchecked();
    
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT usuario_id, usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol FROM USUARIO WHERE usuario_rut = ?"
    )
    .bind(usuario_rut)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(usuario)
}

#[tauri::command]
pub async fn create_usuario(request: CreateUsuarioRequest) -> Result<Usuario, String> {
    let pool = get_db_pool_unchecked();
    
    let result = sqlx::query(
        "INSERT INTO USUARIO (usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&request.usuario_rut)
    .bind(&request.usuario_nombre)
    .bind(&request.usuario_correo)
    .bind(&request.usuario_contrasena)
    .bind(&request.usuario_telefono)
    .bind(&request.usuario_rol)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let usuario_id = result.last_insert_id() as i32;
    
    // Obtener el usuario recién creado
    get_usuario_by_id(usuario_id)
        .await?
        .ok_or_else(|| "Failed to retrieve created usuario".to_string())
}

#[tauri::command]
pub async fn update_usuario(usuario_id: i32, request: UpdateUsuarioRequest) -> Result<Option<Usuario>, String> {
    let pool = get_db_pool_unchecked();
    
    let result = sqlx::query(
        "UPDATE USUARIO SET 
         usuario_rut = COALESCE(?, usuario_rut),
         usuario_nombre = COALESCE(?, usuario_nombre),
         usuario_correo = COALESCE(?, usuario_correo),
         usuario_contrasena = COALESCE(?, usuario_contrasena),
         usuario_telefono = COALESCE(?, usuario_telefono),
         usuario_rol = COALESCE(?, usuario_rol)
         WHERE usuario_id = ?"
    )
    .bind(&request.usuario_rut)
    .bind(&request.usuario_nombre)
    .bind(&request.usuario_correo)
    .bind(&request.usuario_contrasena)
    .bind(&request.usuario_telefono)
    .bind(&request.usuario_rol)
    .bind(usuario_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Ok(None);
    }
    
    get_usuario_by_id(usuario_id).await
}

#[tauri::command]
pub async fn delete_usuario(usuario_id: i32) -> Result<bool, String> {
    let pool = get_db_pool_unchecked();
    
    let result = sqlx::query("DELETE FROM USUARIO WHERE usuario_id = ?")
        .bind(usuario_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(result.rows_affected() > 0)
}

#[tauri::command]
pub async fn authenticate_usuario(usuario_rut: String, usuario_contrasena: String) -> Result<Option<Usuario>, String> {
    let pool = get_db_pool_unchecked();
    
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT usuario_id, usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol 
         FROM USUARIO 
         WHERE usuario_rut = ? AND usuario_contrasena = ?"
    )
    .bind(usuario_rut)
    .bind(usuario_contrasena)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(usuario)
}