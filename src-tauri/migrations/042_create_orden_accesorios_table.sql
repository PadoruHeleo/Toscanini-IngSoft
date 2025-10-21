-- Migration 042: crear tabla orden_accesorios como vínculo entre orden y accesorio

CREATE TABLE IF NOT EXISTS orden_accesorios (
  id INT PRIMARY KEY AUTO_INCREMENT,
  orden_id INT NOT NULL,
  tipo_accesorio_id INT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (orden_id) REFERENCES ORDEN_TRABAJO(orden_id) ON DELETE CASCADE,
  FOREIGN KEY (tipo_accesorio_id) REFERENCES tipos_accesorios(tipo_id) ON DELETE CASCADE
);
