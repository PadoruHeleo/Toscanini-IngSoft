use mail_builder::MessageBuilder;
use mail_send::{SmtpClientBuilder, Credentials};
use std::env;
use chrono_tz::America::Santiago;

pub struct EmailService {
    smtp_host: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
    smtp_from: String,
}

impl EmailService {
    pub fn new() -> Result<Self, String> {
        println!("🔧 Inicializando EmailService con SMTP...");
        
        // Intentar SMTP_SERVER primero, luego SMTP_HOST como fallback
        let smtp_host = env::var("SMTP_SERVER")
            .or_else(|_| env::var("SMTP_HOST"))
            .map_err(|_| {
                eprintln!("❌ Variable de entorno SMTP_SERVER o SMTP_HOST no encontrada");
                eprintln!("🔍 Variables de entorno disponibles:");
                for (key, _) in env::vars() {
                    if key.contains("SMTP") || key.contains("EMAIL") {
                        eprintln!("   - {}", key);
                    }
                }
                "SMTP_SERVER o SMTP_HOST environment variable not found".to_string()
            })?;
        
        let smtp_port = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "587".to_string())
            .parse::<u16>()
            .map_err(|_| "SMTP_PORT debe ser un número válido".to_string())?;
        
        let smtp_username = env::var("SMTP_USERNAME")
            .map_err(|_| "SMTP_USERNAME environment variable not found".to_string())?;
        
        let smtp_password = env::var("SMTP_PASSWORD")
            .map_err(|_| "SMTP_PASSWORD environment variable not found".to_string())?;
        
        let smtp_from = env::var("SMTP_FROM_EMAIL")
            .unwrap_or_else(|_| smtp_username.clone());
        
        println!("✅ Configuración SMTP cargada correctamente");
        println!("   Host: {}", smtp_host);
        println!("   Port: {}", smtp_port);
        println!("   From: {}", smtp_from);
        
