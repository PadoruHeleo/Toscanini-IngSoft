CREATE TABLE IF NOT EXISTS PIEZAS_INFORME (
    pieza_id INT,
    informe_id INT,
    cantidad INT,
    PRIMARY KEY (pieza_id, informe_id),
    FOREIGN KEY (pieza_id) REFERENCES PIEZA(pieza_id),
    FOREIGN KEY (informe_id) REFERENCES INFORME(informe_id)
);