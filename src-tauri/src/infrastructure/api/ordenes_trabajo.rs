use crate::models::ordenes_trabajo::{
    OrdenTrabajo, CreateOrdenTrabajoRequest, UpdateOrdenTrabajoRequest,
    OrdenTrabajoDetallada, Filtros
};
use crate::infrastructure::api::client::get_http_client;
use std::collections::HashMap;

fn get_base_url() -> Result<(reqwest::Client, String), String> {
    let (client, api_url) = get_http_client()?;
    Ok((client, format!("{}/ordenes-trabajo", api_url)))
}

pub async fn get_ordenes_trabajo() -> Result<Vec<OrdenTrabajo>, String> {
    let (client, base_url) = get_base_url()?;
    println!("🔍 Llamando a: {}", base_url);
    let response = client.get(&base_url).send().await.map_err(|e| {
        let error_msg = format!("Error de conexión: {}", e);
        println!("❌ {}", error_msg);
        error_msg
    })?;
    
    let status = response.status();
    println!("📊 Status code: {}", status);
    
    if !status.is_success() {
        let error_body = response.text().await.unwrap_or_else(|_| "No se pudo leer el cuerpo del error".to_string());
        let error_msg = format!("Error API ({}): {}", status, error_body);
        println!("❌ {}", error_msg);
        return Err(error_msg);
    }
    
    response.json::<Vec<OrdenTrabajo>>().await.map_err(|e| {
        let error_msg = format!("Error parseando JSON: {}", e);
        println!("❌ {}", error_msg);
        error_msg
    })
}

pub async fn get_orden_trabajo_by_id(orden_id: i32) -> Result<Option<OrdenTrabajo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, orden_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if response.status() == reqwest::StatusCode::NOT_FOUND { return Ok(None); }
    response.json::<Option<OrdenTrabajo>>().await.map_err(|e| e.to_string())
}

pub async fn create_orden_trabajo(request: CreateOrdenTrabajoRequest) -> Result<OrdenTrabajo, String> {
    let (client, base_url) = get_base_url()?;
    let response = client.post(&base_url).json(&request).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        let error_msg = response.text().await.unwrap_or_default();
        return Err(format!("Error API: {}", error_msg));
    }

    let body_text = response.text().await.map_err(|e| e.to_string())?;
    println!("📦 Respuesta Create Orden: {}", body_text);

    serde_json::from_str(&body_text).map_err(|e| format!("Error decoding response: {} - Body: {}", e, body_text))
}

pub async fn update_orden_trabajo(orden_id: i32, request: UpdateOrdenTrabajoRequest, _updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}", base_url, orden_id);
    let response = client.put(&url).json(&request).send().await.map_err(|e| e.to_string())?;
    if response.status() == reqwest::StatusCode::NOT_FOUND { return Ok(None); }
    response.json::<Option<OrdenTrabajo>>().await.map_err(|e| e.to_string())
}

pub async fn delete_orden_trabajo(orden_id: i32, _deleted_by: i32) -> Result<bool, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/delete", base_url);
    
    let body = serde_json::json!({
        "orden_id": orden_id,
        "deleted_by": _deleted_by
    });
    
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    Ok(response.status().is_success())
}

pub async fn get_ordenes_trabajo_by_cliente(cliente_id: i32) -> Result<Vec<OrdenTrabajo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/cliente/{}", base_url, cliente_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<OrdenTrabajo>>().await.map_err(|e| e.to_string())
}

pub async fn get_ordenes_trabajo_by_equipo(equipo_id: i32) -> Result<Vec<OrdenTrabajo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/equipo/{}", base_url, equipo_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json::<Vec<OrdenTrabajo>>().await.map_err(|e| e.to_string())
}

// --- Implementación de Filtros y Lógica en Cliente (Rust) ---

// Helper functions to fetch data
async fn get_all_data() -> Result<(Vec<OrdenTrabajo>, Vec<crate::models::equipos::Equipo>, Vec<crate::models::clientes::Cliente>), String> {
    let ordenes = get_ordenes_trabajo().await?;
    // Usamos las funciones de los otros módulos API
    let equipos = crate::infrastructure::api::equipos::get_equipos().await?;
    let clientes = crate::infrastructure::api::clientes::get_clientes().await?;
    Ok((ordenes, equipos, clientes))
}

pub async fn get_orden_trabajo_by_codigo(_orden_codigo: String) -> Result<Option<OrdenTrabajo>, String> {
    let ordenes = get_ordenes_trabajo().await?;
    Ok(ordenes.into_iter().find(|o| o.orden_codigo.as_deref() == Some(&_orden_codigo)))
}

