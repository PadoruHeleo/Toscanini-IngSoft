use crate::database::get_db_pool_safe;
use crate::commands::logs::log_action;
use std::collections::HashSet;

use crate::models::terminos_condiciones::{
    TerminoCondicion, 
    TerminoInforme, 
    TerminoCotizacion, 
    CreateTerminoCondicionRequest, 
    UpdateTerminoCondicionRequest, 
    TerminoInformeRequest, 
    TerminoCotizacionRequest
};

/// Obtener todos los términos y condiciones
#[tauri::command]
pub async fn get_terminos_condiciones() -> Result<Vec<TerminoCondicion>, String> {
    let pool = get_db_pool_safe()?;
    
    let terminos = sqlx::query_as::<_, TerminoCondicion>(
        "SELECT termino_id, termino_nombre, termino_descripcion, is_active, 
                tipo_referencia, is_default, created_at, updated_at
         FROM TERMINOS_CONDICIONES
         ORDER BY termino_nombre ASC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(terminos)
}

/// Obtener términos y condiciones activos
#[tauri::command]
pub async fn get_terminos_condiciones_activos() -> Result<Vec<TerminoCondicion>, String> {
    let pool = get_db_pool_safe()?;
    
    let terminos = sqlx::query_as::<_, TerminoCondicion>(
        "SELECT termino_id, termino_nombre, termino_descripcion, is_active, 
                tipo_referencia, is_default, created_at, updated_at
         FROM TERMINOS_CONDICIONES
         WHERE is_active = TRUE
         ORDER BY termino_nombre ASC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(terminos)
}

/// Obtener términos y condiciones por tipo
#[tauri::command]
pub async fn get_terminos_condiciones_by_tipo(tipo: String) -> Result<Vec<TerminoCondicion>, String> {
    let pool = get_db_pool_safe()?;
    
    let terminos = sqlx::query_as::<_, TerminoCondicion>(
        "SELECT termino_id, termino_nombre, termino_descripcion, is_active, 
                tipo_referencia, is_default, created_at, updated_at
         FROM TERMINOS_CONDICIONES
         WHERE is_active = TRUE AND (tipo_referencia = ? OR tipo_referencia = 'ambos')
         ORDER BY is_default DESC, termino_nombre ASC"
    )
    .bind(tipo)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(terminos)
}

/// Obtener términos y condiciones por defecto para un tipo específico
#[tauri::command]
pub async fn get_terminos_condiciones_default(tipo: String) -> Result<Vec<TerminoCondicion>, String> {
    let pool = get_db_pool_safe()?;
    
    let terminos = sqlx::query_as::<_, TerminoCondicion>(
        "SELECT termino_id, termino_nombre, termino_descripcion, is_active, 
                tipo_referencia, is_default, created_at, updated_at
         FROM TERMINOS_CONDICIONES
         WHERE is_active = TRUE AND is_default = TRUE 
               AND (tipo_referencia = ? OR tipo_referencia = 'ambos')
         ORDER BY termino_nombre ASC"
    )
    .bind(tipo)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(terminos)
}

/// Obtener un término y condición por ID
#[tauri::command]
pub async fn get_termino_condicion_by_id(termino_id: i32) -> Result<Option<TerminoCondicion>, String> {
    let pool = get_db_pool_safe()?;
    
    let termino = sqlx::query_as::<_, TerminoCondicion>(
        "SELECT termino_id, termino_nombre, termino_descripcion, is_active, 
                tipo_referencia, is_default, created_at, updated_at
         FROM TERMINOS_CONDICIONES
         WHERE termino_id = ?"
    )
    .bind(termino_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(termino)
}

/// Crear un nuevo término y condición
#[tauri::command]
pub async fn create_termino_condicion(
    request: CreateTerminoCondicionRequest,
    created_by: i32
) -> Result<i32, String> {
    let pool = get_db_pool_safe()?;
    
    // Validar que el tipo de referencia sea válido
    if !["informe", "cotizacion", "ambos"].contains(&request.tipo_referencia.as_str()) {
        return Err("Tipo de referencia inválido. Debe ser 'informe', 'cotizacion' o 'ambos'".to_string());
    }
    
    let result = sqlx::query(
        "INSERT INTO TERMINOS_CONDICIONES 
         (termino_nombre, termino_descripcion, tipo_referencia, is_default)
         VALUES (?, ?, ?, ?)"
    )
    .bind(&request.termino_nombre)
    .bind(&request.termino_descripcion)
    .bind(&request.tipo_referencia)
    .bind(request.is_default.unwrap_or(false))
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let termino_id = result.last_insert_id() as i32;
    
    // Registrar en el log de auditoría
    let _ = log_action(
        "CREATE_TERMINO_CONDICION",
        Some(created_by),
        "TERMINOS_CONDICIONES",
        Some(termino_id),
        None,
        Some(&format!("Término creado: {} ({})", request.termino_nombre, request.tipo_referencia))
    ).await;
    
    Ok(termino_id)
}

/// Actualizar un término y condición
#[tauri::command]
pub async fn update_termino_condicion(
    termino_id: i32,
    request: UpdateTerminoCondicionRequest,
    updated_by: i32
) -> Result<(), String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener el término actual para el log
    let current_termino = get_termino_condicion_by_id(termino_id).await?
        .ok_or("Término y condición no encontrado")?;
    
    // Construir la consulta dinámicamente
    let mut query_parts = Vec::new();
    
    if let Some(_nombre) = &request.termino_nombre {
        query_parts.push("termino_nombre = ?");
    }
    if let Some(_descripcion) = &request.termino_descripcion {
        query_parts.push("termino_descripcion = ?");
    }
    if request.is_active.is_some() {
        query_parts.push("is_active = ?");
    }
    if let Some(tipo) = &request.tipo_referencia {
        if !["informe", "cotizacion", "ambos"].contains(&tipo.as_str()) {
            return Err("Tipo de referencia inválido. Debe ser 'informe', 'cotizacion' o 'ambos'".to_string());
        }
        query_parts.push("tipo_referencia = ?");
    }
    if request.is_default.is_some() {
        query_parts.push("is_default = ?");
    }
    
    if query_parts.is_empty() {
        return Err("No hay campos para actualizar".to_string());
    }
    
    query_parts.push("updated_at = CURRENT_TIMESTAMP");
    
    let query = format!(
        "UPDATE TERMINOS_CONDICIONES SET {} WHERE termino_id = ?",
        query_parts.join(", ")
    );
    
    let mut query_builder = sqlx::query(&query);
    
    if let Some(nombre) = &request.termino_nombre {
        query_builder = query_builder.bind(nombre);
    }
    if let Some(descripcion) = &request.termino_descripcion {
        query_builder = query_builder.bind(descripcion);
    }
    if let Some(is_active) = request.is_active {
        query_builder = query_builder.bind(is_active);
    }
    if let Some(tipo) = &request.tipo_referencia {
        query_builder = query_builder.bind(tipo);
    }
    if let Some(is_default) = request.is_default {
        query_builder = query_builder.bind(is_default);
    }
    
    query_builder = query_builder.bind(termino_id);
    
    let result = query_builder
        .execute(&*pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("Término y condición no encontrado".to_string());
    }
    
    // Registrar en el log de auditoría
    let _ = log_action(
        "UPDATE_TERMINO_CONDICION",
        Some(updated_by),
        "TERMINOS_CONDICIONES",
        Some(termino_id),
        Some(&format!("{}|{}|{}|{}|{}", 
            current_termino.termino_nombre,
            current_termino.termino_descripcion,
            current_termino.is_active.unwrap_or(false),
            current_termino.tipo_referencia,
            current_termino.is_default.unwrap_or(false)
        )),
        Some(&format!("{}|{}|{}|{}|{}", 
            request.termino_nombre.as_ref().unwrap_or(&current_termino.termino_nombre),
            request.termino_descripcion.as_ref().unwrap_or(&current_termino.termino_descripcion),
            request.is_active.unwrap_or(current_termino.is_active.unwrap_or(false)),
            request.tipo_referencia.as_ref().unwrap_or(&current_termino.tipo_referencia),
            request.is_default.unwrap_or(current_termino.is_default.unwrap_or(false))
        ))
    ).await;
    
    Ok(())
}

/// Eliminar (desactivar) un término y condición
#[tauri::command]
pub async fn delete_termino_condicion(termino_id: i32, deleted_by: i32) -> Result<(), String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener el término actual para el log
    let current_termino = get_termino_condicion_by_id(termino_id).await?
        .ok_or("Término y condición no encontrado")?;
    
    let result = sqlx::query(
        "UPDATE TERMINOS_CONDICIONES SET is_active = FALSE, updated_at = CURRENT_TIMESTAMP WHERE termino_id = ?"
    )
    .bind(termino_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("Término y condición no encontrado".to_string());
    }
    
    // Registrar en el log de auditoría
    let _ = log_action(
        "DELETE_TERMINO_CONDICION",
        Some(deleted_by),
        "TERMINOS_CONDICIONES",
        Some(termino_id),
        Some(&format!("Término desactivado: {} ({})", 
            current_termino.termino_nombre, 
            current_termino.tipo_referencia
        )),
        Some("INACTIVE")
    ).await;
    
    Ok(())
}

/// Obtener términos aplicados a un informe específico
#[tauri::command]
pub async fn get_terminos_by_informe(informe_id: i32) -> Result<Vec<TerminoInforme>, String> {
    let pool = get_db_pool_safe()?;
    
    let terminos = sqlx::query_as::<_, TerminoInforme>(
        "SELECT ti.termino_id, ti.informe_id, ti.aplicado, ti.created_at,
                tc.termino_nombre, tc.termino_descripcion
         FROM TERMINOS_INFORME ti
         JOIN TERMINOS_CONDICIONES tc ON ti.termino_id = tc.termino_id
         WHERE ti.informe_id = ?
         ORDER BY tc.termino_nombre ASC"
    )
    .bind(informe_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(terminos)
}

/// Obtener términos aplicados a una cotización específica
#[tauri::command]
pub async fn get_terminos_by_cotizacion(cotizacion_id: i32) -> Result<Vec<TerminoCotizacion>, String> {
    let pool = get_db_pool_safe()?;
    
    let terminos = sqlx::query_as::<_, TerminoCotizacion>(
        "SELECT tc.termino_id, tc.cotizacion_id, tc.aplicado, tc.created_at,
                t.termino_nombre, t.termino_descripcion
         FROM TERMINOS_COTIZACION tc
         JOIN TERMINOS_CONDICIONES t ON tc.termino_id = t.termino_id
         WHERE tc.cotizacion_id = ?
         ORDER BY t.termino_nombre ASC"
    )
    .bind(cotizacion_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(terminos)
}

/// Aplicar términos y condiciones a un informe
#[tauri::command]
pub async fn apply_terminos_to_informe(
    informe_id: i32,
    terminos: Vec<TerminoInformeRequest>,
    applied_by: i32
) -> Result<(), String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener términos actuales del informe
    let terminos_actuales = get_terminos_by_informe(informe_id).await?;
    
    // Normalizar los términos actuales para comparación (solo IDs y estado aplicado)
    let terminos_actuales_set: HashSet<(i32, bool)> = terminos_actuales
        .iter()
        .map(|t| (t.termino_id, t.aplicado.unwrap_or(true)))
        .collect();
    
    // Normalizar los términos nuevos para comparación
    let terminos_nuevos_set: HashSet<(i32, bool)> = terminos
        .iter()
        .map(|t| (t.termino_id, t.aplicado.unwrap_or(true)))
        .collect();
    
    // Comparar si hay cambios
    if terminos_actuales_set == terminos_nuevos_set {
        println!("ℹ️ apply_terminos_to_informe: No hay cambios en los términos para informe_id {}. No se actualiza ni registra en auditoría.", informe_id);
        return Ok(()); // No hay cambios, retornar sin hacer nada ni registrar en auditoría
    }
    
    println!("🔄 apply_terminos_to_informe: Detectados cambios en términos para informe_id {}, procediendo con actualización", informe_id);
    let terminos_count = terminos.len();
    
    // Iniciar transacción
    let mut tx = pool.begin().await.map_err(|e| format!("Database error: {}", e))?;
    
    // Eliminar términos existentes para este informe
    sqlx::query("DELETE FROM TERMINOS_INFORME WHERE informe_id = ?")
        .bind(informe_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Database error removing existing terms: {}", e))?;
    
    // Insertar los nuevos términos
    for termino in terminos {
        sqlx::query(
            "INSERT INTO TERMINOS_INFORME (termino_id, informe_id, aplicado) VALUES (?, ?, ?)"
        )
        .bind(termino.termino_id)
        .bind(informe_id)
        .bind(termino.aplicado.unwrap_or(true))
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Database error applying term: {}", e))?;
    }
    
    // Confirmar transacción
    tx.commit().await.map_err(|e| format!("Database error committing transaction: {}", e))?;
    
    // Registrar en el log de auditoría solo si hubo cambios
    let _ = log_action(
        "APPLY_TERMINOS_INFORME",
        Some(applied_by),
        "TERMINOS_INFORME",
        Some(informe_id),
        None,
        Some(&format!("Aplicados {} términos al informe", terminos_count))
    ).await;
    
    Ok(())
}

/// Aplicar términos y condiciones a una cotización
#[tauri::command]
pub async fn apply_terminos_to_cotizacion(
    cotizacion_id: i32,
    terminos: Vec<TerminoCotizacionRequest>,
    applied_by: i32
) -> Result<(), String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener términos actuales de la cotización
    let terminos_actuales = get_terminos_by_cotizacion(cotizacion_id).await?;
    
    // Normalizar los términos actuales para comparación (solo IDs y estado aplicado)
    let terminos_actuales_set: HashSet<(i32, bool)> = terminos_actuales
        .iter()
        .map(|t| (t.termino_id, t.aplicado.unwrap_or(true)))
        .collect();
    
    // Normalizar los términos nuevos para comparación
    let terminos_nuevos_set: HashSet<(i32, bool)> = terminos
        .iter()
        .map(|t| (t.termino_id, t.aplicado.unwrap_or(true)))
        .collect();
    
    // Comparar si hay cambios
    if terminos_actuales_set == terminos_nuevos_set {
        println!("ℹ️ apply_terminos_to_cotizacion: No hay cambios en los términos para cotizacion_id {}. No se actualiza ni registra en auditoría.", cotizacion_id);
        return Ok(()); // No hay cambios, retornar sin hacer nada ni registrar en auditoría
    }
    
    println!("🔄 apply_terminos_to_cotizacion: Detectados cambios en términos para cotizacion_id {}, procediendo con actualización", cotizacion_id);
    let terminos_count = terminos.len();
    
    // Iniciar transacción
    let mut tx = pool.begin().await.map_err(|e| format!("Database error: {}", e))?;
    
    // Eliminar términos existentes para esta cotización
    sqlx::query("DELETE FROM TERMINOS_COTIZACION WHERE cotizacion_id = ?")
        .bind(cotizacion_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Database error removing existing terms: {}", e))?;
    
    // Insertar los nuevos términos
    for termino in terminos {
        sqlx::query(
            "INSERT INTO TERMINOS_COTIZACION (termino_id, cotizacion_id, aplicado) VALUES (?, ?, ?)"
        )
        .bind(termino.termino_id)
        .bind(cotizacion_id)
        .bind(termino.aplicado.unwrap_or(true))
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Database error applying term: {}", e))?;
    }
    
    // Confirmar transacción
    tx.commit().await.map_err(|e| format!("Database error committing transaction: {}", e))?;
    
    // Registrar en el log de auditoría solo si hubo cambios
    let _ = log_action(
        "APPLY_TERMINOS_COTIZACION",
        Some(applied_by),
        "TERMINOS_COTIZACION",
        Some(cotizacion_id),
        None,
        Some(&format!("Aplicados {} términos a la cotización", terminos_count))
    ).await;
    
    Ok(())
}

/// Aplicar términos por defecto a un informe
#[tauri::command]
pub async fn apply_default_terminos_to_informe(
    informe_id: i32,
    applied_by: i32
) -> Result<(), String> {
    let terminos_default = get_terminos_condiciones_default("informe".to_string()).await?;
    
    let termino_requests: Vec<TerminoInformeRequest> = terminos_default
        .into_iter()
        .map(|t| TerminoInformeRequest {
            termino_id: t.termino_id,
            aplicado: Some(true),
        })
        .collect();
    
    // Registrar específicamente la aplicación de términos por defecto
    let _ = log_action(
        "APPLY_DEFAULT_TERMINOS_INFORME",
        Some(applied_by),
        "TERMINOS_INFORME",
        Some(informe_id),
        None,
        Some(&format!("Aplicados {} términos por defecto al informe", termino_requests.len()))
    ).await;
    
    apply_terminos_to_informe(informe_id, termino_requests, applied_by).await
}

/// Aplicar términos por defecto a una cotización
#[tauri::command]
pub async fn apply_default_terminos_to_cotizacion(
    cotizacion_id: i32,
    applied_by: i32
) -> Result<(), String> {
    let terminos_default = get_terminos_condiciones_default("cotizacion".to_string()).await?;
    
    let termino_requests: Vec<TerminoCotizacionRequest> = terminos_default
        .into_iter()
        .map(|t| TerminoCotizacionRequest {
            termino_id: t.termino_id,
            aplicado: Some(true),
        })
        .collect();
    
    // Registrar específicamente la aplicación de términos por defecto
    let _ = log_action(
        "APPLY_DEFAULT_TERMINOS_COTIZACION",
        Some(applied_by),
        "TERMINOS_COTIZACION",
        Some(cotizacion_id),
        None,
        Some(&format!("Aplicados {} términos por defecto a la cotización", termino_requests.len()))
    ).await;
    
    apply_terminos_to_cotizacion(cotizacion_id, termino_requests, applied_by).await
}

/// Reactivar un término y condición
#[tauri::command]
pub async fn reactivate_termino_condicion(termino_id: i32, reactivated_by: i32) -> Result<(), String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener el término actual para el log
    let current_termino = get_termino_condicion_by_id(termino_id).await?
        .ok_or("Término y condición no encontrado")?;
    
    let result = sqlx::query(
        "UPDATE TERMINOS_CONDICIONES SET is_active = TRUE, updated_at = CURRENT_TIMESTAMP WHERE termino_id = ?"
    )
    .bind(termino_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("Término y condición no encontrado".to_string());
    }
    
    // Registrar en el log de auditoría
    let _ = log_action(
        "REACTIVATE_TERMINO_CONDICION",
        Some(reactivated_by),
        "TERMINOS_CONDICIONES",
        Some(termino_id),
        Some(&format!("Término reactivado: {} ({})", 
            current_termino.termino_nombre, 
            current_termino.tipo_referencia
        )),
        Some("ACTIVE")
    ).await;
    
    Ok(())
}

/// Cambiar estado por defecto de un término
#[tauri::command]
pub async fn toggle_termino_default(
    termino_id: i32,
    is_default: bool,
    updated_by: i32
) -> Result<(), String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener el término actual para el log
    let current_termino = get_termino_condicion_by_id(termino_id).await?
        .ok_or("Término y condición no encontrado")?;
    
    let result = sqlx::query(
        "UPDATE TERMINOS_CONDICIONES SET is_default = ?, updated_at = CURRENT_TIMESTAMP WHERE termino_id = ?"
    )
    .bind(is_default)
    .bind(termino_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("Término y condición no encontrado".to_string());
    }
    
    // Registrar en el log de auditoría
    let _ = log_action(
        "TOGGLE_TERMINO_DEFAULT",
        Some(updated_by),
        "TERMINOS_CONDICIONES",
        Some(termino_id),
        Some(&format!("Estado anterior: {}", current_termino.is_default.unwrap_or(false))),
        Some(&format!("Estado nuevo: {}", is_default))
    ).await;
    
    Ok(())
}

// ==================== MÉTODOS DE RELACIONES ====================

/// Crear relación entre término y condición e informe
#[tauri::command]
pub async fn create_termino_informe_relation(
    termino_id: i32,
    informe_id: i32,
    aplicado: Option<bool>,
    created_by: i32
) -> Result<(), String> {
    let pool = get_db_pool_safe()?;
    
    // Verificar que el término existe
    let termino = get_termino_condicion_by_id(termino_id).await?
        .ok_or("Término y condición no encontrado")?;
    
    let result = sqlx::query(
        "INSERT INTO TERMINOS_INFORME (termino_id, informe_id, aplicado) VALUES (?, ?, ?)"
    )
    .bind(termino_id)
    .bind(informe_id)
    .bind(aplicado.unwrap_or(true))
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("No se pudo crear la relación término-informe".to_string());
    }
    
    // Registrar en el log de auditoría
    let _ = log_action(
        "CREATE_TERMINO_INFORME_RELATION",
        Some(created_by),
        "TERMINOS_INFORME",
        Some(informe_id),
        None,
        Some(&format!("Término '{}' asociado al informe {}", termino.termino_nombre, informe_id))
    ).await;
    
    Ok(())
}

