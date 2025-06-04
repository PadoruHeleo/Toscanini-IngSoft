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
}
