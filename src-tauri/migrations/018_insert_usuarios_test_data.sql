-- Insertar usuarios de prueba
INSERT IGNORE INTO USUARIO (usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol) VALUES
('12345678-9', 'Juan Pérez', 'juan.perez@toscanini.cl', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56912345678', 'admin'),
('98765432-1', 'María González', 'maria.gonzalez@toscanini.cl', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56987654321', 'tecnico'),
('11111111-1', 'Carlos Silva', 'carlos.silva@toscanini.cl', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56911111111', 'recepcion'),
('22222222-2', 'Ana Rodríguez', 'ana.rodriguez@toscanini.cl', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56922222222', 'tecnico'),
('33333333-3', 'Pedro López', 'pedro.lopez@toscanini.cl', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56933333333', 'admin');