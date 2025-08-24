-- Agregar campos de sesión a la tabla USUARIO
ALTER TABLE USUARIO 
ADD COLUMN last_login_at TIMESTAMP NULL,
ADD COLUMN session_expires_at TIMESTAMP NULL,
ADD COLUMN session_token VARCHAR(255) NULL;
