-- Crear tabla de auditoría con campos ampliados integrados
CREATE TABLE IF NOT EXISTS AUDIT_LOG (
    log_id INT PRIMARY KEY AUTO_INCREMENT,
    log_accion VARCHAR(64),
    log_usuario_id INT,
    log_entidad_tabla VARCHAR(24),
    log_entidad_id INT,
    -- Campos ampliados a 512 caracteres (integrado desde migración 030)
    log_prev_v VARCHAR(512),
    log_new_v VARCHAR(512),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (log_usuario_id) REFERENCES USUARIO(usuario_id)
);

