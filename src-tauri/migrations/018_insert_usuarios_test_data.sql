-- Insertar usuarios de prueba con formato de correos basado en roles
-- NOTA: Todos los usuarios usan la misma contraseña hasheada con bcrypt
-- Hash corresponde a una contraseña común para desarrollo/testing
INSERT IGNORE INTO USUARIO (usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol) VALUES
-- Usuarios con formato rol@toscanini.com
('12345678-9', 'Juan Pérez Admin', 'admin@toscanini.com', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56912345678', 'admin'),
('98765432-1', 'María González Técnico', 'tecnico@toscanini.com', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56987654321', 'tecnico'),
('11111111-1', 'Carlos Silva Recepción', 'recepcion@toscanini.com', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56911111111', 'recepcion'),
('22222222-2', 'Ana Rodríguez Técnico 2', 'tecnico2@toscanini.com', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56922222222', 'tecnico'),
('33333333-3', 'Pedro López Admin 2', 'admin2@toscanini.com', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56933333333', 'admin'),

-- Usuario administrador específico
('21197407-9', 'Bastián Benítez', 'benitez.basti0@gmail.com', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56944444444', 'admin');