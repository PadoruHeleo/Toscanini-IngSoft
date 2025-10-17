-- Insertar más datos de prueba para inventario de equipos (sin campos de estado y fechas)
INSERT IGNORE INTO INVENTARIO_EQUIPO (
    equipo_codigo, equipo_nombre, equipo_marca, equipo_modelo, equipo_tipo, 
    equipo_descripcion, equipo_precio, equipo_stock, equipo_ubicacion, 
    proveedor, numero_serie, observaciones
) VALUES
-- Radios Adicionales
('EQ-RAD-003', 'Radio Móvil Digital', 'Motorola', 'DM4600e', 'radio', 
 'Radio móvil digital DMR para vehículos', 520000, 8, 
 'Almacén Principal', 'Radiocomunicaciones SA', 'MOT-DM4600-003', 'Para instalación en vehículos de servicio'),

('EQ-RAD-004', 'Radio Portátil UHF', 'Kenwood', 'TK-3402U16P', 'radio', 
 'Radio portátil analógico UHF', 180000, 12, 
 'Almacén Principal', 'Radiocomunicaciones SA', 'KEN-TK3402-004', 'Radios básicos para personal'),

('EQ-RAD-005', 'Radio Base Dual Band', 'Yaesu', 'FT-8900R', 'radio', 
 'Radio base amateur dual band VHF/UHF', 420000, 3, 
 'Almacén Principal', 'Radioaficionados Chile', 'YAE-FT8900-005', 'Para comunicaciones de emergencia'),

('EQ-RAD-006', 'Radio Satelital', 'Iridium', '9575 Extreme', 'radio', 
 'Teléfono satelital resistente al agua', 1800000, 2, 
 'Caja Fuerte', 'Comunicaciones Globales', 'IRI-9575-006', 'Para emergencias sin cobertura celular'),

-- Antenas Adicionales
('EQ-ANT-003', 'Antena Discone', 'Diamond', 'D130J', 'antena', 
 'Antena discone wideband 25-1300 MHz', 180000, 4, 
 'Almacén Antenas', 'Antenas del Sur', 'DIA-D130J-003', 'Para monitoreo de espectro'),

('EQ-ANT-004', 'Antena Móvil Magnética', 'Larsen', 'NMO-150/450', 'antena', 
 'Antena dual band con base magnética', 85000, 15, 
 'Almacén Antenas', 'Antenas del Sur', 'LAR-NMO150-004', 'Para vehículos móviles'),

('EQ-ANT-005', 'Antena Panel Sectorial', 'Kathrein', '742215', 'antena', 
 'Antena panel 800-960 MHz ganancia 17dBi', 350000, 6, 
 'Almacén Antenas', 'Kathrein Chile', 'KAT-742215-005', 'Para sistemas celulares'),

('EQ-ANT-006', 'Antena Logarítmica', 'Rohde & Schwarz', 'HL050', 'antena', 
 'Antena log-periódica 500 MHz - 18 GHz', 1200000, 1, 
 'Laboratorio RF', 'R&S Chile', 'RS-HL050-006', 'Para mediciones EMC'),

-- Repetidores y Equipos de Red
('EQ-REP-002', 'Repetidor Analógico VHF', 'Vertex Standard', 'VXR-9000V', 'repetidor', 
 'Repetidor analógico VHF 50W', 890000, 2, 
 'Torre Secundaria', 'Vertex Chile', 'VTX-VXR9000-002', 'Backup del sistema principal'),

('EQ-REP-003', 'Duplexor VHF', 'Sinclair', 'SC-4304A', 'repetidor', 
 'Duplexor cavidad para VHF 6dB pérdida', 450000, 3, 
 'Torre Principal', 'RF Components', 'SIN-SC4304-003', 'Para separación Tx/Rx'),

('EQ-NET-001', 'Switch Ethernet Industrial', 'Cisco', 'IE-2000-4TS-G-B', 'accesorio', 
 'Switch industrial 4 puertos Gigabit', 320000, 5, 
 'Rack Comunicaciones', 'Cisco Chile', 'CIS-IE2000-001', 'Para red de datos del sitio'),

('EQ-NET-002', 'Router 4G/LTE', 'Cradlepoint', 'IBR200', 'accesorio', 
 'Router industrial con modem LTE integrado', 680000, 4, 
 'Rack Comunicaciones', 'Cradlepoint SA', 'CRA-IBR200-002', 'Para conectividad remota'),

-- Herramientas de Medición
('EQ-HER-003', 'Generador de RF', 'Keysight', 'E8257D', 'herramienta', 
 'Generador de señales analógicas hasta 20 GHz', 3500000, 1, 
 'Laboratorio RF', 'Keysight Chile', 'KEY-E8257D-003', 'Para calibración de equipos'),

('EQ-HER-004', 'Osciloscopio Digital', 'Tektronix', 'MSO54', 'herramienta', 
 'Osciloscopio de señales mixtas 500 MHz', 2800000, 1, 
 'Taller Técnico', 'Tektronix Chile', 'TEK-MSO54-004', 'Para análisis de señales digitales'),

('EQ-HER-005', 'Medidor de Campo', 'Rohde & Schwarz', 'FSH4', 'herramienta', 
 'Analizador de espectro portátil hasta 3.6 GHz', 1850000, 2, 
 'Vehículo Técnico', 'R&S Chile', 'RS-FSH4-005', 'Para mediciones en terreno'),

