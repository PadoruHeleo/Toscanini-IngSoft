use crate::config::AppConfig;
use crate::models::email::EmailConfig;
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tauri::State;
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
    async fn send_email_internal(&self, to: &str, subject: &str, html_content: &str, attachment: Option<(&[u8], &str)>) -> Result<(), String> {
        let email = self.build_email(to, subject, html_content, attachment)?;
        
        // Enviar email (bloqueante, pero ejecutado en contexto async)
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
        let subject = format!("Orden de Trabajo - {}", orden.orden_codigo.as_deref().unwrap_or("???"));
        
        let html_content = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <body>
                <h2>Detalles de su Orden de Trabajo</h2>
                <p>Estimado/a {},</p>
                <p>Adjuntamos los detalles de su orden de trabajo.</p>
                <ul>
                    <li><strong>Orden:</strong> {}</li>
                    <li><strong>Equipo:</strong> {} {}</li>
                    <li><strong>Estado:</strong> {}</li>
                </ul>
                <p>Atentamente,<br>Equipo Toscanini</p>
            </body>
            </html>
            "#,
            cliente_nombre,
            orden.orden_codigo.as_deref().unwrap_or("N/A"),
            equipo.equipo_marca.as_deref().unwrap_or(""),
            equipo.equipo_modelo.as_deref().unwrap_or(""),
            orden.estado.as_deref().unwrap_or("Recibido")
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

// --- COMANDOS TAURI ---

#[tauri::command]
pub async fn send_test_email(state: State<'_, AppConfig>, to_email: String) -> Result<String, String> {
    println!("📧 [test_email_send] Iniciando prueba de envío de correo a: {}", to_email);
    
    let email_config = state.email_config.as_ref().ok_or("Configuración de email no encontrada")?;
    
    let email_service = EmailService::new(email_config)
        .map_err(|e| {
            let error_msg = format!("Error al inicializar servicio de email: {}", e);
            println!("❌ [test_email_send] {}", error_msg);
            error_msg
        })?;
    
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
        println!("❌ [test_email_send] {}", error_msg);
        error_msg
    })?;
    
    println!("✅ [test_email_send] Correo de prueba enviado exitosamente a: {}", to_email);
    
    Ok(format!("Correo de prueba enviado exitosamente a {}", to_email))
}

/// Comando de Tauri para enviar email de orden de trabajo al cliente
#[tauri::command]
pub async fn send_orden_trabajo_cliente(state: State<'_, AppConfig>, orden_id: i32, _sent_by: i32) -> Result<String, String> {
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
    let email_config = state.email_config.as_ref().ok_or("Configuración de email no encontrada")?;
    let email_service = EmailService::new(email_config)
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
pub async fn send_cotizacion_email(state: State<'_, AppConfig>, cotizacion_id: i32, sent_by: i32) -> Result<String, String> {
    use crate::infrastructure::db::cotizacion::{get_cotizacion_by_id, update_cotizacion};
    use crate::infrastructure::db::ordenes_trabajo::{get_orden_trabajo_by_id, cambiar_estado_orden_trabajo};
    use crate::infrastructure::db::equipos::get_equipo_by_id;
    use crate::infrastructure::db::clientes::get_cliente_by_id;
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
    let cliente_id = equipo.cliente_id.ok_or_else(|| "Equipo sin cliente asociado".to_string())?;
    let cliente = get_cliente_by_id(cliente_id).await?
        .ok_or_else(|| "Cliente no encontrado".to_string())?;

    // Verificar que el cliente tenga email
    if cliente.cliente_correo.is_none() || cliente.cliente_correo.as_ref().unwrap().trim().is_empty() {
        return Err("El cliente no tiene email configurado".to_string());
    }

    // Crear el servicio de email
    let email_config = state.email_config.as_ref().ok_or("Configuración de email no encontrada")?;
    let email_service = EmailService::new(email_config)
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
pub async fn send_informe_email(state: State<'_, AppConfig>, orden_id: i32, _sent_by: i32) -> Result<String, String> {
    use crate::infrastructure::db::ordenes_trabajo::get_orden_trabajo_by_id;
    use crate::infrastructure::db::informe::get_informe_by_id;
    use crate::infrastructure::db::equipos::get_equipo_by_id;
    use crate::infrastructure::db::clientes::get_cliente_by_id;
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
    let cliente_id = equipo.cliente_id.ok_or_else(|| "Equipo sin cliente asociado".to_string())?;
    let cliente = get_cliente_by_id(cliente_id).await?
        .ok_or_else(|| "Cliente no encontrado".to_string())?;

    // Verificar que el cliente tenga email
    if cliente.cliente_correo.is_none() || cliente.cliente_correo.as_ref().unwrap().trim().is_empty() {
        return Err("El cliente no tiene email configurado".to_string());
    }

    // Crear el servicio de email
    let email_config = state.email_config.as_ref().ok_or("Configuración de email no encontrada")?;
    let email_service = EmailService::new(email_config)
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

#[tauri::command]
pub async fn test_email_send(state: State<'_, AppConfig>, to_email: String) -> Result<String, String> {
    println!("📧 [test_email_send] Iniciando prueba de envío de correo a: {}", to_email);
    
    let email_config = state.email_config.as_ref().ok_or("Configuración de email no encontrada")?;
    
    let email_service = EmailService::new(email_config)
        .map_err(|e| {
            let error_msg = format!("Error al inicializar servicio de email: {}", e);
            println!("❌ [test_email_send] {}", error_msg);
            error_msg
        })?;
    
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
        println!("❌ [test_email_send] {}", error_msg);
        error_msg
    })?;
    
    println!("✅ [test_email_send] Correo de prueba enviado exitosamente a: {}", to_email);
    
    Ok(format!("Correo de prueba enviado exitosamente a {}", to_email))
}
