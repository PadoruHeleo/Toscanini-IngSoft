

// API implementation for email (stub)
// Since email is currently handled locally via SMTP, these functions might not be supported via API yet.
// Or the API might have an endpoint to trigger emails.
// For now, we return errors or implement basic stubs.

pub async fn send_test_email(_to_email: String) -> Result<String, String> {
    Ok("Email sending skipped (API mode)".to_string())
}

pub async fn send_orden_trabajo_cliente(_orden_id: i32) -> Result<String, String> {
    // API doesn't support email sending yet, so we just return success to avoid UI errors.
    Ok("Email sending skipped (API mode)".to_string())
}

pub async fn send_cotizacion_email(_cotizacion_id: i32, _sent_by: i32) -> Result<String, String> {
    Ok("Email sending skipped (API mode)".to_string())
}

pub async fn send_informe_email(_orden_id: i32, _sent_by: i32) -> Result<String, String> {
    Ok("Email sending skipped (API mode)".to_string())
}
