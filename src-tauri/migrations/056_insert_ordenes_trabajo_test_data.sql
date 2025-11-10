-- Insertar órdenes de trabajo reales de la base de datos
-- Nota: Los IDs se ajustan al orden de inserción
-- Equipo ID 1 (era 85 en respaldo), Cotización ID 3 (era 57 en respaldo), Informe ID 1 (era 40 en respaldo)
INSERT IGNORE INTO ORDEN_TRABAJO (orden_codigo, orden_desc, prioridad, estado, has_garantia, equipo_id, created_by, cotizacion_id, informe_id, pre_informe) VALUES
('OT-2025-002', 'El equipo Motorola M5 presenta ik', 'media', 'en_reparacion', 0, 1, 1, 3, NULL, 'ik'),
('OT-2025-003', 'El equipo Motorola M5 presenta Esta mojado', 'media', 'recibido', 1, 1, 1, NULL, NULL, 'Esta mojado');
