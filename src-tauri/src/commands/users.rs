use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_unchecked;
use crate::utils::{hash_password, verify_password};
use crate::commands::logs::log_action;

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
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "CREATE_USER",
        None, // No hay usuario autenticado en la creación
        "USUARIO",
        Some(usuario_id),
        None,
        Some(&format!("Usuario creado: {} ({})", request.usuario_nombre, request.usuario_correo))
    ).await;
    
    // Obtener el usuario recién creado
    get_usuario_by_id(usuario_id)
        .await?
        .ok_or_else(|| "Failed to retrieve created usuario".to_string())
}

#[tauri::command]
pub async fn update_usuario(usuario_id: i32, request: UpdateUsuarioRequest) -> Result<Option<Usuario>, String> {
    let pool = get_db_pool_unchecked();
    
    // Obtener el usuario actual para logging
    let current_user = get_usuario_by_id(usuario_id).await?;
    
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
    
    // Registrar la acción en el log de auditoría
    if let Some(ref user) = current_user {
        let prev_data = format!("{}|{}|{}", 
            user.usuario_nombre.as_deref().unwrap_or(""), 
            user.usuario_correo.as_deref().unwrap_or(""),
            user.usuario_rol.as_deref().unwrap_or("")
        );
        let new_data = format!("{}|{}|{}", 
            request.usuario_nombre.as_deref().unwrap_or(user.usuario_nombre.as_deref().unwrap_or("")),
            request.usuario_correo.as_deref().unwrap_or(user.usuario_correo.as_deref().unwrap_or("")),
            request.usuario_rol.as_deref().unwrap_or(user.usuario_rol.as_deref().unwrap_or(""))
        );
        
        let _ = log_action(
            "UPDATE_USER",
            None,
            "USUARIO",
            Some(usuario_id),
            Some(&prev_data),
            Some(&new_data)
        ).await;
    }
    
    get_usuario_by_id(usuario_id).await
}

#[tauri::command]
pub async fn delete_usuario(usuario_id: i32) -> Result<bool, String> {
    let pool = get_db_pool_unchecked();
    
    // Obtener el usuario antes de eliminarlo para logging
    let user_to_delete = get_usuario_by_id(usuario_id).await?;
    
    let result = sqlx::query("DELETE FROM USUARIO WHERE usuario_id = ?")
        .bind(usuario_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    let was_deleted = result.rows_affected() > 0;
    
    // Registrar la acción en el log de auditoría
    if was_deleted {
        if let Some(ref user) = user_to_delete {
            let _ = log_action(
                "DELETE_USER",
                None,
                "USUARIO",
                Some(usuario_id),
                Some(&format!("Usuario eliminado: {} ({})", 
                    user.usuario_nombre.as_deref().unwrap_or("N/A"),
                    user.usuario_correo.as_deref().unwrap_or("N/A")
                )),
                None
            ).await;
        }
    }
    
    Ok(was_deleted)
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
                // Registrar login exitoso en el log de auditoría
                let _ = log_action(
                    "LOGIN_SUCCESS",
                    Some(user.usuario_id),
                    "USUARIO",
                    Some(user.usuario_id),
                    None,
                    Some(&format!("Login exitoso para {}", usuario_correo))
                ).await;
                
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
            } else {
                // Registrar intento de login fallido
                let _ = log_action(
                    "LOGIN_FAILED",
                    None,
                    "USUARIO",
                    Some(user.usuario_id),
                    None,
                    Some(&format!("Intento de login fallido para {}", usuario_correo))
                ).await;
            }
        }
    } else {
        // Registrar intento de login con usuario inexistente
        let _ = log_action(
            "LOGIN_FAILED",
            None,
            "USUARIO",
            None,
            None,
            Some(&format!("Intento de login con usuario inexistente: {}", usuario_correo))
        ).await;
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
        // Registrar que se encontró un admin existente
        let _ = log_action(
            "ADMIN_FOUND",
            None,
            "USUARIO",
            Some(admin.usuario_id),
            None,
            Some(&format!("Usuario administrador existente encontrado: {}", 
                admin.usuario_correo.as_deref().unwrap_or("N/A")
            ))
        ).await;
        
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
    
    // Registrar la creación del admin en el log de auditoría
    let _ = log_action(
        "ADMIN_CREATED",
        None,
        "USUARIO",
        Some(usuario_id),
        None,
        Some("Usuario administrador creado durante la configuración inicial")
    ).await;
    
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