use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use printpdf::*;

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

        // Configurar fuente
        let font = doc.add_builtin_font(BuiltinFont::HelveticaBold)
            .map_err(|e| format!("Error cargando fuente: {}", e))?;

        // Título
        current_layer.use_text(&format!("INFORME TÉCNICO {}", data.informe_codigo), 16.0, Mm(20.0), Mm(270.0), &font);
        
        // Información del cliente
        current_layer.use_text("INFORMACIÓN DEL CLIENTE", 12.0, Mm(20.0), Mm(250.0), &font);
        current_layer.use_text(&format!("Nombre: {}", data.cliente.nombre), 10.0, Mm(20.0), Mm(240.0), &font);
        
        if let Some(email) = &data.cliente.email {
            current_layer.use_text(&format!("Email: {}", email), 10.0, Mm(20.0), Mm(230.0), &font);
        }

        // Información del equipo
        current_layer.use_text("INFORMACIÓN DEL EQUIPO", 12.0, Mm(20.0), Mm(210.0), &font);
        
        if let Some(marca) = &data.equipo.marca {
            current_layer.use_text(&format!("Marca: {}", marca), 10.0, Mm(20.0), Mm(200.0), &font);
        }
        
        if let Some(modelo) = &data.equipo.modelo {
            current_layer.use_text(&format!("Modelo: {}", modelo), 10.0, Mm(20.0), Mm(190.0), &font);
        }

        // Diagnóstico
        current_layer.use_text("DIAGNÓSTICO", 12.0, Mm(20.0), Mm(170.0), &font);
        current_layer.use_text(&data.diagnostico, 10.0, Mm(20.0), Mm(160.0), &font);

        // Técnico responsable
        current_layer.use_text(&format!("Técnico: {}", data.tecnico_responsable), 10.0, Mm(20.0), Mm(140.0), &font);

        // Fecha
        current_layer.use_text(&format!("Fecha: {}", data.fecha.format("%d/%m/%Y")), 10.0, Mm(20.0), Mm(120.0), &font);

        // Garantía
        let garantia = if data.tiene_garantia { "CON GARANTÍA" } else { "SIN GARANTÍA" };
        current_layer.use_text(&format!("Estado: {}", garantia), 10.0, Mm(20.0), Mm(110.0), &font);

        // Generar PDF
        doc.save_to_bytes()
            .map_err(|e| format!("Error generando PDF: {}", e))
    }

    /// Generar PDF básico usando printpdf
    async fn generate_basic_pdf(&self, data: &CotizacionPdfData) -> Result<Vec<u8>, String> {
        let (doc, page1, layer1) = PdfDocument::new("Toscanini - Cotización", Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Configurar fuente
        let font = doc.add_builtin_font(BuiltinFont::HelveticaBold)
            .map_err(|e| format!("Error cargando fuente: {}", e))?;

        // Título
        current_layer.use_text(&format!("COTIZACIÓN {}", data.cotizacion_codigo), 16.0, Mm(20.0), Mm(270.0), &font);
        
        // Información del cliente
        current_layer.use_text("INFORMACIÓN DEL CLIENTE", 12.0, Mm(20.0), Mm(250.0), &font);
        current_layer.use_text(&format!("Nombre: {}", data.cliente.nombre), 10.0, Mm(20.0), Mm(240.0), &font);
        
        if let Some(email) = &data.cliente.email {
            current_layer.use_text(&format!("Email: {}", email), 10.0, Mm(20.0), Mm(230.0), &font);
        }
        
        if let Some(telefono) = &data.cliente.telefono {
            current_layer.use_text(&format!("Teléfono: {}", telefono), 10.0, Mm(20.0), Mm(220.0), &font);
        }

        // Información del equipo
        current_layer.use_text("INFORMACIÓN DEL EQUIPO", 12.0, Mm(20.0), Mm(200.0), &font);
        
        if let Some(marca) = &data.equipo.marca {
            current_layer.use_text(&format!("Marca: {}", marca), 10.0, Mm(20.0), Mm(190.0), &font);
        }
        
        if let Some(modelo) = &data.equipo.modelo {
            current_layer.use_text(&format!("Modelo: {}", modelo), 10.0, Mm(20.0), Mm(180.0), &font);
        }

        // Costos
        current_layer.use_text("COSTOS", 12.0, Mm(20.0), Mm(160.0), &font);
        
        if let Some(revision) = data.costo_revision {
            current_layer.use_text(&format!("Costo de Revisión: ${}", revision), 10.0, Mm(20.0), Mm(150.0), &font);
        }
        
        if let Some(reparacion) = data.costo_reparacion {
            current_layer.use_text(&format!("Costo de Reparación: ${}", reparacion), 10.0, Mm(20.0), Mm(140.0), &font);
        }
        
        current_layer.use_text(&format!("TOTAL: ${}", data.costo_total), 12.0, Mm(20.0), Mm(120.0), &font);

        // Fecha
        current_layer.use_text(&format!("Fecha: {}", data.fecha.format("%d/%m/%Y")), 10.0, Mm(20.0), Mm(100.0), &font);

        // Estado
        let estado = if data.is_aprobada { "APROBADA" } else { "PENDIENTE" };
        current_layer.use_text(&format!("Estado: {}", estado), 10.0, Mm(20.0), Mm(90.0), &font);

        // Generar PDF
        doc.save_to_bytes()
            .map_err(|e| format!("Error generando PDF: {}", e))
    }

    /// Generar HTML para cotización
    fn generate_cotizacion_html(&self, data: &CotizacionPdfData) -> Result<String, String> {
        let estado = if data.is_aprobada { "APROBADA" } else { "PENDIENTE" };
        let estado_color = if data.is_aprobada { "#28a745" } else { "#ffc107" };

        let piezas_html = if data.piezas.is_empty() {
            "<tr><td colspan='5' style='text-align: center; font-style: italic; color: #666;'>No se requieren piezas adicionales</td></tr>".to_string()
        } else {
            data.piezas.iter().map(|pieza| {
                format!(
                    "<tr>
                        <td>{}</td>
                        <td>{}</td>
                        <td style='text-align: center;'>{}</td>
                        <td style='text-align: right;'>${}</td>
                        <td style='text-align: right;'>${}</td>
                    </tr>",
                    pieza.nombre,
                    pieza.marca.as_deref().unwrap_or("N/A"),
                    pieza.cantidad,
                    pieza.precio_unitario,
                    pieza.subtotal
                )
            }).collect::<Vec<String>>().join("")
        };

        let total_piezas: i32 = data.piezas.iter().map(|p| p.subtotal).sum();

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Cotización - Toscanini</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            margin: 0;
            padding: 0;
        }}
        .header {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 30px;
            text-align: center;
        }}
        .company-name {{
            font-size: 28px;
            font-weight: bold;
            margin-bottom: 5px;
        }}
        .company-tagline {{
            font-size: 14px;
            opacity: 0.9;
        }}
        .document-title {{
            background: #f8f9fa;
            padding: 20px;
            text-align: center;
            border-bottom: 3px solid #007bff;
        }}
        .document-title h1 {{
            margin: 0;
            color: #007bff;
            font-size: 24px;
        }}
        .status-badge {{
            background: {};
            color: white;
            padding: 5px 15px;
            border-radius: 20px;
            font-size: 12px;
            font-weight: bold;
            margin-top: 10px;
            display: inline-block;
        }}
        .content {{
            padding: 30px;
        }}
        .info-section {{
            display: flex;
            justify-content: space-between;
            margin-bottom: 30px;
        }}
        .info-box {{
            background: #f8f9fa;
            border: 1px solid #dee2e6;
            border-radius: 8px;
            padding: 20px;
            width: 48%;
        }}
        .info-box h3 {{
            margin-top: 0;
            color: #495057;
            border-bottom: 2px solid #007bff;
            padding-bottom: 5px;
        }}
        .info-item {{
            margin-bottom: 8px;
        }}
        .info-label {{
            font-weight: bold;
            color: #495057;
        }}
        .section {{
            margin-bottom: 30px;
        }}
        .section h3 {{
            background: #007bff;
            color: white;
            padding: 10px 15px;
            margin: 0 0 15px 0;
            border-radius: 5px;
        }}
        .equipment-info {{
            background: #e8f4f8;
            border-left: 4px solid #17a2b8;
            padding: 15px;
            margin-bottom: 20px;
        }}
        .technical-report {{
            background: #fff3cd;
            border: 1px solid #ffeaa7;
            border-radius: 5px;
            padding: 20px;
            margin-bottom: 20px;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin-bottom: 20px;
        }}
        th, td {{
            border: 1px solid #dee2e6;
            padding: 12px;
            text-align: left;
        }}
        th {{
            background: #f8f9fa;
            font-weight: bold;
            color: #495057;
        }}
        .costs-summary {{
            background: #e8f5e8;
            border: 2px solid #28a745;
            border-radius: 8px;
            padding: 20px;
            margin-top: 20px;
        }}
        .cost-row {{
            display: flex;
            justify-content: space-between;
            margin-bottom: 10px;
            padding: 5px 0;
        }}
        .cost-label {{
            font-weight: bold;
        }}
        .cost-value {{
            font-weight: bold;
        }}
        .total-row {{
            border-top: 2px solid #28a745;
            padding-top: 10px;
            font-size: 18px;
            color: #28a745;
        }}
        .footer {{
            background: #f8f9fa;
            padding: 20px;
            text-align: center;
            border-top: 1px solid #dee2e6;
            font-size: 12px;
            color: #6c757d;
        }}
    </style>
