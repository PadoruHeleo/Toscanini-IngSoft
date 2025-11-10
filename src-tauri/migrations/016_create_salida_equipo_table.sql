-- Crear tabla para registrar salidas de equipos
-- Valores válidos para motivo_salida: 'entregado_cliente', 'retirado_sin_reparacion', 'abandonado', 'baja_definitiva'
CREATE TABLE IF NOT EXISTS SALIDA_EQUIPO (
    salida_id INT AUTO_INCREMENT PRIMARY KEY,
    orden_trabajo_id INT NOT NULL,
    motivo_salida VARCHAR(50) NOT NULL,
    fecha_salida DATETIME DEFAULT CURRENT_TIMESTAMP,
    usuario_id INT,
    observaciones TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (orden_trabajo_id) REFERENCES ORDEN_TRABAJO(orden_id),
    FOREIGN KEY (usuario_id) REFERENCES USUARIO(usuario_id)
);

-- Índices para optimizar consultas
CREATE INDEX IF NOT EXISTS idx_salida_equipo_orden ON SALIDA_EQUIPO(orden_trabajo_id);
CREATE INDEX IF NOT EXISTS idx_salida_equipo_fecha ON SALIDA_EQUIPO(fecha_salida);
CREATE INDEX IF NOT EXISTS idx_salida_equipo_motivo ON SALIDA_EQUIPO(motivo_salida);
CREATE INDEX IF NOT EXISTS idx_salida_equipo_usuario ON SALIDA_EQUIPO(usuario_id);