/// Crear relación entre término y condición y cotización
#[tauri::command]
pub async fn create_termino_cotizacion_relation(
    termino_id: i32,
    cotizacion_id: i32,
    aplicado: Option<bool>,
    created_by: i32
) -> Result<(), String> {
    let pool = get_db_pool_safe()?;
    
    // Verificar que el término existe
    let termino = get_termino_condicion_by_id(termino_id).await?
        .ok_or("Término y condición no encontrado")?;
    
    let result = sqlx::query(
        "INSERT INTO TERMINOS_COTIZACION (termino_id, cotizacion_id, aplicado) VALUES (?, ?, ?)"
    )
    .bind(termino_id)
    .bind(cotizacion_id)
    .bind(aplicado.unwrap_or(true))
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("No se pudo crear la relación término-cotización".to_string());
    }
    
    // Registrar en el log de auditoría
    let _ = log_action(
        "CREATE_TERMINO_COTIZACION_RELATION",
        Some(created_by),
        "TERMINOS_COTIZACION",
        Some(cotizacion_id),
        None,
        Some(&format!("Término '{}' asociado a la cotización {}", termino.termino_nombre, cotizacion_id))
    ).await;
    
    Ok(())
}

/// Actualizar relación término-informe (cambiar estado aplicado)
#[tauri::command]
pub async fn update_termino_informe_relation(
    termino_id: i32,
    informe_id: i32,
    aplicado: bool,
    updated_by: i32
) -> Result<(), String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query(
        "UPDATE TERMINOS_INFORME SET aplicado = ? WHERE termino_id = ? AND informe_id = ?"
    )
    .bind(aplicado)
    .bind(termino_id)
    .bind(informe_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("Relación término-informe no encontrada".to_string());
    }
    
    // Registrar en el log de auditoría
    let _ = log_action(
        "UPDATE_TERMINO_INFORME_RELATION",
        Some(updated_by),
        "TERMINOS_INFORME",
        Some(informe_id),
        None,
        Some(&format!("Término {} {} para informe {}", 
            termino_id, 
            if aplicado { "activado" } else { "desactivado" }, 
            informe_id
        ))
    ).await;
    
    Ok(())
}