        Ok(EmailService {
            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
            smtp_from,
        })
    }

    async fn send_email_internal(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
        attachments: Option<Vec<(String, Vec<u8>, &str)>>,
    ) -> Result<(), String> {
        println!("📧 [send_email_internal] Iniciando envío a: {}", to);
        
        // Construir el cliente SMTP
        println!("📧 [send_email_internal] Creando credenciales SMTP...");
        let creds = Credentials::new(
            self.smtp_username.clone(),
            self.smtp_password.clone(),
        );
        
        println!("📧 [send_email_internal] Conectando a SMTP {}:{}...", self.smtp_host, self.smtp_port);
        
        // El puerto 465 requiere SSL/TLS implícito, el puerto 587 usa STARTTLS
        let use_implicit_tls = self.smtp_port == 465;
        println!("📧 [send_email_internal] Usando SSL/TLS implícito: {} (puerto {})", use_implicit_tls, self.smtp_port);
        
        let mut smtp = SmtpClientBuilder::new(self.smtp_host.clone(), self.smtp_port)
            .implicit_tls(use_implicit_tls)
            .credentials(creds)
            .connect()
            .await
            .map_err(|e| {
                let error_msg = format!("Error conectando cliente SMTP a {}:{} - {}", self.smtp_host, self.smtp_port, e);
                println!("❌ [send_email_internal] {}", error_msg);
                error_msg
            })?;
        
        println!("✅ [send_email_internal] Conexión SMTP establecida");

        // Construir el mensaje
        println!("📧 [send_email_internal] Construyendo mensaje...");
        let mut message_builder = MessageBuilder::new()
            .from((
                self.smtp_from.split('@').next().unwrap_or("noreply"),
                self.smtp_from.as_str(),
            ))
            .to((
                to.split('@').next().unwrap_or("recipient"),
                to,
            ))
            .subject(subject)
            .html_body(html_body);

        // Agregar adjuntos si existen
        if let Some(attachments) = attachments {
            println!("📧 [send_email_internal] Agregando {} adjuntos...", attachments.len());
            for (filename, content, content_type) in attachments {
                message_builder = message_builder.attachment(content_type, filename, content);
            }
        }

        // Enviar el email directamente con MessageBuilder
        println!("📧 [send_email_internal] Enviando mensaje...");
        smtp.send(message_builder)
            .await
            .map_err(|e| {
                let error_msg = format!("Error enviando email: {}", e);
                println!("❌ [send_email_internal] {}", error_msg);
                error_msg
            })?;

        println!("✅ [send_email_internal] Email enviado exitosamente a: {}", to);
        Ok(())
    }

    pub async fn send_password_reset_email(&self, to_email: &str, reset_code: &str, user_name: &str) -> Result<(), String> {
        // Verificar el entorno de ejecución
        let app_environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        
        let to = if app_environment == "development" {
            // En desarrollo: usar email de desarrollo
            let dev_email = env::var("DEV_EMAIL_RECIPIENT").unwrap_or_else(|_| "benitez.basti0@gmail.com".to_string());
            
            println!("🔧 MODO DESARROLLO: Enviando código de recuperación de contraseña");
            println!("📧 Enviando a email de desarrollo: {}", dev_email);
            println!("📧 En producción se enviaría a: {}", to_email);
            
            dev_email
        } else {
            // En producción: usar el email real del usuario
            println!("📧 MODO PRODUCCIÓN: Enviando código de recuperación a {}", to_email);
            to_email.to_string()
        };
        
        let subject = "Recuperación de Contraseña - Toscanini";

        let html_content = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
                <h2 style="color: #333; text-align: center;">Recuperación de Contraseña</h2>
                <p>Hola <strong>{user_name}</strong>,</p>
                <p>Hemos recibido una solicitud para restablecer la contraseña de tu cuenta en Toscanini.</p>
                <div style="background-color: #f5f5f5; padding: 20px; margin: 20px 0; text-align: center; border-radius: 5px;">
                    <p style="margin: 0; font-size: 18px;">Tu código de verificación es:</p>
                    <h1 style="color: #007bff; font-size: 32px; margin: 10px 0; letter-spacing: 5px;">{reset_code}</h1>
                </div>
                <p><strong>Importante:</strong></p>
                <ul>
                    <li>Este código expira en 15 minutos</li>
                    <li>Solo puede ser usado una vez</li>
                    <li>Si no solicitaste este cambio, ignora este correo</li>
                </ul>
                <p>Si tienes problemas, contacta a nuestro equipo de soporte.</p>
                <hr style="margin: 30px 0; border: 1px solid #eee;">
                <p style="color: #666; font-size: 12px; text-align: center;">
                    Este es un correo automático, por favor no respondas a este mensaje.
                </p>
            </div>
            "#,
            user_name = user_name,
            reset_code = reset_code
        );

        self.send_email_internal(&to, subject, &html_content, None).await?;

        println!("✅ Email de recuperación enviado exitosamente");
        if app_environment == "development" {
            println!("🔑 Código de recuperación generado: {} (solo visible en desarrollo)", reset_code);
        }
        
        Ok(())
    }

    pub async fn send_informe_email(
        &self, 
        to_email: &str, 
        client_name: &str, 
        informe: &crate::commands::informe::Informe,
        orden_trabajo: &crate::commands::ordenes_trabajo::OrdenTrabajo,
        piezas: &[crate::commands::informe::PiezaInforme]
    ) -> Result<(), String> {
        let subject = format!("Informe Técnico {} - Toscanini", 
            informe.informe_codigo.as_deref().unwrap_or("N/A"));

        // Generar tabla de piezas si existen
        let piezas_html = if piezas.is_empty() {
            "<p><em>No se utilizaron piezas en este servicio.</em></p>".to_string()
        } else {
            let mut tabla = String::from(
                r#"<table style="width: 100%; border-collapse: collapse; margin: 20px 0;">
                    <thead>
                        <tr style="background-color: #f8f9fa;">
                            <th style="border: 1px solid #dee2e6; padding: 12px; text-align: left;">Pieza</th>
                            <th style="border: 1px solid #dee2e6; padding: 12px; text-align: left;">Marca</th>
                            <th style="border: 1px solid #dee2e6; padding: 12px; text-align: center;">Cantidad</th>
                            <th style="border: 1px solid #dee2e6; padding: 12px; text-align: right;">Precio Unit.</th>
                            <th style="border: 1px solid #dee2e6; padding: 12px; text-align: right;">Subtotal</th>
                        </tr>
                    </thead>
                    <tbody>"#
            );

            let mut total = 0;
            for pieza in piezas {
                let precio = pieza.pieza_precio.unwrap_or(0);
                let cantidad = pieza.cantidad.unwrap_or(1);
                let subtotal = precio * cantidad;
                total += subtotal;

                tabla.push_str(&format!(
                    r#"<tr>
                        <td style="border: 1px solid #dee2e6; padding: 12px;">{}</td>
                        <td style="border: 1px solid #dee2e6; padding: 12px;">{}</td>
                        <td style="border: 1px solid #dee2e6; padding: 12px; text-align: center;">{}</td>
                        <td style="border: 1px solid #dee2e6; padding: 12px; text-align: right;">${}</td>
                        <td style="border: 1px solid #dee2e6; padding: 12px; text-align: right;">${}</td>
                    </tr>"#,
                    pieza.pieza_nombre.as_deref().unwrap_or("N/A"),
                    pieza.pieza_marca.as_deref().unwrap_or("N/A"),
                    cantidad,
                    precio,
                    subtotal
                ));
            }

            tabla.push_str(&format!(
                r#"</tbody>
                    <tfoot>
                        <tr style="background-color: #e9ecef; font-weight: bold;">
                            <td colspan="4" style="border: 1px solid #dee2e6; padding: 12px; text-align: right;">Total:</td>
                            <td style="border: 1px solid #dee2e6; padding: 12px; text-align: right;">${}</td>
                        </tr>
                    </tfoot>
                </table>"#,
                total
            ));

            tabla
        };

        let html_content = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px;">
                <div style="text-align: center; margin-bottom: 30px;">
                    <h1 style="color: #333; margin: 0;">Toscanini</h1>
                    <p style="color: #666; margin: 5px 0;">Servicio Técnico Especializado</p>
                </div>
                
                <h2 style="color: #007bff; border-bottom: 2px solid #007bff; padding-bottom: 10px;">
                    Informe Técnico {}
                </h2>
                
                <p>Estimado/a <strong>{}</strong>,</p>
                
                <p>Nos complace enviarle el informe técnico del servicio realizado a su equipo.</p>
                
                <div style="background-color: #f8f9fa; padding: 20px; margin: 20px 0; border-radius: 5px;">
                    <h3 style="margin-top: 0; color: #333;">Detalles del Servicio</h3>
                    <p><strong>Código de Orden:</strong> {}</p>
                    <p><strong>Descripción:</strong> {}</p>
                    <p><strong>Estado:</strong> {}</p>
                    {}
                </div>
                
                <div style="background-color: #ffffff; padding: 20px; margin: 20px 0; border: 1px solid #dee2e6; border-radius: 5px;">
                    <h3 style="margin-top: 0; color: #333;">Diagnóstico y Acciones</h3>
                    <div style="margin-bottom: 15px;">
                        <h4 style="color: #555; margin-bottom: 5px;">Acciones Realizadas:</h4>
                        <p style="margin: 0; line-height: 1.6;">{}</p>
                    </div>
                    {}
                </div>
                
                {}
                
                <div style="background-color: #e8f4f8; padding: 15px; border-radius: 5px; margin: 20px 0;">
                    <p style="margin: 0; text-align: center; color: #333;">
                        <strong>¡Gracias por confiar en Toscanini!</strong><br>
                        Si tiene alguna consulta sobre este informe, no dude en contactarnos.
                    </p>
                </div>
                
                <hr style="margin: 30px 0; border: 1px solid #eee;">
                <p style="color: #666; font-size: 12px; text-align: center;">
                    Este es un correo automático, por favor no respondas a este mensaje.<br>
                    Para consultas, contacta directamente con nuestro equipo de soporte.
                </p>
            </div>
            "#,
            informe.informe_codigo.as_deref().unwrap_or("N/A"),
            client_name,
            orden_trabajo.orden_codigo.as_deref().unwrap_or("N/A"),
            orden_trabajo.orden_desc.as_deref().unwrap_or("Sin descripción"),
            orden_trabajo.estado.as_deref().unwrap_or("N/A"),
            if orden_trabajo.has_garantia.unwrap_or(false) { 
                "<p><strong>Garantía:</strong> ✓ Sí</p>" 
            } else { 
                "<p><strong>Garantía:</strong> ✗ No</p>" 
            },
            informe.informe_acciones.as_deref().unwrap_or("Sin acciones registradas"),
            if let Some(obs) = &informe.informe_obs {
                format!(r#"<div style="margin-top: 15px;">
                    <h4 style="color: #555; margin-bottom: 5px;">Observaciones:</h4>
                    <p style="margin: 0; line-height: 1.6;">{}</p>
                </div>"#, obs)
            } else {
                String::new()
            },
            if !piezas.is_empty() {
                format!(r#"<div style="background-color: #ffffff; padding: 20px; margin: 20px 0; border: 1px solid #dee2e6; border-radius: 5px;">
                    <h3 style="margin-top: 0; color: #333;">Piezas Utilizadas</h3>
                    {}
                </div>"#, piezas_html)
            } else {
                String::new()
            }
        );

        self.send_email_internal(to_email, &subject, &html_content, None).await?;
        Ok(())
    }

    pub async fn send_orden_trabajo_notification(
        &self, 
        orden_trabajo: &crate::commands::ordenes_trabajo::OrdenTrabajo,
        equipo: &crate::commands::equipos::Equipo,
        cliente_nombre: &str
    ) -> Result<String, String> {
        // Verificar el entorno de ejecución
        let app_environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        
        let (to_list, log_message) = if app_environment == "development" {
            // En desarrollo: usar email de desarrollo o simular envío
            let dev_email = env::var("DEV_EMAIL_RECIPIENT").unwrap_or_else(|_| "benitez.basti0@gmail.com".to_string());
            
            println!("🔧 MODO DESARROLLO: Simulando envío de notificación de orden de trabajo");
            println!("📧 En producción se enviaría a los administradores y técnicos de la BD");
            
            // Obtener emails para logging (sin enviar)
            let db_emails = crate::commands::users::get_admin_and_tech_emails().await.unwrap_or_default();
            println!("📋 Emails que recibirían en producción: {:?}", db_emails);
            
            (
                vec![dev_email.clone()],
                format!("Notificación de orden {} enviada a {} (desarrollo)", 
                    orden_trabajo.orden_codigo.as_deref().unwrap_or("N/A"), dev_email)
            )
        } else {
            // En producción: obtener emails reales de la base de datos
            let notification_emails = crate::commands::users::get_admin_and_tech_emails().await?;
            
            if notification_emails.is_empty() {
                return Err("No hay administradores o técnicos con email configurado para enviar notificaciones".to_string());
            }
            
            (
                notification_emails.clone(),
                format!("Notificación de orden {} enviada a {} administradores y técnicos", 
                    orden_trabajo.orden_codigo.as_deref().unwrap_or("N/A"), 
                    notification_emails.len())
            )
        };
        
        let subject = format!("Nueva Orden de Trabajo {} - Toscanini", 
            orden_trabajo.orden_codigo.as_deref().unwrap_or("N/A"));

        let html_content = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px;">
                <div style="text-align: center; margin-bottom: 30px;">
                    <h1 style="color: #333; margin: 0;">Toscanini</h1>
                    <p style="color: #666; margin: 5px 0;">Servicio Técnico Especializado</p>
                </div>
                
                <h2 style="color: #007bff; border-bottom: 2px solid #007bff; padding-bottom: 10px;">
                    Nueva Orden de Trabajo Creada
                </h2>
                
                <p>Se ha creado una nueva orden de trabajo en el sistema.</p>
                
                <div style="background-color: #f8f9fa; padding: 20px; margin: 20px 0; border-radius: 5px;">
                    <h3 style="margin-top: 0; color: #333;">Detalles de la Orden</h3>
                    <p><strong>Código de Orden:</strong> {}</p>
                    <p><strong>Descripción:</strong> {}</p>
                    <p><strong>Prioridad:</strong> {}</p>
                    <p><strong>Estado:</strong> {}</p>
                    <p><strong>Garantía:</strong> {}</p>
                    <p><strong>Fecha de Creación:</strong> {}</p>
                </div>
                
                <div style="background-color: #ffffff; padding: 20px; margin: 20px 0; border: 1px solid #dee2e6; border-radius: 5px;">
                    <h3 style="margin-top: 0; color: #333;">Información del Equipo</h3>
                    <p><strong>Cliente:</strong> {}</p>
                    <p><strong>Número de Serie:</strong> {}</p>
                    <p><strong>Marca:</strong> {}</p>
                    <p><strong>Modelo:</strong> {}</p>
                    <p><strong>Tipo:</strong> {}</p>
                    {}
                </div>
                
                <div style="background-color: #ffffff; padding: 20px; margin: 20px 0; border: 1px solid #dee2e6; border-radius: 5px;">
                    <h3 style="margin-top: 0; color: #333;">Pre-informe</h3>
                    <p style="margin: 0; line-height: 1.6;">{}</p>
                </div>
                
                <div style="background-color: #e8f4f8; padding: 15px; border-radius: 5px; margin: 20px 0;">
                    <p style="margin: 0; text-align: center; color: #333;">
                        <strong>Notificación automática del sistema Toscanini</strong><br>
                        Esta orden requiere atención para continuar con el proceso de reparación.
                    </p>
                </div>
                
                <hr style="margin: 30px 0; border: 1px solid #eee;">
                <p style="color: #666; font-size: 12px; text-align: center;">
                    Este es un correo automático, por favor no respondas a este mensaje.<br>
                    Para consultas, contacta directamente con el equipo de soporte.
                </p>
            </div>
            "#,
            orden_trabajo.orden_codigo.as_deref().unwrap_or("N/A"),
            orden_trabajo.orden_desc.as_deref().unwrap_or("Sin descripción"),
            match orden_trabajo.prioridad.as_deref() {
                Some("alta") => "🔴 Alta",
                Some("media") => "🟡 Media", 
                Some("baja") => "🟢 Baja",
                _ => "N/A"
            },
            match orden_trabajo.estado.as_deref() {
                Some("recibido") => "Recibido",
                Some("cotizacion_enviada") => "Cotización Enviada",
                Some("aprobacion_pendiente") => "Aprobación Pendiente",
                Some("en_reparacion") => "En Reparación",
                Some("espera_de_retiro") => "Espera de Retiro",
                Some("entregado") => "Entregado",
                Some("abandonado") => "Abandonado",
                Some("equipo_no_reparable") => "Equipo No Reparable",
                _ => "N/A"
            },
            if orden_trabajo.has_garantia.unwrap_or(false) { 
                "✓ Sí" 
            } else { 
                "✗ No" 
            },
            orden_trabajo.created_at
                .map(|dt| dt.format("%d/%m/%Y %H:%M").to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            cliente_nombre,
            equipo.numero_serie.as_deref().unwrap_or("N/A"),
            equipo.equipo_marca.as_deref().unwrap_or("N/A"),
            equipo.equipo_modelo.as_deref().unwrap_or("N/A"),
            equipo.equipo_tipo.as_deref().unwrap_or("N/A"),
            if let Some(ref ubicacion) = equipo.equipo_ubicacion {
                format!("<p><strong>Ubicación:</strong> {}</p>", ubicacion)
            } else {
                String::new()
            },
            orden_trabajo.pre_informe.as_deref().unwrap_or("Sin pre-informe registrado")
        );

        // En desarrollo, detectar emails de prueba y simular envío
        let is_test_email = |email: &str| -> bool {
            email.contains("@toscanini.com") || 
            email.contains("@test.") || 
            email.contains("@ejemplo.") || 
            email.contains("@prueba.") ||
            email.starts_with("admin@") ||
            email.starts_with("tecnico") ||
            email.starts_with("recepcion@")
        };
        
        if app_environment == "development" && to_list.iter().any(|email| is_test_email(email)) {
            println!("📧 SIMULANDO envío de email a: {:?}", to_list);
            println!("📄 Asunto: {}", subject);
            println!("✅ Email simulado correctamente (no se envió realmente)");
            return Ok(log_message);
        }
        
        // Enviar email real a cada destinatario
        for recipient in &to_list {
            self.send_email_internal(recipient, &subject, &html_content, None).await?;
            println!("📧 Email enviado exitosamente a: {}", recipient);
        }
        
        Ok(log_message)
    }

    pub async fn send_orden_trabajo_cliente(
        &self,
        cliente_email: &str,
        cliente_nombre: &str,
        orden_trabajo: &crate::commands::ordenes_trabajo::OrdenTrabajo,
        equipo: &crate::commands::equipos::Equipo,
    ) -> Result<(), String> {
        let subject = format!(
            "Orden de Trabajo Creada - Toscanini (Código: {})",
            orden_trabajo.orden_codigo.as_deref().unwrap_or("N/A")
        );

        let html_content = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px;">
                <div style="text-align: center; margin-bottom: 30px;">
                    <h1 style="color: #333; margin: 0;">Toscanini</h1>
                    <p style="color: #666; margin: 5px 0;">Servicio Técnico Especializado</p>
                </div>
                <h2 style="color: #007bff; border-bottom: 2px solid #007bff; padding-bottom: 10px;">
                    ¡Hemos recibido tu equipo!
                </h2>
                <p>Estimado/a <strong>{}</strong>,</p>
                <p>Te informamos que hemos recibido tu equipo y se ha generado una orden de trabajo en nuestro sistema.</p>
                <div style="background-color: #f8f9fa; padding: 20px; margin: 20px 0; border-radius: 5px;">
                    <h3 style="margin-top: 0; color: #333;">Detalles de la Orden</h3>
                    <p><strong>Código de Orden:</strong> {}</p>
                    <p><strong>Fecha de Ingreso:</strong> {}</p>
                </div>
                <div style="background-color: #ffffff; padding: 20px; margin: 20px 0; border: 1px solid #dee2e6; border-radius: 5px;">
                    <h3 style="margin-top: 0; color: #333;">Información del Equipo</h3>
                    <p><strong>Marca:</strong> {}</p>
                    <p><strong>Modelo:</strong> {}</p>
                    <p><strong>Número de Serie:</strong> {}</p>
                    <p><strong>Tipo:</strong> {}</p>
                    {}
                </div>
                <div style="background-color: #f8f9fa; padding: 20px; margin: 20px 0; border-radius: 5px;">
                    <h3 style="margin-top: 0; color: #333;">Pre-informe</h3>
                    <p>{}</p>
                </div>
                <div style="background-color: #e8f4f8; padding: 15px; border-radius: 5px; margin: 20px 0;">
                    <p style="margin: 0; text-align: center; color: #333;">
                        <strong>Gracias por confiar en Toscanini.</strong><br>
                        Te mantendremos informado sobre el avance de tu equipo.<br>
                        <br>
                        <strong>Siguientes pasos:</strong><br>
                        1. Nuestro equipo técnico evaluará el estado de tu equipo.<br>
                        2. Te contactaremos con el diagnóstico y cotización.<br>
                        3. Podrás aprobar o rechazar la reparación.<br>
                        4. Recibirás notificaciones sobre el avance.<br>
                    </p>
                </div>
                <hr style="margin: 30px 0; border: 1px solid #eee;">
                <p style="color: #666; font-size: 12px; text-align: center;">
                    Este es un correo automático, por favor no respondas a este mensaje.<br>
                    Para consultas, contáctanos directamente.
                </p>
            </div>
            "#,
            cliente_nombre,
            orden_trabajo.orden_codigo.as_deref().unwrap_or("N/A"),
            orden_trabajo.created_at
                .map(|dt| {
                    let local_dt = dt.with_timezone(&Santiago);
                    local_dt.format("%d/%m/%Y %H:%M").to_string()
                })
                .unwrap_or_else(|| "N/A".to_string()),
            equipo.equipo_marca.as_deref().unwrap_or("N/A"),
            equipo.equipo_modelo.as_deref().unwrap_or("N/A"),
            equipo.numero_serie.as_deref().unwrap_or("N/A"),
            equipo.equipo_tipo.as_deref().unwrap_or("N/A"),
            if let Some(ref ubicacion) = equipo.equipo_ubicacion {
                format!("<p><strong>Ubicación:</strong> {}</p>", ubicacion)
            } else {
                String::new()
            },
            orden_trabajo.pre_informe.as_deref().unwrap_or("Sin pre-informe registrado")
        );

        self.send_email_internal(cliente_email, &subject, &html_content, None).await?;
        Ok(())
    }

    pub async fn send_password_email(
        &self,
        to_email: &str,
        user_name: &str,
        temp_password: &str,
    ) -> Result<(), String> {
        let subject = "Acceso a Toscanini - Credenciales Temporales";

        let html_content = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
                <h2 style="color: #333; text-align: center;">Bienvenido a Toscanini</h2>
                <p>Hola <strong>{user_name}</strong>,</p>
                <p>Tu cuenta ha sido creada exitosamente. Aquí tienes tus credenciales temporales para el primer acceso:</p>
                <ul style="background-color: #f5f5f5; padding: 20px; border-radius: 5px; margin: 20px 0;">
                    <li><strong>Nombre de usuario:</strong> {to_email}</li>
                    <li><strong>Contraseña temporal:</strong> <span style="color: #007bff; font-size: 18px;">{temp_password}</span></li>
                </ul>
                <h3>Instrucciones para el primer acceso:</h3>
                <ol>
                    <li>Ingresa al sistema con tu correo y la contraseña temporal.</li>
                    <li>Por seguridad, <strong>cambia tu contraseña</strong> inmediatamente después de iniciar sesión.</li>
                </ol>
                <p style="color: #d9534f;"><strong>Recomendación:</strong> No compartas tu contraseña y cámbiala tras el primer ingreso.</p>
                <hr style="margin: 30px 0; border: 1px solid #eee;">
                <p style="color: #666; font-size: 12px; text-align: center;">
                    Este es un correo automático, por favor no respondas a este mensaje.
                </p>
            </div>
            "#,
            user_name = user_name,
            to_email = to_email,
            temp_password = temp_password
        );

        self.send_email_internal(to_email, subject, &html_content, None).await?;
        println!("Email enviado exitosamente");
        Ok(())
    }

    /// Enviar email de cotización con PDF adjunto usando SMTP
    pub async fn send_cotizacion_email_with_pdf(
        &self,
        cliente_email: &str,
        cliente_nombre: &str,
        cotizacion: &crate::commands::cotizacion::Cotizacion,
        orden_trabajo: &crate::commands::ordenes_trabajo::OrdenTrabajo,
        equipo: &crate::commands::equipos::Equipo,
        pdf_bytes: &[u8],
    ) -> Result<(), String> {
        // Nombre del archivo PDF
        let pdf_filename = format!(
            "Cotizacion_{}.pdf",
            cotizacion.cotizacion_codigo.as_deref().unwrap_or("N/A")
        );

        // Calcular totales para mostrar en el email
        let costo_revision = cotizacion.costo_revision.unwrap_or(0);
        let costo_reparacion = cotizacion.costo_reparacion.unwrap_or(0);
        let costo_total = cotizacion.costo_total.unwrap_or(costo_revision + costo_reparacion);

        // Contenido HTML conciso (el PDF tiene toda la información detallada)
        let html_content = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
                <div style="text-align: center; margin-bottom: 30px;">
                    <h1 style="color: #333; margin: 0;">Toscanini</h1>
                    <p style="color: #666; margin: 5px 0;">Servicio Técnico Especializado</p>
                </div>
                
                <h2 style="color: #007bff; border-bottom: 2px solid #007bff; padding-bottom: 10px;">
                    Cotización de Reparación
                </h2>
                
                <p>Estimado/a <strong>{}</strong>,</p>
                
                <p>Te enviamos la cotización para la reparación de tu equipo. La cotización completa con todos los detalles, piezas y términos y condiciones está disponible en el archivo PDF adjunto.</p>
                
                <div style="background-color: #e8f4f8; padding: 20px; margin: 20px 0; border-radius: 5px; border-left: 4px solid #007bff;">
                    <p style="margin: 0; color: #333; font-size: 16px;">
                        <strong>📎 Archivo adjunto:</strong> {}.pdf
                    </p>
                    <p style="margin: 10px 0 0 0; color: #666; font-size: 14px;">
                        Por favor, revisa el documento PDF adjunto para ver todos los detalles de la cotización.
                    </p>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 15px; margin: 20px 0; border-radius: 5px;">
                    <h3 style="margin-top: 0; color: #333; font-size: 16px;">Resumen</h3>
                    <p style="margin: 5px 0;"><strong>Código de Cotización:</strong> {}</p>
                    <p style="margin: 5px 0;"><strong>Código de Orden:</strong> {}</p>
                    <p style="margin: 5px 0;"><strong>Equipo:</strong> {} {}</p>
                    <p style="margin: 5px 0;"><strong>Total:</strong> <span style="color: #007bff; font-size: 18px; font-weight: bold;">${}</span></p>
                </div>
                
                <div style="background-color: #fff3cd; padding: 15px; border-radius: 5px; margin: 20px 0; border-left: 4px solid #ffc107;">
                    <p style="margin: 0; color: #856404;">
                        <strong>⚠️ Importante:</strong> Por favor revisa el PDF adjunto para ver el diagnóstico técnico completo, desglose detallado de costos, piezas requeridas y términos y condiciones.
                    </p>
                </div>
                
                <div style="background-color: #e8f4f8; padding: 15px; border-radius: 5px; margin: 20px 0;">
                    <p style="margin: 0; text-align: center; color: #333;">
                        <strong>Próximos pasos:</strong><br>
                        1. Revisa la cotización detallada en el PDF adjunto.<br>
                        2. Puedes aprobar o rechazar esta cotización.<br>
                        3. Una vez aprobada, procederemos con la reparación.<br>
                        4. Te mantendremos informado sobre el avance.<br>
                    </p>
                </div>
                
                <hr style="margin: 30px 0; border: 1px solid #eee;">
                <p style="color: #666; font-size: 12px; text-align: center;">
                    Este es un correo automático, por favor no respondas a este mensaje.<br>
                    Para consultas o aprobar esta cotización, contáctanos directamente.
                </p>
            </div>
            "#,
            cliente_nombre,
            cotizacion.cotizacion_codigo.as_deref().unwrap_or("N/A"),
            cotizacion.cotizacion_codigo.as_deref().unwrap_or("N/A"),
            orden_trabajo.orden_codigo.as_deref().unwrap_or("N/A"),
            equipo.equipo_marca.as_deref().unwrap_or("N/A"),
            equipo.equipo_modelo.as_deref().unwrap_or("N/A"),
            costo_total
        );

        let attachments = vec![
            (pdf_filename, pdf_bytes.to_vec(), "application/pdf")
        ];

        self.send_email_internal(
            cliente_email,
            &format!(
                "Cotización de Reparación - Toscanini (Código: {})",
                cotizacion.cotizacion_codigo.as_deref().unwrap_or("N/A")
            ),
            &html_content,
            Some(attachments),
        ).await?;

        println!("📧 Email de cotización con PDF enviado exitosamente a: {}", cliente_email);
        Ok(())
    }

    /// Enviar email al cliente informando que su equipo está listo para retiro con el informe PDF adjunto
    pub async fn send_informe_email_with_pdf(
        &self,
        cliente_email: &str,
        cliente_nombre: &str,
        informe: &crate::commands::informe::Informe,
        orden_trabajo: &crate::commands::ordenes_trabajo::OrdenTrabajo,
        equipo: &crate::commands::equipos::Equipo,
        pdf_bytes: &[u8],
    ) -> Result<(), String> {
        // Nombre del archivo PDF
        let pdf_filename = format!(
            "Informe_{}.pdf",
            informe.informe_codigo.as_deref().unwrap_or("N/A")
        );

        // Contenido HTML conciso (el PDF tiene toda la información detallada)
        let html_content = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
                <div style="text-align: center; margin-bottom: 30px;">
                    <h1 style="color: #333; margin: 0;">Toscanini</h1>
                    <p style="color: #666; margin: 5px 0;">Servicio Técnico Especializado</p>
                </div>
                
                <h2 style="color: #28a745; border-bottom: 2px solid #28a745; padding-bottom: 10px;">
                    ✅ Equipo Listo para Retiro
                </h2>
                
                <p>Estimado/a <strong>{}</strong>,</p>
                
                <p>Nos complace informarte que tu equipo ha sido reparado exitosamente y está <strong>listo para su retiro</strong>.</p>
                
                <div style="background-color: #d4edda; padding: 20px; margin: 20px 0; border-radius: 5px; border-left: 4px solid #28a745;">
                    <p style="margin: 0; color: #155724; font-size: 16px;">
                        <strong>🎉 ¡Tu equipo está listo!</strong>
                    </p>
                    <p style="margin: 10px 0 0 0; color: #155724; font-size: 14px;">
                        Puedes retirar tu equipo en nuestras instalaciones durante nuestro horario de atención.
                    </p>
                </div>
                
                <div style="background-color: #e8f4f8; padding: 20px; margin: 20px 0; border-radius: 5px; border-left: 4px solid #007bff;">
                    <p style="margin: 0; color: #333; font-size: 16px;">
                        <strong>📎 Archivo adjunto:</strong> {}.pdf
                    </p>
                    <p style="margin: 10px 0 0 0; color: #666; font-size: 14px;">
                        El informe técnico completo con todos los detalles de la reparación, piezas utilizadas y recomendaciones está disponible en el documento PDF adjunto.
                    </p>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 15px; margin: 20px 0; border-radius: 5px;">
                    <h3 style="margin-top: 0; color: #333; font-size: 16px;">Información del Equipo</h3>
                    <p style="margin: 5px 0;"><strong>Código de Orden:</strong> {}</p>
                    <p style="margin: 5px 0;"><strong>Código de Informe:</strong> {}</p>
                    <p style="margin: 5px 0;"><strong>Equipo:</strong> {} {}</p>
                    <p style="margin: 5px 0;"><strong>Técnico Responsable:</strong> {}</p>
                </div>
                
                <div style="background-color: #fff3cd; padding: 15px; border-radius: 5px; margin: 20px 0; border-left: 4px solid #ffc107;">
                    <p style="margin: 0; color: #856404;">
                        <strong>📋 Importante:</strong> Por favor revisa el PDF adjunto para ver el diagnóstico completo, solución aplicada, piezas utilizadas y recomendaciones para el cuidado de tu equipo.
                    </p>
                </div>
                
                <div style="background-color: #e8f4f8; padding: 15px; border-radius: 5px; margin: 20px 0;">
                    <p style="margin: 0; text-align: center; color: #333;">
                        <strong>Próximos pasos:</strong><br>
                        1. Revisa el informe técnico completo en el PDF adjunto.<br>
                        2. Acude a nuestras instalaciones para retirar tu equipo.<br>
                        3. Trae tu identificación al momento del retiro.<br>
                        4. Si tienes alguna consulta, no dudes en contactarnos.<br>
                    </p>
                </div>
                
                <hr style="margin: 30px 0; border: 1px solid #eee;">
                <p style="color: #666; font-size: 12px; text-align: center;">
                    Este es un correo automático, por favor no respondas a este mensaje.<br>
                    Para consultas o coordinar el retiro, contáctanos directamente.
                </p>
            </div>
            "#,
            cliente_nombre,
            informe.informe_codigo.as_deref().unwrap_or("N/A"),
            orden_trabajo.orden_codigo.as_deref().unwrap_or("N/A"),
            informe.informe_codigo.as_deref().unwrap_or("N/A"),
            equipo.equipo_marca.as_deref().unwrap_or("N/A"),
            equipo.equipo_modelo.as_deref().unwrap_or("N/A"),
            informe.tecnico_responsable.as_deref().unwrap_or("No especificado")
        );

        let attachments = vec![
            (pdf_filename, pdf_bytes.to_vec(), "application/pdf")
        ];

        self.send_email_internal(
            cliente_email,
            &format!("✅ Tu equipo está listo para retiro - {}", orden_trabajo.orden_codigo.as_deref().unwrap_or("N/A")),
            &html_content,
            Some(attachments),
        ).await?;

        println!("📧 Email de informe con PDF enviado exitosamente a: {}", cliente_email);
        Ok(())
    }
}

