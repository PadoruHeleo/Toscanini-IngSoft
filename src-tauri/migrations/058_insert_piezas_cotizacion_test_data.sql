-- Insertar relaciones piezas-cotización reales de la base de datos
-- Nota: Los IDs se ajustan al orden de inserción
-- En el respaldo: cotizacion_id 55, 56, 57 → En migración: 1, 2, 3
-- Pieza ID 9=Display LCD, 13=Capacitor, 15=Cable Coaxial
INSERT IGNORE INTO PIEZAS_COTIZACION (pieza_id, cotizacion_id, cantidad) VALUES
(9, 1, 1),  -- Display LCD para COT-2025-001 (era cotizacion_id 55)
(13, 3, 1), -- Capacitor Electrolítico para COT-2025-003 (era cotizacion_id 57)
(15, 3, 1); -- Cable Coaxial RG-58 para COT-2025-003 (era cotizacion_id 57)
