use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_safe;
use crate::utils::{hash_password, verify_password};
use crate::commands::logs::log_action;
use crate::email::EmailService;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub usuario_id: i32,
    pub usuario_rut: Option<String>,
    pub usuario_nombre: Option<String>,
    pub usuario_correo: Option<String>,
    pub usuario_contrasena: Option<String>,
    pub usuario_telefono: Option<String>,
    pub usuario_rol: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub session_expires_at: Option<DateTime<Utc>>,
    pub session_token: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PasswordReset {
    pub reset_id: i32,
    pub usuario_id: i32,
    pub reset_code: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
}

#[derive(Debug, Deserialize)]
pub struct RequestPasswordResetRequest {
    pub usuario_correo: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub reset_code: String,
    pub nueva_contrasena: String,
}

#[tauri::command]
pub async fn get_usuarios() -> Result<Vec<Usuario>, String> {
    let pool = get_db_pool_safe()?;
      let usuarios = sqlx::query_as::<_, Usuario>(
        "SELECT usuario_id, usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol, last_login_at, session_expires_at, session_token FROM USUARIO"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(usuarios)
}

#[tauri::command]
pub async fn get_usuario_by_id(usuario_id: i32) -> Result<Option<Usuario>, String> {
    let pool = get_db_pool_safe()?;
      let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT usuario_id, usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol, last_login_at, session_expires_at, session_token FROM USUARIO WHERE usuario_id = ?"
    )
    .bind(usuario_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(usuario)
}

#[tauri::command]
pub async fn get_usuario_by_rut(usuario_rut: String) -> Result<Option<Usuario>, String> {
    let pool = get_db_pool_safe()?;
      let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT usuario_id, usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol, last_login_at, session_expires_at, session_token FROM USUARIO WHERE usuario_rut = ?"
    )
    .bind(usuario_rut)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(usuario)
}

#[tauri::command]
pub async fn create_usuario(request: CreateUsuarioRequest) -> Result<Usuario, String> {
    let pool = get_db_pool_safe()?;
    
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
    let pool = get_db_pool_safe()?;
    
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
    let pool = get_db_pool_safe()?;
    
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
    let pool = get_db_pool_safe()?;
    
    // Buscar el usuario por email
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT usuario_id, usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol, last_login_at, session_expires_at, session_token 
         FROM USUARIO 
         WHERE usuario_correo = ?"
    )
    .bind(&usuario_correo)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    // Si encontramos el usuario, verificar la contraseña
    if let Some(user) = usuario {
        if let Some(ref stored_password) = user.usuario_contrasena {            // Verificar la contraseña usando bcrypt
            if verify_password(&usuario_contrasena, stored_password)? {
                // Generar token de sesión
                let session_token = Uuid::new_v4().to_string();
                let login_time = Utc::now();
                let session_expires = login_time + Duration::hours(16);
                
                // Actualizar campos de sesión en la base de datos
                let _ = sqlx::query(
                    "UPDATE USUARIO SET last_login_at = ?, session_expires_at = ?, session_token = ? WHERE usuario_id = ?"
                )
                .bind(login_time)
                .bind(session_expires)
                .bind(&session_token)
                .bind(user.usuario_id)
                .execute(pool)
                .await;
                
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
                    last_login_at: Some(login_time),
                    session_expires_at: Some(session_expires),
                    session_token: Some(session_token),
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
    let pool = get_db_pool_safe()?;
      // Verificar si ya existe un usuario admin
    let existing_admin = sqlx::query_as::<_, Usuario>(
        "SELECT usuario_id, usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol, last_login_at, session_expires_at, session_token 
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
            last_login_at: admin.last_login_at,
            session_expires_at: admin.session_expires_at,
            session_token: None, // No retornar el token de sesión por seguridad
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
        last_login_at: created_user.last_login_at,
        session_expires_at: created_user.session_expires_at,
        session_token: None, // No retornar el token de sesión por seguridad
    })
}

#[tauri::command]
pub async fn request_password_reset(request: RequestPasswordResetRequest) -> Result<String, String> {
    let pool = get_db_pool_safe()?;
      // Buscar el usuario por email
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT usuario_id, usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol, last_login_at, session_expires_at, session_token 
         FROM USUARIO 
         WHERE usuario_correo = ?"
    )
    .bind(&request.usuario_correo)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let user = match usuario {
        Some(u) => u,
        None => {
            // Registrar intento de recuperación con correo inexistente
            let _ = log_action(
                "PASSWORD_RESET_FAILED",
                None,
                "USUARIO",
                None,
                None,
                Some(&format!("Intento de recuperación con correo inexistente: {}", request.usuario_correo))
            ).await;
            return Err("Correo electrónico no encontrado".to_string());
        }
    };
      // Generar código de 6 dígitos
    let reset_code: String = {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..6).map(|_| rng.gen_range(0..10).to_string()).collect()
    };
    
    // Calcular tiempo de expiración (15 minutos desde ahora)
    let expires_at = Utc::now() + Duration::minutes(15);
    
    // Limpiar códigos anteriores no usados de este usuario
    let _ = sqlx::query(
        "UPDATE PASSWORD_RESET SET used = TRUE WHERE usuario_id = ? AND used = FALSE"
    )
    .bind(user.usuario_id)
    .execute(pool)
    .await;
    
    // Insertar nuevo código de recuperación
    sqlx::query(
        "INSERT INTO PASSWORD_RESET (usuario_id, reset_code, expires_at) VALUES (?, ?, ?)"
    )
    .bind(user.usuario_id)
    .bind(&reset_code)
    .bind(expires_at)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    // Enviar correo electrónico
    let email_service = EmailService::new()
        .map_err(|e| format!("Email service error: {}", e))?;
    
    let user_name = user.usuario_nombre.as_deref().unwrap_or("Usuario");
    email_service.send_password_reset_email(&request.usuario_correo, &reset_code, user_name)
        .await
        .map_err(|e| format!("Error sending email: {}", e))?;
    
    // Registrar solicitud exitosa
    let _ = log_action(
        "PASSWORD_RESET_REQUESTED",
        Some(user.usuario_id),
        "USUARIO",
        Some(user.usuario_id),
        None,
        Some(&format!("Código de recuperación enviado a {}", request.usuario_correo))
    ).await;
    
    Ok("Código de recuperación enviado a tu correo electrónico".to_string())
}