/// Comando de Tauri para probar el envío de correo electrónico
/// Permite enviar un correo de prueba para verificar la configuración SMTP
#[tauri::command]
pub async fn test_email_send(to_email: String) -> Result<String, String> {
    println!("🧪 [test_email_send] Iniciando prueba de envío de correo a: {}", to_email);
    
    // Crear el servicio de email
    println!("🧪 [test_email_send] Inicializando EmailService...");
    let email_service = EmailService::new()
        .map_err(|e| {
            let error_msg = format!("Error al inicializar servicio de email: {}", e);
            println!("❌ [test_email_send] {}", error_msg);
            error_msg
        })?;
    
    println!("✅ [test_email_send] EmailService inicializado correctamente");
    
    // Crear contenido HTML de prueba
    println!("🧪 [test_email_send] Creando contenido HTML...");
    let html_content = format!(
        r#"
        <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
            <div style="text-align: center; margin-bottom: 30px;">
                <h1 style="color: #333; margin: 0;">🧪 Correo de Prueba</h1>
                <p style="color: #666; margin: 5px 0;">Sistema Toscanini</p>
            </div>
            
            <div style="background-color: #e8f4f8; padding: 20px; margin: 20px 0; border-radius: 5px; border-left: 4px solid #007bff;">
                <h2 style="color: #007bff; margin-top: 0;">✅ Configuración SMTP Correcta</h2>
                <p>Este es un correo de prueba para verificar que la configuración SMTP está funcionando correctamente.</p>
            </div>
            
            <div style="background-color: #f8f9fa; padding: 15px; margin: 20px 0; border-radius: 5px;">
                <h3 style="margin-top: 0; color: #333;">Información de la Prueba</h3>
                <p><strong>Destinatario:</strong> {}</p>
                <p><strong>Fecha:</strong> {}</p>
                <p><strong>Estado:</strong> <span style="color: #28a745;">✓ Enviado exitosamente</span></p>
            </div>
            
            <div style="background-color: #fff3cd; padding: 15px; border-radius: 5px; margin: 20px 0; border-left: 4px solid #ffc107;">
                <p style="margin: 0; color: #856404;">
                    <strong>ℹ️ Nota:</strong> Si recibiste este correo, significa que la configuración SMTP está funcionando correctamente.
                </p>
            </div>
            
            <hr style="margin: 30px 0; border: 1px solid #eee;">
            <p style="color: #666; font-size: 12px; text-align: center;">
                Este es un correo automático de prueba del sistema Toscanini.
            </p>
        </div>
        "#,
        to_email,
        chrono::Utc::now().format("%d/%m/%Y %H:%M:%S UTC")
    );
    
    println!("🧪 [test_email_send] Llamando a send_email_internal...");
    // Enviar el correo de prueba
    email_service.send_email_internal(
        &to_email,
        "🧪 Correo de Prueba - Sistema Toscanini",
        &html_content,
        None,
    ).await.map_err(|e| {
        let error_msg = format!("Error enviando correo de prueba: {}", e);
        println!("❌ [test_email_send] {}", error_msg);
        error_msg
    })?;
    
    println!("✅ [test_email_send] Correo de prueba enviado exitosamente a: {}", to_email);
    
    Ok(format!("Correo de prueba enviado exitosamente a {}", to_email))
}

