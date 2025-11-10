use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use printpdf::{PdfDocument, BuiltinFont, Color, Rgb, Mm, Line, Point};
use crate::pdf::common::{EmpresaInfo, ClienteInfo, EquipoInfo, PiezaPdf, TerminoPdf, wrap_text};

#[derive(Debug, Serialize, Deserialize)]
pub struct CotizacionPdfData {
    pub cotizacion_codigo: String,
    pub fecha: DateTime<Utc>,
    pub empresa: EmpresaInfo,
    pub cliente: ClienteInfo,
    pub equipo: EquipoInfo,
    pub informe_tecnico: String,
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub costo_total: i32,
    pub piezas: Vec<PiezaPdf>,
    pub is_aprobada: bool,
    pub orden_codigo: Option<String>,
    pub terminos_condiciones: Vec<TerminoPdf>,
}

pub struct CotizacionPdfGenerator;

impl CotizacionPdfGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generar PDF de cotización
    pub async fn generate_cotizacion_pdf(&self, data: CotizacionPdfData) -> Result<Vec<u8>, String> {
        self.generate_basic_pdf(&data).await
    }

    async fn generate_basic_pdf(&self, data: &CotizacionPdfData) -> Result<Vec<u8>, String> {
        let (doc, page1, layer1) = PdfDocument::new("Toscanini - Cotización", Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Configurar fuentes
        let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
            .map_err(|e| format!("Error cargando fuente bold: {}", e))?;
        let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica)
            .map_err(|e| format!("Error cargando fuente regular: {}", e))?;

        // Colores - estilo formal (solo negro y gris)
        let black_color = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
        let gray_color = Color::Rgb(Rgb::new(0.3, 0.3, 0.3, None));

        // === TÍTULO DEL DOCUMENTO (izquierda) ===
        current_layer.set_fill_color(black_color.clone());
        current_layer.use_text("COTIZACIÓN", 16.0, Mm(20.0), Mm(280.0), &font_bold);
        
        // === NÚMERO DE COTIZACIÓN (derecha, grande y destacado) ===
        current_layer.use_text(&data.cotizacion_codigo, 24.0, Mm(140.0), Mm(280.0), &font_bold);
        current_layer.use_text("Cotización", 9.0, Mm(140.0), Mm(270.0), &font_regular);
        
        // === FECHAS (derecha, debajo del número) ===
        current_layer.set_fill_color(gray_color.clone());
        current_layer.use_text(&format!("Fecha de Emisión: {}", data.fecha.format("%d/%m/%Y")), 9.0, Mm(140.0), Mm(260.0), &font_regular);
        current_layer.use_text(&format!("Documento Impreso: {}", chrono::Utc::now().format("%d/%m/%Y")), 9.0, Mm(140.0), Mm(253.0), &font_regular);
        
        // Estado de aprobación
        current_layer.set_fill_color(black_color.clone());
        if data.is_aprobada {
            current_layer.use_text("Estado: APROBADA", 9.0, Mm(140.0), Mm(246.0), &font_bold);
        } else {
            current_layer.use_text("Estado: PENDIENTE", 9.0, Mm(140.0), Mm(246.0), &font_regular);
        }

        // === INFORMACIÓN DEL CLIENTE ===
        let mut y_pos = 250.0;
        current_layer.set_fill_color(black_color.clone());
        current_layer.use_text("Cliente", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
        current_layer.use_text(&data.cliente.nombre, 10.0, Mm(70.0), Mm(y_pos), &font_regular);
        y_pos -= 6.0;
        
        if let Some(direccion) = &data.cliente.direccion {
            current_layer.use_text("Dirección", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
            current_layer.use_text(direccion, 10.0, Mm(70.0), Mm(y_pos), &font_regular);
            y_pos -= 6.0;
        }
        
        if let Some(email) = &data.cliente.email {
            current_layer.use_text("Atención Sr.", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
            current_layer.use_text(email, 10.0, Mm(70.0), Mm(y_pos), &font_regular);
            y_pos -= 6.0;
        }
        
        if let Some(telefono) = &data.cliente.telefono {
            current_layer.use_text("Teléfono", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
            current_layer.use_text(telefono, 10.0, Mm(70.0), Mm(y_pos), &font_regular);
            y_pos -= 6.0;
        }

        // === INFORMACIÓN DEL EQUIPO ===
        y_pos -= 10.0;
        current_layer.use_text("Marca", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
        if let Some(marca) = &data.equipo.marca {
            current_layer.use_text(marca, 10.0, Mm(70.0), Mm(y_pos), &font_regular);
        }
        y_pos -= 6.0;
        
        current_layer.use_text("Modelo", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
        if let Some(modelo) = &data.equipo.modelo {
            current_layer.use_text(modelo, 10.0, Mm(70.0), Mm(y_pos), &font_regular);
        }
        y_pos -= 6.0;
        
        current_layer.use_text("Serie", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
        if let Some(serie) = &data.equipo.numero_serie {
            current_layer.use_text(serie, 10.0, Mm(70.0), Mm(y_pos), &font_regular);
        }
        y_pos -= 6.0;
        
        current_layer.use_text("Tipo de Producto", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
        if let Some(tipo) = &data.equipo.tipo {
            current_layer.use_text(tipo, 10.0, Mm(70.0), Mm(y_pos), &font_regular);
        }
        y_pos -= 6.0;

        // === TRABAJO SOLICITADO ===
        y_pos -= 10.0;
        current_layer.use_text("Diagnóstico y Trabajo a Realizado", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
        y_pos -= 6.0;
        
        if !data.informe_tecnico.trim().is_empty() {
            let informe_lines = wrap_text(&data.informe_tecnico, 85);
            for line in informe_lines.iter().take(8) {
                current_layer.use_text(line, 10.0, Mm(25.0), Mm(y_pos), &font_regular);
                y_pos -= 6.0;
            }
        } else {
            current_layer.use_text("Reparación y revisión general", 10.0, Mm(25.0), Mm(y_pos), &font_regular);
            y_pos -= 6.0;
        }

        // === DETALLE DE COSTOS (TABLA) ===
        y_pos -= 8.0;
        current_layer.use_text("Detalle de Costos", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
        y_pos -= 12.0;
        
        // Definir posiciones de columnas de la tabla
        let col_desc_x = 20.0;
        let col_cant_x = 110.0;
        let col_precio_x = 135.0;
        let col_subtotal_x = 165.0;
        let table_width = 170.0;
        let row_height = 4.0;
        let table_top_y = y_pos + 4.0; // Posición superior de la tabla
        
        // Línea superior de la tabla (borde superior)
        let top_line = Line {
            points: vec![
                (Point::new(Mm(col_desc_x), Mm(table_top_y)), false),
                (Point::new(Mm(col_desc_x + table_width), Mm(table_top_y)), false)
            ],
            is_closed: false
        };
        current_layer.set_outline_thickness(0.5);
        current_layer.set_outline_color(black_color.clone());
        current_layer.add_line(top_line);
        
        // Dibujar encabezado de la tabla
        current_layer.set_fill_color(black_color.clone());
        current_layer.use_text("Descripción", 9.0, Mm(col_desc_x + 2.0), Mm(y_pos), &font_bold);
        current_layer.use_text("Cant.", 9.0, Mm(col_cant_x), Mm(y_pos), &font_bold);
        current_layer.use_text("Precio Unit.", 9.0, Mm(col_precio_x), Mm(y_pos), &font_bold);
        current_layer.use_text("Subtotal", 9.0, Mm(col_subtotal_x), Mm(y_pos), &font_bold);
        
        // Línea debajo del encabezado
        y_pos -= row_height;
        let header_line = Line {
            points: vec![
                (Point::new(Mm(col_desc_x), Mm(y_pos + 1.0)), false),
                (Point::new(Mm(col_desc_x + table_width), Mm(y_pos + 1.0)), false)
            ],
            is_closed: false
        };
        current_layer.set_outline_thickness(0.5);
        current_layer.set_outline_color(black_color.clone());
        current_layer.add_line(header_line);
        
        y_pos -= 3.0; // Espacio adicional después de la línea del encabezado
        
        // Línea vertical izquierda (se extenderá después)
        let header_top_y = table_top_y;
        
        // Agregar filas de datos
        current_layer.set_fill_color(black_color.clone());
        
        // Fila de costo de revisión
        if let Some(revision) = data.costo_revision {
            current_layer.use_text("Costo de Revisión", 9.0, Mm(col_desc_x + 2.0), Mm(y_pos), &font_regular);
            current_layer.use_text("1", 9.0, Mm(col_cant_x), Mm(y_pos), &font_regular);
            current_layer.use_text(&format!("${}", revision), 9.0, Mm(col_precio_x), Mm(y_pos), &font_regular);
            current_layer.use_text(&format!("${}", revision), 9.0, Mm(col_subtotal_x), Mm(y_pos), &font_regular);
            y_pos -= row_height;
            
            // Línea horizontal entre filas (con espacio adecuado)
            let h_line = Line {
                points: vec![
                    (Point::new(Mm(col_desc_x), Mm(y_pos + 1.0)), false),
                    (Point::new(Mm(col_desc_x + table_width), Mm(y_pos + 1.0)), false)
                ],
                is_closed: false
            };
            current_layer.add_line(h_line);
            y_pos -= 3.0; // Espacio adicional después de la línea
        }
        
        // Fila de costo de reparación
        if let Some(reparacion) = data.costo_reparacion {
            current_layer.use_text("Costo de Reparación", 9.0, Mm(col_desc_x + 2.0), Mm(y_pos), &font_regular);
            current_layer.use_text("1", 9.0, Mm(col_cant_x), Mm(y_pos), &font_regular);
            current_layer.use_text(&format!("${}", reparacion), 9.0, Mm(col_precio_x), Mm(y_pos), &font_regular);
            current_layer.use_text(&format!("${}", reparacion), 9.0, Mm(col_subtotal_x), Mm(y_pos), &font_regular);
            y_pos -= row_height;
            
            // Línea horizontal entre filas (con espacio adecuado)
            let h_line = Line {
                points: vec![
                    (Point::new(Mm(col_desc_x), Mm(y_pos + 1.0)), false),
                    (Point::new(Mm(col_desc_x + table_width), Mm(y_pos + 1.0)), false)
                ],
                is_closed: false
            };
            current_layer.add_line(h_line);
            y_pos -= 3.0; // Espacio adicional después de la línea
        }
        
        // Filas de piezas
        for pieza in &data.piezas {
            let desc_text = if let Some(marca) = &pieza.marca {
                format!("{} ({})", pieza.nombre, marca)
            } else {
                pieza.nombre.clone()
            };
            
            // Si la descripción es muy larga, truncarla
            let desc_display = if desc_text.len() > 30 {
                format!("{}...", &desc_text[..27])
            } else {
                desc_text
            };
            
            current_layer.use_text(&desc_display, 9.0, Mm(col_desc_x + 2.0), Mm(y_pos), &font_regular);
            current_layer.use_text(&format!("{}", pieza.cantidad), 9.0, Mm(col_cant_x), Mm(y_pos), &font_regular);
            current_layer.use_text(&format!("${}", pieza.precio_unitario), 9.0, Mm(col_precio_x), Mm(y_pos), &font_regular);
            current_layer.use_text(&format!("${}", pieza.subtotal), 9.0, Mm(col_subtotal_x), Mm(y_pos), &font_regular);
            y_pos -= row_height;
            
            // Línea horizontal entre filas (con espacio adecuado)
            let h_line = Line {
                points: vec![
                    (Point::new(Mm(col_desc_x), Mm(y_pos + 1.0)), false),
                    (Point::new(Mm(col_desc_x + table_width), Mm(y_pos + 1.0)), false)
                ],
                is_closed: false
            };
            current_layer.add_line(h_line);
            y_pos -= 3.0; // Espacio adicional después de la línea
        }
        
        // Fila de total (con línea más gruesa arriba, solo en las columnas de precio y subtotal)
        // Primero dibujar la línea más arriba para dar espacio adecuado
        y_pos -= 0.0; // Espacio adicional antes de la línea del TOTAL
        //let total_line = Line {
         //   points: vec![
        //        (Point::new(Mm(col_precio_x - 2.0), Mm(y_pos + 2.0)), false),
        //        (Point::new(Mm(col_desc_x + table_width), Mm(y_pos + 2.0)), false)
        //    ],
        //    is_closed: false
        //};
        current_layer.set_outline_thickness(1.0);
        //current_layer.add_line(total_line);
        current_layer.set_outline_thickness(0.5);
        
        // Ahora escribir el texto del TOTAL más abajo con espacio adecuado
        y_pos -= 0.0; // Espacio entre la línea y el texto del TOTAL
        current_layer.set_fill_color(black_color.clone());
        current_layer.use_text("TOTAL", 10.0, Mm(col_desc_x + 2.0), Mm(y_pos), &font_bold);
        // Las columnas Cant. y Precio Unit. quedan vacías en la fila de TOTAL
        current_layer.use_text(&format!("${}", data.costo_total), 10.0, Mm(col_subtotal_x), Mm(y_pos), &font_bold);
        
        // Línea inferior de la tabla
        y_pos -= row_height;
        let bottom_line = Line {
            points: vec![
                (Point::new(Mm(col_desc_x), Mm(y_pos + 1.0)), false),
                (Point::new(Mm(col_desc_x + table_width), Mm(y_pos + 1.0)), false)
            ],
            is_closed: false
        };
        current_layer.set_outline_thickness(0.5);
        current_layer.add_line(bottom_line);
        
        // Dibujar líneas verticales de la tabla (después de conocer la altura final)
        let table_bottom_y = y_pos + 1.0;
        
        // Línea vertical izquierda
        let v_line_left = Line {
            points: vec![
                (Point::new(Mm(col_desc_x), Mm(header_top_y)), false),
                (Point::new(Mm(col_desc_x), Mm(table_bottom_y)), false)
            ],
            is_closed: false
        };
        current_layer.add_line(v_line_left);
        
        // Línea vertical entre descripción y cantidad
        let v_line_cant = Line {
            points: vec![
                (Point::new(Mm(col_cant_x - 2.0), Mm(header_top_y)), false),
                (Point::new(Mm(col_cant_x - 2.0), Mm(table_bottom_y)), false)
            ],
            is_closed: false
        };
        current_layer.add_line(v_line_cant);
        
        // Línea vertical entre cantidad y precio
        let v_line_precio = Line {
            points: vec![
                (Point::new(Mm(col_precio_x - 2.0), Mm(header_top_y)), false),
                (Point::new(Mm(col_precio_x - 2.0), Mm(table_bottom_y)), false)
            ],
            is_closed: false
        };
        current_layer.add_line(v_line_precio);
        
        // Línea vertical entre precio y subtotal
        let v_line_subtotal = Line {
            points: vec![
                (Point::new(Mm(col_subtotal_x - 2.0), Mm(header_top_y)), false),
                (Point::new(Mm(col_subtotal_x - 2.0), Mm(table_bottom_y)), false)
            ],
            is_closed: false
        };
        current_layer.add_line(v_line_subtotal);
        
        // Línea vertical derecha
        let v_line_right = Line {
            points: vec![
                (Point::new(Mm(col_desc_x + table_width), Mm(header_top_y)), false),
                (Point::new(Mm(col_desc_x + table_width), Mm(table_bottom_y)), false)
            ],
            is_closed: false
        };
        current_layer.add_line(v_line_right);
        
        y_pos -= 5.0;

        // === NOTAS Y TÉRMINOS Y CONDICIONES ===
        let mut y_terminos = y_pos - 8.0;
        
        current_layer.set_fill_color(black_color.clone());
        current_layer.use_text("Términos y Condiciones", 11.0, Mm(20.0), Mm(y_terminos), &font_bold);
        
        // Renderizar cada término y condición en su propia línea
        y_terminos -= 8.0;
        
        // Variables para el manejo de páginas
        let mut current_page_id = page1;
        let mut current_layer_id = layer1;
        let mut current_y = y_terminos;
        
        for (i, termino) in data.terminos_condiciones.iter().enumerate() {
            let page_layer = doc.get_page(current_page_id).get_layer(current_layer_id);
            
            // Construir el texto del término completo
            let termino_texto = if !termino.descripcion.is_empty() {
                format!("{}. {} - {}", i + 1, termino.nombre, termino.descripcion)
            } else {
                format!("{}. {}", i + 1, termino.nombre)
            };
            
            // Si el término es muy largo, dividirlo en múltiples líneas pero manteniendo el término completo
            let termino_lines = wrap_text(&termino_texto, 120);
            
            for line in termino_lines.iter() {
                if current_y > 50.0 {
                    page_layer.set_fill_color(black_color.clone());
                    page_layer.use_text(line, 8.5, Mm(25.0), Mm(current_y), &font_regular);
                    current_y -= 4.5;
                } else {
                    // Crear nueva página para continuar con los términos
                    let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Términos - Página 2");
                    current_page_id = new_page;
                    current_layer_id = new_layer;
                    current_y = 280.0; // Empezar desde arriba de la nueva página
                    
                    let new_page_layer = doc.get_page(current_page_id).get_layer(current_layer_id);
                    
                    // Header en la nueva página
                    new_page_layer.set_fill_color(black_color.clone());
                    new_page_layer.use_text("TOSCANINI - Cotización", 11.0, Mm(20.0), Mm(current_y), &font_bold);
                    current_y -= 20.0;
                    
                    // Continuar renderizando
                    new_page_layer.set_fill_color(black_color.clone());
                    new_page_layer.use_text(line, 8.5, Mm(25.0), Mm(current_y), &font_regular);
                    current_y -= 4.5;
                }
            }
        }

        // Generar PDF
        doc.save_to_bytes()
            .map_err(|e| format!("Error generando PDF: {}", e))
    }
}

