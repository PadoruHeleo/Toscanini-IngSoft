-- Insertar informes reales de la base de datos
-- Nota: Los IDs se ajustan al orden de inserción
-- Usuario ID 1 (era 6 en respaldo)
INSERT IGNORE INTO INFORME (informe_codigo, informe_acciones, informe_obs, is_borrador, created_by, diagnostico, recomendaciones, solucion_aplicada, tecnico_responsable) VALUES
('INF-2025-001', 'Se necesita remplazar la panalla', 'evitar golpear el equipo', 0, 1, 'Se necesita remplazar la panalla', 'evitar golpear el equipo', 'Se cambio la pantalla', 'Bastián Benítez');
