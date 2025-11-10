use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use printpdf::{PdfDocument, BuiltinFont, Color, Rgb, Mm, Line, Point};
use crate::pdf::common::{EmpresaInfo, ClienteInfo, EquipoInfo, PiezaPdf, TerminoPdf, wrap_text, compile_terminos_text};

#[derive(Debug, Serialize, Deserialize)]
pub struct InformePdfData {
    pub informe_codigo: String,
    pub fecha: DateTime<Utc>,
    pub empresa: EmpresaInfo,
    pub cliente: ClienteInfo,
    pub equipo: EquipoInfo,
    pub diagnostico: String,
    pub recomendaciones: Option<String>,
    pub solucion_aplicada: Option<String>,
    pub tecnico_responsable: String,
    pub piezas: Vec<PiezaPdf>,
    pub orden_codigo: Option<String>,
    pub tiene_garantia: bool,
    pub terminos_condiciones: Vec<TerminoPdf>,
}

pub struct InformePdfGenerator;

impl InformePdfGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generar PDF de informe
    pub async fn generate_informe_pdf(&self, data: InformePdfData) -> Result<Vec<u8>, String> {
        self.generate_basic_informe_pdf(&data).await
    }

    /// Generar PDF básico de informe usando printpdf
    async fn generate_basic_informe_pdf(&self, data: &InformePdfData) -> Result<Vec<u8>, String> {
        let (doc, page1, layer1) = PdfDocument::new("Toscanini - Informe", Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Configurar fuentes
        let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
            .map_err(|e| format!("Error cargando fuente bold: {}", e))?;
        let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica)
            .map_err(|e| format!("Error cargando fuente regular: {}", e))?;

        // Colores
        let blue_color = Color::Rgb(Rgb::new(0.2, 0.4, 0.8, None));
        let green_color = Color::Rgb(Rgb::new(0.2, 0.7, 0.3, None));
        let gray_color = Color::Rgb(Rgb::new(0.5, 0.5, 0.5, None));
        let black_color = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));

        // === HEADER CORPORATIVO ===
        current_layer.set_fill_color(blue_color.clone());
        
        // Línea superior azul
        let header_line = Line {
            points: vec![
                (Point::new(Mm(20.0), Mm(280.0)), false),
                (Point::new(Mm(190.0), Mm(280.0)), false)
            ],
            is_closed: false
        };
        current_layer.set_outline_thickness(3.0);
        current_layer.add_line(header_line);

        // Título de la empresa
        current_layer.use_text("TOSCANINI", 20.0, Mm(20.0), Mm(275.0), &font_bold);
        current_layer.set_fill_color(gray_color.clone());
        current_layer.use_text("SERVICIO TÉCNICO ESPECIALIZADO", 10.0, Mm(75.0), Mm(275.0), &font_regular);
        
        // === TÍTULO DEL DOCUMENTO ===
        current_layer.set_fill_color(green_color.clone());
        current_layer.use_text("INFORME TÉCNICO", 18.0, Mm(20.0), Mm(260.0), &font_bold);
        current_layer.set_fill_color(black_color.clone());
        current_layer.use_text(&format!("Código: {}", data.informe_codigo), 12.0, Mm(20.0), Mm(250.0), &font_bold);
        
        // Fecha en esquina superior derecha
        current_layer.set_fill_color(gray_color.clone());
        current_layer.use_text(&format!("Fecha: {}", data.fecha.format("%d/%m/%Y")), 10.0, Mm(140.0), Mm(270.0), &font_regular);
        if let Some(orden) = &data.orden_codigo {
            current_layer.use_text(&format!("Orden: {}", orden), 10.0, Mm(140.0), Mm(260.0), &font_regular);
        }

        // === SECCIÓN CLIENTE Y EQUIPO (DOS COLUMNAS) ===
        current_layer.set_fill_color(blue_color.clone());
        current_layer.use_text("INFORMACIÓN DEL CLIENTE", 12.0, Mm(20.0), Mm(235.0), &font_bold);
        current_layer.use_text("INFORMACIÓN DEL EQUIPO", 12.0, Mm(110.0), Mm(235.0), &font_bold);
        
        // Líneas separadoras
        let separator_client = Line {
            points: vec![
                (Point::new(Mm(20.0), Mm(230.0)), false),
                (Point::new(Mm(100.0), Mm(230.0)), false)
            ],
            is_closed: false
        };
        current_layer.set_outline_thickness(1.0);
        current_layer.set_outline_color(blue_color.clone());
        current_layer.add_line(separator_client);
        
        let separator_equipo = Line {
            points: vec![
                (Point::new(Mm(110.0), Mm(230.0)), false),
                (Point::new(Mm(190.0), Mm(230.0)), false)
            ],
            is_closed: false
        };
        current_layer.add_line(separator_equipo);

        // Información del cliente (columna izquierda)
        current_layer.set_fill_color(black_color.clone());
        let mut y_client = 220.0;
        current_layer.use_text(&format!("• Cliente: {}", data.cliente.nombre), 9.0, Mm(25.0), Mm(y_client), &font_regular);
        y_client -= 8.0;
        
        if let Some(email) = &data.cliente.email {
            current_layer.use_text(&format!("• Email: {}", email), 9.0, Mm(25.0), Mm(y_client), &font_regular);
            y_client -= 8.0;
        }
        
        if let Some(telefono) = &data.cliente.telefono {
            current_layer.use_text(&format!("• Teléfono: {}", telefono), 9.0, Mm(25.0), Mm(y_client), &font_regular);
        }

        // Información del equipo (columna derecha)
        let mut y_equipo = 220.0;
        if let Some(marca) = &data.equipo.marca {
            current_layer.use_text(&format!("• Marca: {}", marca), 9.0, Mm(115.0), Mm(y_equipo), &font_regular);
            y_equipo -= 8.0;
        }
        
        if let Some(modelo) = &data.equipo.modelo {
            current_layer.use_text(&format!("• Modelo: {}", modelo), 9.0, Mm(115.0), Mm(y_equipo), &font_regular);
            y_equipo -= 8.0;
        }
        
        if let Some(numero_serie) = &data.equipo.numero_serie {
            current_layer.use_text(&format!("• N° Serie: {}", numero_serie), 9.0, Mm(115.0), Mm(y_equipo), &font_regular);
        }

        // === SECCIÓN DIAGNÓSTICO ===
        current_layer.set_fill_color(green_color.clone());
        current_layer.use_text("DIAGNÓSTICO Y TRABAJO REALIZADO", 12.0, Mm(20.0), Mm(190.0), &font_bold);
        
        let separator_diag = Line {
            points: vec![
                (Point::new(Mm(20.0), Mm(185.0)), false),
                (Point::new(Mm(190.0), Mm(185.0)), false)
            ],
            is_closed: false
        };
        current_layer.set_outline_color(green_color.clone());
        current_layer.add_line(separator_diag);

        // Diagnóstico con texto envuelto
        current_layer.set_fill_color(black_color.clone());
        let diagnostico_lines = wrap_text(&data.diagnostico, 90);
        let mut y_diag = 175.0;
        for line in diagnostico_lines.iter().take(8) {
            current_layer.use_text(line, 9.0, Mm(25.0), Mm(y_diag), &font_regular);
            y_diag -= 6.0;
        }

        // Solución aplicada si existe
        if let Some(solucion) = &data.solucion_aplicada {
            if !solucion.trim().is_empty() {
                current_layer.set_fill_color(blue_color.clone());
                current_layer.use_text("Solución Aplicada:", 10.0, Mm(25.0), Mm(y_diag - 8.0), &font_bold);
                
                current_layer.set_fill_color(black_color.clone());
                let solucion_lines = wrap_text(solucion, 90);
                y_diag -= 18.0;
                for line in solucion_lines.iter().take(6) {
                    current_layer.use_text(line, 9.0, Mm(25.0), Mm(y_diag), &font_regular);
                    y_diag -= 6.0;
                }
            }
        }

        // === INFORMACIÓN TÉCNICA ===
        let y_final = y_diag - 15.0;
        current_layer.set_fill_color(blue_color.clone());
        current_layer.use_text("TÉCNICO RESPONSABLE Y GARANTÍA", 11.0, Mm(20.0), Mm(y_final), &font_bold);
        
        current_layer.set_fill_color(black_color.clone());
        current_layer.use_text(&format!("Técnico: {}", data.tecnico_responsable), 10.0, Mm(25.0), Mm(y_final - 12.0), &font_regular);

        // Estado de garantía - solo mostrar si HAY garantía
        if data.tiene_garantia {
            current_layer.set_fill_color(green_color.clone());
            current_layer.use_text("✓ TRABAJO CON GARANTÍA", 11.0, Mm(25.0), Mm(y_final - 25.0), &font_bold);
        }

        // === TÉRMINOS Y CONDICIONES ===
        let mut y_terminos = y_final - 40.0;
        println!("DEBUG: Renderizando {} términos en informe PDF", data.terminos_condiciones.len());
        
        current_layer.set_fill_color(blue_color.clone());
        current_layer.use_text(&format!("TÉRMINOS Y CONDICIONES ({})", data.terminos_condiciones.len()), 11.0, Mm(20.0), Mm(y_terminos), &font_bold);
        
        let separator_terminos = Line {
            points: vec![
                (Point::new(Mm(20.0), Mm(y_terminos - 5.0)), false),
                (Point::new(Mm(190.0), Mm(y_terminos - 5.0)), false)
            ],
            is_closed: false
        };
        current_layer.set_outline_color(blue_color.clone());
        current_layer.add_line(separator_terminos);

        // Compilar TODOS los términos en un solo texto
        let texto_terminos = compile_terminos_text(&data.terminos_condiciones);
        
        // Renderizar el texto completo con wrapping y soporte para múltiples páginas
        current_layer.set_fill_color(black_color.clone());
        y_terminos -= 15.0;
        let terminos_lines = wrap_text(&texto_terminos, 85);
        
        // Variables para el manejo de páginas
        let mut current_page_id = page1;
        let mut current_layer_id = layer1;
        let mut current_y = y_terminos;
        
        for (i, line) in terminos_lines.iter().enumerate() {
            let page_layer = doc.get_page(current_page_id).get_layer(current_layer_id);
            
            if current_y > 50.0 {
                page_layer.use_text(line, 8.0, Mm(25.0), Mm(current_y), &font_regular);
                current_y -= 5.0;
            } else {
                // Crear nueva página para continuar con los términos
                println!("DEBUG: Creando nueva página para términos restantes (línea {})", i + 1);
                
                let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Términos - Página 2");
                current_page_id = new_page;
                current_layer_id = new_layer;
                current_y = 280.0; // Empezar desde arriba de la nueva página
                
                let new_page_layer = doc.get_page(current_page_id).get_layer(current_layer_id);
                
                // Header en la nueva página
                new_page_layer.set_fill_color(blue_color.clone());
                new_page_layer.use_text("TOSCANINI - Términos y Condiciones (Continuación)", 12.0, Mm(20.0), Mm(current_y), &font_bold);
                current_y -= 20.0;
                
                // Continuar renderizando
                new_page_layer.set_fill_color(black_color.clone());
                new_page_layer.use_text(line, 8.0, Mm(25.0), Mm(current_y), &font_regular);
                current_y -= 5.0;
            }
        }

        // === FOOTER PROFESIONAL ===
        // Usar la primera página para el footer (o la última página usada)
        let footer_layer = doc.get_page(page1).get_layer(layer1);
        footer_layer.set_fill_color(gray_color.clone());
        let footer_line = Line {
            points: vec![
                (Point::new(Mm(20.0), Mm(40.0)), false),
                (Point::new(Mm(190.0), Mm(40.0)), false)
            ],
            is_closed: false
        };
        footer_layer.set_outline_thickness(1.0);
        footer_layer.set_outline_color(gray_color.clone());
        footer_layer.add_line(footer_line);
        
        footer_layer.use_text("TOSCANINI - Servicio Técnico Especializado", 8.0, Mm(20.0), Mm(35.0), &font_regular);
        footer_layer.use_text("Este documento certifica el trabajo realizado y garantiza la calidad del servicio.", 8.0, Mm(20.0), Mm(30.0), &font_regular);

        // Generar PDF
        doc.save_to_bytes()
            .map_err(|e| format!("Error generando PDF: {}", e))
    }
}

