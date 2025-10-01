-- Insertar registros de auditoría de prueba (simulando actividad del sistema)
INSERT IGNORE INTO AUDIT_LOG (log_accion, log_usuario_id, log_entidad_tabla, log_entidad_id, log_prev_v, log_new_v) VALUES
('CREATE_CLIENTE', 1, 'CLIENTE', 1, NULL, 'Empresa Telecomunicaciones SA'),
('CREATE_EQUIPO', 1, 'EQUIPO', 1, NULL, 'MOT001-2024'),
('CREATE_ORDEN_TRABAJO', 1, 'ORDEN_TRABAJO', 1, NULL, 'OT-001-2024'),
('UPDATE_ORDEN_TRABAJO', 2, 'ORDEN_TRABAJO', 1, 'recibido', 'en_reparacion'),
('CREATE_INFORME', 2, 'INFORME', 1, NULL, 'INF-001-2024'),
('CREATE_COTIZACION', 1, 'COTIZACION', 1, NULL, 'COT-001-2024'),
('UPDATE_ORDEN_TRABAJO', 1, 'ORDEN_TRABAJO', 1, 'en_reparacion', 'entregado'),
('CREATE_TERMINO_INFORME_RELATION', 2, 'TERMINOS_INFORME', 1, NULL, 'Término Garantía Estándar asociado al informe 1'),
('CREATE_TERMINO_COTIZACION_RELATION', 1, 'TERMINOS_COTIZACION', 1, NULL, 'Término Garantía Estándar asociado a la cotización 1'),
('UPDATE_COTIZACION', 1, 'COTIZACION', 1, 'NULL', 'TRUE'),
('CREATE_CLIENTE', 2, 'CLIENTE', 2, NULL, 'Radio Comunicaciones Ltda'),
('CREATE_EQUIPO', 2, 'EQUIPO', 2, NULL, 'KEN002-2024'),
('LOGIN', 1, 'USUARIO', 1, NULL, 'Inicio sesión exitoso'),
('LOGIN', 2, 'USUARIO', 2, NULL, 'Inicio sesión exitoso'),
('CREATE_PIEZA', 1, 'PIEZA', 1, NULL, 'Batería Ion-Litio 7.4V');