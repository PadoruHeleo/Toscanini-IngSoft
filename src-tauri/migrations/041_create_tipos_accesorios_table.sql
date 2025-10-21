-- Migration 041: crear tabla tipos_accesorios (sin es_comun ni descripcion)

CREATE TABLE IF NOT EXISTS tipos_accesorios (
  tipo_id INT PRIMARY KEY AUTO_INCREMENT,
  nombre VARCHAR(64) UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
