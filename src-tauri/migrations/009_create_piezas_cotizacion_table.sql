CREATE TABLE IF NOT EXISTS PIEZAS_COTIZACION (
    pieza_id INT,
    cotizacion_id INT,
    cantidad INT,
    PRIMARY KEY (pieza_id, cotizacion_id),
    FOREIGN KEY (pieza_id) REFERENCES PIEZA(pieza_id),
    FOREIGN KEY (cotizacion_id) REFERENCES COTIZACION(cotizacion_id)
);