</head>
<body>
    <div class="header">
        <div class="company-name">{}</div>
        <div class="company-tagline">Servicio Técnico Especializado</div>
        <div>{}</div>
        <div>{}</div>
    </div>

        <div class="document-title">
            <h1>COTIZACIÓN {}</h1>
            <div class="status-badge">Estado: {}</div>
            <div style="margin-top: 10px; font-size: 14px;">
                Fecha: {}
            </div>
        </div>    <div class="content">
        <div class="info-section">
            <div class="info-box">
                <h3>Información del Cliente</h3>
                <div class="info-item">
                    <span class="info-label">Nombre:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">Email:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">Teléfono:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">Dirección:</span> {}
                </div>
            </div>
            
            <div class="info-box">
                <h3>Información del Equipo</h3>
                <div class="info-item">
                    <span class="info-label">Marca:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">Modelo:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">Tipo:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">N° Serie:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">Ubicación:</span> {}
                </div>
            </div>
        </div>

        <div class="section">
            <h3>Informe Técnico</h3>
            <div class="technical-report">
                {}
            </div>
        </div>

        <div class="section">
            <h3>Detalle de Piezas</h3>
            <table>
                <thead>
                    <tr>
                        <th>Pieza</th>
                        <th>Marca</th>
                        <th>Cantidad</th>
                        <th>Precio Unitario</th>
                        <th>Subtotal</th>
                    </tr>
                </thead>
                <tbody>
                    {}
                </tbody>
            </table>
        </div>

        <div class="costs-summary">
            <h3 style="margin-top: 0; color: #28a745;">Resumen de Costos</h3>
            {}
            {}
            <div class="cost-row">
                <span class="cost-label">Subtotal Piezas:</span>
                <span class="cost-value">${}</span>
            </div>
            <div class="cost-row total-row">
                <span class="cost-label">TOTAL:</span>
                <span class="cost-value">${}</span>
            </div>
        </div>
    </div>

    <div class="footer">
        <p><strong>Toscanini - Servicio Técnico Especializado</strong></p>
        <p>Esta cotización es válida por 30 días desde la fecha de emisión.</p>
        <p>Para consultas o aprobación de la cotización, contáctenos a través de los medios indicados.</p>
    </div>
