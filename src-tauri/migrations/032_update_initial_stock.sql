-- Actualizar stock inicial para las piezas existentes
UPDATE PIEZA SET pieza_stock = CASE 
    WHEN pieza_id = 1 THEN 15  -- Batería Ion-Litio 7.4V
    WHEN pieza_id = 2 THEN 8   -- Antena Helicoidal VHF
    WHEN pieza_id = 3 THEN 12  -- Micrófono de Solapa
    WHEN pieza_id = 4 THEN 6   -- Cargador de Mesa
    WHEN pieza_id = 5 THEN 20  -- Correa de Transporte
    WHEN pieza_id = 6 THEN 10  -- Filtro de Audio
    WHEN pieza_id = 7 THEN 25  -- Conector SMA Macho
    WHEN pieza_id = 8 THEN 3   -- Placa Principal
    WHEN pieza_id = 9 THEN 7   -- Display LCD
    WHEN pieza_id = 10 THEN 18 -- Potenciómetro de Volumen
    WHEN pieza_id = 11 THEN 30 -- Cristal de Cuarzo
    WHEN pieza_id = 12 THEN 5  -- Transistor de RF
    WHEN pieza_id = 13 THEN 45 -- Capacitor Electrolítico
    WHEN pieza_id = 14 THEN 50 -- Fusible Cerámico 5A
    WHEN pieza_id = 15 THEN 100-- Cable Coaxial RG-58 (metros)
    WHEN pieza_id = 16 THEN 22 -- Conector PL-259
    WHEN pieza_id = 17 THEN 35 -- O-Ring de Sellado
    WHEN pieza_id = 18 THEN 200-- Tornillo Allen M3x8
    WHEN pieza_id = 19 THEN 12 -- Disipador de Calor
    WHEN pieza_id = 20 THEN 8  -- Ventilador 12V
    ELSE 0
END
WHERE pieza_id BETWEEN 1 AND 20;