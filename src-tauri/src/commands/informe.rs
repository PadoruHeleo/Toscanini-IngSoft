use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_safe;
use crate::commands::logs::log_action;
use crate::commands::terminos_condiciones::apply_default_terminos_to_informe;
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
    pub deleted_at: Option<DateTime<Utc>>,
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
                diagnostico, recomendaciones, solucion_aplicada, tecnico_responsable, deleted_at
         FROM INFORME 
         WHERE deleted_at IS NULL
         ORDER BY created_at DESC"
    )
    .fetch_all(&*pool)
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
         WHERE i.deleted_at IS NULL
         ORDER BY i.created_at DESC"
    )
    .fetch_all(&*pool)
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
                diagnostico, recomendaciones, solucion_aplicada, tecnico_responsable, deleted_at
         FROM INFORME 
         WHERE informe_id = ? AND deleted_at IS NULL"
    )
    .bind(informe_id)
    .fetch_optional(&*pool)
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
                diagnostico, recomendaciones, solucion_aplicada, tecnico_responsable, deleted_at
         FROM INFORME 
         WHERE informe_codigo = ? AND deleted_at IS NULL"
    )
    .bind(&informe_codigo)
    .fetch_optional(&*pool)
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
        "SELECT informe_codigo FROM INFORME WHERE informe_codigo LIKE ? AND deleted_at IS NULL ORDER BY informe_id DESC LIMIT 1"
    )
    .bind(format!("INF-{}-%", year))
    .fetch_one(&*pool)
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
    
    // Aplicar términos y condiciones por defecto automáticamente
    let _ = apply_default_terminos_to_informe(informe_id, request.created_by).await;
    
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

