CREATE TABLE IF NOT EXISTS PIEZA (
    pieza_id INT PRIMARY KEY AUTO_INCREMENT,
    pieza_nombre VARCHAR(30),
    pieza_marca VARCHAR(30),
    pieza_desc VARCHAR(256),
    pieza_precio INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);