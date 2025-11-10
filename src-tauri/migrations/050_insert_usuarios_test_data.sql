-- Insertar usuarios reales de la base de datos
INSERT IGNORE INTO USUARIO (usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol, is_active) VALUES
('21197407-9', 'Bastián Benítez', 'benitez.basti0@gmail.com', '$2b$12$MI6ome54Uh7knM6ZI7XCj.4R4sP6nZY.QNmSltXAtoFWt8XhyIOty', '+56944444444', 'admin', 1),
('21.119.740-7', 'Jose', 'onlyalight@gmail.com', '$2b$12$jCelxbbbiJ4Yz8CblnhXWu5.Jk0nvyzYEJgIDOIRSOCPMstTLJhq6', NULL, 'tecnico', 1);
