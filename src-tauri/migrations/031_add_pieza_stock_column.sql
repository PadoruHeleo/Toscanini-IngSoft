-- Agregar columna de stock/cantidad a la tabla PIEZA
ALTER TABLE PIEZA ADD COLUMN pieza_stock INT DEFAULT 0 AFTER pieza_precio;