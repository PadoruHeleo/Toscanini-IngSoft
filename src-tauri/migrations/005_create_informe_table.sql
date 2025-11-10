-- Crear tabla de informes con campos adicionales integrados
CREATE TABLE IF NOT EXISTS INFORME (
    informe_id INT PRIMARY KEY AUTO_INCREMENT,
    informe_codigo VARCHAR(12) UNIQUE,
    informe_acciones TEXT,
    informe_obs TINYTEXT,
    is_borrador BOOLEAN DEFAULT FALSE,
    created_by INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Campos adicionales (integrados desde migración 013)
    diagnostico TEXT,
    recomendaciones TEXT,
    solucion_aplicada TEXT,
    tecnico_responsable VARCHAR(255),
    FOREIGN KEY (created_by) REFERENCES USUARIO(usuario_id)
);
