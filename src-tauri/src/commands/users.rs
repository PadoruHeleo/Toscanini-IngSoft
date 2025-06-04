use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_unchecked;
use crate::utils::{hash_password, verify_password};

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
    
    // Encriptar la contraseña antes de guardarla
    let hashed_password = hash_password(&request.usuario_contrasena)?;
    
    let result = sqlx::query(
        "INSERT INTO USUARIO (usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&request.usuario_rut)
    .bind(&request.usuario_nombre)
    .bind(&request.usuario_correo)
    .bind(&hashed_password) // Usar la contraseña encriptada
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
    
    // Encriptar la contraseña si se proporciona
    let hashed_password = if let Some(ref password) = request.usuario_contrasena {
        Some(hash_password(password)?)
    } else {
        None
    };
    
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
    .bind(&hashed_password) // Usar la contraseña encriptada
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
pub async fn authenticate_usuario(usuario_correo: String, usuario_contrasena: String) -> Result<Option<Usuario>, String> {
    let pool = get_db_pool_unchecked();
    
    // Buscar el usuario por email
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT usuario_id, usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol 
         FROM USUARIO 
         WHERE usuario_correo = ?"
    )
    .bind(&usuario_correo)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    // Si encontramos el usuario, verificar la contraseña
    if let Some(user) = usuario {
        if let Some(ref stored_password) = user.usuario_contrasena {
            // Verificar la contraseña usando bcrypt
            if verify_password(&usuario_contrasena, stored_password)? {
                // Crear una copia del usuario sin la contraseña para enviar al frontend
                let safe_user = Usuario {
                    usuario_id: user.usuario_id,
                    usuario_rut: user.usuario_rut,
                    usuario_nombre: user.usuario_nombre,
                    usuario_correo: user.usuario_correo,
                    usuario_contrasena: None, // No enviar la contraseña al frontend
                    usuario_telefono: user.usuario_telefono,
                    usuario_rol: user.usuario_rol,
                };
                return Ok(Some(safe_user));
            }
        }
    }
    
    // Credenciales inválidas
    Ok(None)
}

#[tauri::command]
pub async fn create_admin_user() -> Result<Usuario, String> {
    let pool = get_db_pool_unchecked();
    
    // Verificar si ya existe un usuario admin
    let existing_admin = sqlx::query_as::<_, Usuario>(
        "SELECT usuario_id, usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol 
         FROM USUARIO 
         WHERE usuario_rol = 'admin' OR usuario_correo = ? OR usuario_rut = ?"
    )
    .bind("admin@toscanini.com")
    .bind("12345678-9")
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if let Some(admin) = existing_admin {
        // Retornar el usuario administrador existente sin la contraseña
        return Ok(Usuario {
            usuario_id: admin.usuario_id,
            usuario_rut: admin.usuario_rut,
            usuario_nombre: admin.usuario_nombre,
            usuario_correo: admin.usuario_correo,
            usuario_contrasena: None, // No retornar la contraseña por seguridad
            usuario_telefono: admin.usuario_telefono,
            usuario_rol: admin.usuario_rol,
        });
    }
    
    // Crear el usuario admin
    let hashed_password = hash_password("admin123")?;
    
    let result = sqlx::query(
        "INSERT INTO USUARIO (usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind("12345678-9")
    .bind("Administrador")
    .bind("admin@toscanini.com")
    .bind(&hashed_password)
    .bind("+56912345678")
    .bind("admin")
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let usuario_id = result.last_insert_id() as i32;
    
    // Obtener el usuario recién creado y retornarlo sin la contraseña
    let created_user = get_usuario_by_id(usuario_id)
        .await?
        .ok_or_else(|| "Failed to retrieve created admin user".to_string())?;
    
    Ok(Usuario {
        usuario_id: created_user.usuario_id,
        usuario_rut: created_user.usuario_rut,
        usuario_nombre: created_user.usuario_nombre,
        usuario_correo: created_user.usuario_correo,
        usuario_contrasena: None, // No retornar la contraseña por seguridad
        usuario_telefono: created_user.usuario_telefono,
        usuario_rol: created_user.usuario_rol,
    })
}