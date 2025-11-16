-- Agregar campo deleted_at a COTIZACION para eliminación lógica
ALTER TABLE COTIZACION ADD COLUMN deleted_at TIMESTAMP NULL DEFAULT NULL;

-- Agregar campo deleted_at a INFORME para eliminación lógica
ALTER TABLE INFORME ADD COLUMN deleted_at TIMESTAMP NULL DEFAULT NULL;

