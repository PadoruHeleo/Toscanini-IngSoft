use reqwest::Client;
use reqwest::header;
use std::env;

const DEFAULT_API_URL: &str = "http://localhost:3000/api";
const DEFAULT_API_TOKEN: &str = "mi_secreto_super_seguro_123";

pub fn get_api_key() -> String {
    env::var("API_TOKEN").unwrap_or_else(|_| DEFAULT_API_TOKEN.to_string())
}

pub fn get_http_client() -> Result<(Client, String), String> {
    let api_token = get_api_key();
     
    let mut headers = header::HeaderMap::new();
    let mut api_key_header_val = header::HeaderValue::from_str(&api_token)
        .map_err(|e| format!("Error en header x-api-key: {}", e))?;
    api_key_header_val.set_sensitive(true);
    headers.insert(header::HeaderName::from_static("x-api-key"), api_key_header_val);

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| format!("Error creando cliente HTTP: {}", e))?;

    let base_url = env::var("API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string());

    Ok((client, base_url))
}
