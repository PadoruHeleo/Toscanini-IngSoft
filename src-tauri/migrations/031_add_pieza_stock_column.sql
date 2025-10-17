-- Agregar columna de stock/cantidad a la tabla PIEZA
ALTER TABLE PIEZA ADD COLUMN IF NOT EXISTS pieza_stock INT DEFAULT 0 AFTER pieza_precio;