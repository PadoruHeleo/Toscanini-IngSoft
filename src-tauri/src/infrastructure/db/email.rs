use crate::models::email::EmailConfig;
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use crate::models::ordenes_trabajo::OrdenTrabajo;
use crate::models::cotizacion::Cotizacion;
use crate::models::informe::Informe;
use crate::models::equipos::Equipo;

pub struct EmailService {
    mailer: SmtpTransport,
    from_email: String,
    from_name: String,
}

impl EmailService {
    pub fn new(email_config: &EmailConfig) -> Result<Self, String> {
        // Validar configuración
        if email_config.smtp_server.is_empty() || email_config.smtp_user.is_empty() || email_config.smtp_password.is_empty() {
            return Err("Configuración de email incompleta".to_string());
        }
        
        // Configurar transporte SMTP
        let creds = Credentials::new(
            email_config.smtp_user.clone(),
            email_config.smtp_password.clone(),
        );
        
        let mailer = SmtpTransport::relay(&email_config.smtp_server)
            .map_err(|e| format!("Error configurando servidor SMTP: {}", e))?
            .credentials(creds)
            .port(email_config.smtp_port)
            .build();
            
        Ok(Self {
            mailer,
            from_email: email_config.sender_email.clone(),
            from_name: email_config.sender_name.clone(),
        })
    }
    
    fn build_email(&self, to: &str, subject: &str, html_content: &str, attachment: Option<(&[u8], &str)>) -> Result<Message, String> {
        let from = format!("{} <{}>", self.from_name, self.from_email);
        
        let builder = Message::builder()
            .from(from.parse().map_err(|e| format!("Error en remitente: {}", e))?)
            .to(to.parse().map_err(|e| format!("Error en destinatario: {}", e))?)
            .subject(subject);
            
        let mut multipart = MultiPart::mixed()
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(html_content.to_string())
            );
            
        // Agregar adjunto si existe (para PDF)
        if let Some((data, filename)) = attachment {
            let content_type = if filename.ends_with(".pdf") {
                header::ContentType::parse("application/pdf").unwrap()
            } else {
                header::ContentType::parse("application/octet-stream").unwrap()
            };
            
            multipart = multipart.singlepart(
                SinglePart::builder()
                    .header(content_type)
                    .header(header::ContentDisposition::attachment(filename))
                    .body(data.to_vec())
            );
        }
            
