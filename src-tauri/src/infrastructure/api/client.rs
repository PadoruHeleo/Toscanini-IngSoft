use reqwest::Client;
use reqwest::header;
use std::env;

const DEFAULT_API_URL: &str = "http://localhost:3000/api";
const API_TOKEN: &str = "mi_secreto_super_seguro_123";

pub fn get_http_client() -> Result<(Client, String), String> {
    let mut headers = header::HeaderMap::new();
    let auth_value = format!("Bearer {}", API_TOKEN);
    let mut auth_header_val = header::HeaderValue::from_str(&auth_value)
        .map_err(|e| format!("Error en header auth: {}", e))?;
    auth_header_val.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_header_val);

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| format!("Error creando cliente HTTP: {}", e))?;

    let base_url = env::var("API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string());

    Ok((client, base_url))
}
