CREATE TABLE IF NOT EXISTS EQUIPO (
    equipo_id INT PRIMARY KEY AUTO_INCREMENT,
    numero_serie VARCHAR(32) UNIQUE,
    equipo_marca VARCHAR(32),
    equipo_modelo VARCHAR(64),
    equipo_tipo SET('radio', 'antena', 'repetidor', 'otro'),
    equipo_precio INT,
    equipo_ubicacion VARCHAR(256),
    cliente_id INT,
    created_by INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (cliente_id) REFERENCES CLIENTE(cliente_id),
    FOREIGN KEY (created_by) REFERENCES USUARIO(usuario_id)
);