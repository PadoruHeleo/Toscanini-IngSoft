use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_safe;
use crate::commands::logs::log_action;
use chrono::{DateTime, Utc};
use chrono::Datelike;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Informe {
    pub informe_id: i32,
    pub informe_codigo: Option<String>,
    pub informe_acciones: Option<String>,
    pub informe_obs: Option<String>,
    pub is_borrador: Option<bool>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    // Nuevos campos para compatibilidad con el frontend
    pub diagnostico: Option<String>,
    pub recomendaciones: Option<String>,
    pub solucion_aplicada: Option<String>,
    pub tecnico_responsable: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PiezaInforme {
    pub pieza_id: i32,
    pub informe_id: i32,
    pub cantidad: Option<i32>,
    // Campos adicionales para JOINs
    pub pieza_nombre: Option<String>,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InformeDetallado {
    pub informe_id: i32,
    pub informe_codigo: Option<String>,
    pub informe_acciones: Option<String>,
    pub informe_obs: Option<String>,
    pub is_borrador: Option<bool>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub created_by_nombre: Option<String>,
    // Nuevos campos para compatibilidad con el frontend
    pub diagnostico: Option<String>,
    pub recomendaciones: Option<String>,
    pub solucion_aplicada: Option<String>,
    pub tecnico_responsable: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInformeRequest {
    // informe_codigo se genera automáticamente
    pub informe_acciones: String,
    pub informe_obs: Option<String>,
    pub is_borrador: Option<bool>,
    pub created_by: i32,
    pub piezas: Option<Vec<PiezaInformeRequest>>,
    // Nuevos campos
    pub diagnostico: String,
    pub recomendaciones: Option<String>,
    pub solucion_aplicada: Option<String>,
    pub tecnico_responsable: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateInformeRequest {
    pub informe_codigo: Option<String>,
    pub informe_acciones: Option<String>,
    pub informe_obs: Option<String>,
    pub is_borrador: Option<bool>,
    // Nuevos campos
    pub diagnostico: Option<String>,
    pub recomendaciones: Option<String>,
    pub solucion_aplicada: Option<String>,
    pub tecnico_responsable: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PiezaInformeRequest {
    pub pieza_id: i32,
    pub cantidad: i32,
}

/// Obtener todos los informes
#[tauri::command]
pub async fn get_informes() -> Result<Vec<Informe>, String> {
    let pool = get_db_pool_safe()?;
      let informes = sqlx::query_as::<_, Informe>(
        "SELECT informe_id, informe_codigo, informe_acciones, informe_obs, 
                is_borrador, created_by, created_at,
                diagnostico, recomendaciones, solucion_aplicada, tecnico_responsable
         FROM INFORME 
         ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(informes)
}

/// Obtener informes con información detallada
#[tauri::command]
pub async fn get_informes_detallados() -> Result<Vec<InformeDetallado>, String> {
    let pool = get_db_pool_safe()?;
      let informes = sqlx::query_as::<_, InformeDetallado>(
        "SELECT i.informe_id, i.informe_codigo, i.informe_acciones, i.informe_obs,
                i.is_borrador, i.created_by, i.created_at,
                u.usuario_nombre as created_by_nombre,
                i.diagnostico, i.recomendaciones, i.solucion_aplicada, i.tecnico_responsable
         FROM INFORME i
         LEFT JOIN USUARIO u ON i.created_by = u.usuario_id
         ORDER BY i.created_at DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(informes)
}

/// Obtener un informe por ID
#[tauri::command]
pub async fn get_informe_by_id(informe_id: i32) -> Result<Option<Informe>, String> {
    println!("[DEBUG] get_informe_by_id: Recibido informe_id = {}", informe_id);
    let pool = get_db_pool_safe()?;
    let informe = sqlx::query_as::<_, Informe>(
        "SELECT informe_id, informe_codigo, informe_acciones, informe_obs,
                is_borrador, created_by, created_at,
                diagnostico, recomendaciones, solucion_aplicada, tecnico_responsable
         FROM INFORME 
         WHERE informe_id = ?"
    )
    .bind(informe_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    println!("[DEBUG] get_informe_by_id: Resultado = {:?}", informe);
    Ok(informe)
}

/// Obtener un informe por código
#[tauri::command]
pub async fn get_informe_by_codigo(informe_codigo: String) -> Result<Option<Informe>, String> {
    let pool = get_db_pool_safe()?;
      let informe = sqlx::query_as::<_, Informe>(
        "SELECT informe_id, informe_codigo, informe_acciones, informe_obs,
                is_borrador, created_by, created_at,
                diagnostico, recomendaciones, solucion_aplicada, tecnico_responsable
         FROM INFORME 
         WHERE informe_codigo = ?"
    )
    .bind(&informe_codigo)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(informe)
}

/// Crear un nuevo informe
#[tauri::command]
pub async fn create_informe(request: CreateInformeRequest) -> Result<Informe, String> {
    let pool = get_db_pool_safe()?;
    
    // Generar código automático: INF-YYYY-XXX
    let year = chrono::Utc::now().year();
    
    // Buscar el mayor número correlativo existente para el año actual
    let last_codigo: Option<String> = sqlx::query_scalar(
        "SELECT informe_codigo FROM INFORME WHERE informe_codigo LIKE ? ORDER BY informe_id DESC LIMIT 1"
    )
    .bind(format!("INF-{}-%", year))
    .fetch_one(pool)
    .await
    .ok();
    
    let next_number = if let Some(codigo) = last_codigo {
        // Extraer el número correlativo actual y sumarle 1
        let parts: Vec<&str> = codigo.split('-').collect();
        if parts.len() == 3 {
            parts[2].parse::<u32>().unwrap_or(0) + 1
        } else {
            1
        }
    } else {
        1
    };
    
    let codigo = format!("INF-{}-{:03}", year, next_number);
    
    // Iniciar transacción
    let mut tx = pool.begin().await.map_err(|e| format!("Database error: {}", e))?;
      // Crear el informe
    let result = sqlx::query(
        "INSERT INTO INFORME (informe_codigo, informe_acciones, informe_obs, 
                             is_borrador, created_by, diagnostico, recomendaciones, 
                             solucion_aplicada, tecnico_responsable) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&codigo)
    .bind(&request.informe_acciones)
    .bind(&request.informe_obs)
    .bind(request.is_borrador.unwrap_or(true))
    .bind(request.created_by)
    .bind(&request.diagnostico)
    .bind(&request.recomendaciones)
    .bind(&request.solucion_aplicada)
    .bind(&request.tecnico_responsable)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let informe_id = result.last_insert_id() as i32;
    
    // Agregar piezas si se proporcionaron
    if let Some(ref piezas) = request.piezas {
        for pieza in piezas {
            sqlx::query(
                "INSERT INTO PIEZAS_INFORME (pieza_id, informe_id, cantidad) VALUES (?, ?, ?)"
            )
            .bind(pieza.pieza_id)
            .bind(informe_id)
            .bind(pieza.cantidad)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Database error adding part: {}", e))?;
        }
    }
    
    // Confirmar transacción
    tx.commit().await.map_err(|e| format!("Database error: {}", e))?;
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "CREATE_INFORME",
        Some(request.created_by),
        "INFORME",
        Some(informe_id),
        None,
        Some(&format!("Informe creado: {}", codigo))
    ).await;
    
    // Obtener el informe recién creado
    get_informe_by_id(informe_id)
        .await?
        .ok_or_else(|| "Failed to retrieve created informe".to_string())
}

/// Actualizar un informe existente
#[tauri::command]
pub async fn update_informe(informe_id: i32, request: UpdateInformeRequest, updated_by: i32) -> Result<Option<Informe>, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener el informe actual para logging
    let current_informe = get_informe_by_id(informe_id).await?;
    
    // Verificar que el código no está en uso por otro informe (si se está actualizando)
    if let Some(ref new_codigo) = request.informe_codigo {
        if let Some(existing_informe) = get_informe_by_codigo(new_codigo.clone()).await? {
            if existing_informe.informe_id != informe_id {
                return Err("Ya existe otro informe con este código".to_string());
            }
        }
    }
      let result = sqlx::query(
        "UPDATE INFORME SET 
         informe_codigo = COALESCE(?, informe_codigo),
         informe_acciones = COALESCE(?, informe_acciones),
         informe_obs = COALESCE(?, informe_obs),
         is_borrador = COALESCE(?, is_borrador),
         diagnostico = COALESCE(?, diagnostico),
         recomendaciones = COALESCE(?, recomendaciones),
         solucion_aplicada = COALESCE(?, solucion_aplicada),
         tecnico_responsable = COALESCE(?, tecnico_responsable)
         WHERE informe_id = ?"
    )
    .bind(&request.informe_codigo)
    .bind(&request.informe_acciones)
    .bind(&request.informe_obs)
    .bind(request.is_borrador)
    .bind(&request.diagnostico)
    .bind(&request.recomendaciones)
    .bind(&request.solucion_aplicada)
    .bind(&request.tecnico_responsable)
    .bind(informe_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Ok(None);
    }
    
    // Registrar la acción en el log de auditoría
    if let Some(ref informe) = current_informe {
        let prev_data = format!("{}|{}|{}|{}", 
            informe.informe_codigo.as_deref().unwrap_or(""), 
            informe.informe_acciones.as_deref().unwrap_or(""),
            informe.informe_obs.as_deref().unwrap_or(""),
            informe.is_borrador.map_or("".to_string(), |p| p.to_string())
        );
        let new_data = format!("{}|{}|{}|{}", 
            request.informe_codigo.as_deref().unwrap_or(informe.informe_codigo.as_deref().unwrap_or("")),
            request.informe_acciones.as_deref().unwrap_or(informe.informe_acciones.as_deref().unwrap_or("")),
            request.informe_obs.as_deref().unwrap_or(informe.informe_obs.as_deref().unwrap_or("")),
            request.is_borrador
                .or(informe.is_borrador)
                .map_or("".to_string(), |p| p.to_string())
        );
        
        let _ = log_action(
            "UPDATE_INFORME",
            Some(updated_by),
            "INFORME",
            Some(informe_id),
            Some(&prev_data),
            Some(&new_data)
        ).await;
    }
    
    get_informe_by_id(informe_id).await
}

/// Eliminar un informe
#[tauri::command]
pub async fn delete_informe(informe_id: i32, deleted_by: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener el informe antes de eliminarlo para logging
    let informe_to_delete = get_informe_by_id(informe_id).await?;
    
    // Verificar si el informe tiene órdenes de trabajo asociadas
    let has_dependencies = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM ORDEN_TRABAJO WHERE informe_id = ?"
    )
    .bind(informe_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Database error checking dependencies: {}", e))?;
    
    if has_dependencies > 0 {
        return Err("No se puede eliminar el informe porque tiene órdenes de trabajo asociadas".to_string());
    }
    
    // Iniciar transacción
    let mut tx = pool.begin().await.map_err(|e| format!("Database error: {}", e))?;
    
    // Eliminar primero las relaciones con piezas
    sqlx::query("DELETE FROM PIEZAS_INFORME WHERE informe_id = ?")
        .bind(informe_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Luego eliminar el informe
    let result = sqlx::query("DELETE FROM INFORME WHERE informe_id = ?")
        .bind(informe_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    let was_deleted = result.rows_affected() > 0;
    
    // Confirmar transacción
    tx.commit().await.map_err(|e| format!("Database error: {}", e))?;
    
    // Registrar la acción en el log de auditoría
    if was_deleted {
        if let Some(ref informe) = informe_to_delete {
            let _ = log_action(
                "DELETE_INFORME",
                Some(deleted_by),
                "INFORME",
                Some(informe_id),
                Some(&format!("Informe eliminado: {}", 
                    informe.informe_codigo.as_deref().unwrap_or("N/A")
                )),
                None
            ).await;
        }
    }
    
    Ok(was_deleted)
}

/// Buscar informes por texto
#[tauri::command]
pub async fn search_informes(search_term: String) -> Result<Vec<InformeDetallado>, String> {
    let pool = get_db_pool_safe()?;
    
    let search_pattern = format!("%{}%", search_term);
      let informes = sqlx::query_as::<_, InformeDetallado>(
        "SELECT i.informe_id, i.informe_codigo, i.informe_acciones, i.informe_obs,
                i.is_borrador, i.created_by, i.created_at,
                u.usuario_nombre as created_by_nombre,
                i.diagnostico, i.recomendaciones, i.solucion_aplicada, i.tecnico_responsable
         FROM INFORME i
         LEFT JOIN USUARIO u ON i.created_by = u.usuario_id
         WHERE i.informe_codigo LIKE ? 
         ORDER BY i.created_at DESC"
    )
    .bind(&search_pattern)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(informes)
}

/// Contar total de informes
#[tauri::command]
pub async fn count_informes() -> Result<i64, String> {
    let pool = get_db_pool_safe()?;
    
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM INFORME")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(count)
}

/// Obtener informes con paginación
#[tauri::command]
pub async fn get_informes_with_pagination(offset: i64, limit: i64) -> Result<Vec<InformeDetallado>, String> {
    let pool = get_db_pool_safe()?;
      let informes = sqlx::query_as::<_, InformeDetallado>(
        "SELECT i.informe_id, i.informe_codigo, i.informe_acciones, i.informe_obs,
                i.is_borrador, i.created_by, i.created_at,
                u.usuario_nombre as created_by_nombre,
                i.diagnostico, i.recomendaciones, i.solucion_aplicada, i.tecnico_responsable
         FROM INFORME i
         LEFT JOIN USUARIO u ON i.created_by = u.usuario_id
         ORDER BY i.created_at DESC
         LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(informes)
}

/// Obtener las piezas asociadas a un informe
#[tauri::command]
pub async fn get_piezas_informe(informe_id: i32) -> Result<Vec<PiezaInforme>, String> {
    let pool = get_db_pool_safe()?;
    
    let piezas = sqlx::query_as::<_, PiezaInforme>(
        "SELECT pi.pieza_id, pi.informe_id, COALESCE(pi.cantidad, 1) as cantidad, 
                p.pieza_nombre, p.pieza_marca, p.pieza_desc, p.pieza_precio 
         FROM PIEZAS_INFORME pi 
         LEFT JOIN PIEZA p ON pi.pieza_id = p.pieza_id 
         WHERE pi.informe_id = ?"
    )
    .bind(informe_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(piezas)
}

/// Enviar informe por email al cliente
#[tauri::command]
pub async fn send_informe_to_client(informe_id: i32, sent_by: i32) -> Result<bool, String> {    use crate::email::EmailService;
    use crate::commands::ordenes_trabajo::get_orden_trabajo_by_informe_id;
    
    let pool = get_db_pool_safe()?;
    
    // Obtener el informe
    let informe = get_informe_by_id(informe_id).await?
        .ok_or_else(|| "Informe no encontrado".to_string())?;
    
    // Obtener la orden de trabajo asociada al informe
    let orden_trabajo = get_orden_trabajo_by_informe_id(informe_id).await?
        .ok_or_else(|| "No se encontró orden de trabajo asociada al informe".to_string())?;
    
    // Obtener información del cliente desde el equipo
    let cliente_info = sqlx::query_as::<_, (i32, String, Option<String>)>(
        "SELECT c.cliente_id, c.cliente_nombre, c.cliente_correo 
         FROM CLIENTE c 
         INNER JOIN EQUIPO e ON c.cliente_id = e.cliente_id 
         WHERE e.equipo_id = ?"
    )
    .bind(orden_trabajo.equipo_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?
    .ok_or_else(|| "No se encontró información del cliente".to_string())?;
    
    let cliente_email = cliente_info.2
        .ok_or_else(|| "El cliente no tiene un correo electrónico registrado".to_string())?;
    
    // Obtener las piezas del informe
    let piezas_informe = get_piezas_informe(informe_id).await?;
    
    // Crear el servicio de email
    let email_service = EmailService::new()
        .map_err(|e| format!("Error inicializando servicio de email: {}", e))?;
    
    // Enviar el email
    email_service.send_informe_email(
        &cliente_email,
        &cliente_info.1,
        &informe,
        &orden_trabajo,
        &piezas_informe,
    ).await
    .map_err(|e| format!("Error enviando email: {}", e))?;
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "SEND_INFORME",
        Some(sent_by),
        "INFORME",
        Some(informe_id),
        None,
        Some(&format!("Informe {} enviado a {}", 
            informe.informe_codigo.as_deref().unwrap_or("N/A"),
            cliente_email
        ))
    ).await;
    
    Ok(true)
}