/// Comando de Tauri para enviar email de orden de trabajo al cliente
#[tauri::command]
pub async fn send_orden_trabajo_cliente(orden_id: i32, _sent_by: i32) -> Result<String, String> {
    use crate::commands::ordenes_trabajo::get_orden_trabajo_by_id;
    use crate::commands::equipos::get_equipo_by_id;
    use crate::commands::clientes::get_cliente_by_id;

    // Obtener la orden de trabajo
    let orden = get_orden_trabajo_by_id(orden_id).await?
        .ok_or_else(|| "Orden de trabajo no encontrada".to_string())?;

    // Obtener el equipo
    let equipo_id = orden.equipo_id.ok_or_else(|| "La orden no tiene equipo asociado".to_string())?;
    let equipo = get_equipo_by_id(equipo_id).await?
        .ok_or_else(|| "Equipo no encontrado".to_string())?;

    // Obtener el cliente
    let cliente_id = equipo.cliente_id.ok_or_else(|| "El equipo no tiene cliente asociado".to_string())?;
    let cliente = get_cliente_by_id(cliente_id).await?
        .ok_or_else(|| "Cliente no encontrado".to_string())?;

    // Verificar que el cliente tenga email
    if cliente.cliente_correo.is_none() || cliente.cliente_correo.as_ref().unwrap().trim().is_empty() {
        return Err("El cliente no tiene email configurado".to_string());
    }

    // Crear el servicio de email
    let email_service = EmailService::new()
        .map_err(|e| format!("Error al inicializar servicio de email: {}", e))?;

    // Enviar el email
    email_service.send_orden_trabajo_cliente(
        &cliente.cliente_correo.unwrap(),
        &cliente.cliente_nombre.unwrap_or_else(|| "Cliente".to_string()),
        &orden,
        &equipo,
    ).await?;

    Ok("Email enviado exitosamente".to_string())
}

