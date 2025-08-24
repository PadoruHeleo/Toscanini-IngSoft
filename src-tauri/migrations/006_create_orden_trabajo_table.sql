CREATE TABLE IF NOT EXISTS ORDEN_TRABAJO (
    orden_id INT PRIMARY KEY AUTO_INCREMENT,
    orden_codigo VARCHAR(50) UNIQUE,
    orden_desc VARCHAR(1024),
    prioridad ENUM('baja', 'media', 'alta'),    estado ENUM(
        'recibido',
        'cotizacion_enviada',
        'aprobacion_pendiente',
        'en_reparacion',
        'espera_de_retiro',
        'entregado',
        'abandonado',
        'equipo_no_reparable',
        'cotizacion_rechazada'
    ) NOT NULL,
    has_garantia BOOLEAN,
    equipo_id INT,
    created_by INT,
    cotizacion_id INT,
    informe_id INT,
    pre_informe TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    finished_at TIMESTAMP NULL DEFAULT NULL,
    FOREIGN KEY (equipo_id) REFERENCES EQUIPO(equipo_id),
    FOREIGN KEY (created_by) REFERENCES USUARIO(usuario_id),
    FOREIGN KEY (cotizacion_id) REFERENCES COTIZACION(cotizacion_id),
    FOREIGN KEY (informe_id) REFERENCES INFORME(informe_id)
);