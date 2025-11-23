-- Insertar relaciones términos-informe reales de la base de datos
-- Nota: Los IDs se ajustan al orden de inserción
-- Términos IDs 1-4, Informe ID 1 (era 40 en respaldo)
INSERT IGNORE INTO TERMINOS_INFORME (termino_id, informe_id, aplicado) VALUES
(1, 1, 1), -- Garantía Estándar
(2, 1, 1), -- Responsabilidad por Pérdida
(3, 1, 1), -- Condiciones de Pago
(4, 1, 1); -- Diagnóstico Técnico