pub async fn get_ordenes_trabajo_by_estado(estado: String) -> Result<Vec<OrdenTrabajo>, String> {
    let ordenes = get_ordenes_trabajo().await?;
    Ok(ordenes.into_iter().filter(|o| o.estado.as_deref() == Some(&estado)).collect())
}

pub async fn get_ordenes_trabajo_by_prioridad(prioridad: String) -> Result<Vec<OrdenTrabajo>, String> {
    let ordenes = get_ordenes_trabajo().await?;
    Ok(ordenes.into_iter().filter(|o| o.prioridad.as_deref() == Some(&prioridad)).collect())
}

pub async fn get_ordenes_trabajo_by_usuario(usuario_id: i32) -> Result<Vec<OrdenTrabajo>, String> {
    let ordenes = get_ordenes_trabajo().await?;
    Ok(ordenes.into_iter().filter(|o| o.created_by == Some(usuario_id)).collect())
}

pub async fn get_ordenes_trabajo_detalladas() -> Result<Vec<OrdenTrabajoDetallada>, String> {
    let (ordenes, equipos, clientes) = get_all_data().await?;
    
    let equipos_map: HashMap<i32, &crate::models::equipos::Equipo> = 
        equipos.iter().map(|e| (e.equipo_id, e)).collect();
        
    let clientes_map: HashMap<i32, &crate::models::clientes::Cliente> = 
        clientes.iter().map(|c| (c.cliente_id, c)).collect();

    let mut detalladas = Vec::new();

    for o in ordenes {
        let equipo = o.equipo_id.and_then(|id| equipos_map.get(&id));
        let cliente = equipo.and_then(|e| e.cliente_id).and_then(|id| clientes_map.get(&id));
        
        detalladas.push(OrdenTrabajoDetallada {
            orden_id: o.orden_id,
            orden_codigo: o.orden_codigo,
            orden_desc: o.orden_desc,
            prioridad: o.prioridad,
            estado: o.estado,
            has_garantia: o.has_garantia,
            equipo_id: o.equipo_id,
            created_by: o.created_by,
            cotizacion_id: o.cotizacion_id,
            informe_id: o.informe_id,
            pre_informe: o.pre_informe,
            created_at: o.created_at,
            finished_at: o.finished_at,
            
            numero_serie: equipo.and_then(|e| e.numero_serie.clone()),
            equipo_marca: equipo.and_then(|e| e.equipo_marca.clone()),
            equipo_modelo: equipo.and_then(|e| e.equipo_modelo.clone()),
            equipo_tipo: equipo.and_then(|e| e.equipo_tipo.clone()),
            
            cliente_id: cliente.map(|c| c.cliente_id),
            cliente_nombre: cliente.and_then(|c| c.cliente_nombre.clone()),
            
            creador_nombre: None,
            cotizacion_codigo: None,
            estados: None,
        });
    }
    
    // Ordenar por fecha de creación descendente
    detalladas.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    Ok(detalladas)
}

pub async fn get_orden_trabajo_detallada_by_id(orden_id: i32) -> Result<Option<OrdenTrabajoDetallada>, String> {
    let detalladas = get_ordenes_trabajo_detalladas().await?;
    Ok(detalladas.into_iter().find(|o| o.orden_id == orden_id))
}

pub async fn cambiar_estado_orden_trabajo(orden_id: i32, nuevo_estado: String, updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}/estado", base_url, orden_id);
    
    let body = serde_json::json!({
        "nuevo_estado": nuevo_estado,
        "updated_by": updated_by
    });

    let response = client.patch(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<Option<OrdenTrabajo>>().await.map_err(|e| e.to_string())
}

pub async fn asignar_cotizacion_orden_trabajo(orden_id: i32, cotizacion_id: i32, updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let (client, base_url) = get_base_url()?;
    // Intentamos primero con el endpoint corregido
    let url = format!("{}/associate-cotizacion", base_url);
    
    let body = serde_json::json!({
        "orden_id": orden_id,
        "cotizacion_id": cotizacion_id,
        "updated_by": updated_by
    });
    
    println!("🔗 Asignando cotización {} a orden {} en URL: {}", cotizacion_id, orden_id, url);

    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            println!("⚠️ Endpoint /associate-cotizacion no encontrado (404). Intentando fallback a /associate/cotizacion");
            let fallback_url = format!("{}/associate/cotizacion", base_url);
            let fallback_response = client.post(&fallback_url).json(&body).send().await.map_err(|e| e.to_string())?;
            
            if !fallback_response.status().is_success() {
                let error_msg = fallback_response.text().await.unwrap_or_default();
                println!("❌ Error en fallback asignando cotización: {}", error_msg);
                return Err(format!("Error API (Fallback): {}", error_msg));
            }
        } else {
            let error_msg = response.text().await.unwrap_or_default();
            println!("❌ Error asignando cotización: {}", error_msg);
            return Err(format!("Error API: {}", error_msg));
        }
    }

    // Hacemos un get explícito para devolver la orden actualizada.
    get_orden_trabajo_by_id(orden_id).await
}

