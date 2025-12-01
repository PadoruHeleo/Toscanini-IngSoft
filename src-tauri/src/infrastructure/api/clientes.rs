use crate::models::clientes::{
    Cliente,
    CreateClienteRequest,
    UpdateClienteRequest,
    FiltrosClientes,
    DeleteClienteRequest,
    // Importamos los structs auxiliares para las listas
    // Asegúrate de que estén definidos y sean públicos en models/clientes.rs
};
use crate::infrastructure::api::client::get_http_client;
use serde_json::json;

// Helper para construir la URL base
fn get_base_url() -> Result<(reqwest::Client, String), String> {
    let (client, api_url) = get_http_client()?;
    Ok((client, format!("{}/clientes", api_url)))
}

pub async fn get_clientes() -> Result<Vec<Cliente>, String> {
    let (client, base_url) = get_base_url()?;
    
    let response = client.get(&base_url)
        .send()
        .await
        .map_err(|e| format!("Error de conexión: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }

    response.json::<Vec<Cliente>>()
        .await
        .map_err(|e| format!("Error decodificando respuesta: {}", e))
}

pub async fn get_cliente_by_id(cliente_id: i32) -> Result<Option<Cliente>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, cliente_id);

    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }

    response.json::<Option<Cliente>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_cliente_by_rut(cliente_rut: String) -> Result<Option<Cliente>, String> {
    let (client, base_url) = get_base_url()?;
    // Codificamos el RUT por seguridad (aunque suele ser seguro)
    let encoded_rut = urlencoding::encode(&cliente_rut);
    let url = format!("{}/rut/{}", base_url, encoded_rut);

    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }

    response.json::<Option<Cliente>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_clientes_by_created_by(created_by: i32) -> Result<Vec<Cliente>, String> {
    // Como no definimos un endpoint específico para esto en el controlador,
    // reutilizamos get_clientes y filtramos en memoria (estrategia segura)
    // Opcionalmente, podrías usar el endpoint de filtros si lo prefieres.
    let todos = get_clientes().await?;
    
    let filtrados = todos.into_iter()
        .filter(|c| c.created_by == Some(created_by))
        .collect();
        
    Ok(filtrados)
}

pub async fn search_clientes(search_term: String) -> Result<Vec<Cliente>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/search/query", base_url);

    let response = client.get(&url)
        .query(&[("term", search_term)])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }

    response.json::<Vec<Cliente>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_cliente(request: CreateClienteRequest) -> Result<Cliente, String> {
    let (client, base_url) = get_base_url()?;

    let response = client.post(&base_url)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        // Intentamos leer el mensaje de error del backend (ej: "RUT duplicado")
        let error_msg = response.text().await.unwrap_or_else(|_| "Error desconocido".to_string());
        // Parseamos el JSON de error si es posible {"message": "..."}
        if let Ok(json_err) = serde_json::from_str::<serde_json::Value>(&error_msg) {
            if let Some(msg) = json_err.get("message").and_then(|v| v.as_str()) {
                return Err(msg.to_string());
            }
        }
        return Err(error_msg);
    }

    response.json::<Cliente>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_cliente(cliente_id: i32, request: UpdateClienteRequest, updated_by: i32) -> Result<Option<Cliente>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, cliente_id);

    // Tu controlador espera 'updated_by' en el body, pero el struct Request no lo tiene.
    // Combinamos ambos en un JSON dinámico.
    let mut body_json = serde_json::to_value(&request).map_err(|e| e.to_string())?;
    
    if let Some(obj) = body_json.as_object_mut() {
        obj.insert("updated_by".to_string(), json!(updated_by));
    }

    let response = client.put(&url)
        .json(&body_json)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    
    if !response.status().is_success() {
        let error_msg = response.text().await.unwrap_or_default();
        if let Ok(json_err) = serde_json::from_str::<serde_json::Value>(&error_msg) {
            if let Some(msg) = json_err.get("message").and_then(|v| v.as_str()) {
                return Err(msg.to_string());
            }
        }
        return Err(error_msg);
    }

    // La API devuelve el cliente actualizado
    let cliente = response.json::<Cliente>()
        .await
        .map_err(|e| e.to_string())?;
        
    Ok(Some(cliente))
}

pub async fn delete_cliente(request: DeleteClienteRequest) -> Result<bool, String> {
    let (client, base_url) = get_base_url()?;
    // Mapeado a POST /delete según tu API_MAPPING_CLIENTES.md
    let url = format!("{}/delete", base_url);

    let response = client.post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_msg = response.text().await.unwrap_or_default();
        if let Ok(json_err) = serde_json::from_str::<serde_json::Value>(&error_msg) {
            if let Some(msg) = json_err.get("message").and_then(|v| v.as_str()) {
                return Err(msg.to_string());
            }
        }
        return Err(error_msg);
    }

    Ok(true)
}

pub async fn reactivate_cliente(cliente_id: i32, reactivated_by: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/reactivate", base_url);

    let body = json!({
        "cliente_id": cliente_id,
        "reactivated_by": reactivated_by
    });

    let response = client.post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_msg = response.text().await.unwrap_or_default();
        if let Ok(json_err) = serde_json::from_str::<serde_json::Value>(&error_msg) {
            if let Some(msg) = json_err.get("message").and_then(|v| v.as_str()) {
                return Err(msg.to_string());
            }
        }
        return Err(error_msg);
    }

    Ok(true)
}

pub async fn count_clientes() -> Result<i64, String> {
    // Si no creaste un endpoint /count, obtenemos todos y contamos (menos eficiente pero funcional)
    let clientes = get_clientes().await?;
    Ok(clientes.len() as i64)
}

pub async fn get_clientes_with_pagination(offset: i64, limit: i64) -> Result<Vec<Cliente>, String> {
    // La estrategia ideal es usar el endpoint de filtros que ya implementaste
    let filtros = FiltrosClientes {
        fecha_inicio: None,
        fecha_fin: None,
        correo: None,
        rut: None,
        ciudad: None,
        search: None,
        estado: Some(vec![true]), // Solo activos por defecto
        ordenamiento: None,
    };
    
    // Nota: Si tu endpoint /filter no soporta limit/offset en el JSON,
    // tendrás que obtener todos y cortar el vector aquí (slicing).
    // Asumiendo slicing local por seguridad:
    let todos = get_clientes_filtrados(filtros).await?;
    
    let start = offset as usize;
    if start >= todos.len() {
        return Ok(Vec::new());
    }
    
    let end = std::cmp::min(start + limit as usize, todos.len());
    Ok(todos[start..end].to_vec())
}

pub async fn get_clientes_filtrados(filtros: FiltrosClientes) -> Result<Vec<Cliente>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/filter", base_url);

    let response = client.post(&url)
        .json(&filtros)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }

    response.json::<Vec<Cliente>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_ruts_clientes() -> Result<Vec<String>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/list/ruts", base_url);

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Vec<String>>().await.map_err(|e| e.to_string())
}

pub async fn get_correos_clientes() -> Result<Vec<String>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/list/emails", base_url);

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Vec<String>>().await.map_err(|e| e.to_string())
}

pub async fn get_ciudades_clientes() -> Result<Vec<String>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/list/ciudades", base_url);

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Vec<String>>().await.map_err(|e| e.to_string())
}