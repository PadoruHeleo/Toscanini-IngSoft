use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub usuario_id: i32,
    pub usuario_rut: Option<String>,
    pub usuario_nombre: Option<String>,
    pub usuario_correo: Option<String>,
    pub usuario_contrasena: Option<String>,
    pub usuario_telefono: Option<String>,
    pub usuario_rol: Option<String>,
    pub is_active: Option<bool>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub session_expires_at: Option<DateTime<Utc>>,
    pub session_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUsuarioRequest {
    pub usuario_rut: String,
    pub usuario_nombre: String,
    pub usuario_correo: String,
    pub usuario_contrasena: String,
    pub usuario_telefono: Option<String>,
    pub usuario_rol: String, // 'admin', 'tecnico', 'cliente'
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestPasswordResetRequest {
    pub usuario_correo: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetPasswordRequest {
    pub reset_code: String,
    pub nueva_contrasena: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChangeEmailRequest {
    pub new_email: String,
    pub password: String, // Requiere contraseña actual para confirmar
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChangePhoneRequest {
    pub new_phone: String,
    pub password: String, // Requiere contraseña actual para confirmar
}
