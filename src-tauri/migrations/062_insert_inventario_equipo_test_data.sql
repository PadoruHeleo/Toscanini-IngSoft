-- Insertar datos reales de inventario de equipos de la base de datos
INSERT IGNORE INTO INVENTARIO_EQUIPO (
    equipo_codigo, equipo_nombre, equipo_marca, equipo_modelo, equipo_tipo, 
    equipo_descripcion, equipo_precio, equipo_stock, equipo_estado, 
    equipo_ubicacion, proveedor, numero_serie, observaciones, created_by
) VALUES
('EQ-RAD-001', 'Radio Portátil Profesional', 'Motorola', 'DGP5550e', 'radio', 
 'Radio digital portátil con GPS integrado', 450000, 5, 'disponible', 
 'Almacén Principal', 'Radiocomunicaciones SA', 'MOT-DGP-001', 'Radio profesional con GPS', NULL),

('EQ-RAD-002', 'Radio Base VHF', 'Kenwood', 'TKM-D710E', 'radio', 
 'Radio base dual band para estación fija', 850000, 2, 'disponible', 
 'Almacén Principal', 'Radiocomunicaciones SA', 'KEN-TKM-002', 'Para estación base', NULL),

('EQ-ANT-001', 'Antena Yagi VHF', 'Diamond', 'X-510N', 'antena', 
 'Antena direccional para comunicaciones VHF', 120000, 8, 'disponible', 
 'Almacén Antenas', 'Antenas del Sur', 'DIA-X510-001', 'Antena direccional', NULL),

('EQ-ANT-002', 'Antena Omnidireccional UHF', 'Comet', 'GP-23', 'antena', 
 'Antena omnidireccional para base UHF', 95000, 6, 'disponible', 
 'Almacén Antenas', 'Antenas del Sur', 'COM-GP23-002', 'Para base UHF', NULL),

('EQ-REP-001', 'Repetidor Digital', 'Hytera', 'RD985', 'repetidor', 
 'Repetidor digital DMR con capacidad para 1000 usuarios', 2500000, 1, 'disponible', 
 'Torre Principal', 'Hytera Chile', 'HYT-RD985-001', 'Sistema principal', NULL),

('EQ-HER-001', 'Analizador de Espectro', 'Rigol', 'DSA815-TG', 'herramienta', 
 'Analizador de espectro hasta 1.5 GHz con generador de seguimiento', 1200000, 1, 'disponible', 
 'Taller Técnico', 'Instrumentos Técnicos', 'RIG-DSA815-001', 'Para mediciones RF', NULL),

('EQ-HER-002', 'Multímetro Digital', 'Fluke', '87V', 'herramienta', 
 'Multímetro industrial de alta precisión', 380000, 3, 'disponible', 
 'Taller Técnico', 'Instrumentos Técnicos', 'FLU-87V-002', 'Multímetro profesional', NULL),

('EQ-ACC-001', 'Fuente de Poder Regulada', 'Alinco', 'DM-330FXE', 'accesorio', 
 'Fuente conmutada 13.8V 30A con protecciones', 180000, 4, 'disponible', 
 'Almacén Accesorios', 'Radiocomunicaciones SA', 'ALI-DM330-001', 'Con protecciones', NULL),

('EQ-ACC-002', 'Cargador Múltiple', 'Motorola', 'WPLN4226', 'accesorio', 
 'Cargador de 6 posiciones para radios DGP', 250000, 2, 'disponible', 
 'Almacén Accesorios', 'Radiocomunicaciones SA', 'MOT-WPN-002', 'Para radios DGP', NULL),

('EQ-OTR-001', 'Medidor de ROE/Watímetro', 'MFJ', 'MFJ-815C', 'otro', 
 'Medidor de ROE y potencia para HF/VHF/UHF', 95000, 2, 'disponible', 
 'Taller Técnico', 'Instrumentos Técnicos', 'MFJ-815C-001', 'Para medición de ROE', NULL);
