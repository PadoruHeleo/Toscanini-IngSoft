-- Crear tabla de relación entre términos y condiciones con informes
CREATE TABLE IF NOT EXISTS TERMINOS_INFORME (
    termino_id INT,
    informe_id INT,
    aplicado BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (termino_id, informe_id),
    FOREIGN KEY (termino_id) REFERENCES TERMINOS_CONDICIONES(termino_id) ON DELETE CASCADE,
    FOREIGN KEY (informe_id) REFERENCES INFORME(informe_id) ON DELETE CASCADE
);