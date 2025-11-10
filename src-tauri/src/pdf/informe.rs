use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use printpdf::{PdfDocument, BuiltinFont, Color, Rgb, Mm};
use crate::pdf::common::{EmpresaInfo, ClienteInfo, EquipoInfo, PiezaPdf, TerminoPdf, wrap_text};

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

        // Colores - estilo formal (solo negro y gris)
        let black_color = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
        let gray_color = Color::Rgb(Rgb::new(0.3, 0.3, 0.3, None));

        // === TÍTULO DEL DOCUMENTO (izquierda) ===
        current_layer.set_fill_color(black_color.clone());
        current_layer.use_text("INFORME TÉCNICO", 16.0, Mm(20.0), Mm(280.0), &font_bold);
        
        // === NÚMERO DE INFORME (derecha, grande y destacado) ===
        current_layer.use_text(&data.informe_codigo, 24.0, Mm(140.0), Mm(280.0), &font_bold);
        current_layer.use_text("Informe", 9.0, Mm(140.0), Mm(270.0), &font_regular);
        
        // === FECHAS (derecha, debajo del número) ===
        current_layer.set_fill_color(gray_color.clone());
        current_layer.use_text(&format!("Fecha de Emisión: {}", data.fecha.format("%d/%m/%Y")), 9.0, Mm(140.0), Mm(260.0), &font_regular);
        current_layer.use_text(&format!("Documento Impreso: {}", chrono::Utc::now().format("%d/%m/%Y")), 9.0, Mm(140.0), Mm(253.0), &font_regular);
        
        // Orden de trabajo si existe
        current_layer.set_fill_color(black_color.clone());
        if let Some(orden) = &data.orden_codigo {
            current_layer.use_text(&format!("Orden: {}", orden), 9.0, Mm(140.0), Mm(246.0), &font_regular);
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

        // === DIAGNÓSTICO Y TRABAJO REALIZADO ===
        y_pos -= 10.0;
        current_layer.use_text("Diagnóstico y Trabajo Realizado", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
        y_pos -= 6.0;
        
        if !data.diagnostico.trim().is_empty() {
            let diagnostico_lines = wrap_text(&data.diagnostico, 85);
            for line in diagnostico_lines.iter().take(8) {
                current_layer.use_text(line, 10.0, Mm(25.0), Mm(y_pos), &font_regular);
                y_pos -= 6.0;
            }
        } else {
            current_layer.use_text("Diagnóstico técnico realizado", 10.0, Mm(25.0), Mm(y_pos), &font_regular);
            y_pos -= 6.0;
        }

        // Solución aplicada si existe
        if let Some(solucion) = &data.solucion_aplicada {
            if !solucion.trim().is_empty() {
                y_pos -= 6.0;
                current_layer.use_text("Solución Aplicada", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
                y_pos -= 6.0;
                
                let solucion_lines = wrap_text(solucion, 85);
                for line in solucion_lines.iter().take(6) {
                    current_layer.use_text(line, 10.0, Mm(25.0), Mm(y_pos), &font_regular);
                    y_pos -= 6.0;
                }
            }
        }

        // Recomendaciones si existen
        if let Some(recomendaciones) = &data.recomendaciones {
            if !recomendaciones.trim().is_empty() {
                y_pos -= 6.0;
                current_layer.use_text("Recomendaciones", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
                y_pos -= 6.0;
                
                let recomendaciones_lines = wrap_text(recomendaciones, 85);
                for line in recomendaciones_lines.iter().take(6) {
                    current_layer.use_text(line, 10.0, Mm(25.0), Mm(y_pos), &font_regular);
                    y_pos -= 6.0;
                }
            }
        }

        // === INFORMACIÓN TÉCNICA ===
        y_pos -= 10.0;
        current_layer.use_text("Técnico Responsable", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
        current_layer.use_text(&data.tecnico_responsable, 10.0, Mm(70.0), Mm(y_pos), &font_regular);
        y_pos -= 6.0;

        // Estado de garantía - solo mostrar si HAY garantía
        if data.tiene_garantia {
            current_layer.use_text("Garantía", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
            current_layer.use_text("TRABAJO CON GARANTÍA", 10.0, Mm(70.0), Mm(y_pos), &font_bold);
            y_pos -= 6.0;
        }

        y_pos -= 5.0;

        // === TÉRMINOS Y CONDICIONES ===
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
                    new_page_layer.use_text("TOSCANINI - Informe", 11.0, Mm(20.0), Mm(current_y), &font_bold);
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