        builder
            .multipart(multipart)
            .map_err(|e| format!("Error construyendo email: {}", e))
    }
    
    // Método interno para enviar el email
    pub async fn send_email_internal(&self, to: &str, subject: &str, html_content: &str, attachment: Option<(&[u8], &str)>) -> Result<(), String> {
        let email = self.build_email(to, subject, html_content, attachment)?;
        
        // Enviar email (bloqueante, pero ejecutado en contexto async)
        // Note: lettre's send is synchronous unless using AsyncSmtpTransport. 
        // Here we are using SmtpTransport which is sync. 
        // Ideally we should wrap this in spawn_blocking or use async transport.
        // For now keeping as is to match original code.
        match self.mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error enviando email SMTP: {}", e)),
        }
    }

    // Enviar correo de restablecimiento de contraseña
    pub async fn send_password_reset_email(&self, to_email: &str, code: &str, user_name: &str) -> Result<String, String> {
        let subject = "Restablecimiento de Contraseña - Toscanini";
        
        let html_content = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <body>
                <h2>Restablecimiento de Contraseña</h2>
                <p>Hola {},</p>
                <p>Has solicitado restablecer tu contraseña. Utiliza el siguiente código para continuar:</p>
                <h3 style="background-color: #f0f0f0; padding: 10px; text-align: center; letter-spacing: 5px;">{}</h3>
                <p>Este código expirará en 15 minutos.</p>
                <p>Si no solicitaste esto, puedes ignorar este correo.</p>
            </body>
            </html>
            "#,
            user_name,
            code
        );
        
        self.send_email_internal(to_email, subject, &html_content, None).await?;
        Ok(format!("Correo de restablecimiento enviado a {}", to_email))
    }

    // Enviar correo con nueva contraseña temporal
    pub async fn send_password_email(&self, to_email: &str, user_name: &str, temp_pass: &str) -> Result<String, String> {
        let subject = "Nueva Contraseña - Toscanini";
        
        let html_content = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <body>
                <h2>Nueva Contraseña</h2>
                <p>Hola {},</p>
                <p>Tu contraseña ha sido restablecida exitosamente.</p>
                <p>Tu nueva contraseña temporal es:</p>
                <h3 style="background-color: #f0f0f0; padding: 10px; text-align: center;">{}</h3>
                <p>Te recomendamos cambiar esta contraseña inmediatamente después de iniciar sesión.</p>
            </body>
            </html>
            "#,
            user_name,
            temp_pass
        );
        
        self.send_email_internal(to_email, subject, &html_content, None).await?;
        Ok(format!("Correo de contraseña enviado a {}", to_email))
    }
    
    // Plantilla para notificación de orden de trabajo
    pub async fn send_orden_trabajo_notification(&self, orden: &OrdenTrabajo, equipo: &Equipo, cliente_nombre: &str) -> Result<String, String> {
        let _subject = format!("Orden de Trabajo Recibida - OT-{}", orden.orden_codigo.as_deref().unwrap_or("???"));
        
        let _html_content = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; border: 1px solid #ddd; border-radius: 5px; }}
                    .header {{ background-color: #f8f9fa; padding: 10px; text-align: center; border-bottom: 1px solid #ddd; }}
                    .content {{ padding: 20px 0; }}
                    .footer {{ margin-top: 20px; font-size: 12px; color: #777; text-align: center; border-top: 1px solid #ddd; padding-top: 10px; }}
                    .info-item {{ margin-bottom: 10px; }}
                    .label {{ font-weight: bold; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h2>Orden de Trabajo Recibida</h2>
                    </div>
                    <div class="content">
                        <p>Estimado/a {},</p>
                        <p>Hemos recibido su equipo y generado una orden de trabajo.</p>
                        
                        <div class="info-item">
                            <span class="label">Orden:</span> {}
                        </div>
                        <div class="info-item">
                            <span class="label">Equipo:</span> {} {}
                        </div>
                        <div class="info-item">
                            <span class="label">Modelo:</span> {}
                        </div>
                        <div class="info-item">
                            <span class="label">Serie:</span> {}
                        </div>
                        <div class="info-item">
                            <span class="label">Descripción:</span> {}
                        </div>
                        
                        <p>Le notificaremos cuando tengamos un diagnóstico o cotización disponible.</p>
                    </div>
                    <div class="footer">
                        <p>Este es un mensaje automático, por favor no responder.</p>
                        <p>Toscanini - Servicio Técnico Especializado</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            cliente_nombre,
            orden.orden_codigo.as_deref().unwrap_or("N/A"),
            equipo.equipo_marca.as_deref().unwrap_or(""),
            equipo.equipo_tipo.as_deref().unwrap_or(""),
            equipo.equipo_modelo.as_deref().unwrap_or("N/A"),
            equipo.numero_serie.as_deref().unwrap_or("N/A"),
            orden.orden_desc.as_deref().unwrap_or("Sin descripción")
        );
        
        Ok(format!("Email preparado para {}", cliente_nombre))
    }

    // Enviar orden de trabajo al cliente (implementación real)
    pub async fn send_orden_trabajo_cliente(&self, to_email: &str, cliente_nombre: &str, orden: &OrdenTrabajo, equipo: &Equipo) -> Result<(), String> {
        let subject = format!("Nueva Orden de Trabajo - {}", orden.orden_codigo.as_deref().unwrap_or("???"));
        
        // Formatear fecha
        let fecha_creacion = orden.created_at
            .map(|d| d.format("%d/%m/%Y %H:%M").to_string())
            .unwrap_or_else(|| "Fecha desconocida".to_string());

        // Formatear garantía
        let garantia = if orden.has_garantia.unwrap_or(false) { "✅ Sí" } else { "❌ No" };

        // Formatear prioridad (color dot)
        let prioridad = orden.prioridad.as_deref().unwrap_or("Normal");
        let prioridad_color = match prioridad.to_lowercase().as_str() {
            "alta" | "urgente" | "critica" => "red",
            "media" => "orange",
            _ => "green"
        };

        let html_content = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; color: #333; line-height: 1.6; margin: 0; padding: 0; background-color: #f4f4f4; }}
                    .container {{ max-width: 600px; margin: 20px auto; background-color: #ffffff; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
                    .header {{ text-align: center; margin-bottom: 30px; }}
                    .header h1 {{ margin: 0; font-size: 28px; color: #2c3e50; letter-spacing: 1px; }}
                    .header p {{ margin: 5px 0 0; color: #7f8c8d; font-size: 14px; text-transform: uppercase; letter-spacing: 2px; }}
                    .title-bar {{ border-bottom: 2px solid #3498db; padding-bottom: 10px; margin-bottom: 20px; }}
                    .title {{ color: #3498db; font-size: 22px; font-weight: 600; margin: 0; }}
                    .intro {{ margin-bottom: 25px; color: #555; }}
                    .section {{ background-color: #f8f9fa; padding: 20px; border-radius: 6px; margin-bottom: 20px; border: 1px solid #e9ecef; }}
                    .section-title {{ font-weight: 700; font-size: 16px; margin-bottom: 15px; color: #2c3e50; border-bottom: 1px solid #dee2e6; padding-bottom: 5px; }}
                    .field {{ margin-bottom: 8px; font-size: 14px; }}
                    .label {{ font-weight: 600; color: #555; width: 140px; display: inline-block; }}
                    .value {{ color: #333; }}
                    .priority-dot {{ height: 10px; width: 10px; background-color: {prioridad_color}; border-radius: 50%; display: inline-block; margin-right: 6px; }}
                    .footer {{ text-align: center; font-size: 12px; color: #95a5a6; margin-top: 40px; border-top: 1px solid #eee; padding-top: 20px; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>Toscanini</h1>
                        <p>Servicio Técnico Especializado</p>
                    </div>

                    <div class="title-bar">
                        <h2 class="title">Nueva Orden de Trabajo Creada</h2>
                    </div>

                    <p class="intro">Se ha creado una nueva orden de trabajo en el sistema con los siguientes detalles:</p>

                    <div class="section">
                        <div class="section-title">Detalles de la Orden</div>
                        <div class="field"><span class="label">Código de Orden:</span> <span class="value">{orden_codigo}</span></div>
                        <div class="field"><span class="label">Descripción:</span> <span class="value">{orden_desc}</span></div>
                        <div class="field"><span class="label">Prioridad:</span> <span class="priority-dot"></span><span class="value">{prioridad}</span></div>
                        <div class="field"><span class="label">Estado:</span> <span class="value">{estado}</span></div>
                        <div class="field"><span class="label">Garantía:</span> <span class="value">{garantia}</span></div>
                        <div class="field"><span class="label">Fecha de Creación:</span> <span class="value">{fecha_creacion}</span></div>
                    </div>

                    <div class="section">
                        <div class="section-title">Información del Equipo</div>
                        <div class="field"><span class="label">Cliente:</span> <span class="value">{cliente_nombre}</span></div>
                        <div class="field"><span class="label">Número de Serie:</span> <span class="value">{numero_serie}</span></div>
                        <div class="field"><span class="label">Marca:</span> <span class="value">{marca}</span></div>
                        <div class="field"><span class="label">Modelo:</span> <span class="value">{modelo}</span></div>
                        <div class="field"><span class="label">Tipo:</span> <span class="value">{tipo}</span></div>
                        <div class="field"><span class="label">Ubicación:</span> <span class="value">{ubicacion}</span></div>
                    </div>

                    <div class="section">
                        <div class="section-title">Pre-informe</div>
                        <div class="value" style="white-space: pre-wrap;">{pre_informe}</div>
                    </div>
                    
                    <div class="footer">
                        <p>Este es un mensaje automático generado por el sistema Toscanini.</p>
                        <p>Por favor no responder a este correo.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            prioridad_color = prioridad_color,
            orden_codigo = orden.orden_codigo.as_deref().unwrap_or("N/A"),
            orden_desc = orden.orden_desc.as_deref().unwrap_or("Sin descripción"),
            prioridad = prioridad,
            estado = orden.estado.as_deref().unwrap_or("Recibido"),
            garantia = garantia,
            fecha_creacion = fecha_creacion,
            cliente_nombre = cliente_nombre,
            numero_serie = equipo.numero_serie.as_deref().unwrap_or("N/A"),
            marca = equipo.equipo_marca.as_deref().unwrap_or("N/A"),
            modelo = equipo.equipo_modelo.as_deref().unwrap_or("N/A"),
            tipo = equipo.equipo_tipo.as_deref().unwrap_or("N/A"),
            ubicacion = equipo.equipo_ubicacion.as_deref().unwrap_or("N/A"),
            pre_informe = orden.pre_informe.as_deref().unwrap_or("Sin observaciones iniciales")
        );
        
        self.send_email_internal(to_email, &subject, &html_content, None).await
    }

    // Enviar notificación interna a staff (admin/tecnico)
    pub async fn send_orden_trabajo_staff_notification(&self, to_email: &str, staff_nombre: &str, orden: &OrdenTrabajo, equipo: &Equipo, cliente_nombre: &str) -> Result<(), String> {
        let subject = format!("🔔 Nueva OT Asignada - {}", orden.orden_codigo.as_deref().unwrap_or("???"));
        
        // Formatear fecha
        let fecha_creacion = orden.created_at
            .map(|d| d.format("%d/%m/%Y %H:%M").to_string())
            .unwrap_or_else(|| "Fecha desconocida".to_string());

        // Formatear prioridad (color dot)
        let prioridad = orden.prioridad.as_deref().unwrap_or("Normal");
        let prioridad_color = match prioridad.to_lowercase().as_str() {
            "alta" | "urgente" | "critica" => "red",
            "media" => "orange",
            _ => "green"
        };

        let html_content = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; color: #333; line-height: 1.6; margin: 0; padding: 0; background-color: #f4f4f4; }}
                    .container {{ max-width: 600px; margin: 20px auto; background-color: #ffffff; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); border-left: 5px solid #e74c3c; }}
                    .header {{ margin-bottom: 20px; border-bottom: 1px solid #eee; padding-bottom: 10px; }}
                    .header h2 {{ margin: 0; color: #c0392b; font-size: 20px; }}
                    .meta {{ font-size: 12px; color: #7f8c8d; margin-top: 5px; }}
                    .section {{ margin-bottom: 15px; }}
                    .field {{ margin-bottom: 5px; font-size: 14px; }}
                    .label {{ font-weight: 700; color: #555; width: 120px; display: inline-block; }}
                    .priority-badge {{ background-color: {prioridad_color}; color: white; padding: 2px 8px; border-radius: 12px; font-size: 12px; font-weight: bold; }}
                    .footer {{ margin-top: 30px; font-size: 12px; color: #95a5a6; border-top: 1px solid #eee; padding-top: 10px; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h2>Nueva Orden de Trabajo Ingresada</h2>
                        <div class="meta">Notificación automática para Staff</div>
                    </div>

                    <p>Hola {staff_nombre}, se ha ingresado una nueva orden al sistema:</p>

                    <div class="section">
                        <div class="field"><span class="label">Orden:</span> <strong>{orden_codigo}</strong></div>
                        <div class="field"><span class="label">Prioridad:</span> <span class="priority-badge">{prioridad}</span></div>
                        <div class="field"><span class="label">Fecha:</span> {fecha_creacion}</div>
                        <div class="field"><span class="label">Cliente:</span> {cliente_nombre}</div>
                    </div>

                    <div class="section" style="background-color: #f9f9f9; padding: 15px; border-radius: 4px;">
                        <div class="field"><span class="label">Equipo:</span> {marca} {modelo}</div>
                        <div class="field"><span class="label">Serie:</span> {numero_serie}</div>
                        <div class="field"><span class="label">Problema:</span></div>
                        <div style="margin-top: 5px; font-style: italic;">"{orden_desc}"</div>
                    </div>

                    <div class="footer">
                        <p>Sistema Toscanini - Panel de Control</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            prioridad_color = prioridad_color,
            staff_nombre = staff_nombre,
            orden_codigo = orden.orden_codigo.as_deref().unwrap_or("N/A"),
            prioridad = prioridad,
            fecha_creacion = fecha_creacion,
            cliente_nombre = cliente_nombre,
            marca = equipo.equipo_marca.as_deref().unwrap_or(""),
            modelo = equipo.equipo_modelo.as_deref().unwrap_or(""),
            numero_serie = equipo.numero_serie.as_deref().unwrap_or("N/A"),
            orden_desc = orden.orden_desc.as_deref().unwrap_or("Sin descripción")
        );
        
        self.send_email_internal(to_email, &subject, &html_content, None).await
    }

    // Enviar cotización con PDF
    pub async fn send_cotizacion_email_with_pdf(
        &self, 
        to_email: &str, 
        cliente_nombre: &str, 
        cotizacion: &Cotizacion, 
        orden: &OrdenTrabajo,
        equipo: &Equipo,
        pdf_bytes: &[u8]
    ) -> Result<(), String> {
        let subject = format!("Cotización - {}", cotizacion.cotizacion_codigo.as_deref().unwrap_or("???"));
        let filename = format!("Cotizacion_{}.pdf", cotizacion.cotizacion_codigo.as_deref().unwrap_or("doc"));
        
        let html_content = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <body>
                <h2>Cotización Disponible</h2>
                <p>Estimado/a {},</p>
                <p>Adjuntamos la cotización para la reparación de su equipo.</p>
                <ul>
                    <li><strong>Equipo:</strong> {} {}</li>
                    <li><strong>Orden:</strong> {}</li>
                    <li><strong>Total:</strong> ${}</li>
                </ul>
                <p>Quedamos atentos a su aprobación.</p>
                <p>Atentamente,<br>Equipo Toscanini</p>
            </body>
            </html>
            "#,
            cliente_nombre,
            equipo.equipo_marca.as_deref().unwrap_or(""),
            equipo.equipo_modelo.as_deref().unwrap_or(""),
            orden.orden_codigo.as_deref().unwrap_or("N/A"),
            cotizacion.costo_total.unwrap_or(0)
        );
        
        self.send_email_internal(to_email, &subject, &html_content, Some((pdf_bytes, &filename))).await
    }

    // Enviar informe con PDF
    pub async fn send_informe_email_with_pdf(
        &self, 
        to_email: &str, 
        cliente_nombre: &str, 
        informe: &Informe, 
        orden: &OrdenTrabajo,
        equipo: &Equipo,
        pdf_bytes: &[u8]
    ) -> Result<(), String> {
        let subject = format!("Informe Técnico - {}", orden.orden_codigo.as_deref().unwrap_or("???"));
        let filename = format!("Informe_{}.pdf", orden.orden_codigo.as_deref().unwrap_or("doc"));
        
        let html_content = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <body>
                <h2>Informe Técnico Disponible</h2>
                <p>Estimado/a {},</p>
                <p>Su equipo está listo para retiro. Adjuntamos el informe técnico con los detalles del servicio realizado.</p>
                <ul>
                    <li><strong>Equipo:</strong> {} {}</li>
                    <li><strong>Orden:</strong> {}</li>
                    <li><strong>Diagnóstico:</strong> {}</li>
                </ul>
                <p>Puede pasar a retirar su equipo en nuestro horario de atención.</p>
                <p>Atentamente,<br>Equipo Toscanini</p>
            </body>
            </html>
            "#,
            cliente_nombre,
            equipo.equipo_marca.as_deref().unwrap_or(""),
            equipo.equipo_modelo.as_deref().unwrap_or(""),
            orden.orden_codigo.as_deref().unwrap_or("N/A"),
            informe.diagnostico.as_deref().unwrap_or("N/A")
        );
        
        self.send_email_internal(to_email, &subject, &html_content, Some((pdf_bytes, &filename))).await
    }
}

// --- Public Functions for DB Impl ---

pub async fn send_test_email(config: &EmailConfig, to_email: String) -> Result<String, String> {
    println!("📧 [db_impl] Iniciando prueba de envío de correo a: {}", to_email);
    
    let email_service = EmailService::new(config)?;
    
    let html_content = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <style>
            body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
            .container { max-width: 600px; margin: 0 auto; padding: 20px; border: 1px solid #ddd; border-radius: 5px; }
            .header { background-color: #4a90e2; color: white; padding: 10px; text-align: center; border-radius: 5px 5px 0 0; }
            .content { padding: 20px; }
            .footer { margin-top: 20px; font-size: 12px; color: #777; text-align: center; border-top: 1px solid #ddd; padding-top: 10px; }
        </style>
    </head>
    <body>
        <div class="container">
            <div class="header">
                <h2>Prueba de Correo</h2>
            </div>
            <div class="content">
                <p>Hola,</p>
                <p>Este es un correo de prueba enviado desde el sistema <strong>Toscanini</strong>.</p>
                <p>Si estás leyendo esto, la configuración SMTP está funcionando correctamente. ✅</p>
            </div>
            <div class="footer">
                <p>Enviado automáticamente por el sistema Toscanini.</p>
            </div>
        </div>
    </body>
    </html>
    "#;

    email_service.send_email_internal(
        &to_email,
        "🧪 Correo de Prueba - Sistema Toscanini",
        html_content,
        None,
    ).await.map_err(|e| {
        let error_msg = format!("Error enviando correo de prueba: {}", e);
        println!("❌ [db_impl] {}", error_msg);
        error_msg
    })?;
    
    Ok(format!("Correo de prueba enviado exitosamente a {}", to_email))
}

pub async fn send_orden_trabajo_cliente(config: &EmailConfig, orden_id: i32) -> Result<String, String> {
    use crate::infrastructure::db::ordenes_trabajo::get_orden_trabajo_by_id;
    use crate::infrastructure::db::equipos::get_equipo_by_id;
    use crate::infrastructure::db::clientes::get_cliente_by_id;

    // Obtener la orden de trabajo
    let orden = get_orden_trabajo_by_id(orden_id).await?
        .ok_or_else(|| "Orden de trabajo no encontrada".to_string())?;

    // Obtener el equipo
    let equipo_id = orden.equipo_id.ok_or_else(|| "La orden no tiene equipo asociado".to_string())?;
    let equipo = get_equipo_by_id(equipo_id).await?
        .ok_or_else(|| "Equipo no encontrado".to_string())?;

    // Obtener el cliente
    let cliente_id = equipo.cliente_id.ok_or_else(|| "Equipo sin cliente asociado".to_string())?;
    
    let cliente = get_cliente_by_id(cliente_id).await?
        .ok_or_else(|| "Cliente no encontrado".to_string())?;

    // Verificar que el cliente tenga email
    if cliente.cliente_correo.is_none() || cliente.cliente_correo.as_ref().unwrap().trim().is_empty() {
        return Err("El cliente no tiene email configurado".to_string());
    }

    // Crear el servicio de email
    let email_service = EmailService::new(config)?;

    // Enviar el email
    email_service.send_orden_trabajo_cliente(
        &cliente.cliente_correo.unwrap(),
        &cliente.cliente_nombre.unwrap_or_else(|| "Cliente".to_string()),
        &orden,
        &equipo,
    ).await?;

    Ok("Email enviado exitosamente".to_string())
}

pub async fn send_cotizacion_email(config: &EmailConfig, cotizacion_id: i32, sent_by: i32) -> Result<String, String> {
    use crate::infrastructure::db::cotizacion::{get_cotizacion_by_id, update_cotizacion};
    use crate::infrastructure::db::ordenes_trabajo::{get_orden_trabajo_by_id, cambiar_estado_orden_trabajo};
    use crate::infrastructure::db::equipos::get_equipo_by_id;
    use crate::infrastructure::db::clientes::get_cliente_by_id;
    use crate::pdf::db_data::get_cotizacion_pdf_data;
    use crate::pdf::CotizacionPdfGenerator;
    use crate::database::get_db_pool_safe;

    // Obtener la cotización
    let cotizacion = get_cotizacion_by_id(cotizacion_id).await?
        .ok_or_else(|| "Cotización no encontrada".to_string())?;

    // Generar el PDF
    let pdf_data = get_cotizacion_pdf_data(cotizacion_id).await?;
    let pdf_bytes = CotizacionPdfGenerator::new().generate_cotizacion_pdf(pdf_data).await?;

    // Buscar la orden de trabajo asociada
    let pool = get_db_pool_safe()?;
    let orden_id: Option<i32> = sqlx::query_scalar(
        "SELECT orden_id FROM ORDEN_TRABAJO WHERE cotizacion_id = ? LIMIT 1"
    )
    .bind(cotizacion_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error buscando orden asociada: {}", e))?;

    let orden_id = orden_id.ok_or_else(|| "La cotización no está asociada a ninguna orden de trabajo".to_string())?;

    // Obtener la orden de trabajo
    let orden_trabajo = get_orden_trabajo_by_id(orden_id).await?
        .ok_or_else(|| "Orden de trabajo no encontrada".to_string())?;

    // Obtener el equipo
    let equipo_id = orden_trabajo.equipo_id.ok_or_else(|| "La orden no tiene equipo asociado".to_string())?;
    let equipo = get_equipo_by_id(equipo_id).await?
        .ok_or_else(|| "Equipo no encontrado".to_string())?;

    // Obtener el cliente
    let cliente_id = equipo.cliente_id.ok_or_else(|| "Equipo sin cliente asociado".to_string())?;
    let cliente = get_cliente_by_id(cliente_id).await?
        .ok_or_else(|| "Cliente no encontrado".to_string())?;

    // Verificar que el cliente tenga email
    if cliente.cliente_correo.is_none() || cliente.cliente_correo.as_ref().unwrap().trim().is_empty() {
        return Err("El cliente no tiene email configurado".to_string());
    }

    // Crear el servicio de email
    let email_service = EmailService::new(config)?;

    // Enviar el email con PDF
    email_service.send_cotizacion_email_with_pdf(
        &cliente.cliente_correo.unwrap(),
        &cliente.cliente_nombre.unwrap_or_else(|| "Cliente".to_string()),
        &cotizacion,
        &orden_trabajo,
        &equipo,
        &pdf_bytes,
    ).await?;

    // Actualizar estados
    let _ = update_cotizacion(
        cotizacion_id,
        crate::models::cotizacion::UpdateCotizacionRequest {
            cotizacion_codigo: None,
            costo_revision: None,
            costo_reparacion: None,
            costo_total: None,
            is_aprobada: None,
            is_borrador: Some(false),
            informe: None,
            piezas: None,
        },
        sent_by,
    ).await.map_err(|e| format!("Error actualizando estado de cotización: {}", e))?;

    if let Some(estado_actual) = &orden_trabajo.estado {
        if estado_actual == "recibido" {
            let _ = cambiar_estado_orden_trabajo(
                orden_id,
                "cotizacion_enviada".to_string(),
                sent_by,
            ).await.map_err(|e| format!("Error actualizando estado de orden: {}", e))?;
        }
    }

    Ok("Email de cotización con PDF enviado exitosamente y estados actualizados".to_string())
}

pub async fn send_informe_email(config: &EmailConfig, orden_id: i32, _sent_by: i32) -> Result<String, String> {
    use crate::infrastructure::db::ordenes_trabajo::get_orden_trabajo_by_id;
    use crate::infrastructure::db::informe::get_informe_by_id;
    use crate::infrastructure::db::equipos::get_equipo_by_id;
    use crate::infrastructure::db::clientes::get_cliente_by_id;
    use crate::pdf::db_data::get_informe_pdf_data;
    use crate::pdf::InformePdfGenerator;

    // Obtener la orden de trabajo
    let orden_trabajo = get_orden_trabajo_by_id(orden_id).await?
        .ok_or_else(|| "Orden de trabajo no encontrada".to_string())?;

    // Verificar que la orden tenga un informe asociado
    let informe_id = orden_trabajo.informe_id
        .ok_or_else(|| "La orden de trabajo no tiene un informe asociado".to_string())?;

    // Obtener el informe
    let informe = get_informe_by_id(informe_id).await?
        .ok_or_else(|| "Informe no encontrado".to_string())?;

    // Generar el PDF del informe
    let pdf_data = get_informe_pdf_data(informe_id).await?;
    let pdf_bytes = InformePdfGenerator::new().generate_informe_pdf(pdf_data).await?;

    // Obtener el equipo
    let equipo_id = orden_trabajo.equipo_id.ok_or_else(|| "La orden no tiene equipo asociado".to_string())?;
    let equipo = get_equipo_by_id(equipo_id).await?
        .ok_or_else(|| "Equipo no encontrado".to_string())?;

    // Obtener el cliente
    let cliente_id = equipo.cliente_id.ok_or_else(|| "Equipo sin cliente asociado".to_string())?;
    let cliente = get_cliente_by_id(cliente_id).await?
        .ok_or_else(|| "Cliente no encontrado".to_string())?;

    // Verificar que el cliente tenga email
    if cliente.cliente_correo.is_none() || cliente.cliente_correo.as_ref().unwrap().trim().is_empty() {
        return Err("El cliente no tiene email configurado".to_string());
    }

    // Crear el servicio de email
    let email_service = EmailService::new(config)?;

    // Enviar el email con PDF
    email_service.send_informe_email_with_pdf(
        &cliente.cliente_correo.unwrap(),
        &cliente.cliente_nombre.unwrap_or_else(|| "Cliente".to_string()),
        &informe,
        &orden_trabajo,
        &equipo,
        &pdf_bytes,
    ).await?;

    Ok("Email de informe con PDF enviado exitosamente al cliente".to_string())
}
