-- Actualizar registros existentes eliminando referencia a campo estado y fechas
-- (Los registros se mantienen pero se ignoran los campos obsoletos)

-- Esta migración es informativa ya que los campos estado y fechas
-- siguen existiendo en la tabla pero no se usan en la aplicación

-- Opcional: Si se desea limpiar la base de datos completamente,
-- se puede ejecutar:
-- ALTER TABLE INVENTARIO_EQUIPO DROP COLUMN equipo_estado;
-- ALTER TABLE INVENTARIO_EQUIPO DROP COLUMN fecha_adquisicion;
-- ALTER TABLE INVENTARIO_EQUIPO DROP COLUMN garantia_vencimiento;

-- Pero por compatibilidad, mantenemos las columnas