/// Actualizar relación término-cotización (cambiar estado aplicado)
#[tauri::command]
pub async fn update_termino_cotizacion_relation(
    termino_id: i32,
    cotizacion_id: i32,
    aplicado: bool,
    updated_by: i32
) -> Result<(), String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query(
        "UPDATE TERMINOS_COTIZACION SET aplicado = ? WHERE termino_id = ? AND cotizacion_id = ?"
    )
    .bind(aplicado)
    .bind(termino_id)
    .bind(cotizacion_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("Relación término-cotización no encontrada".to_string());
    }
    
    // Registrar en el log de auditoría
    let _ = log_action(
        "UPDATE_TERMINO_COTIZACION_RELATION",
        Some(updated_by),
        "TERMINOS_COTIZACION",
        Some(cotizacion_id),
        None,
        Some(&format!("Término {} {} para cotización {}", 
            termino_id, 
            if aplicado { "activado" } else { "desactivado" }, 
            cotizacion_id
        ))
    ).await;
    
    Ok(())
}

/// Eliminar relación término-informe
#[tauri::command]
pub async fn delete_termino_informe_relation(
    termino_id: i32,
    informe_id: i32,
    deleted_by: i32
) -> Result<(), String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query(
        "DELETE FROM TERMINOS_INFORME WHERE termino_id = ? AND informe_id = ?"
    )
    .bind(termino_id)
    .bind(informe_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("Relación término-informe no encontrada".to_string());
    }
    
    // Registrar en el log de auditoría
    let _ = log_action(
        "DELETE_TERMINO_INFORME_RELATION",
        Some(deleted_by),
        "TERMINOS_INFORME",
        Some(informe_id),
        None,
        Some(&format!("Eliminada relación término {} con informe {}", termino_id, informe_id))
    ).await;
    
    Ok(())
}

