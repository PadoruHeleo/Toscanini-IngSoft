-- Insertar clientes de prueba
INSERT IGNORE INTO CLIENTE (cliente_rut, cliente_nombre, cliente_correo, cliente_telefono, cliente_direccion, created_by) VALUES
('16789123-4', 'Empresa Telecomunicaciones SA', 'contacto@telecom.cl', '+56987123456', 'Av. Principal 1234, Santiago', 1),
('17890234-5', 'Radio Comunicaciones Ltda', 'info@radiocom.cl', '+56976543210', 'Calle Secundaria 5678, Valparaíso', 1),
('18901345-6', 'Servicios de Radio FM', 'gerencia@radiofm.cl', '+56965432109', 'Av. Las Flores 9101, Concepción', 2),
('19012456-7', 'Antenas y Repetidores Chile', 'ventas@antenas.cl', '+56954321098', 'Pasaje Los Pinos 1121, La Serena', 1),
('20123567-8', 'Comunicaciones del Norte', 'admin@comnorte.cl', '+56943210987', 'Av. Central 3141, Antofagasta', 2),
('21234678-9', 'Radio Taxi Metropolitano', 'operaciones@radiotaxi.cl', '+56932109876', 'Calle Mayor 5161, Santiago', 3),
('22345789-0', 'Seguridad Integral SpA', 'contacto@seguridadint.cl', '+56921098765', 'Av. Seguridad 7181, Viña del Mar', 1),
('23456890-1', 'Minera Los Andes', 'comunicaciones@mineraandes.cl', '+56910987654', 'Ruta Minera Km 25, Copiapó', 2);