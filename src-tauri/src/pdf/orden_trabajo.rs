use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use printpdf::{PdfDocument, BuiltinFont, Color, Rgb, Mm};
use crate::pdf::common::{EmpresaInfo, ClienteInfo, EquipoInfo, wrap_text};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrdenTrabajoPdfData {
    pub orden_codigo: String,
    pub fecha: DateTime<Utc>,
    pub fecha_finalizacion: Option<DateTime<Utc>>,
    pub empresa: EmpresaInfo,
    pub cliente: ClienteInfo,
    pub equipo: EquipoInfo,
    pub orden_desc: Option<String>,
    pub pre_informe: String,
    pub prioridad: Option<String>,
    pub estado: Option<String>,
    pub has_garantia: bool,
    pub creador_nombre: Option<String>,
    pub cotizacion_codigo: Option<String>,
    pub informe_codigo: Option<String>,
}

pub struct OrdenTrabajoPdfGenerator;

impl OrdenTrabajoPdfGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generar PDF de orden de trabajo
    pub async fn generate_orden_trabajo_pdf(&self, data: OrdenTrabajoPdfData) -> Result<Vec<u8>, String> {
        self.generate_basic_pdf(&data).await
    }

    async fn generate_basic_pdf(&self, data: &OrdenTrabajoPdfData) -> Result<Vec<u8>, String> {
        let (doc, page1, layer1) = PdfDocument::new("Toscanini - Orden de Trabajo", Mm(210.0), Mm(297.0), "Layer 1");
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
        current_layer.use_text("ORDEN DE TRABAJO", 16.0, Mm(20.0), Mm(280.0), &font_bold);
        
        // === NÚMERO DE ORDEN (derecha, grande y destacado) ===
        current_layer.use_text(&data.orden_codigo, 24.0, Mm(140.0), Mm(280.0), &font_bold);
        current_layer.use_text("Orden", 9.0, Mm(140.0), Mm(270.0), &font_regular);
        
        // === FECHAS (derecha, debajo del número) ===
        current_layer.set_fill_color(gray_color.clone());
        current_layer.use_text(&format!("Fecha de Creación: {}", data.fecha.format("%d/%m/%Y")), 9.0, Mm(140.0), Mm(260.0), &font_regular);
        
        if let Some(fecha_fin) = data.fecha_finalizacion {
            current_layer.use_text(&format!("Fecha de Finalización: {}", fecha_fin.format("%d/%m/%Y")), 9.0, Mm(140.0), Mm(253.0), &font_regular);
        }
        
        current_layer.use_text(&format!("Documento Impreso: {}", chrono::Utc::now().format("%d/%m/%Y")), 9.0, Mm(140.0), Mm(246.0), &font_regular);
        
        // Estado de la orden
        current_layer.set_fill_color(black_color.clone());
        if let Some(estado) = &data.estado {
            let estado_text = format!("Estado: {}", estado.to_uppercase());
            current_layer.use_text(&estado_text, 9.0, Mm(140.0), Mm(239.0), &font_bold);
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
            current_layer.use_text("Email", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
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


        // === INFORMACIÓN DE LA ORDEN ===
        y_pos -= 10.0;
        current_layer.use_text("Información de la Orden", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
        y_pos -= 6.0;
        
        // Prioridad
        if let Some(prioridad) = &data.prioridad {
            current_layer.use_text("Prioridad", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
            current_layer.use_text(&prioridad.to_uppercase(), 10.0, Mm(70.0), Mm(y_pos), &font_regular);
            y_pos -= 6.0;
        }
        
        // Garantía
        current_layer.use_text("Garantía", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
        if data.has_garantia {
            current_layer.use_text("SÍ", 10.0, Mm(70.0), Mm(y_pos), &font_bold);
        } else {
            current_layer.use_text("NO", 10.0, Mm(70.0), Mm(y_pos), &font_regular);
        }
        y_pos -= 6.0;
        
        // Creador
        if let Some(creador) = &data.creador_nombre {
            current_layer.use_text("Creado por", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
            current_layer.use_text(creador, 10.0, Mm(70.0), Mm(y_pos), &font_regular);
            y_pos -= 6.0;
        }
        
        // Documentos asociados
        if data.cotizacion_codigo.is_some() || data.informe_codigo.is_some() {
            y_pos -= 6.0;
            current_layer.use_text("Documentos Asociados", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
            y_pos -= 6.0;
            
            if let Some(cotizacion) = &data.cotizacion_codigo {
                current_layer.use_text("Cotización", 10.0, Mm(25.0), Mm(y_pos), &font_bold);
                current_layer.use_text(cotizacion, 10.0, Mm(70.0), Mm(y_pos), &font_regular);
                y_pos -= 6.0;
            }
            
            if let Some(informe) = &data.informe_codigo {
                current_layer.use_text("Informe", 10.0, Mm(25.0), Mm(y_pos), &font_bold);
                current_layer.use_text(informe, 10.0, Mm(70.0), Mm(y_pos), &font_regular);
                y_pos -= 6.0;
            }
        }

        // === DESCRIPCIÓN DE LA ORDEN ===
        y_pos -= 10.0;
        current_layer.use_text("Descripción de la Orden", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
        y_pos -= 6.0;
        
        if let Some(desc) = &data.orden_desc {
            if !desc.trim().is_empty() {
                let desc_lines = wrap_text(desc, 85);
                for line in desc_lines.iter().take(5) {
                    current_layer.use_text(line, 10.0, Mm(25.0), Mm(y_pos), &font_regular);
                    y_pos -= 6.0;
                }
            }
        } else {
            current_layer.use_text("Sin descripción adicional", 10.0, Mm(25.0), Mm(y_pos), &font_regular);
            y_pos -= 6.0;
        }

        // === PRE-INFORME / DIAGNÓSTICO INICIAL ===
        y_pos -= 10.0;
        current_layer.use_text("Diagnóstico Inicial / Pre-Informe", 11.0, Mm(20.0), Mm(y_pos), &font_bold);
        y_pos -= 6.0;
        
        if !data.pre_informe.trim().is_empty() {
            let pre_informe_lines = wrap_text(&data.pre_informe, 85);
            for line in pre_informe_lines.iter().take(10) {
                current_layer.use_text(line, 10.0, Mm(25.0), Mm(y_pos), &font_regular);
                y_pos -= 6.0;
            }
        } else {
            current_layer.use_text("Sin diagnóstico inicial registrado", 10.0, Mm(25.0), Mm(y_pos), &font_regular);
            y_pos -= 6.0;
        }

        // Generar PDF
        doc.save_to_bytes()
            .map_err(|e| format!("Error generando PDF: {}", e))
    }
}

