-- Insertar relaciones piezas-informe de prueba
INSERT IGNORE INTO PIEZAS_INFORME (pieza_id, informe_id, cantidad) VALUES
-- Informe 1: Reemplazo de micrófono
(1, 1, 1), -- Batería Ion-Litio
(2, 1, 1), -- Antena helicoidal

-- Informe 2: Reparación de potencia
(3, 2, 1), -- Micrófono de solapa  
(4, 2, 1), -- Cargador de batería
(5, 2, 1), -- Auricular con PTT

-- Informe 3: Calibración (sin piezas de repuesto)

-- Informe 4: Reparación de display
(6, 4, 1), -- Clip para cinturón
(7, 4, 2), -- Conector SMA
(8, 4, 1); -- Placa principal

-- Informe 5: Mantenimiento preventivo (sin piezas de repuesto)