CREATE TABLE IF NOT EXISTS AUDIT_LOG (
    log_id INT PRIMARY KEY AUTO_INCREMENT,
    log_accion VARCHAR(64),
    log_usuario_id INT,
    log_entidad_tabla VARCHAR(24),
    log_entidad_id INT,
    log_prev_v VARCHAR(32),
    log_new_v VARCHAR(32),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (log_usuario_id) REFERENCES USUARIO(usuario_id)
);