/// Comando de Tauri para enviar email de cotización con PDF al cliente
/// También actualiza el estado de la cotización y la orden después de enviar exitosamente
#[tauri::command]
pub async fn send_cotizacion_email(cotizacion_id: i32, sent_by: i32) -> Result<String, String> {
    use crate::commands::cotizacion::{get_cotizacion_by_id, update_cotizacion};
    use crate::commands::ordenes_trabajo::{get_orden_trabajo_by_id, cambiar_estado_orden_trabajo};
    use crate::commands::equipos::get_equipo_by_id;
    use crate::commands::clientes::get_cliente_by_id;
    use crate::pdf::commands::generate_cotizacion_pdf_command;
    use crate::database::get_db_pool_safe;

    // Obtener la cotización
    let cotizacion = get_cotizacion_by_id(cotizacion_id).await?
        .ok_or_else(|| "Cotización no encontrada".to_string())?;

    // Generar el PDF (permite borradores, los actualizaremos después)
    println!("📄 Generando PDF de cotización {}...", cotizacion_id);
    let pdf_bytes = generate_cotizacion_pdf_command(cotizacion_id).await?;
    println!("✅ PDF generado exitosamente ({} bytes)", pdf_bytes.len());

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
    let cliente_id = equipo.cliente_id.ok_or_else(|| "El equipo no tiene cliente asociado".to_string())?;
    let cliente = get_cliente_by_id(cliente_id).await?
        .ok_or_else(|| "Cliente no encontrado".to_string())?;

    // Verificar que el cliente tenga email
    if cliente.cliente_correo.is_none() || cliente.cliente_correo.as_ref().unwrap().trim().is_empty() {
        return Err("El cliente no tiene email configurado".to_string());
    }

    // Crear el servicio de email
    let email_service = EmailService::new()
        .map_err(|e| format!("Error al inicializar servicio de email: {}", e))?;

    // Enviar el email con PDF
    println!("📧 Enviando email de cotización con PDF a {}...", cliente.cliente_correo.as_ref().unwrap());
    email_service.send_cotizacion_email_with_pdf(
        &cliente.cliente_correo.unwrap(),
        &cliente.cliente_nombre.unwrap_or_else(|| "Cliente".to_string()),
        &cotizacion,
        &orden_trabajo,
        &equipo,
        &pdf_bytes,
    ).await?;

    println!("✅ Email enviado exitosamente. Actualizando estados...");

    // Después de enviar exitosamente, actualizar el estado de la cotización
    let _ = update_cotizacion(
        cotizacion_id,
        crate::commands::cotizacion::UpdateCotizacionRequest {
            cotizacion_codigo: None,
            costo_revision: None,
            costo_reparacion: None,
            costo_total: None,
            is_aprobada: None,
            is_borrador: Some(false), // Marcar como enviada (no borrador)
            informe: None,
        },
        sent_by,
    ).await.map_err(|e| format!("Error actualizando estado de cotización: {}", e))?;

    println!("✅ Cotización marcada como enviada");

    // Cambiar el estado de la orden a "cotizacion_enviada" si está en "recibido"
    if let Some(estado_actual) = &orden_trabajo.estado {
        if estado_actual == "recibido" {
            let _ = cambiar_estado_orden_trabajo(
                orden_id,
                "cotizacion_enviada".to_string(),
                sent_by,
            ).await.map_err(|e| format!("Error actualizando estado de orden: {}", e))?;
            println!("✅ Estado de orden cambiado a 'cotizacion_enviada'");
        }
    }

    Ok("Email de cotización con PDF enviado exitosamente y estados actualizados".to_string())
}