pub async fn asignar_informe_orden_trabajo(orden_id: i32, informe_id: i32, updated_by: i32) -> Result<Option<OrdenTrabajo>, String> {
    let (client, base_url) = get_base_url()?;
    // Corregido según API_MAPPING_ORDEN_TRABAJO.md: /associate-informe
    let url = format!("{}/associate-informe", base_url);
    
    let body = serde_json::json!({
        "orden_id": orden_id,
        "informe_id": informe_id,
        "updated_by": updated_by
    });
    
    println!("🔗 Asignando informe {} a orden {} en URL: {}", informe_id, orden_id, url);

    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        let error_msg = response.text().await.unwrap_or_default();
        println!("❌ Error asignando informe: {}", error_msg);
        return Err(format!("Error API: {}", error_msg));
    }

    get_orden_trabajo_by_id(orden_id).await
}

pub async fn get_ordenes_trabajo_stats() -> Result<serde_json::Value, String> {
    let ordenes = get_ordenes_trabajo().await?;
    
    let total = ordenes.len();
    let con_garantia = ordenes.iter().filter(|o| o.has_garantia.unwrap_or(false)).count();
    
    let mut por_estado = HashMap::new();
    let mut por_prioridad = HashMap::new();
    
    for o in &ordenes {
        if let Some(est) = &o.estado {
            *por_estado.entry(est.clone()).or_insert(0) += 1;
        }
        if let Some(prio) = &o.prioridad {
            *por_prioridad.entry(prio.clone()).or_insert(0) += 1;
        }
    }
    
    Ok(serde_json::json!({
        "total": total,
        "con_garantia": con_garantia,
        "por_estado": por_estado.into_iter().map(|(k, v)| serde_json::json!({"estado": k, "count": v})).collect::<Vec<_>>(),
        "por_prioridad": por_prioridad.into_iter().map(|(k, v)| serde_json::json!({"prioridad": k, "count": v})).collect::<Vec<_>>()
    }))
}

pub async fn search_ordenes_trabajo(search_term: String) -> Result<Vec<OrdenTrabajoDetallada>, String> {
    let detalladas = get_ordenes_trabajo_detalladas().await?;
    let term = search_term.to_lowercase();
    
    Ok(detalladas.into_iter().filter(|o| {
        o.orden_codigo.as_ref().map_or(false, |s| s.to_lowercase().contains(&term)) ||
        o.cliente_nombre.as_ref().map_or(false, |s| s.to_lowercase().contains(&term)) ||
        o.equipo_modelo.as_ref().map_or(false, |s| s.to_lowercase().contains(&term))
    }).collect())
}

pub async fn send_orden_trabajo_notification(_orden_id: i32, _sent_by: i32) -> Result<bool, String> {
    Ok(true)
}

pub async fn get_orden_trabajo_by_informe_id(informe_id: i32) -> Result<Option<OrdenTrabajo>, String> {
    let ordenes = get_ordenes_trabajo().await?;
    Ok(ordenes.into_iter().find(|o| o.informe_id == Some(informe_id)))
}

