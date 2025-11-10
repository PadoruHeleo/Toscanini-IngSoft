use resend_rs::{Resend, types::CreateEmailBaseOptions};
use std::env;
use chrono_tz::America::Santiago;

pub struct EmailService {
    resend: Resend,
}

impl EmailService {
    pub fn new() -> Result<Self, String> {
        println!("🔧 Inicializando EmailService...");
        
        let api_key = env::var("RESEND_API_KEY")
            .map_err(|_| {
                eprintln!("❌ Variable de entorno RESEND_API_KEY no encontrada");
                eprintln!("🔍 Variables de entorno disponibles:");
                for (key, _) in env::vars() {
                    if key.contains("RESEND") || key.contains("EMAIL") || key.contains("API") {
                        eprintln!("   - {}", key);
                    }
                }
                "RESEND_API_KEY environment variable not found".to_string()
            })?;
        
        println!("✅ RESEND_API_KEY cargada correctamente (longitud: {})", api_key.len());
        
        let resend = Resend::new(&api_key);
        
        println!("✅ EmailService inicializado exitosamente");
        Ok(EmailService { resend })
    }

    pub async fn send_password_reset_email(&self, to_email: &str, reset_code: &str, user_name: &str) -> Result<(), String> {
        use std::env;
        
        // Verificar el entorno de ejecución
        let app_environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        
        let (from, to) = if app_environment == "development" {
            // En desarrollo: usar email de desarrollo
            let dev_email = env::var("DEV_EMAIL_RECIPIENT").unwrap_or_else(|_| "benitez.basti0@gmail.com".to_string());
            
            println!("🔧 MODO DESARROLLO: Enviando código de recuperación de contraseña");
            println!("📧 Enviando a email de desarrollo: {}", dev_email);
            println!("📧 En producción se enviaría a: {}", to_email);
            
            (
                "noreply@beniteztech.com".to_string(),
                vec![dev_email]
            )
        } else {
            // En producción: usar el email real del usuario
            println!("📧 MODO PRODUCCIÓN: Enviando código de recuperación a {}", to_email);
            (
                "noreply@beniteztech.com".to_string(),
                vec![to_email.to_string()]
            )
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

        let email = CreateEmailBaseOptions::new(&from, to.clone(), subject)
            .with_html(&html_content);

        match self.resend.emails.send(email).await {
            Ok(response) => {
                println!("✅ Email de recuperación enviado exitosamente: {:?}", response);
                if app_environment == "development" {
                    println!("🔑 Código de recuperación generado: {} (solo visible en desarrollo)", reset_code);
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ Error detallado enviando email de recuperación: {:?}", e);
                eprintln!("📧 Destinatario: {:?}", to);
                eprintln!("🔑 API Key configurada: {}", env::var("RESEND_API_KEY").is_ok());
                Err(format!("Error sending password reset email: {}", e))
            }
        }
    }
    pub async fn send_informe_email(
        &self, 
        to_email: &str, 
        client_name: &str, 
        informe: &crate::commands::informe::Informe,
        orden_trabajo: &crate::commands::ordenes_trabajo::OrdenTrabajo,
        piezas: &[crate::commands::informe::PiezaInforme]
    ) -> Result<(), String> {
        let from = "noreply@beniteztech.com";
        let to = vec![to_email.to_string()];
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

        let email = CreateEmailBaseOptions::new(from, to, subject)
            .with_html(&html_content);

        self.resend.emails.send(email).await
            .map_err(|e| format!("Error sending email: {}", e))?;        Ok(())
    }

    pub async fn send_orden_trabajo_notification(
        &self, 
        orden_trabajo: &crate::commands::ordenes_trabajo::OrdenTrabajo,
        equipo: &crate::commands::equipos::Equipo,
        cliente_nombre: &str
    ) -> Result<String, String> {
        use std::env;
        
        // Verificar el entorno de ejecución
        let app_environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        
        let (from, to, log_message) = if app_environment == "development" {
            // En desarrollo: usar email de desarrollo o simular envío
            let dev_email = env::var("DEV_EMAIL_RECIPIENT").unwrap_or_else(|_| "benitez.basti0@gmail.com".to_string());
            
            println!("🔧 MODO DESARROLLO: Simulando envío de notificación de orden de trabajo");
            println!("📧 En producción se enviaría a los administradores y técnicos de la BD");
            
            // Obtener emails para logging (sin enviar)
            let db_emails = crate::commands::users::get_admin_and_tech_emails().await.unwrap_or_default();
            println!("📋 Emails que recibirían en producción: {:?}", db_emails);
            
            (
                "noreply@beniteztech.com".to_string(),
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
                "noreply@beniteztech.com".to_string(),
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
        
        if app_environment == "development" && to.iter().any(|email| is_test_email(email)) {
            println!("📧 SIMULANDO envío de email a: {:?}", to);
            println!("📄 Asunto: {}", subject);
            println!("✅ Email simulado correctamente (no se envió realmente)");
            return Ok(log_message);
        }
        
        // Enviar email real (en producción o desarrollo con email válido)
        let email = CreateEmailBaseOptions::new(&from, to.clone(), &subject)
            .with_html(&html_content);

        match self.resend.emails.send(email).await {
            Ok(response) => {
                println!("📧 Email enviado exitosamente a: {:?}", to);
                println!("📋 Respuesta: {:?}", response);
                Ok(log_message)
            }
            Err(e) => {
                eprintln!("❌ Error enviando email: {:?}", e);
                Err(format!("Error sending email: {}", e))
            }
        }
    }

    pub async fn send_orden_trabajo_cliente(
        &self,
        cliente_email: &str,
        cliente_nombre: &str,
        orden_trabajo: &crate::commands::ordenes_trabajo::OrdenTrabajo,
        equipo: &crate::commands::equipos::Equipo,
    ) -> Result<(), String> {
        let from = "noreply@beniteztech.com";
        let to = vec![cliente_email.to_string()];
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

        let email = CreateEmailBaseOptions::new(from, to, subject)
            .with_html(&html_content);

        self.resend.emails.send(email).await
            .map_err(|e| format!("Error enviando email al cliente: {}", e))?;

        Ok(())
    }

    pub async fn send_password_email(
        &self,
        to_email: &str,
        user_name: &str,
        temp_password: &str,
    ) -> Result<(), String> {
        let from = "noreply@beniteztech.com";
        let to = vec![to_email.to_string()];
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

        let email = CreateEmailBaseOptions::new(from, to, subject)
            .with_html(&html_content);

        match self.resend.emails.send(email).await {
            Ok(response) => {
                println!("Email enviado exitosamente: {:?}", response);
                Ok(())
            }
            Err(e) => {
                eprintln!("Error detallado enviando email: {:?}", e);
                Err(format!("Error sending email: {}", e))
            }
        }
    }

    /// Enviar email de cotización con PDF adjunto usando API REST de Resend
    pub async fn send_cotizacion_email_with_pdf(
        &self,
        cliente_email: &str,
        cliente_nombre: &str,
        cotizacion: &crate::commands::cotizacion::Cotizacion,
        orden_trabajo: &crate::commands::ordenes_trabajo::OrdenTrabajo,
        equipo: &crate::commands::equipos::Equipo,
        pdf_bytes: &[u8],
    ) -> Result<(), String> {
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        use reqwest::Client;
        use serde_json::json;
        use std::env;

        let api_key = env::var("RESEND_API_KEY")
            .map_err(|_| "RESEND_API_KEY no encontrada".to_string())?;

        // Nombre del archivo PDF
        let pdf_filename = format!(
            "Cotizacion_{}.pdf",
            cotizacion.cotizacion_codigo.as_deref().unwrap_or("N/A")
        );

        // Codificar PDF en base64
        let pdf_base64 = STANDARD.encode(pdf_bytes);

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

        // Preparar el payload para Resend API
        let payload = json!({
            "from": "noreply@beniteztech.com",
            "to": [cliente_email],
            "subject": format!(
                "Cotización de Reparación - Toscanini (Código: {})",
                cotizacion.cotizacion_codigo.as_deref().unwrap_or("N/A")
            ),
            "html": html_content,
            "attachments": [{
                "filename": pdf_filename,
                "content": pdf_base64,
                "content_type": "application/pdf"
            }]
        });

        let client = Client::new();
        let response = client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Error enviando request a Resend API: {}", e))?;

        let status = response.status();
        if status.is_success() {
            println!("📧 Email de cotización con PDF enviado exitosamente a: {}", cliente_email);
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Error desconocido".to_string());
            eprintln!("❌ Error de API Resend: {}", error_text);
            Err(format!("Error enviando email: Status {} - {}", status, error_text))
        }
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
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        use reqwest::Client;
        use serde_json::json;
        use std::env;

        let api_key = env::var("RESEND_API_KEY")
            .map_err(|_| "RESEND_API_KEY no encontrada".to_string())?;

        // Nombre del archivo PDF
        let pdf_filename = format!(
            "Informe_{}.pdf",
            informe.informe_codigo.as_deref().unwrap_or("N/A")
        );

        // Codificar PDF en base64
        let pdf_base64 = STANDARD.encode(pdf_bytes);

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

        // Preparar el payload para Resend API
        let payload = json!({
            "from": "noreply@beniteztech.com",
            "to": [cliente_email],
            "subject": format!("✅ Tu equipo está listo para retiro - {}", orden_trabajo.orden_codigo.as_deref().unwrap_or("N/A")),
            "html": html_content,
            "attachments": [
                {
                    "filename": pdf_filename,
                    "content": pdf_base64
                }
            ]
        });

        // Enviar el email usando reqwest directamente (ya que resend_rs no soporta attachments fácilmente)
        let client = Client::new();
        let response = client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Error enviando request a Resend API: {}", e))?;

        let status = response.status();
        if status.is_success() {
            println!("📧 Email de informe con PDF enviado exitosamente a: {}", cliente_email);
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Error desconocido".to_string());
            eprintln!("❌ Error de API Resend: {}", error_text);
            Err(format!("Error enviando email: Status {} - {}", status, error_text))
        }
    }
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
