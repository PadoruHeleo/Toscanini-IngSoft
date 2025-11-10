-- Insertar relaciones piezas-informe reales de la base de datos
-- Nota: Los IDs se ajustan al orden de inserción
-- Pieza ID 9 = Display LCD, Informe ID 1 (era 40 en respaldo)
INSERT IGNORE INTO PIEZAS_INFORME (pieza_id, informe_id, cantidad) VALUES
(9, 1, 1);
