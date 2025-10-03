-- Insertar términos y condiciones de ejemplo
INSERT INTO TERMINOS_CONDICIONES (termino_nombre, termino_descripcion, tipo_referencia, is_default) VALUES
('Garantía Estándar', 'Los trabajos de reparación tienen una garantía de 90 días a partir de la fecha de entrega del equipo.', 'ambos', TRUE),
('Responsabilidad por Pérdida', 'El cliente es responsable de cualquier pérdida o daño del equipo mientras esté en nuestras instalaciones.', 'ambos', TRUE),
('Condiciones de Pago', 'El pago debe realizarse antes de la entrega del equipo reparado. Se acepta efectivo, tarjeta o transferencia bancaria.', 'cotizacion', TRUE),
('Diagnóstico Técnico', 'El diagnóstico técnico incluye la identificación de fallas y evaluación del estado general del equipo.', 'informe', TRUE),
('Plazo de Retiro', 'El cliente tiene un plazo máximo de 30 días para retirar el equipo una vez notificada la finalización del trabajo.', 'ambos', FALSE),
('Piezas de Repuesto', 'Las piezas utilizadas en la reparación son originales o equivalentes de calidad garantizada.', 'informe', FALSE),
('Cotización Válida', 'Esta cotización tiene una validez de 15 días hábiles desde su fecha de emisión.', 'cotizacion', FALSE),
('Trabajo No Garantizado', 'Trabajos realizados en equipos con daños por líquidos o golpes externos no tienen garantía.', 'ambos', FALSE);