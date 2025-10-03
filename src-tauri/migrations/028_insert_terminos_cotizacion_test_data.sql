-- Insertar relaciones términos-cotización de prueba
INSERT IGNORE INTO TERMINOS_COTIZACION (termino_id, cotizacion_id, aplicado) VALUES
-- Cotización 1: Términos estándar
(1, 1, TRUE), -- Garantía Estándar
(2, 1, TRUE), -- Responsabilidad por Pérdida
(3, 1, TRUE), -- Condiciones de Pago

-- Cotización 2: Reparación compleja
(1, 2, TRUE), -- Garantía Estándar
(2, 2, TRUE), -- Responsabilidad por Pérdida
(3, 2, TRUE), -- Condiciones de Pago

-- Cotización 3: Servicio simple
(1, 3, TRUE), -- Garantía Estándar
(3, 3, TRUE), -- Condiciones de Pago

-- Cotización 4: Reparación con garantía especial
(1, 4, TRUE), -- Garantía Estándar
(2, 4, TRUE), -- Responsabilidad por Pérdida
(3, 4, TRUE), -- Condiciones de Pago

-- Cotización 5: Mantenimiento
(1, 5, TRUE), -- Garantía Estándar
(3, 5, TRUE); -- Condiciones de Pago