-- Migración para corregir datos de inventario de equipos
-- (La migración 034 fue modificada, esta corrección actualiza los datos existentes)

-- Primero eliminamos los datos antiguos que pueden tener formato incorrecto
DELETE FROM INVENTARIO_EQUIPO WHERE equipo_codigo LIKE 'EQ-%';

-- Insertar datos corregidos
INSERT IGNORE INTO INVENTARIO_EQUIPO (
    equipo_codigo, equipo_nombre, equipo_marca, equipo_modelo, equipo_tipo, 
    equipo_descripcion, equipo_precio, equipo_stock, 
    equipo_ubicacion, proveedor, numero_serie, observaciones
) VALUES
('EQ-RAD-001', 'Radio Portátil Profesional', 'Motorola', 'DGP5550e', 'radio', 
 'Radio digital portátil con GPS integrado', 450000, 5, 
 'Almacén Principal', 'Radiocomunicaciones SA', 'MOT-DGP-001', 'Radio profesional con GPS'),

('EQ-RAD-002', 'Radio Base VHF', 'Kenwood', 'TKM-D710E', 'radio', 
 'Radio base dual band para estación fija', 850000, 2, 
 'Almacén Principal', 'Radiocomunicaciones SA', 'KEN-TKM-002', 'Para estación base'),

('EQ-ANT-001', 'Antena Yagi VHF', 'Diamond', 'X-510N', 'antena', 
 'Antena direccional para comunicaciones VHF', 120000, 8, 
 'Almacén Antenas', 'Antenas del Sur', 'DIA-X510-001', 'Antena direccional'),

('EQ-ANT-002', 'Antena Omnidireccional UHF', 'Comet', 'GP-23', 'antena', 
 'Antena omnidireccional para base UHF', 95000, 6, 
 'Almacén Antenas', 'Antenas del Sur', 'COM-GP23-002', 'Para base UHF'),

('EQ-REP-001', 'Repetidor Digital', 'Hytera', 'RD985', 'repetidor', 
 'Repetidor digital DMR con capacidad para 1000 usuarios', 2500000, 1, 
 'Torre Principal', 'Hytera Chile', 'HYT-RD985-001', 'Sistema principal'),

('EQ-HER-001', 'Analizador de Espectro', 'Rigol', 'DSA815-TG', 'herramienta', 
 'Analizador de espectro hasta 1.5 GHz con generador de seguimiento', 1200000, 1, 
 'Taller Técnico', 'Instrumentos Técnicos', 'RIG-DSA815-001', 'Para mediciones RF'),

('EQ-HER-002', 'Multímetro Digital', 'Fluke', '87V', 'herramienta', 
 'Multímetro industrial de alta precisión', 380000, 3, 
 'Taller Técnico', 'Instrumentos Técnicos', 'FLU-87V-002', 'Multímetro profesional'),

('EQ-ACC-001', 'Fuente de Poder Regulada', 'Alinco', 'DM-330FXE', 'accesorio', 
 'Fuente conmutada 13.8V 30A con protecciones', 180000, 4, 
 'Almacén Accesorios', 'Radiocomunicaciones SA', 'ALI-DM330-001', 'Con protecciones'),

('EQ-ACC-002', 'Cargador Múltiple', 'Motorola', 'WPLN4226', 'accesorio', 
 'Cargador de 6 posiciones para radios DGP', 250000, 2, 
 'Almacén Accesorios', 'Radiocomunicaciones SA', 'MOT-WPN-002', 'Para radios DGP'),

('EQ-OTR-001', 'Medidor de ROE/Watímetro', 'MFJ', 'MFJ-815C', 'otro', 
 'Medidor de ROE y potencia para HF/VHF/UHF', 95000, 2, 
 'Taller Técnico', 'Instrumentos Técnicos', 'MFJ-815C-001', 'Para medición de ROE');