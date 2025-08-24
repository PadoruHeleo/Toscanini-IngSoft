CREATE TABLE IF NOT EXISTS USUARIO (
    usuario_id INT PRIMARY KEY AUTO_INCREMENT,
    usuario_rut VARCHAR(11) UNIQUE,
    usuario_nombre VARCHAR(64),
    usuario_correo VARCHAR(256),
    usuario_contrasena VARCHAR(64),
    usuario_telefono VARCHAR(16),
    usuario_rol ENUM('admin', 'tecnico', 'recepcion')
);