-- Insertar clientes reales de la base de datos
-- Nota: created_by se ajusta al orden de inserción (usuario_id 1 = Bastián Benítez, era 6 en respaldo)
INSERT IGNORE INTO CLIENTE (cliente_rut, cliente_nombre, cliente_correo, cliente_telefono, cliente_direccion, created_by, is_active) VALUES
('21197407-9', 'Bastian Benitez', 'benitez.basti0@gmail.com', NULL, NULL, 1, 1);
