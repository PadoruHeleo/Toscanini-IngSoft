-- Insertar equipos de prueba
INSERT IGNORE INTO EQUIPO (numero_serie, equipo_marca, equipo_modelo, equipo_tipo, equipo_precio, equipo_ubicacion, cliente_id, created_by) VALUES
('MOT001-2024', 'Motorola', 'DGP8050e', 'radio', 450000, 'Central de Operaciones', 1, 1),
('KEN002-2024', 'Kenwood', 'TK-3301', 'radio', 280000, 'Vehículo Patrulla 01', 2, 2),
('ICO003-2024', 'Icom', 'IC-F3400D', 'radio', 320000, 'Base Principal', 3, 1),
('YAE004-2024', 'Yaesu', 'FT-7250DR', 'radio', 380000, 'Torre de Comunicaciones', 4, 2),
('ANT005-2024', 'Diamond', 'X-510N', 'antena', 150000, 'Techo Edificio A', 1, 1),
('REP006-2024', 'Hytera', 'RD985', 'repetidor', 2500000, 'Cerro Las Antenas', 5, 3),
('MOT007-2024', 'Motorola', 'XPR7550e', 'radio', 520000, 'Seguridad Sector Norte', 6, 2),
('KEN008-2024', 'Kenwood', 'NX-200G', 'radio', 290000, 'Patrulla Móvil 03', 7, 1),
('ICO009-2024', 'Icom', 'IC-FR6000', 'repetidor', 1800000, 'Repetidora Zona Sur', 8, 2),
('YAE010-2024', 'Yaesu', 'FTM-300DR', 'radio', 420000, 'Vehículo de Emergencia', 2, 3),
('ANT011-2024', 'Comet', 'GP-3', 'antena', 120000, 'Mástil Principal', 3, 1),
('MOT012-2024', 'Motorola', 'DP4400e', 'radio', 380000, 'Oficina Central', 4, 2),
('HYT013-2024', 'Hytera', 'PD785G', 'radio', 350000, 'Seguridad Perimetral', 6, 1),
('KEN014-2024', 'Kenwood', 'TK-8360H', 'radio', 310000, 'Base Operaciones', 1, 3),
('ICO015-2024', 'Icom', 'IC-A220', 'radio', 450000, 'Torre de Control', 5, 2);