-- Crear tabla de relación entre términos y condiciones con cotizaciones
CREATE TABLE IF NOT EXISTS TERMINOS_COTIZACION (
    termino_id INT,
    cotizacion_id INT,
    aplicado BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (termino_id, cotizacion_id),
    FOREIGN KEY (termino_id) REFERENCES TERMINOS_CONDICIONES(termino_id) ON DELETE CASCADE,
    FOREIGN KEY (cotizacion_id) REFERENCES COTIZACION(cotizacion_id) ON DELETE CASCADE
);