/// Eliminar un informe en estado de borrador
#[tauri::command]
pub async fn rechazar_informe_borrador(informe_id: i32, motivo_eliminacion: String, updated_by: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;

    // Cambia el estado del informe a "rechazado" y guarda el motivo
    


    // Desvincula el informe de la orden de trabajo
    let result = sqlx::query("UPDATE ORDEN_TRABAJO SET informe_id = NULL WHERE informe_id = ?")
        .bind(informe_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error al desvincular informe: {}", e))?;

    // Registrar AuditLog
    let was_updated = result.rows_affected() > 0;
    if was_updated {
        let _ = log_action(
            "RECHAZAR_INFORME_BORRADOR",
            Some(updated_by),
            "INFORME",
            Some(informe_id),
            Some("Informe en borrador eliminado"),
            Some(&motivo_eliminacion)
        ).await;
    }

    Ok(was_updated)
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
    .execute(&*pool)
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

/// Eliminar un informe (eliminación lógica solo si fue enviado al cliente)
#[tauri::command]
pub async fn delete_informe(informe_id: i32, deleted_by: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener el informe antes de eliminarlo para logging (sin filtrar por deleted_at)
    let informe_to_delete = sqlx::query_as::<_, Informe>(
        "SELECT informe_id, informe_codigo, informe_acciones, informe_obs,
                is_borrador, created_by, created_at,
                diagnostico, recomendaciones, solucion_aplicada, tecnico_responsable, deleted_at
         FROM INFORME 
         WHERE informe_id = ?"
    )
    .bind(informe_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?
    .ok_or_else(|| "Informe no encontrado".to_string())?;
    
    // Si ya está eliminado lógicamente, no hacer nada
    if informe_to_delete.deleted_at.is_some() {
        return Err("El informe ya fue eliminado".to_string());
    }
    
    // Verificar si el informe tiene órdenes de trabajo asociadas
    let has_dependencies = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM ORDEN_TRABAJO WHERE informe_id = ?"
    )
    .bind(informe_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Database error checking dependencies: {}", e))?;
    
    // Verificar si fue enviado al cliente (is_borrador == false)
    let fue_enviado = informe_to_delete.is_borrador == Some(false);
    
    // Iniciar transacción
    let mut tx = pool.begin().await.map_err(|e| format!("Database error: {}", e))?;
    
    let was_deleted = if fue_enviado && has_dependencies > 0 {
        // Eliminación lógica: solo si fue enviado al cliente Y tiene órdenes asociadas
        // Marcar como eliminado y desvincular de órdenes
        sqlx::query("UPDATE INFORME SET deleted_at = CURRENT_TIMESTAMP WHERE informe_id = ?")
            .bind(informe_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        
        // Desvincular de órdenes de trabajo
        sqlx::query("UPDATE ORDEN_TRABAJO SET informe_id = NULL WHERE informe_id = ?")
            .bind(informe_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        
        true
    } else {
        // Eliminación física: borradores o sin órdenes asociadas
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
        
        result.rows_affected() > 0
    };
    
    // Confirmar transacción
    tx.commit().await.map_err(|e| format!("Database error: {}", e))?;
    
    // Registrar la acción en el log de auditoría
    if was_deleted {
        let action_type = if fue_enviado && has_dependencies > 0 {
            "DELETE_INFORME_LOGICAL"
        } else {
            "DELETE_INFORME"
        };
        let _ = log_action(
            action_type,
            Some(deleted_by),
            "INFORME",
            Some(informe_id),
            Some(&format!("Informe {} eliminado: {}", 
                if fue_enviado && has_dependencies > 0 { "lógicamente" } else { "físicamente" },
                informe_to_delete.informe_codigo.as_deref().unwrap_or("N/A")
            )),
            None
        ).await;
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
         WHERE i.informe_codigo LIKE ? AND i.deleted_at IS NULL
         ORDER BY i.created_at DESC"
    )
    .bind(&search_pattern)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(informes)
}

/// Contar total de informes
#[tauri::command]
pub async fn count_informes() -> Result<i64, String> {
    let pool = get_db_pool_safe()?;
    
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM INFORME WHERE deleted_at IS NULL")
        .fetch_one(&*pool)
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
         WHERE i.deleted_at IS NULL
         ORDER BY i.created_at DESC
         LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&*pool)
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
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(piezas)
}

/// Enviar informe por email al cliente
#[tauri::command]
pub async fn send_informe_to_client(informe_id: i32, sent_by: i32) -> Result<bool, String> {
    use crate::commands::ordenes_trabajo::get_orden_trabajo_by_informe_id;
    use crate::commands::equipos::get_equipo_by_id;
    use crate::commands::clientes::get_cliente_by_id;
    use crate::pdf::commands::generate_informe_pdf_command;
    
    println!("🔍 [DEBUG] send_informe_to_client: inicio, informe_id={}", informe_id);
    
    // Obtener el informe
    let informe = get_informe_by_id(informe_id).await?
        .ok_or_else(|| {
            println!("❌ [DEBUG] Informe no encontrado: {}", informe_id);
            "Informe no encontrado".to_string()
        })?;
    println!("✅ [DEBUG] Informe encontrado: {:?}", informe.informe_codigo);
    
    // Obtener la orden de trabajo asociada al informe
    println!("🔍 [DEBUG] Buscando orden de trabajo para informe_id={}", informe_id);
    let orden_trabajo = match get_orden_trabajo_by_informe_id(informe_id).await {
        Ok(Some(orden)) => {
            println!("✅ [DEBUG] Orden de trabajo encontrada: {:?}", orden.orden_codigo);
            orden
        }
        Ok(None) => {
            println!("❌ [DEBUG] No se encontró orden de trabajo para informe_id={}", informe_id);
            return Err("No se encontró orden de trabajo asociada al informe. Asegúrate de que el informe esté asociado a una orden de trabajo antes de enviarlo.".to_string());
        }
        Err(e) => {
            println!("❌ [DEBUG] Error al buscar orden de trabajo: {}", e);
            return Err(format!("Error al buscar orden de trabajo: {}", e));
        }
    };

    // Obtener el equipo
    let equipo_id = orden_trabajo.equipo_id
        .ok_or_else(|| {
            println!("❌ [DEBUG] La orden {} no tiene equipo asociado", orden_trabajo.orden_id);
            "La orden no tiene equipo asociado".to_string()
        })?;
    println!("✅ [DEBUG] Equipo ID: {}", equipo_id);
    
    let equipo = get_equipo_by_id(equipo_id).await?
        .ok_or_else(|| {
            println!("❌ [DEBUG] Equipo no encontrado: {}", equipo_id);
            "Equipo no encontrado".to_string()
        })?;
    
    // Obtener el cliente
    let cliente_id = equipo.cliente_id
        .ok_or_else(|| {
            println!("❌ [DEBUG] El equipo {} no tiene cliente asociado", equipo_id);
            "El equipo no tiene cliente asociado".to_string()
        })?;
    println!("✅ [DEBUG] Cliente ID: {}", cliente_id);
    
    let cliente = get_cliente_by_id(cliente_id).await?
        .ok_or_else(|| {
            println!("❌ [DEBUG] Cliente no encontrado: {}", cliente_id);
            "Cliente no encontrado".to_string()
        })?;
    
    // Verificar email del cliente
    if cliente.cliente_correo.is_none() || cliente.cliente_correo.as_ref().unwrap().trim().is_empty() {
        println!("❌ [DEBUG] Cliente {} no tiene email configurado", cliente_id);
        return Err("El cliente no tiene email configurado".to_string());
    }
    println!("✅ [DEBUG] Email del cliente: {}", cliente.cliente_correo.as_ref().unwrap());
    
    // Generar el PDF del informe
    println!("📄 [DEBUG] Generando PDF de informe {}...", informe_id);
    let pdf_bytes = match generate_informe_pdf_command(informe_id).await {
        Ok(bytes) => {
            println!("✅ [DEBUG] PDF generado exitosamente ({} bytes)", bytes.len());
            bytes
        }
        Err(e) => {
            println!("❌ [DEBUG] Error generando PDF: {}", e);
            return Err(format!("Error generando PDF del informe: {}", e));
        }
    };
    
    // Obtener el email del cliente (clonar para evitar problemas de borrow)
    let cliente_email = cliente.cliente_correo
        .clone()
        .ok_or_else(|| "El cliente no tiene email configurado".to_string())?;
    
    // Verificar RESEND_API_KEY
    use std::env;
    match env::var("RESEND_API_KEY") {
        Ok(_) => println!("✅ [DEBUG] RESEND_API_KEY encontrada"),
        Err(_) => {
            println!("❌ [DEBUG] RESEND_API_KEY no encontrada");
            return Err("RESEND_API_KEY no configurada en las variables de entorno".to_string());
        }
    }
    
    // Crear el servicio de email
    let email_service = match crate::email::EmailService::new() {
        Ok(service) => {
            println!("✅ [DEBUG] EmailService inicializado");
            service
        }
        Err(e) => {
            println!("❌ [DEBUG] Error inicializando EmailService: {}", e);
            return Err(format!("Error inicializando servicio de email: {}", e));
        }
    };
    
    // Enviar el email con PDF
    println!("📧 [DEBUG] Enviando email de informe con PDF a {}...", cliente_email);
    match email_service.send_informe_email_with_pdf(
        &cliente_email,
        &cliente.cliente_nombre.unwrap_or_else(|| "Cliente".to_string()),
        &informe,
        &orden_trabajo,
        &equipo,
        &pdf_bytes,
    ).await {
        Ok(_) => {
            println!("✅ [DEBUG] Email enviado exitosamente");
        }
        Err(e) => {
            println!("❌ [DEBUG] Error enviando email: {}", e);
            return Err(format!("Error enviando email: {}", e));
        }
    }
    
    // Registrar la acción en el log de auditoría
    let _ = log_action(
        "SEND_INFORME",
        Some(sent_by),
        "INFORME",
        Some(informe_id),
        None,
        Some(&format!("Informe {} enviado a {} con PDF adjunto", 
            informe.informe_codigo.as_deref().unwrap_or("N/A"),
            cliente_email
        ))
    ).await;
    
    println!("✅ [DEBUG] send_informe_to_client completado exitosamente");
    Ok(true)
}

#[tauri::command]
pub async fn get_informes_by_cliente(cliente_id: i32) -> Result<Vec<Informe>, String> {
    let pool = get_db_pool_safe()?;
    let informes = sqlx::query_as::<_, Informe>(
        "SELECT i.informe_id, i.informe_codigo, i.informe_acciones, i.informe_obs,
                i.is_borrador, i.created_by, i.created_at,
                i.diagnostico, i.recomendaciones, i.solucion_aplicada, i.tecnico_responsable, i.deleted_at
         FROM INFORME i
         INNER JOIN ORDEN_TRABAJO ot ON i.informe_id = ot.informe_id
         INNER JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         WHERE e.cliente_id = ? AND i.deleted_at IS NULL
         ORDER BY i.created_at DESC"
    )
    .bind(cliente_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error al obtener informes del cliente: {}", e))?;
    Ok(informes)
}