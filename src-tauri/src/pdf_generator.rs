use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use printpdf::{PdfDocument, BuiltinFont, Color, Rgb, Mm, Line, Point};

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
}

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

        // Estado de garantía con colores
        let garantia_text = if data.tiene_garantia { "✓ TRABAJO CON GARANTÍA" } else { "⚠ TRABAJO SIN GARANTÍA" };
        let garantia_color = if data.tiene_garantia { green_color.clone() } else { Color::Rgb(Rgb::new(0.9, 0.3, 0.0, None)) };
        
        current_layer.set_fill_color(garantia_color);
        current_layer.use_text(garantia_text, 11.0, Mm(25.0), Mm(y_final - 25.0), &font_bold);

        // === FOOTER PROFESIONAL ===
        current_layer.set_fill_color(gray_color.clone());
        let footer_line = Line {
            points: vec![
                (Point::new(Mm(20.0), Mm(40.0)), false),
                (Point::new(Mm(190.0), Mm(40.0)), false)
            ],
            is_closed: false
        };
        current_layer.set_outline_thickness(1.0);
        current_layer.set_outline_color(gray_color.clone());
        current_layer.add_line(footer_line);
        
        current_layer.use_text("TOSCANINI - Servicio Técnico Especializado", 8.0, Mm(20.0), Mm(35.0), &font_regular);
        current_layer.use_text("Este documento certifica el trabajo realizado y garantiza la calidad del servicio.", 8.0, Mm(20.0), Mm(30.0), &font_regular);

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

        // === FOOTER PROFESIONAL ===
        current_layer.set_fill_color(gray_color.clone());
        let footer_line = Line {
            points: vec![
                (Point::new(Mm(20.0), Mm(40.0)), false),
                (Point::new(Mm(190.0), Mm(40.0)), false)
            ],
            is_closed: false
        };
        current_layer.set_outline_thickness(1.0);
        current_layer.set_outline_color(gray_color.clone());
        current_layer.add_line(footer_line);
        
        current_layer.use_text("TOSCANINI - Servicio Técnico Especializado", 8.0, Mm(20.0), Mm(35.0), &font_regular);
        current_layer.use_text("Esta cotización tiene validez por 30 días a partir de la fecha de emisión.", 8.0, Mm(20.0), Mm(30.0), &font_regular);

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
    
    // Obtener datos de la cotización
    let cotizacion = sqlx::query!(
        "SELECT c.cotizacion_codigo, c.costo_revision, c.costo_reparacion, c.costo_total, 
                c.is_aprobada, c.informe, c.created_at,
                ot.orden_codigo,
                cl.cliente_nombre, cl.cliente_correo, cl.cliente_telefono, cl.cliente_direccion,
                e.equipo_marca, e.equipo_modelo, e.equipo_tipo, e.numero_serie, e.equipo_ubicacion
         FROM COTIZACION c
         LEFT JOIN ORDEN_TRABAJO ot ON c.cotizacion_id = ot.cotizacion_id
         LEFT JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         LEFT JOIN CLIENTE cl ON e.cliente_id = cl.cliente_id
         WHERE c.cotizacion_id = ?",
        cotizacion_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Error obteniendo cotización: {}", e))?
    .ok_or_else(|| format!("Cotización con ID {} no encontrada", cotizacion_id))?;

    // Obtener piezas de la cotización
    let piezas_rows = sqlx::query!(
        "SELECT p.pieza_nombre, p.pieza_marca, p.pieza_precio, pc.cantidad
         FROM PIEZAS_COTIZACION pc
         INNER JOIN PIEZA p ON pc.pieza_id = p.pieza_id
         WHERE pc.cotizacion_id = ?",
        cotizacion_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error obteniendo piezas: {}", e))?;

    let piezas: Vec<PiezaPdf> = piezas_rows.iter().map(|row| {
        let precio = row.pieza_precio.unwrap_or(0);
        let cantidad = row.cantidad.unwrap_or(1);
        PiezaPdf {
            nombre: row.pieza_nombre.clone().unwrap_or_else(|| "Pieza sin nombre".to_string()),
            marca: row.pieza_marca.clone(),
            cantidad,
            precio_unitario: precio,
            subtotal: precio * cantidad,
        }
    }).collect();

    // Crear estructura de datos para el PDF
    let pdf_data = CotizacionPdfData {
        cotizacion_codigo: cotizacion.cotizacion_codigo.unwrap_or_else(|| "COT-0000".to_string()),
        fecha: cotizacion.created_at,
        empresa: EmpresaInfo {
            nombre: "Toscanini".to_string(),
            direccion: Some("Dirección de la empresa".to_string()),
            telefono: Some("+56 9 1234 5678".to_string()),
            email: Some("contacto@toscanini.cl".to_string()),
            website: Some("www.toscanini.cl".to_string()),
        },
        cliente: ClienteInfo {
            nombre: cotizacion.cliente_nombre.unwrap_or_else(|| "Cliente sin nombre".to_string()),
            email: cotizacion.cliente_correo,
            telefono: cotizacion.cliente_telefono,
            direccion: cotizacion.cliente_direccion,
        },
        equipo: EquipoInfo {
            marca: cotizacion.equipo_marca,
            modelo: cotizacion.equipo_modelo,
            tipo: cotizacion.equipo_tipo,
            numero_serie: cotizacion.numero_serie,
            ubicacion: cotizacion.equipo_ubicacion,
        },
        informe_tecnico: cotizacion.informe,
        costo_revision: cotizacion.costo_revision,
        costo_reparacion: cotizacion.costo_reparacion,
        costo_total: cotizacion.costo_total.unwrap_or(0),
        piezas,
        is_aprobada: cotizacion.is_aprobada.unwrap_or(0) == 1,
        orden_codigo: cotizacion.orden_codigo,
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
    let informe = sqlx::query!(
        "SELECT i.informe_codigo, i.diagnostico, i.recomendaciones, i.solucion_aplicada, 
                i.tecnico_responsable, i.created_at,
                ot.orden_codigo, ot.has_garantia,
                cl.cliente_nombre, cl.cliente_correo, cl.cliente_telefono, cl.cliente_direccion,
                e.equipo_marca, e.equipo_modelo, e.equipo_tipo, e.numero_serie, e.equipo_ubicacion
         FROM INFORME i
         LEFT JOIN ORDEN_TRABAJO ot ON i.informe_id = ot.informe_id
         LEFT JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         LEFT JOIN CLIENTE cl ON e.cliente_id = cl.cliente_id
         WHERE i.informe_id = ?",
        informe_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Error obteniendo informe: {}", e))?
    .ok_or_else(|| format!("Informe con ID {} no encontrado", informe_id))?;

    // Obtener piezas del informe
    let piezas_rows = sqlx::query!(
        "SELECT p.pieza_nombre, p.pieza_marca, p.pieza_precio, pi.cantidad
         FROM PIEZAS_INFORME pi
         INNER JOIN PIEZA p ON pi.pieza_id = p.pieza_id
         WHERE pi.informe_id = ?",
        informe_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error obteniendo piezas del informe: {}", e))?;

    let piezas: Vec<PiezaPdf> = piezas_rows.iter().map(|row| {
        let precio = row.pieza_precio.unwrap_or(0);
        let cantidad = row.cantidad.unwrap_or(1);
        PiezaPdf {
            nombre: row.pieza_nombre.clone().unwrap_or_else(|| "Pieza sin nombre".to_string()),
            marca: row.pieza_marca.clone(),
            cantidad,
            precio_unitario: precio,
            subtotal: precio * cantidad,
        }
    }).collect();

    // Crear estructura de datos para el PDF
    let pdf_data = InformePdfData {
        informe_codigo: informe.informe_codigo.unwrap_or_else(|| "INF-0000".to_string()),
        fecha: informe.created_at,
        empresa: EmpresaInfo {
            nombre: "Toscanini".to_string(),
            direccion: Some("Dirección de la empresa".to_string()),
            telefono: Some("+56 9 1234 5678".to_string()),
            email: Some("contacto@toscanini.cl".to_string()),
            website: Some("www.toscanini.cl".to_string()),
        },
        cliente: ClienteInfo {
            nombre: informe.cliente_nombre.unwrap_or_else(|| "Cliente sin nombre".to_string()),
            email: informe.cliente_correo,
            telefono: informe.cliente_telefono,
            direccion: informe.cliente_direccion,
        },
        equipo: EquipoInfo {
            marca: informe.equipo_marca,
            modelo: informe.equipo_modelo,
            tipo: informe.equipo_tipo,
            numero_serie: informe.numero_serie,
            ubicacion: informe.equipo_ubicacion,
        },
        diagnostico: informe.diagnostico.unwrap_or_else(|| "Sin diagnóstico".to_string()),
        recomendaciones: informe.recomendaciones,
        solucion_aplicada: informe.solucion_aplicada,
        tecnico_responsable: informe.tecnico_responsable.unwrap_or_else(|| "No especificado".to_string()),
        piezas,
        orden_codigo: informe.orden_codigo,
        tiene_garantia: informe.has_garantia.unwrap_or(0) == 1,
    };

    // Generar PDF
    let generator = PdfGenerator::new();
    generator.generate_informe_pdf(pdf_data).await
}