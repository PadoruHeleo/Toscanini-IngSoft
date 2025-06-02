CREATE TABLE IF NOT EXISTS CLIENTE (
    cliente_id INT PRIMARY KEY AUTO_INCREMENT,
    cliente_rut VARCHAR(11) UNIQUE,
    cliente_nombre VARCHAR(64),
    cliente_correo VARCHAR(256),
    cliente_telefono VARCHAR(16),
    cliente_direccion VARCHAR(512),
    created_by INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES USUARIO(usuario_id)
);