('EQ-HER-006', 'Soldador de Precisión', 'Weller', 'WE1010NA', 'herramienta', 
 'Estación de soldado digital 70W', 220000, 4, 
 'Taller Técnico', 'Weller Chile', 'WEL-WE1010-006', 'Para reparación componentes SMD'),

('EQ-HER-007', 'Crimpeadora Coaxial', 'Canare', 'TCD-50H', 'herramienta', 
 'Herramienta para conectores coaxiales', 95000, 6, 
 'Taller Técnico', 'Canare Japan', 'CAN-TCD50H-007', 'Para fabricación de cables RF'),

-- Fuentes de Poder y Accesorios
('EQ-ACC-003', 'UPS Industrial', 'APC', 'SURT3000XLIM', 'accesorio', 
 'UPS en línea 3000VA rack mount', 850000, 3, 
 'Sala de Equipos', 'APC Chile', 'APC-SURT3000-003', 'Para respaldo de energía crítica'),

('EQ-ACC-004', 'Banco de Baterías', 'Trojan', 'T-1275+', 'accesorio', 
 'Baterías de ciclo profundo 12V 150Ah', 180000, 20, 
 'Sala de Baterías', 'Baterías Industriales', 'TRO-T1275-004', 'Para sistema de respaldo'),

('EQ-ACC-005', 'Cargador Inteligente', 'Victron', 'Blue Smart 12/25', 'accesorio', 
 'Cargador de baterías con Bluetooth', 150000, 8, 
 'Sala de Baterías', 'Victron Chile', 'VIC-BLUE12-005', 'Para mantenimiento de baterías'),

('EQ-ACC-006', 'Panel Solar', 'Canadian Solar', 'CS3K-300MS', 'accesorio', 
 'Panel fotovoltaico monocristalino 300W', 120000, 12, 
 'Almacén Energía', 'Solar Chile', 'CAN-CS3K300-006', 'Para sistema solar backup'),

-- Cables y Conectores
('EQ-CAB-001', 'Cable Coaxial RG-213', 'Times Microwave', 'LMR-400', 'accesorio', 
 'Cable coaxial baja pérdida 50 ohm', 8500, 500, 
 'Almacén Cables', 'RF Components', 'TIM-LMR400-001', 'Por metro, para instalaciones RF'),

('EQ-CAB-002', 'Conectores N Macho', 'Amphenol', '82-5230', 'accesorio', 
 'Conector N macho para LMR-400', 12000, 100, 
 'Almacén Conectores', 'RF Components', 'AMP-825230-002', 'Para cables principales'),

('EQ-CAB-003', 'Conectores SMA Hembra', 'Huber+Suhner', '22 SMA-50-0-1', 'accesorio', 
 'Conector SMA hembra panel mount', 8500, 50, 
 'Almacén Conectores', 'Huber+Suhner', 'HS-22SMA-003', 'Para equipos portátiles'),

-- Equipos de Calibración
('EQ-CAL-001', 'Carga Fantasma 50W', 'Bird', '8860-50', 'herramienta', 
 'Carga fantasma 50 ohm 50W hasta 1 GHz', 180000, 4, 
 'Laboratorio RF', 'Bird Electronics', 'BIR-8860-001', 'Para pruebas de transmisores'),

('EQ-CAL-002', 'Kit Calibración OSL', 'Keysight', '85052D', 'herramienta', 
 'Kit de calibración 3.5mm hasta 26.5 GHz', 920000, 1, 
 'Laboratorio RF', 'Keysight Chile', 'KEY-85052D-002', 'Para calibración VNA'),

-- Herramientas Mecánicas
('EQ-MEC-001', 'Taladro Industrial', 'Hilti', 'TE 2-A22', 'herramienta', 
 'Taladro rotomartillo a batería SDS-plus', 280000, 3, 
 'Taller Mecánico', 'Hilti Chile', 'HIL-TE2A22-001', 'Para instalaciones en torre'),

('EQ-MEC-002', 'Escalera Telescópica', 'Little Giant', 'M26', 'herramienta', 
 'Escalera multiposiciones fibra de vidrio', 420000, 2, 
 'Bodega Herramientas', 'Safety Equipment', 'LG-M26-002', 'Para trabajos en altura'),

('EQ-MEC-003', 'Polipasto Eléctrico', 'CM', 'Lodestar L1', 'herramienta', 
 'Polipasto de cadena eléctrico 250kg', 1200000, 1, 
 'Torre Principal', 'CM Chile', 'CM-LODE1-003', 'Para izar equipos pesados'),

-- Equipos de Seguridad
('EQ-SEG-001', 'Detector de Gas', 'Honeywell', 'BW Ultra', 'herramienta', 
 'Detector multigases portátil', 380000, 4, 
 'Vehículo Seguridad', 'Honeywell Chile', 'HON-BWULTRA-001', 'Para trabajos en espacios confinados'),

('EQ-SEG-002', 'Arnés de Seguridad', '3M', 'DBI-SALA ExoFit', 'accesorio', 
 'Arnés de cuerpo completo torre', 95000, 10, 
 'Bodega Seguridad', '3M Chile', '3M-EXOFIT-002', 'Para trabajos en torre');