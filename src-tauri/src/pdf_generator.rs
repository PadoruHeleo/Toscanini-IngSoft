use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use printpdf::{PdfDocument, BuiltinFont, Color, Rgb, Mm, Line, Point};
use sqlx::Row;

// Estructuras para datos del PDF
#[derive(Debug, Serialize, Deserialize)]
pub struct EmpresaInfo {
    pub nombre: String,
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClienteInfo {
    pub nombre: String,
    pub email: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquipoInfo {
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub tipo: Option<String>,
    pub numero_serie: Option<String>,
    pub ubicacion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PiezaPdf {
    pub nombre: String,
    pub marca: Option<String>,
    pub cantidad: i32,
    pub precio_unitario: i32,
    pub subtotal: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerminoPdf {
    pub nombre: String,
    pub descripcion: String,
}

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

// Estructuras no utilizadas - comentadas para evitar warnings
// #[derive(Debug)]
// struct CotizacionRow {
//     cotizacion_codigo: Option<String>,
//     costo_revision: Option<i32>,
//     costo_reparacion: Option<i32>,
//     costo_total: Option<i32>,
//     is_aprobada: Option<i32>,
//     informe: Option<String>,
//     created_at: chrono::DateTime<chrono::Utc>,
//     orden_codigo: Option<String>,
//     cliente_nombre: Option<String>,
//     cliente_correo: Option<String>,
//     cliente_telefono: Option<String>,
//     cliente_direccion: Option<String>,
//     equipo_marca: Option<String>,
//     equipo_modelo: Option<String>,
//     equipo_tipo: Option<String>,
//     numero_serie: Option<String>,
//     equipo_ubicacion: Option<String>,
// }

// #[derive(Debug)]
// struct PiezaRow {
//     pieza_nombre: Option<String>,
//     pieza_marca: Option<String>,
//     pieza_precio: Option<i32>,
//     cantidad: Option<i32>,
// }

// #[derive(Debug)]
// struct TerminoRow {
//     termino_nombre: String,
//     termino_descripcion: String,
// }

// #[derive(Debug)]
// struct InformeRow {
//     informe_codigo: Option<String>,
//     diagnostico: Option<String>,
//     recomendaciones: Option<String>,
//     solucion_aplicada: Option<String>,
//     tecnico_responsable: Option<String>,
//     created_at: chrono::DateTime<chrono::Utc>,
//     orden_codigo: Option<String>,
//     has_garantia: Option<i32>,
//     cliente_nombre: Option<String>,
//     cliente_correo: Option<String>,
//     cliente_telefono: Option<String>,
//     cliente_direccion: Option<String>,
//     equipo_marca: Option<String>,
//     equipo_modelo: Option<String>,
//     equipo_tipo: Option<String>,
//     numero_serie: Option<String>,
//     equipo_ubicacion: Option<String>,
// }

pub struct PdfGenerator;

impl PdfGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Función auxiliar para dividir texto en líneas
    fn wrap_text(&self, text: &str, max_chars: usize) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in words {
            if current_line.len() + word.len() + 1 > max_chars {
                if !current_line.is_empty() {
                    lines.push(current_line.trim().to_string());
                    current_line = String::new();
                }
            }
            
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
        
        if !current_line.is_empty() {
            lines.push(current_line.trim().to_string());
        }
        
        if lines.is_empty() {
            lines.push(String::new());
        }
        
        lines
    }

    // Función para compilar todos los términos y condiciones en un solo texto
    fn compile_terminos_text(&self, terminos: &[TerminoPdf]) -> String {
        if terminos.is_empty() {
            return "No se han definido términos y condiciones específicos para este documento.".to_string();
        }

        let mut texto_completo = String::new();
        
        for (i, termino) in terminos.iter().enumerate() {
            // Agregar el nombre del término con numeración (formato mejorado para múltiples páginas)
            texto_completo.push_str(&format!("{}. {}", i + 1, termino.nombre));
            
            // Agregar la descripción si existe
            if !termino.descripcion.is_empty() {
                texto_completo.push_str(&format!("- {}", termino.descripcion));
            }
            
            // Agregar salto de línea entre términos para mejor legibilidad en múltiples páginas
            if i < terminos.len() - 1 {
                texto_completo.push_str(" || ");
            }
        }
        
        println!("DEBUG: Texto compilado de {} términos: {}", terminos.len(), &texto_completo[..texto_completo.len().min(100)]);
        println!("DEBUG: Longitud total del texto: {} caracteres", texto_completo.len());
        texto_completo
    }

    /// Generar PDF de cotización
    pub async fn generate_cotizacion_pdf(&self, data: CotizacionPdfData) -> Result<Vec<u8>, String> {
        self.generate_basic_pdf(&data).await
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
        let diagnostico_lines = self.wrap_text(&data.diagnostico, 90);
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
                let solucion_lines = self.wrap_text(solucion, 90);
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
        let texto_terminos = self.compile_terminos_text(&data.terminos_condiciones);
        
        // Renderizar el texto completo con wrapping y soporte para múltiples páginas
        current_layer.set_fill_color(black_color.clone());
        y_terminos -= 15.0;
        let terminos_lines = self.wrap_text(&texto_terminos, 85);
        
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

    async fn generate_basic_pdf(&self, data: &CotizacionPdfData) -> Result<Vec<u8>, String> {
        let (doc, page1, layer1) = PdfDocument::new("Toscanini - Cotización", Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Configurar fuentes
        let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
            .map_err(|e| format!("Error cargando fuente bold: {}", e))?;
        let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica)
            .map_err(|e| format!("Error cargando fuente regular: {}", e))?;

        // Colores
        let blue_color = Color::Rgb(Rgb::new(0.2, 0.4, 0.8, None));
        let green_color = Color::Rgb(Rgb::new(0.2, 0.7, 0.3, None));
        let orange_color = Color::Rgb(Rgb::new(0.9, 0.6, 0.0, None));
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
        current_layer.set_fill_color(blue_color.clone());
        current_layer.use_text("COTIZACIÓN", 18.0, Mm(20.0), Mm(260.0), &font_bold);
        current_layer.set_fill_color(black_color.clone());
        current_layer.use_text(&format!("Código: {}", data.cotizacion_codigo), 12.0, Mm(20.0), Mm(250.0), &font_bold);
        
        // Fecha y estado en esquina superior derecha
        current_layer.set_fill_color(gray_color.clone());
        current_layer.use_text(&format!("Fecha: {}", data.fecha.format("%d/%m/%Y")), 10.0, Mm(140.0), Mm(270.0), &font_regular);
        
        let (estado_text, estado_color) = if data.is_aprobada { 
            ("✓ APROBADA", green_color.clone()) 
        } else { 
            ("⏳ PENDIENTE", orange_color.clone()) 
        };
        current_layer.set_fill_color(estado_color);
        current_layer.use_text(estado_text, 10.0, Mm(140.0), Mm(260.0), &font_bold);

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
        
        if let Some(tipo) = &data.equipo.tipo {
            current_layer.use_text(&format!("• Tipo: {}", tipo), 9.0, Mm(115.0), Mm(y_equipo), &font_regular);
        }

        // === SECCIÓN COSTOS ===
        current_layer.set_fill_color(green_color.clone());
        current_layer.use_text("DETALLE DE COSTOS", 12.0, Mm(20.0), Mm(190.0), &font_bold);
        
        let separator_costos = Line {
            points: vec![
                (Point::new(Mm(20.0), Mm(185.0)), false),
                (Point::new(Mm(190.0), Mm(185.0)), false)
            ],
            is_closed: false
        };
        current_layer.set_outline_color(green_color.clone());
        current_layer.add_line(separator_costos);

        // Costos individuales
        current_layer.set_fill_color(black_color.clone());
        let mut y_costos = 175.0;
        
        if let Some(revision) = data.costo_revision {
            current_layer.use_text(&format!("• Costo de Revisión: ${}", revision), 10.0, Mm(25.0), Mm(y_costos), &font_regular);
            y_costos -= 10.0;
        }
        
        if let Some(reparacion) = data.costo_reparacion {
            current_layer.use_text(&format!("• Costo de Reparación: ${}", reparacion), 10.0, Mm(25.0), Mm(y_costos), &font_regular);
            y_costos -= 10.0;
        }

        // Total destacado
        y_costos -= 5.0;
        current_layer.set_fill_color(green_color.clone());
        current_layer.use_text(&format!("TOTAL: ${}", data.costo_total), 14.0, Mm(25.0), Mm(y_costos), &font_bold);

        // === INFORME TÉCNICO ===
        if !data.informe_tecnico.trim().is_empty() {
            y_costos -= 25.0;
            current_layer.set_fill_color(blue_color.clone());
            current_layer.use_text("INFORME TÉCNICO", 11.0, Mm(20.0), Mm(y_costos), &font_bold);
            
            current_layer.set_fill_color(black_color.clone());
            let informe_lines = self.wrap_text(&data.informe_tecnico, 90);
            y_costos -= 12.0;
            for line in informe_lines.iter().take(6) {
                current_layer.use_text(line, 9.0, Mm(25.0), Mm(y_costos), &font_regular);
                y_costos -= 6.0;
            }
        }

        // === TÉRMINOS Y CONDICIONES ===
        let mut y_terminos = y_costos - 15.0;
        println!("DEBUG: Renderizando {} términos en cotización PDF", data.terminos_condiciones.len());
        
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
        let texto_terminos = self.compile_terminos_text(&data.terminos_condiciones);
        
        // Renderizar el texto completo con wrapping y soporte para múltiples páginas
        current_layer.set_fill_color(black_color.clone());
        y_terminos -= 15.0;
        let terminos_lines = self.wrap_text(&texto_terminos, 85);
        
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
        footer_layer.use_text("Esta cotización tiene validez por 30 días a partir de la fecha de emisión.", 8.0, Mm(20.0), Mm(30.0), &font_regular);

        // Generar PDF
        doc.save_to_bytes()
            .map_err(|e| format!("Error generando PDF: {}", e))
    }
}

// Comandos de Tauri
use tauri::command;
use crate::database::get_db_pool_safe;

#[command]
pub async fn generate_cotizacion_pdf_command(
    cotizacion_id: i32
) -> Result<Vec<u8>, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener datos de la cotización - Usar query en lugar de query!
    let cotizacion_row = sqlx::query(
        "SELECT c.cotizacion_codigo, c.costo_revision, c.costo_reparacion, c.costo_total, 
                c.is_aprobada, c.informe, c.created_at,
                ot.orden_codigo,
                cl.cliente_nombre, cl.cliente_correo, cl.cliente_telefono, cl.cliente_direccion,
                e.equipo_marca, e.equipo_modelo, e.equipo_tipo, e.numero_serie, e.equipo_ubicacion
         FROM COTIZACION c
         LEFT JOIN ORDEN_TRABAJO ot ON c.cotizacion_id = ot.cotizacion_id
         LEFT JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         LEFT JOIN CLIENTE cl ON e.cliente_id = cl.cliente_id
         WHERE c.cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo cotización: {}", e))?
    .ok_or_else(|| format!("Cotización con ID {} no encontrada", cotizacion_id))?;

    // Acceder a los campos por nombre o índice
    let cotizacion_codigo: Option<String> = cotizacion_row.try_get("cotizacion_codigo").ok();
    let costo_revision: Option<i32> = cotizacion_row.try_get("costo_revision").ok();
    let costo_reparacion: Option<i32> = cotizacion_row.try_get("costo_reparacion").ok();
    let costo_total: Option<i32> = cotizacion_row.try_get("costo_total").ok();
    let is_aprobada: Option<i32> = cotizacion_row.try_get("is_aprobada").ok();
    let informe: Option<String> = cotizacion_row.try_get("informe").ok();
    let created_at: chrono::DateTime<chrono::Utc> = cotizacion_row.try_get("created_at").ok().unwrap();
    let orden_codigo: Option<String> = cotizacion_row.try_get("orden_codigo").ok();
    let cliente_nombre: Option<String> = cotizacion_row.try_get("cliente_nombre").ok();
    let cliente_correo: Option<String> = cotizacion_row.try_get("cliente_correo").ok();
    let cliente_telefono: Option<String> = cotizacion_row.try_get("cliente_telefono").ok();
    let cliente_direccion: Option<String> = cotizacion_row.try_get("cliente_direccion").ok();
    let equipo_marca: Option<String> = cotizacion_row.try_get("equipo_marca").ok();
    let equipo_modelo: Option<String> = cotizacion_row.try_get("equipo_modelo").ok();
    let equipo_tipo: Option<String> = cotizacion_row.try_get("equipo_tipo").ok();
    let numero_serie: Option<String> = cotizacion_row.try_get("numero_serie").ok();
    let equipo_ubicacion: Option<String> = cotizacion_row.try_get("equipo_ubicacion").ok();

    // Obtener piezas de la cotización
    let piezas_rows = sqlx::query(
        "SELECT p.pieza_nombre, p.pieza_marca, p.pieza_precio, pc.cantidad
         FROM PIEZAS_COTIZACION pc
         INNER JOIN PIEZA p ON pc.pieza_id = p.pieza_id
         WHERE pc.cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo piezas: {}", e))?;

    // Obtener términos y condiciones de la cotización
    let terminos_rows = sqlx::query(
        "SELECT tc.termino_nombre, tc.termino_descripcion
         FROM TERMINOS_COTIZACION tcot
         INNER JOIN TERMINOS_CONDICIONES tc ON tcot.termino_id = tc.termino_id
         WHERE tcot.cotizacion_id = ? AND tcot.aplicado = TRUE AND tc.is_active = TRUE
         ORDER BY tc.termino_nombre"
    )
    .bind(cotizacion_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo términos y condiciones: {}", e))?;
    
    println!("DEBUG: Encontrados {} términos para cotización {}", terminos_rows.len(), cotizacion_id);

    let piezas: Vec<PiezaPdf> = piezas_rows.iter().map(|row| {
        let pieza_nombre: String = row.try_get("pieza_nombre")
            .ok()
            .flatten()
            .unwrap_or_else(|| "Pieza sin nombre".to_string());
        let pieza_marca: Option<String> = row.try_get("pieza_marca").ok().flatten();
        let pieza_precio: Option<i32> = row.try_get("pieza_precio").ok().flatten();
        let cantidad: Option<i32> = row.try_get("cantidad").ok().flatten();
        
        PiezaPdf {
            nombre: pieza_nombre,
            marca: pieza_marca,
            cantidad: cantidad.unwrap_or(1),
            precio_unitario: pieza_precio.unwrap_or(0),
            subtotal: (pieza_precio.unwrap_or(0) * cantidad.unwrap_or(1)),
        }
    }).collect();

    let terminos_condiciones: Vec<TerminoPdf> = terminos_rows.iter().map(|row| {
        let termino_nombre: String = row.try_get("termino_nombre").unwrap();
        let termino_descripcion: String = row.try_get("termino_descripcion").unwrap();
        TerminoPdf {
            nombre: termino_nombre,
            descripcion: termino_descripcion,
        }
    }).collect();

    // Crear estructura de datos para el PDF
    let pdf_data = CotizacionPdfData {
        cotizacion_codigo: cotizacion_codigo.unwrap_or_else(|| "COT-0000".to_string()),
        fecha: created_at,
        empresa: EmpresaInfo {
            nombre: "Toscanini".to_string(),
            direccion: Some("Dirección de la empresa".to_string()),
            telefono: Some("+56 9 1234 5678".to_string()),
            email: Some("contacto@toscanini.cl".to_string()),
            website: Some("www.toscanini.cl".to_string()),
        },
        cliente: ClienteInfo {
            nombre: cliente_nombre.unwrap_or_else(|| "Cliente sin nombre".to_string()),
            email: cliente_correo,
            telefono: cliente_telefono,
            direccion: cliente_direccion,
        },
        equipo: EquipoInfo {
            marca: equipo_marca,
            modelo: equipo_modelo,
            tipo: equipo_tipo,
            numero_serie: numero_serie,
            ubicacion: equipo_ubicacion,
        },
        informe_tecnico: informe.unwrap_or_else(|| "Sin informe técnico".to_string()),
        costo_revision: costo_revision,
        costo_reparacion: costo_reparacion,
        costo_total: costo_total.unwrap_or(0),
        piezas,
        is_aprobada: is_aprobada.unwrap_or(0) == 1,
        orden_codigo: orden_codigo,
        terminos_condiciones,
    };

    // Generar PDF
    let generator = PdfGenerator::new();
    generator.generate_cotizacion_pdf(pdf_data).await
}

#[command]  
pub async fn generate_informe_pdf_command(
    informe_id: i32
) -> Result<Vec<u8>, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener datos del informe
    let informe_row = sqlx::query(
        "SELECT i.informe_codigo, i.diagnostico, i.recomendaciones, i.solucion_aplicada, 
                i.tecnico_responsable, i.created_at,
                ot.orden_codigo, ot.has_garantia,
                cl.cliente_nombre, cl.cliente_correo, cl.cliente_telefono, cl.cliente_direccion,
                e.equipo_marca, e.equipo_modelo, e.equipo_tipo, e.numero_serie, e.equipo_ubicacion
         FROM INFORME i
         LEFT JOIN ORDEN_TRABAJO ot ON i.informe_id = ot.informe_id
         LEFT JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         LEFT JOIN CLIENTE cl ON e.cliente_id = cl.cliente_id
         WHERE i.informe_id = ?"
    )
    .bind(informe_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo informe: {}", e))?
    .ok_or_else(|| format!("Informe con ID {} no encontrado", informe_id))?;

    // Acceder a los campos por nombre o índice
    let informe_codigo: Option<String> = informe_row.try_get("informe_codigo").ok();
    let diagnostico: Option<String> = informe_row.try_get("diagnostico").ok();
    let recomendaciones: Option<String> = informe_row.try_get("recomendaciones").ok();
    let solucion_aplicada: Option<String> = informe_row.try_get("solucion_aplicada").ok();
    let tecnico_responsable: Option<String> = informe_row.try_get("tecnico_responsable").ok();
    let created_at: chrono::DateTime<chrono::Utc> = informe_row.try_get("created_at").ok().unwrap();
    let orden_codigo: Option<String> = informe_row.try_get("orden_codigo").ok();
    let has_garantia: Option<i32> = informe_row.try_get("has_garantia").ok();
    let cliente_nombre: Option<String> = informe_row.try_get("cliente_nombre").ok();
    let cliente_correo: Option<String> = informe_row.try_get("cliente_correo").ok();
    let cliente_telefono: Option<String> = informe_row.try_get("cliente_telefono").ok();
    let cliente_direccion: Option<String> = informe_row.try_get("cliente_direccion").ok();
    let equipo_marca: Option<String> = informe_row.try_get("equipo_marca").ok();
    let equipo_modelo: Option<String> = informe_row.try_get("equipo_modelo").ok();
    let equipo_tipo: Option<String> = informe_row.try_get("equipo_tipo").ok();
    let numero_serie: Option<String> = informe_row.try_get("numero_serie").ok();
    let equipo_ubicacion: Option<String> = informe_row.try_get("equipo_ubicacion").ok();

    // Obtener términos y condiciones del informe
    let terminos_rows = sqlx::query(
        "SELECT tc.termino_nombre, tc.termino_descripcion
         FROM TERMINOS_INFORME ti
         INNER JOIN TERMINOS_CONDICIONES tc ON ti.termino_id = tc.termino_id
         WHERE ti.informe_id = ? AND ti.aplicado = TRUE AND tc.is_active = TRUE
         ORDER BY tc.termino_nombre"
    )
    .bind(informe_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo términos y condiciones del informe: {}", e))?;
    
    println!("DEBUG: Encontrados {} términos para informe {}", terminos_rows.len(), informe_id);

    // Obtener piezas del informe
    let piezas_rows = sqlx::query(
        "SELECT p.pieza_nombre, p.pieza_marca, p.pieza_precio, pi.cantidad
         FROM PIEZAS_INFORME pi
         INNER JOIN PIEZA p ON pi.pieza_id = p.pieza_id
         WHERE pi.informe_id = ?"
    )
    .bind(informe_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo piezas del informe: {}", e))?;

    let piezas: Vec<PiezaPdf> = piezas_rows.iter().map(|row| {
        let pieza_nombre: String = row.try_get("pieza_nombre")
            .ok()
            .flatten()
            .unwrap_or_else(|| "Pieza sin nombre".to_string());
        let pieza_marca: Option<String> = row.try_get("pieza_marca").ok();
        let pieza_precio: Option<i32> = row.try_get("pieza_precio").ok();
        let cantidad: Option<i32> = row.try_get("cantidad").ok();
        PiezaPdf {
            nombre: pieza_nombre,
            
            marca: pieza_marca,
            cantidad: cantidad.unwrap_or(1),
            precio_unitario: pieza_precio.unwrap_or(0),
            subtotal: (pieza_precio.unwrap_or(0) * cantidad.unwrap_or(1)),
        }
    }).collect();

    let terminos_condiciones: Vec<TerminoPdf> = terminos_rows.iter().map(|row| {
        let termino_nombre: String = row.try_get("termino_nombre").unwrap();
        let termino_descripcion: String = row.try_get("termino_descripcion").unwrap();
        TerminoPdf {
            nombre: termino_nombre,
            descripcion: termino_descripcion,
        }
    }).collect();

    // Crear estructura de datos para el PDF
    let pdf_data = InformePdfData {
        informe_codigo: informe_codigo.unwrap_or_else(|| "INF-0000".to_string()),
        fecha: created_at,
        empresa: EmpresaInfo {
            nombre: "Toscanini".to_string(),
            direccion: Some("Dirección de la empresa".to_string()),
            telefono: Some("+56 9 1234 5678".to_string()),
            email: Some("contacto@toscanini.cl".to_string()),
            website: Some("www.toscanini.cl".to_string()),
        },
        cliente: ClienteInfo {
            nombre: cliente_nombre.unwrap_or_else(|| "Cliente sin nombre".to_string()),
            email: cliente_correo,
            telefono: cliente_telefono,
            direccion: cliente_direccion,
        },
        equipo: EquipoInfo {
            marca: equipo_marca,
            modelo: equipo_modelo,
            tipo: equipo_tipo,
            numero_serie: numero_serie,
            ubicacion: equipo_ubicacion,
        },
        diagnostico: diagnostico.unwrap_or_else(|| "Sin diagnóstico".to_string()),
        recomendaciones: recomendaciones,
        solucion_aplicada: solucion_aplicada,
        tecnico_responsable: tecnico_responsable.unwrap_or_else(|| "No especificado".to_string()),
        piezas,
        orden_codigo: orden_codigo,
        tiene_garantia: has_garantia.unwrap_or(0) == 1,
        terminos_condiciones,
    };

    // Generar PDF
    let generator = PdfGenerator::new();
    generator.generate_informe_pdf(pdf_data).await
}