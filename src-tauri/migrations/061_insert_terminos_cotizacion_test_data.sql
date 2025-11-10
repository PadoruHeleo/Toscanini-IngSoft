-- Insertar relaciones términos-cotización reales de la base de datos
-- Nota: Los IDs se ajustan al orden de inserción
-- Términos IDs 1-3 y 10, Cotizaciones IDs 1-3 (eran 55, 56, 57 en respaldo)
INSERT IGNORE INTO TERMINOS_COTIZACION (termino_id, cotizacion_id, aplicado) VALUES
(1, 1, 1),  -- Garantía Estándar para COT-2025-001
(1, 2, 1),  -- Garantía Estándar para COT-2025-002
(1, 3, 1),  -- Garantía Estándar para COT-2025-003
(2, 1, 1),  -- Responsabilidad por Pérdida para COT-2025-001
(2, 2, 1),  -- Responsabilidad por Pérdida para COT-2025-002
(2, 3, 1),  -- Responsabilidad por Pérdida para COT-2025-003
(3, 1, 1),  -- Condiciones de Pago para COT-2025-001
(3, 2, 1),  -- Condiciones de Pago para COT-2025-002
(3, 3, 1),  -- Condiciones de Pago para COT-2025-003
