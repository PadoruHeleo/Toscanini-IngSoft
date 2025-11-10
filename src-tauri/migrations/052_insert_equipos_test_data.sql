-- Insertar equipos reales de la base de datos
-- Nota: Los IDs se ajustan al orden de inserción
-- Cliente ID 1 (era 9 en respaldo), Usuario ID 1 (era 6 en respaldo)
INSERT IGNORE INTO EQUIPO (numero_serie, equipo_marca, equipo_modelo, equipo_tipo, equipo_precio, equipo_ubicacion, cliente_id, created_by) VALUES
('38345384567', 'Motorola', 'M5', '', 0, 'Central', 1, 1);
