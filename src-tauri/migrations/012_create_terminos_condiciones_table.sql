-- Crear tabla para términos y condiciones
CREATE TABLE IF NOT EXISTS TERMINOS_CONDICIONES (
    termino_id INT PRIMARY KEY AUTO_INCREMENT,
    termino_nombre VARCHAR(128) NOT NULL,
    termino_descripcion TEXT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    tipo_referencia ENUM('informe', 'cotizacion', 'ambos') NOT NULL,
    is_default BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_tipo_referencia (tipo_referencia),
    INDEX idx_is_active (is_active),
    INDEX idx_is_default (is_default)
);

