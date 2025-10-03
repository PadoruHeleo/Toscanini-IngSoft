-- Ampliar los campos de la tabla AUDIT_LOG para permitir mensajes más largos
-- Los campos log_prev_v y log_new_v son muy pequeños para los mensajes de registro de salida

ALTER TABLE AUDIT_LOG 
MODIFY COLUMN log_prev_v VARCHAR(512),
MODIFY COLUMN log_new_v VARCHAR(512);

-- Nota: Se amplían a 512 caracteres para acomodar mensajes detallados de auditoría
-- como los registros de salida de equipos que incluyen:
-- - Número de serie del equipo
-- - Motivo de salida  
-- - Estados anterior y nuevo
-- - Observaciones adicionales