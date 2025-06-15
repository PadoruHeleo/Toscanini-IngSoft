CREATE TABLE IF NOT EXISTS COTIZACION (
    cotizacion_id INT PRIMARY KEY AUTO_INCREMENT,
    cotizacion_codigo VARCHAR(12) UNIQUE,
    costo_revision INT,
    costo_reparacion INT,
    costo_total INT,
    is_aprobada BOOLEAN,
    is_borrador BOOLEAN DEFAULT FALSE,
    informe TEXT NOT NULL,
    created_by INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES USUARIO(usuario_id)
);