/// Eliminar relación término-cotización
#[tauri::command]
pub async fn delete_termino_cotizacion_relation(
    termino_id: i32,
    cotizacion_id: i32,
    deleted_by: i32
) -> Result<(), String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query(
        "DELETE FROM TERMINOS_COTIZACION WHERE termino_id = ? AND cotizacion_id = ?"
    )
    .bind(termino_id)
    .bind(cotizacion_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Err("Relación término-cotización no encontrada".to_string());
    }
    
    // Registrar en el log de auditoría
    let _ = log_action(
        "DELETE_TERMINO_COTIZACION_RELATION",
        Some(deleted_by),
        "TERMINOS_COTIZACION",
        Some(cotizacion_id),
        None,
        Some(&format!("Eliminada relación término {} con cotización {}", termino_id, cotizacion_id))
    ).await;
    
    Ok(())
}

/// Obtener todos los informes que tienen un término específico aplicado
#[tauri::command]
pub async fn get_informes_by_termino(termino_id: i32) -> Result<Vec<TerminoInforme>, String> {
    let pool = get_db_pool_safe()?;
    
    let informes = sqlx::query_as::<_, TerminoInforme>(
        "SELECT ti.termino_id, ti.informe_id, ti.aplicado, ti.created_at,
                tc.termino_nombre, tc.termino_descripcion
         FROM TERMINOS_INFORME ti
         JOIN TERMINOS_CONDICIONES tc ON ti.termino_id = tc.termino_id
         WHERE ti.termino_id = ?
         ORDER BY ti.created_at DESC"
    )
    .bind(termino_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(informes)
}

/// Obtener todas las cotizaciones que tienen un término específico aplicado
#[tauri::command]
pub async fn get_cotizaciones_by_termino(termino_id: i32) -> Result<Vec<TerminoCotizacion>, String> {
    let pool = get_db_pool_safe()?;
    
    let cotizaciones = sqlx::query_as::<_, TerminoCotizacion>(
        "SELECT tc.termino_id, tc.cotizacion_id, tc.aplicado, tc.created_at,
                t.termino_nombre, t.termino_descripcion
         FROM TERMINOS_COTIZACION tc
         JOIN TERMINOS_CONDICIONES t ON tc.termino_id = t.termino_id
         WHERE tc.termino_id = ?
         ORDER BY tc.created_at DESC"
    )
    .bind(termino_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizaciones)
}

/// Verificar si un término está aplicado a un informe específico
#[tauri::command]
pub async fn check_termino_in_informe(
    termino_id: i32,
    informe_id: i32
) -> Result<Option<bool>, String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query_scalar::<_, Option<bool>>(
        "SELECT aplicado FROM TERMINOS_INFORME WHERE termino_id = ? AND informe_id = ?"
    )
    .bind(termino_id)
    .bind(informe_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(result.flatten())
}

/// Verificar si un término está aplicado a una cotización específica
#[tauri::command]
pub async fn check_termino_in_cotizacion(
    termino_id: i32,
    cotizacion_id: i32
) -> Result<Option<bool>, String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query_scalar::<_, Option<bool>>(
        "SELECT aplicado FROM TERMINOS_COTIZACION WHERE termino_id = ? AND cotizacion_id = ?"
    )
    .bind(termino_id)
    .bind(cotizacion_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(result.flatten())
}