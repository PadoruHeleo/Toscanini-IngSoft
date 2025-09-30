-- Insertar cotizaciones de prueba
INSERT IGNORE INTO COTIZACION (cotizacion_codigo, costo_revision, costo_reparacion, costo_total, is_aprobada, informe, created_by) VALUES
('COT-001-2024', 25000, 95000, 120000, TRUE, 'Reparación de radio Motorola DGP8050e - Reemplazo de micrófono y limpieza general. Incluye mano de obra y garantía de 90 días.', 1),
('COT-002-2024', 30000, 180000, 210000, FALSE, 'Reparación de radio Kenwood TK-3301 - Reemplazo de transistor de RF y componentes dañados. Reparación completa de etapa de potencia.', 2),
('COT-003-2024', 20000, 45000, 65000, TRUE, 'Calibración y ajuste de radio Icom IC-F3400D - Ajuste de frecuencias y calibración de potencia. Servicio especializado.', 1),
('COT-004-2024', 25000, 125000, 150000, NULL, 'Reparación de display y sellado de radio Yaesu FT-7250DR - Reemplazo de pantalla LCD y mejora del sistema de sellado.', 3),
('COT-005-2024', 15000, 35000, 50000, TRUE, 'Mantenimiento preventivo para radio Motorola XPR7550e - Limpieza, actualización de firmware y verificación completa.', 2),
('COT-006-2024', 35000, 2800000, 2835000, FALSE, 'Reparación mayor de repetidor Hytera RD985 - Reemplazo de placa principal y componentes críticos. Incluye reprogramación completa.', 1),
('COT-007-2024', 20000, 75000, 95000, NULL, 'Reparación de antena Diamond X-510N - Reemplazo de elementos radiantes y ajuste de ROE. Incluye pruebas de campo.', 2);