pub async fn get_ordenes_trabajo_filtradas(filtros: Filtros) -> Result<Vec<OrdenTrabajoDetallada>, String> {
    let mut ordenes = get_ordenes_trabajo_detalladas().await?;
    
    if let Some(search) = filtros.search {
        if !search.is_empty() {
            let term = search.to_lowercase();
            ordenes.retain(|o| {
                o.orden_codigo.as_ref().map_or(false, |s| s.to_lowercase().contains(&term)) ||
                o.cliente_nombre.as_ref().map_or(false, |s| s.to_lowercase().contains(&term)) ||
                o.equipo_modelo.as_ref().map_or(false, |s| s.to_lowercase().contains(&term))
            });
        }
    }
    
    if let Some(marcas) = filtros.marcas {
        if !marcas.is_empty() {
            ordenes.retain(|o| o.equipo_marca.as_ref().map_or(false, |m| marcas.contains(m)));
        }
    }
    
    if let Some(modelos) = filtros.modelos {
        if !modelos.is_empty() {
            ordenes.retain(|o| o.equipo_modelo.as_ref().map_or(false, |m| modelos.contains(m)));
        }
    }
    
    if let Some(prioridades) = filtros.prioridades {
        if !prioridades.is_empty() {
            let prioridades_lower: Vec<String> = prioridades.iter().map(|p| p.to_lowercase()).collect();
            ordenes.retain(|o| o.prioridad.as_ref().map_or(false, |p| prioridades_lower.contains(&p.to_lowercase())));
        }
    }
    
    if let Some(clientes) = filtros.clientes {
        if !clientes.is_empty() {
            ordenes.retain(|o| o.cliente_nombre.as_ref().map_or(false, |c| clientes.contains(c)));
        }
    }
    
    if let Some(estados) = filtros.estados {
        if !estados.is_empty() {
            let estados_lower: Vec<String> = estados.iter().map(|e| e.to_lowercase()).collect();
            ordenes.retain(|o| o.estado.as_ref().map_or(false, |e| estados_lower.contains(&e.to_lowercase())));
        }
    }
    
    if let (Some(inicio), Some(fin)) = (filtros.fecha_inicio, filtros.fecha_fin) {
        if let (Ok(date_inicio), Ok(date_fin)) = (
            chrono::DateTime::parse_from_rfc3339(&inicio),
            chrono::DateTime::parse_from_rfc3339(&fin)
        ) {
            ordenes.retain(|o| {
                if let Some(created) = o.created_at {
                    created >= date_inicio.with_timezone(&chrono::Utc) && 
                    created <= date_fin.with_timezone(&chrono::Utc)
                } else {
                    false
                }
            });
        }
    }
    
    Ok(ordenes)
}

pub async fn get_modelos_disponibles() -> Result<Vec<String>, String> {
    let equipos = crate::infrastructure::api::equipos::get_equipos().await?;
    let mut modelos: Vec<String> = equipos.into_iter()
        .filter_map(|e| e.equipo_modelo)
        .filter(|s| !s.is_empty())
        .collect();
    modelos.sort();
    modelos.dedup();
    Ok(modelos)
}

pub async fn get_marcas_disponibles() -> Result<Vec<String>, String> {
    let equipos = crate::infrastructure::api::equipos::get_equipos().await?;
    let mut marcas: Vec<String> = equipos.into_iter()
        .filter_map(|e| e.equipo_marca)
        .filter(|s| !s.is_empty())
        .collect();
    marcas.sort();
    marcas.dedup();
    Ok(marcas)
}

pub async fn get_clientes_disponibles() -> Result<Vec<String>, String> {
    let clientes = crate::infrastructure::api::clientes::get_clientes().await?;
    let mut nombres: Vec<String> = clientes.into_iter()
        .filter_map(|c| c.cliente_nombre)
        .filter(|s| !s.is_empty())
        .collect();
    nombres.sort();
    nombres.dedup();
    Ok(nombres)
}

pub async fn remove_cotizacion_from_ordenes(cotizacion_id: i32, _updated_by: i32) -> Result<bool, String> {
    let ordenes = get_ordenes_trabajo().await?;
    if let Some(orden) = ordenes.into_iter().find(|o| o.cotizacion_id == Some(cotizacion_id)) {
         let (client, base_url) = get_base_url()?;
         let url = format!("{}/{}", base_url, orden.orden_id);
         let body = serde_json::json!({ "cotizacion_id": null, "updated_by": _updated_by });
         let _ = client.put(&url).json(&body).send().await;
         Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn remove_informe_from_ordenes(informe_id: i32, _updated_by: i32) -> Result<bool, String> {
    let ordenes = get_ordenes_trabajo().await?;
    if let Some(orden) = ordenes.into_iter().find(|o| o.informe_id == Some(informe_id)) {
         let (client, base_url) = get_base_url()?;
         let url = format!("{}/{}", base_url, orden.orden_id);
         let body = serde_json::json!({ "informe_id": null, "updated_by": _updated_by });
         let _ = client.put(&url).json(&body).send().await;
         Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn get_orden_trabajo_pdf_data(orden_id: i32) -> Result<serde_json::Value, String> {
    let (client, base_url) = get_base_url()?;
    let url = format!("{}/{}/pdf-data", base_url, orden_id);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Error API: {}", response.status()));
    }
    
    response.json::<serde_json::Value>().await.map_err(|e| e.to_string())
}