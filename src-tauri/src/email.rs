use resend_rs::{Resend, types::CreateEmailBaseOptions};
use std::env;
use chrono::{DateTime, Utc, NaiveDateTime};
use chrono_tz::America::Santiago;

pub struct EmailService {
    resend: Resend,
}

impl EmailService {
    pub fn new() -> Result<Self, String> {
        let api_key = env::var("RESEND_API_KEY")
            .map_err(|_| "RESEND_API_KEY environment variable not found".to_string())?;
        
        let resend = Resend::new(&api_key);
        
        Ok(EmailService { resend })
    }

    pub async fn send_password_reset_email(&self, to_email: &str, reset_code: &str, user_name: &str) -> Result<(), String> {
        let from = "onboarding@resend.dev"; // Cambiar por tu dominio verificado
        let to = vec![to_email.to_string()];
        let subject = "Recuperación de Contraseña - Toscanini";

        let html_content = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
                <h2 style="color: #333; text-align: center;">Recuperación de Contraseña</h2>
                <p>Hola <strong>{}</strong>,</p>
                <p>Hemos recibido una solicitud para restablecer la contraseña de tu cuenta en Toscanini.</p>
                <div style="background-color: #f5f5f5; padding: 20px; margin: 20px 0; text-align: center; border-radius: 5px;">
                    <p style="margin: 0; font-size: 18px;">Tu código de verificación es:</p>
                    <h1 style="color: #007bff; font-size: 32px; margin: 10px 0; letter-spacing: 5px;">{}</h1>
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
            user_name, reset_code
        );

        let email = CreateEmailBaseOptions::new(from, to, subject)
            .with_html(&html_content);

        self.resend.emails.send(email).await
            .map_err(|e| format!("Error sending email: {}", e))?;

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
        let from = "onboarding@resend.dev"; // Cambiar por tu dominio verificado
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
    ) -> Result<(), String> {
        let from = "onboarding@resend.dev"; // Cambiar por tu dominio verificado
        let to = vec!["benitez.basti0@gmail.com".to_string()];
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

        let email = CreateEmailBaseOptions::new(from, to, subject)
            .with_html(&html_content);

        self.resend.emails.send(email).await
            .map_err(|e| format!("Error sending email: {}", e))?;

        Ok(())
    }

    pub async fn send_orden_trabajo_cliente(
        &self,
        cliente_email: &str,
        cliente_nombre: &str,
        orden_trabajo: &crate::commands::ordenes_trabajo::OrdenTrabajo,
        equipo: &crate::commands::equipos::Equipo,
    ) -> Result<(), String> {
        let from = "onboarding@resend.dev"; // Cambia por tu dominio verificado
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
        let from = "onboarding@resend.dev"; // Cambia por tu dominio verificado
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

        self.resend.emails.send(email).await
            .map_err(|e| format!("Error sending email: {}", e))?;

        Ok(())
    }
}
