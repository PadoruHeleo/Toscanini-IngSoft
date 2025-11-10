-- Crear tabla de usuarios con campos de sesión integrados
CREATE TABLE IF NOT EXISTS USUARIO (
    usuario_id INT PRIMARY KEY AUTO_INCREMENT,
    usuario_rut VARCHAR(12) UNIQUE,
    usuario_nombre VARCHAR(64),
    usuario_correo VARCHAR(256),
    usuario_contrasena VARCHAR(64),
    usuario_telefono VARCHAR(16),
    is_active BOOLEAN DEFAULT TRUE,
    usuario_rol ENUM('admin', 'tecnico', 'recepcion'),
    -- Campos de sesión (integrados desde migración 012)
    last_login_at TIMESTAMP NULL,
    session_expires_at TIMESTAMP NULL,
    session_token VARCHAR(255) NULL
);
