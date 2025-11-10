-- Crear tabla de piezas con campo de stock integrado
CREATE TABLE IF NOT EXISTS PIEZA (
    pieza_id INT PRIMARY KEY AUTO_INCREMENT,
    pieza_nombre VARCHAR(30),
    pieza_marca VARCHAR(30),
    pieza_desc VARCHAR(256),
    pieza_precio INT,
    -- Campo de stock (integrado desde migración 031)
    pieza_stock INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