/// Comando de Tauri para enviar email de informe con PDF al cliente cuando el equipo está listo para retiro
/// Se ejecuta automáticamente cuando la orden cambia a "espera_de_retiro"
#[tauri::command]
pub async fn send_informe_email(orden_id: i32, _sent_by: i32) -> Result<String, String> {
    use crate::commands::ordenes_trabajo::get_orden_trabajo_by_id;
    use crate::commands::informe::get_informe_by_id;
    use crate::commands::equipos::get_equipo_by_id;
    use crate::commands::clientes::get_cliente_by_id;
    use crate::pdf::commands::generate_informe_pdf_command;

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
    println!("📄 Generando PDF de informe {}...", informe_id);
    let pdf_bytes = generate_informe_pdf_command(informe_id).await?;
    println!("✅ PDF generado exitosamente ({} bytes)", pdf_bytes.len());

    // Obtener el equipo
    let equipo_id = orden_trabajo.equipo_id.ok_or_else(|| "La orden no tiene equipo asociado".to_string())?;
    let equipo = get_equipo_by_id(equipo_id).await?
        .ok_or_else(|| "Equipo no encontrado".to_string())?;

    // Obtener el cliente
    let cliente_id = equipo.cliente_id.ok_or_else(|| "El equipo no tiene cliente asociado".to_string())?;
    let cliente = get_cliente_by_id(cliente_id).await?
        .ok_or_else(|| "Cliente no encontrado".to_string())?;

    // Verificar que el cliente tenga email
    if cliente.cliente_correo.is_none() || cliente.cliente_correo.as_ref().unwrap().trim().is_empty() {
        return Err("El cliente no tiene email configurado".to_string());
    }

    // Crear el servicio de email
    let email_service = EmailService::new()
        .map_err(|e| format!("Error al inicializar servicio de email: {}", e))?;

    // Enviar el email con PDF
    println!("📧 Enviando email de informe con PDF a {}...", cliente.cliente_correo.as_ref().unwrap());
    email_service.send_informe_email_with_pdf(
        &cliente.cliente_correo.unwrap(),
        &cliente.cliente_nombre.unwrap_or_else(|| "Cliente".to_string()),
        &informe,
        &orden_trabajo,
        &equipo,
        &pdf_bytes,
    ).await?;

    println!("✅ Email de informe enviado exitosamente");

    Ok("Email de informe con PDF enviado exitosamente al cliente".to_string())
}
