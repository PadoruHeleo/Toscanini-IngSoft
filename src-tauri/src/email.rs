use resend_rs::{Resend, types::CreateEmailBaseOptions};
use std::env;

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
            .map_err(|e| format!("Error sending email: {}", e))?;

        Ok(())
    }
}
