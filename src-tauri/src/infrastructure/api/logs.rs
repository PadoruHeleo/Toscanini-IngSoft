use crate::models::logs::{AuditLog, CreateAuditLogRequest, FiltrosLogs};
use crate::infrastructure::api::client::get_http_client;

fn get_base_url() -> Result<(reqwest::Client, String), String> {
    let (client, api_url) = get_http_client()?;
    Ok((client, format!("{}/logs", api_url)))
}

// Esta es la función clave que usan todos los demás módulos
pub async fn log_action(
    accion: &str,
    usuario_id: Option<i32>,
    entidad_tabla: &str,
    entidad_id: Option<i32>,
    prev_value: Option<&str>,
    new_value: Option<&str>,
) -> Result<(), String> {
    let request = CreateAuditLogRequest {
        log_accion: accion.to_string(),
        log_usuario_id: usuario_id,
        log_entidad_tabla: entidad_tabla.to_string(),
        log_entidad_id: entidad_id,
        log_prev_v: prev_value.map(|s| s.to_string()),
        log_new_v: new_value.map(|s| s.to_string()),
    };
    
    // Llamar a create_audit_log pero sin esperar el objeto de retorno (fire & forget style o await simple)
    let _ = create_audit_log(request).await?;
    Ok(())
}

pub async fn create_audit_log(request: CreateAuditLogRequest) -> Result<AuditLog, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.post(&base_url).json(&request).send().await.map_err(|e| e.to_string())?;
    response.json::<AuditLog>().await.map_err(|e| e.to_string())
}

pub async fn get_audit_log_by_id(log_id: i32) -> Result<Option<AuditLog>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, log_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if response.status() == reqwest::StatusCode::NOT_FOUND { return Ok(None); }
    response.json::<Option<AuditLog>>().await.map_err(|e| e.to_string())
}

pub async fn get_logs(limit: i64, offset: i64) -> Result<Vec<AuditLog>, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.get(&base_url)
        .query(&[("limit", limit), ("offset", offset)])
        .send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<AuditLog>>().await.map_err(|e| e.to_string())
}

pub async fn get_logs_filtrados(filtros: FiltrosLogs) -> Result<Vec<AuditLog>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/filter", base_url);
    let response = client.post(&url).json(&filtros).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<AuditLog>>().await.map_err(|e| e.to_string())
}

pub async fn get_acciones_logs() -> Result<Vec<String>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/list/acciones", base_url);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<String>>().await.map_err(|e| e.to_string())
}

pub async fn get_entidades_logs() -> Result<Vec<String>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/list/entidades", base_url);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<String>>().await.map_err(|e| e.to_string())
}