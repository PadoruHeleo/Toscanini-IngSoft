-- Insertar relaciones piezas-cotización de prueba
INSERT IGNORE INTO PIEZAS_COTIZACION (pieza_id, cotizacion_id, cantidad) VALUES
-- Cotización 1: Micrófono y limpieza
(1, 1, 1), -- Batería Ion-Litio
(3, 1, 1), -- Micrófono de solapa

-- Cotización 2: Reparación de potencia
(4, 2, 1), -- Cargador de batería
(5, 2, 1), -- Auricular con PTT
(8, 2, 1), -- Placa principal

-- Cotización 3: Calibración (mano de obra principalmente)
(6, 3, 1), -- Clip para cinturón

-- Cotización 4: Display y sellado
(9, 4, 1), -- Display LCD
(10, 4, 1), -- Teclado de membrana

-- Cotización 5: Mantenimiento (sin piezas mayores)
(1, 5, 1); -- Batería Ion-Litio (reemplazo preventivo)