</body>
</html>"#,
            // CSS estado color
            estado_color,
            // Header empresa
            data.empresa.nombre,
            data.empresa.telefono.as_deref().unwrap_or(""),
            data.empresa.email.as_deref().unwrap_or(""),
            // Title
            data.cotizacion_codigo,
            estado,
            format!("{} | {}", data.fecha.format("%d/%m/%Y").to_string(), data.orden_codigo.as_deref().unwrap_or("Sin orden asociada")),
            // Cliente info
            data.cliente.nombre,
            data.cliente.email.as_deref().unwrap_or("No especificado"),
            data.cliente.telefono.as_deref().unwrap_or("No especificado"),
            data.cliente.direccion.as_deref().unwrap_or("No especificada"),
            // Equipo info
            data.equipo.marca.as_deref().unwrap_or("No especificada"),
            data.equipo.modelo.as_deref().unwrap_or("No especificado"),
            data.equipo.tipo.as_deref().unwrap_or("No especificado"),
            data.equipo.numero_serie.as_deref().unwrap_or("No especificado"),
            data.equipo.ubicacion.as_deref().unwrap_or("No especificada"),
            // Informe técnico
            data.informe_tecnico,
            // Piezas
            piezas_html,
            // Costos
            if let Some(revision) = data.costo_revision {
                format!("<div class=\"cost-row\"><span class=\"cost-label\">Costo de Revisión:</span><span class=\"cost-value\">${}</span></div>", revision)
            } else { String::new() },
            if let Some(reparacion) = data.costo_reparacion {
                format!("<div class=\"cost-row\"><span class=\"cost-label\">Costo de Reparación:</span><span class=\"cost-value\">${}</span></div>", reparacion)
            } else { String::new() },
            total_piezas,
            data.costo_total
        );

        Ok(html)
    }

    /// Generar HTML para informe técnico
    fn generate_informe_html(&self, data: &InformePdfData) -> Result<String, String> {
        let garantia_badge = if data.tiene_garantia { 
            "<span style='background: #28a745; color: white; padding: 3px 8px; border-radius: 12px; font-size: 11px;'>CON GARANTÍA</span>"
        } else { 
            "<span style='background: #dc3545; color: white; padding: 3px 8px; border-radius: 12px; font-size: 11px;'>SIN GARANTÍA</span>"
        };

        let piezas_html = if data.piezas.is_empty() {
            "<tr><td colspan='5' style='text-align: center; font-style: italic; color: #666;'>No se utilizaron piezas en este servicio</td></tr>".to_string()
        } else {
            data.piezas.iter().map(|pieza| {
                format!(
                    "<tr>
                        <td>{}</td>
                        <td>{}</td>
                        <td style='text-align: center;'>{}</td>
                        <td style='text-align: right;'>${}</td>
                        <td style='text-align: right;'>${}</td>
                    </tr>",
                    pieza.nombre,
                    pieza.marca.as_deref().unwrap_or("N/A"),
                    pieza.cantidad,
                    pieza.precio_unitario,
                    pieza.subtotal
                )
            }).collect::<Vec<String>>().join("")
        };

        let total_piezas: i32 = data.piezas.iter().map(|p| p.subtotal).sum();

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Informe Técnico - Toscanini</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            margin: 0;
            padding: 0;
        }}
        .header {{
            background: linear-gradient(135deg, #28a745 0%, #20c997 100%);
            color: white;
            padding: 30px;
            text-align: center;
        }}
        .company-name {{
            font-size: 28px;
            font-weight: bold;
            margin-bottom: 5px;
        }}
        .company-tagline {{
            font-size: 14px;
            opacity: 0.9;
        }}
        .document-title {{
            background: #f8f9fa;
            padding: 20px;
            text-align: center;
            border-bottom: 3px solid #28a745;
        }}
        .document-title h1 {{
            margin: 0;
            color: #28a745;
            font-size: 24px;
        }}
        .content {{
            padding: 30px;
        }}
        .info-section {{
            display: flex;
            justify-content: space-between;
            margin-bottom: 30px;
        }}
        .info-box {{
            background: #f8f9fa;
            border: 1px solid #dee2e6;
            border-radius: 8px;
            padding: 20px;
            width: 48%;
        }}
        .info-box h3 {{
            margin-top: 0;
            color: #495057;
            border-bottom: 2px solid #28a745;
            padding-bottom: 5px;
        }}
        .info-item {{
            margin-bottom: 8px;
        }}
        .info-label {{
            font-weight: bold;
            color: #495057;
        }}
        .section {{
            margin-bottom: 30px;
        }}
        .section h3 {{
            background: #28a745;
            color: white;
            padding: 10px 15px;
            margin: 0 0 15px 0;
            border-radius: 5px;
        }}
        .diagnostic-box {{
            background: #fff3cd;
            border-left: 4px solid #ffc107;
            padding: 20px;
            margin-bottom: 20px;
        }}
        .recommendations-box {{
            background: #d1ecf1;
            border-left: 4px solid #17a2b8;
            padding: 20px;
            margin-bottom: 20px;
        }}
        .solution-box {{
            background: #d4edda;
            border-left: 4px solid #28a745;
            padding: 20px;
            margin-bottom: 20px;
        }}
        .technician-info {{
            background: #e2e3e5;
            border: 1px solid #d6d8db;
            border-radius: 5px;
            padding: 15px;
            margin-bottom: 20px;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin-bottom: 20px;
        }}
        th, td {{
            border: 1px solid #dee2e6;
            padding: 12px;
            text-align: left;
        }}
        th {{
            background: #f8f9fa;
            font-weight: bold;
            color: #495057;
        }}
        .parts-summary {{
            background: #f8f9fa;
            border: 1px solid #dee2e6;
            border-radius: 8px;
            padding: 15px;
            margin-top: 10px;
            text-align: right;
        }}
        .footer {{
            background: #f8f9fa;
            padding: 20px;
            text-align: center;
            border-top: 1px solid #dee2e6;
            font-size: 12px;
            color: #6c757d;
        }}
        .signature-section {{
            margin-top: 40px;
            border-top: 1px solid #dee2e6;
            padding-top: 20px;
        }}
        .signature-box {{
            border: 1px solid #dee2e6;
            height: 80px;
            margin-top: 10px;
            background: #f8f9fa;
        }}
    </style>
</head>
<body>
    <div class="header">
        <div class="company-name">{}</div>
        <div class="company-tagline">Servicio Técnico Especializado</div>
        <div>{}</div>
        <div>{}</div>
    </div>

        <div class="document-title">
            <h1>INFORME TÉCNICO {}</h1>
            <div style="margin-top: 10px; font-size: 14px;">
                Fecha: {}
            </div>
        </div>    <div class="content">
        <div class="info-section">
            <div class="info-box">
                <h3>Información del Cliente</h3>
                <div class="info-item">
                    <span class="info-label">Nombre:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">Email:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">Teléfono:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">Dirección:</span> {}
                </div>
            </div>
            
            <div class="info-box">
                <h3>Información del Equipo</h3>
                <div class="info-item">
                    <span class="info-label">Marca:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">Modelo:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">Tipo:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">N° Serie:</span> {}
                </div>
                <div class="info-item">
                    <span class="info-label">Ubicación:</span> {}
                </div>
            </div>
        </div>

        <div class="section">
            <h3>Diagnóstico del Problema</h3>
            <div class="diagnostic-box">
                <strong>Diagnóstico:</strong><br>
                {}
            </div>
        </div>

        {}

        {}

        <div class="section">
            <h3>Técnico Responsable</h3>
            <div class="technician-info">
                <strong>Técnico a cargo:</strong> {}
            </div>
        </div>

        <div class="section">
            <h3>Piezas Utilizadas</h3>
            <table>
                <thead>
                    <tr>
                        <th>Pieza</th>
                        <th>Marca</th>
                        <th>Cantidad</th>
                        <th>Precio Unitario</th>
                        <th>Subtotal</th>
                    </tr>
                </thead>
                <tbody>
                    {}
                </tbody>
            </table>
            {}
        </div>

        <div class="signature-section">
            <h3>Conformidad del Cliente</h3>
            <p>Con mi firma certifico que he recibido el equipo en las condiciones descritas y que el trabajo realizado es satisfactorio:</p>
            
            <div style="display: flex; justify-content: space-between; margin-top: 30px;">
                <div style="width: 45%;">
                    <p><strong>Firma del Cliente:</strong></p>
                    <div class="signature-box"></div>
                    <p style="text-align: center; margin-top: 5px; font-size: 12px;">Firma</p>
                </div>
                <div style="width: 45%;">
                    <p><strong>Firma del Técnico:</strong></p>
                    <div class="signature-box"></div>
                    <p style="text-align: center; margin-top: 5px; font-size: 12px;">Firma y RUT</p>
                </div>
            </div>
        </div>
    </div>

    <div class="footer">
        <p><strong>Toscanini - Servicio Técnico Especializado</strong></p>
        <p>Este documento certifica los trabajos realizados en el equipo del cliente.</p>
        <p>Para consultas sobre este informe, contáctenos a través de los medios indicados.</p>
    </div>
</body>
</html>"#,
            // Header empresa
            data.empresa.nombre,
            data.empresa.telefono.as_deref().unwrap_or(""),
            data.empresa.email.as_deref().unwrap_or(""),
            // Title
            data.informe_codigo,
            format!("{} | {} | {}", data.fecha.format("%d/%m/%Y").to_string(), data.orden_codigo.as_deref().unwrap_or("Sin orden asociada"), garantia_badge),
            // Cliente info
            data.cliente.nombre,
            data.cliente.email.as_deref().unwrap_or("No especificado"),
            data.cliente.telefono.as_deref().unwrap_or("No especificado"),
            data.cliente.direccion.as_deref().unwrap_or("No especificada"),
            // Equipo info
            data.equipo.marca.as_deref().unwrap_or("No especificada"),
            data.equipo.modelo.as_deref().unwrap_or("No especificado"),
            data.equipo.tipo.as_deref().unwrap_or("No especificado"),
            data.equipo.numero_serie.as_deref().unwrap_or("No especificado"),
            data.equipo.ubicacion.as_deref().unwrap_or("No especificada"),
            // Diagnóstico
            data.diagnostico,
            // Recomendaciones (opcional)
            if let Some(ref recomendaciones) = data.recomendaciones {
                format!(
                    r#"<div class="section">
                        <h3>Recomendaciones</h3>
                        <div class="recommendations-box">
                            <strong>Recomendaciones:</strong><br>
                            {}
                        </div>
                    </div>"#,
                    recomendaciones
                )
            } else { String::new() },
            // Solución aplicada (opcional)
            if let Some(ref solucion) = data.solucion_aplicada {
                format!(
                    r#"<div class="section">
                        <h3>Solución Aplicada</h3>
                        <div class="solution-box">
                            <strong>Trabajo realizado:</strong><br>
                            {}
                        </div>
                    </div>"#,
                    solucion
                )
            } else { String::new() },
            // Técnico
            data.tecnico_responsable,
            // Piezas
            piezas_html,
            // Summary de piezas
            if !data.piezas.is_empty() {
                format!(r#"<div class="parts-summary">
                    <strong>Total en piezas: ${}</strong>
                </div>"#, total_piezas)
            } else { String::new() }
        );

        Ok(html)
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