#[tauri::command]
pub async fn verify_reset_code(reset_code: String) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Buscar código válido y no expirado
    let reset_entry = sqlx::query_as::<_, PasswordReset>(
        "SELECT reset_id, usuario_id, reset_code, created_at, expires_at, used 
         FROM PASSWORD_RESET 
         WHERE reset_code = ? AND used = FALSE AND expires_at > UTC_TIMESTAMP()"
    )
    .bind(&reset_code)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    match reset_entry {
        Some(_) => Ok(true),
        None => {
            // Registrar intento de verificación fallido
            let _ = log_action(
                "PASSWORD_RESET_VERIFY_FAILED",
                None,
                "PASSWORD_RESET",
                None,
                None,
                Some(&format!("Código de verificación inválido o expirado: {}", reset_code))
            ).await;
            Ok(false)
        }
    }
}

#[tauri::command]
pub async fn reset_password_with_code(request: ResetPasswordRequest) -> Result<String, String> {
    let pool = get_db_pool_safe()?;
    
    // Buscar código válido y no expirado
    let reset_entry = sqlx::query_as::<_, PasswordReset>(
        "SELECT reset_id, usuario_id, reset_code, created_at, expires_at, used 
         FROM PASSWORD_RESET 
         WHERE reset_code = ? AND used = FALSE AND expires_at > UTC_TIMESTAMP()"
    )
    .bind(&request.reset_code)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let reset = match reset_entry {
        Some(r) => r,
        None => {
            let _ = log_action(
                "PASSWORD_RESET_FAILED",
                None,
                "PASSWORD_RESET",
                None,
                None,
                Some(&format!("Intento de cambio con código inválido: {}", request.reset_code))
            ).await;
            return Err("Código de recuperación inválido o expirado".to_string());
        }
    };
    
    // Encriptar la nueva contraseña
    let hashed_password = hash_password(&request.nueva_contrasena)?;
    
    // Actualizar la contraseña del usuario
    let result = sqlx::query(
        "UPDATE USUARIO SET usuario_contrasena = ? WHERE usuario_id = ?"
    )
    .bind(&hashed_password)
    .bind(reset.usuario_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("Usuario no encontrado".to_string());
    }
    
    // Marcar el código como usado
    sqlx::query("UPDATE PASSWORD_RESET SET used = TRUE WHERE reset_id = ?")
        .bind(reset.reset_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Registrar cambio exitoso
    let _ = log_action(
        "PASSWORD_RESET_SUCCESS",
        Some(reset.usuario_id),
        "USUARIO",
        Some(reset.usuario_id),
        None,
        Some("Contraseña cambiada exitosamente usando código de recuperación")
    ).await;
    
    Ok("Contraseña cambiada exitosamente".to_string())
}

#[tauri::command]
pub async fn cleanup_expired_reset_codes() -> Result<u64, String> {
    let pool = get_db_pool_safe()?;
      let result = sqlx::query(
        "DELETE FROM PASSWORD_RESET WHERE expires_at < UTC_TIMESTAMP() OR used = TRUE"
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let deleted_count = result.rows_affected();
    
    // Registrar limpieza
    let _ = log_action(
        "CLEANUP_RESET_CODES",
        None,
        "PASSWORD_RESET",
        None,
        None,
        Some(&format!("Códigos de recuperación eliminados: {}", deleted_count))
    ).await;
    
    Ok(deleted_count)
}

#[tauri::command]
pub async fn validate_session(session_token: String) -> Result<Option<Usuario>, String> {
    let pool = get_db_pool_safe()?;
    
    // Buscar usuario por token de sesión válido y no expirado
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT usuario_id, usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol, last_login_at, session_expires_at, session_token
         FROM USUARIO 
         WHERE session_token = ? AND session_expires_at > UTC_TIMESTAMP()"
    )
    .bind(&session_token)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    match usuario {
        Some(user) => {
            // Registrar validación exitosa
            let _ = log_action(
                "SESSION_VALIDATED",
                Some(user.usuario_id),
                "USUARIO",
                Some(user.usuario_id),
                None,
                Some(&format!("Sesión validada para usuario {}", user.usuario_correo.as_deref().unwrap_or("N/A")))
            ).await;
            
            // Retornar usuario sin datos sensibles
            Ok(Some(Usuario {
                usuario_id: user.usuario_id,
                usuario_rut: user.usuario_rut,
                usuario_nombre: user.usuario_nombre,
                usuario_correo: user.usuario_correo,
                usuario_contrasena: None,
                usuario_telefono: user.usuario_telefono,
                usuario_rol: user.usuario_rol,
                last_login_at: user.last_login_at,
                session_expires_at: user.session_expires_at,
                session_token: Some(session_token),
            }))
        }
        None => {
            // Registrar validación fallida
            let _ = log_action(
                "SESSION_VALIDATION_FAILED",
                None,
                "USUARIO",
                None,
                None,
                Some(&format!("Token de sesión inválido o expirado: {}", session_token))
            ).await;
            Ok(None)
        }
    }
}

#[tauri::command]
pub async fn logout_user(session_token: String) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Limpiar token de sesión del usuario
    let result = sqlx::query(
        "UPDATE USUARIO SET session_token = NULL, session_expires_at = NULL WHERE session_token = ?"
    )
    .bind(&session_token)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let was_logged_out = result.rows_affected() > 0;
    
    if was_logged_out {
        let _ = log_action(
            "LOGOUT_SUCCESS",
            None,
            "USUARIO",
            None,
            None,
            Some("Usuario cerró sesión exitosamente")
        ).await;
    }
    
    Ok(was_logged_out)
}

#[tauri::command]
pub async fn cleanup_expired_sessions() -> Result<u64, String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query(
        "UPDATE USUARIO SET session_token = NULL, session_expires_at = NULL, last_login_at = NULL 
         WHERE session_expires_at < UTC_TIMESTAMP() AND session_token IS NOT NULL"
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let cleaned_count = result.rows_affected();
    
    // Registrar limpieza
    let _ = log_action(
        "CLEANUP_EXPIRED_SESSIONS",
        None,
        "USUARIO",
        None,
        None,
        Some(&format!("Sesiones expiradas limpiadas: {}", cleaned_count))
    ).await;
    
    Ok(cleaned_count)
}
