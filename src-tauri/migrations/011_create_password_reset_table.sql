-- Crear tabla para códigos de recuperación de contraseña
CREATE TABLE IF NOT EXISTS PASSWORD_RESET (
    reset_id INT AUTO_INCREMENT PRIMARY KEY,
    usuario_id INT NOT NULL,
    reset_code VARCHAR(6) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL,
    used BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (usuario_id) REFERENCES USUARIO(usuario_id) ON DELETE CASCADE,
    INDEX idx_reset_code (reset_code),
    INDEX idx_usuario_id (usuario_id),
    INDEX idx_expires_at (expires_at)
);
