-- Insertar relaciones términos-informe de prueba
INSERT IGNORE INTO TERMINOS_INFORME (termino_id, informe_id, aplicado) VALUES
-- Informe 1: Términos por defecto + específicos
(1, 1, TRUE), -- Garantía Estándar
(2, 1, TRUE), -- Responsabilidad por Pérdida
(3, 1, TRUE), -- Condiciones de Pago

-- Informe 2: Términos por defecto
(1, 2, TRUE), -- Garantía Estándar
(2, 2, TRUE), -- Responsabilidad por Pérdida
(4, 2, TRUE), -- Diagnóstico Técnico

-- Informe 3: Solo términos básicos
(1, 3, TRUE), -- Garantía Estándar
(3, 3, TRUE), -- Condiciones de Pago

-- Informe 4: Términos completos
(1, 4, TRUE), -- Garantía Estándar
(2, 4, TRUE), -- Responsabilidad por Pérdida
(3, 4, TRUE), -- Condiciones de Pago

-- Informe 5: Mantenimiento preventivo
(1, 5, TRUE), -- Garantía Estándar
(2, 5, TRUE); -- Responsabilidad por Pérdida