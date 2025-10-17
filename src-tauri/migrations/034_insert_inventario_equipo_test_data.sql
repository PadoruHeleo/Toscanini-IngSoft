-- Insertar datos de prueba para inventario de equipos
INSERT IGNORE INTO INVENTARIO_EQUIPO (
    equipo_codigo, equipo_nombre, equipo_marca, equipo_modelo, equipo_tipo, 
    equipo_descripcion, equipo_precio, equipo_stock, equipo_estado, 
    equipo_ubicacion, fecha_adquisicion, proveedor, numero_serie, 
    garantia_vencimiento, created_by
) VALUES
('EQ-RAD-001', 'Radio Portátil Profesional', 'Motorola', 'DGP5550e', 'radio', 
 'Radio digital portátil con GPS integrado', 450000, 5, 'disponible', 
 'Almacén Principal', '2024-01-15', 'Radiocomunicaciones SA', 'MOT-DGP-001', '2026-01-15', 1),

('EQ-RAD-002', 'Radio Base VHF', 'Kenwood', 'TKM-D710E', 'radio', 
 'Radio base dual band para estación fija', 850000, 2, 'disponible', 
 'Almacén Principal', '2024-02-20', 'Radiocomunicaciones SA', 'KEN-TKM-002', '2026-02-20', 1),

('EQ-ANT-001', 'Antena Yagi VHF', 'Diamond', 'X-510N', 'antena', 
 'Antena direccional para comunicaciones VHF', 120000, 8, 'disponible', 
 'Almacén Antenas', '2024-01-10', 'Antenas del Sur', 'DIA-X510-001', '2026-01-10', 1),

('EQ-ANT-002', 'Antena Omnidireccional UHF', 'Comet', 'GP-23', 'antena', 
 'Antena omnidireccional para base UHF', 95000, 6, 'disponible', 
 'Almacén Antenas', '2024-03-05', 'Antenas del Sur', 'COM-GP23-002', '2026-03-05', 1),

('EQ-REP-001', 'Repetidor Digital', 'Hytera', 'RD985', 'repetidor', 
 'Repetidor digital DMR con capacidad para 1000 usuarios', 2500000, 1, 'en_uso', 
 'Torre Principal', '2023-12-01', 'Hytera Chile', 'HYT-RD985-001', '2025-12-01', 1),

('EQ-HER-001', 'Analizador de Espectro', 'Rigol', 'DSA815-TG', 'herramienta', 
 'Analizador de espectro hasta 1.5 GHz con generador de seguimiento', 1200000, 1, 'disponible', 
 'Taller Técnico', '2024-04-12', 'Instrumentos Técnicos', 'RIG-DSA815-001', '2026-04-12', 1),

('EQ-HER-002', 'Multímetro Digital', 'Fluke', '87V', 'herramienta', 
 'Multímetro industrial de alta precisión', 380000, 3, 'disponible', 
 'Taller Técnico', '2024-01-08', 'Instrumentos Técnicos', 'FLU-87V-002', '2026-01-08', 1),

('EQ-ACC-001', 'Fuente de Poder Regulada', 'Alinco', 'DM-330FXE', 'accesorio', 
 'Fuente conmutada 13.8V 30A con protecciones', 180000, 4, 'disponible', 
 'Almacén Accesorios', '2024-02-15', 'Radiocomunicaciones SA', 'ALI-DM330-001', '2026-02-15', 1),

('EQ-ACC-002', 'Cargador Múltiple', 'Motorola', 'WPLN4226', 'accesorio', 
 'Cargador de 6 posiciones para radios DGP', 250000, 2, 'disponible', 
 'Almacén Accesorios', '2024-01-20', 'Radiocomunicaciones SA', 'MOT-WPN-002', '2026-01-20', 1),

('EQ-OTR-001', 'Medidor de ROE/Watímetro', 'MFJ', 'MFJ-815C', 'otro', 
 'Medidor de ROE y potencia para HF/VHF/UHF', 95000, 2, 'disponible', 
 'Taller Técnico', '2024-03-10', 'Instrumentos Técnicos', 'MFJ-815C-001', '2026-03-10', 1);