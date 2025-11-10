-- Insertar términos y condiciones reales de la base de datos
INSERT IGNORE INTO TERMINOS_CONDICIONES (termino_nombre, termino_descripcion, tipo_referencia, is_default, is_active) VALUES
('Garantía Estándar', 'Los trabajos de reparación tienen una garantía de 90 días a partir de la fecha de entrega del equipo.', 'ambos', TRUE, TRUE),
('Responsabilidad por Pérdida', 'El cliente es responsable de cualquier pérdida o daño del equipo mientras esté en nuestras instalaciones.', 'ambos', TRUE, TRUE),
('Condiciones de Pago', 'El pago debe realizarse antes de la entrega del equipo reparado. Se acepta efectivo, tarjeta o transferencia bancaria.', 'ambos', TRUE, TRUE),
('Diagnóstico Técnico', 'El diagnóstico técnico incluye la identificación de fallas y evaluación del estado general del equipo.', 'informe', TRUE, TRUE),
('Plazo de Retiro', 'El cliente tiene un plazo máximo de 30 días para retirar el equipo una vez notificada la finalización del trabajo.', 'ambos', FALSE, TRUE),
('Piezas de Repuesto', 'Las piezas utilizadas en la reparación son originales o equivalentes de calidad garantizada.', 'informe', FALSE, TRUE),
('Cotización Válida', 'Esta cotización tiene una validez de 15 días hábiles desde su fecha de emisión.', 'cotizacion', FALSE, TRUE),
('Trabajo No Garantizado', 'Trabajos realizados en equipos con daños por líquidos o golpes externos no tienen garantía.', 'ambos', FALSE, TRUE),
('Entrega', 'Entrega a tiempo', 'informe', FALSE, TRUE)
