-- Script de inserción forzada de datos de prueba
-- Ejecutar manualmente si las migraciones no insertan datos automáticamente

-- Verificar si hay datos existentes
SELECT 'Verificando usuarios existentes...' as status;
SELECT COUNT(*) as usuarios_count FROM USUARIO;

-- Insertar datos de prueba solo si no existen (actualizado con nuevos formatos de correo)
INSERT IGNORE INTO USUARIO (usuario_rut, usuario_nombre, usuario_correo, usuario_contrasena, usuario_telefono, usuario_rol) 
SELECT * FROM (VALUES 
    ('12345678-9', 'Juan Pérez Admin', 'admin@toscanini.com', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56912345678', 'admin'),
    ('98765432-1', 'María González Técnico', 'tecnico@toscanini.com', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56987654321', 'tecnico'),
    ('11111111-1', 'Carlos Silva Recepción', 'recepcion@toscanini.com', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56911111111', 'recepcion'),
    ('22222222-2', 'Ana Rodríguez Técnico 2', 'tecnico2@toscanini.com', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56922222222', 'tecnico'),
    ('33333333-3', 'Pedro López Admin 2', 'admin2@toscanini.com', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56933333333', 'admin'),
    ('21197407-9', 'Bastián Benítez', 'benitez.basti0@gmail.com', '$2b$12$5Q4ZmiyKpJ9V4rAea1hy7.O1PiObC9q8Tod86Ud3BqfeuntSIckwa', '+56944444444', 'admin')
) AS new_users
WHERE NOT EXISTS (SELECT 1 FROM USUARIO WHERE usuario_rut IN ('12345678-9', '98765432-1', '11111111-1', '22222222-2', '33333333-3', '21197407-9'));

SELECT 'Usuarios después de inserción:' as status;
SELECT COUNT(*) as usuarios_count FROM USUARIO;