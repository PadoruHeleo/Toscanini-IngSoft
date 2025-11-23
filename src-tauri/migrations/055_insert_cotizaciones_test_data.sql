-- Insertar cotizaciones reales de la base de datos
-- Nota: Los IDs se ajustan al orden de inserción
-- Usuario ID 1 (era 6 en respaldo)
INSERT IGNORE INTO COTIZACION (cotizacion_codigo, costo_revision, costo_reparacion, costo_total, is_aprobada, is_borrador, informe, created_by) VALUES
('COT-2025-001', 25000, 25000, 125000, 1, 0, 'Se necesita remplazar la panalla', 1),
('COT-2025-002', 25000, 78865, 103865, 0, 0, 'u', 1),
('COT-2025-003', 25000, 89789, 124289, 0, 1, 'fghj', 1);
