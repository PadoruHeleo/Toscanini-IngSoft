-- Insertar datos de prueba para salidas de equipos (coherentes con órdenes existentes)
INSERT IGNORE INTO SALIDA_EQUIPO 
(orden_trabajo_id, motivo_salida, fecha_salida, usuario_id, observaciones) VALUES

-- Salidas para órdenes que están en estado "entregado"
(1, 'entregado_cliente', '2024-10-10 14:30:00', 1, 'Reparación completada satisfactoriamente. Radio Motorola funcionando correctamente después de reemplazo de micrófono.'),
(3, 'entregado_cliente', '2024-10-12 09:15:00', 1, 'Calibración anual completada. Radio Icom calibrado según especificaciones técnicas. Equipo en óptimas condiciones.'),
(5, 'entregado_cliente', '2024-10-14 16:45:00', 2, 'Mantenimiento preventivo realizado. Limpieza completa y verificación de todos los componentes.');

-- Nota: Las órdenes 2 y 4 están en estado "cotizacion_enviada" por lo que no tienen salida registrada aún
-- Esto mantiene coherencia: solo las órdenes "entregado" tienen registro en SALIDA_EQUIPO