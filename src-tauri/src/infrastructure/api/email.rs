

// API implementation for email (stub)
// Since email is currently handled locally via SMTP, these functions might not be supported via API yet.
// Or the API might have an endpoint to trigger emails.
// For now, we return errors or implement basic stubs.

pub async fn send_test_email(_to_email: String) -> Result<String, String> {
    Err("Sending test email via API is not supported yet".to_string())
}

pub async fn send_orden_trabajo_cliente(_orden_id: i32) -> Result<String, String> {
    Err("Sending OT email via API is not supported yet".to_string())
}

pub async fn send_cotizacion_email(_cotizacion_id: i32, _sent_by: i32) -> Result<String, String> {
    Err("Sending cotizacion email via API is not supported yet".to_string())
}

pub async fn send_informe_email(_orden_id: i32, _sent_by: i32) -> Result<String, String> {
    Err("Sending informe email via API is not supported yet".to_string())
}
