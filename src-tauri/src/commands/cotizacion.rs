use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_safe;
use crate::commands::logs::log_action;
use chrono::{DateTime, Utc};
use chrono::Datelike;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cotizacion {
    pub cotizacion_id: i32,
    pub cotizacion_codigo: Option<String>,
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub costo_total: Option<i32>,
    pub is_aprobada: Option<bool>,
    pub is_borrador: Option<bool>,
    pub informe: String,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Pieza {
    pub pieza_id: i32,
    pub pieza_nombre: Option<String>,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PiezaCotizacion {
    pub pieza_id: i32,
    pub cotizacion_id: i32,
    pub cantidad: Option<i32>,
    // Campos adicionales para JOINs
    pub pieza_nombre: Option<String>,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CotizacionDetallada {
    pub cotizacion_id: i32,
    pub cotizacion_codigo: Option<String>,
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub costo_total: Option<i32>,
    pub is_aprobada: Option<bool>,
    pub is_borrador: Option<bool>,
    pub informe: String,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub created_by_nombre: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCotizacionRequest {
    // pub cotizacion_codigo: String, // Eliminar este campo
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub costo_total: Option<i32>,
    pub is_aprobada: Option<bool>,
    pub is_borrador: Option<bool>,
    pub informe: String,
    pub created_by: i32,
    pub piezas: Option<Vec<PiezaCotizacionRequest>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCotizacionRequest {
    pub cotizacion_codigo: Option<String>,
    pub costo_revision: Option<i32>,
    pub costo_reparacion: Option<i32>,
    pub costo_total: Option<i32>,
    pub is_aprobada: Option<bool>,
    pub is_borrador: Option<bool>,
    pub informe: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePiezaRequest {
    pub pieza_nombre: String,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PiezaCotizacionRequest {
    pub pieza_id: i32,
    pub cantidad: i32,
}

/// Obtener todas las cotizaciones
#[tauri::command]
pub async fn get_cotizaciones() -> Result<Vec<Cotizacion>, String> {
    let pool = get_db_pool_safe()?;
    
    let cotizaciones = sqlx::query_as::<_, Cotizacion>(
        "SELECT cotizacion_id, cotizacion_codigo, costo_revision, costo_reparacion, \
                costo_total, is_aprobada, is_borrador, informe, created_by, created_at \
         FROM COTIZACION \
         ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizaciones)
}

/// Obtener cotizaciones con información detallada
#[tauri::command]
pub async fn get_cotizaciones_detalladas() -> Result<Vec<CotizacionDetallada>, String> {
    let pool = get_db_pool_safe()?;
    
    let cotizaciones = sqlx::query_as::<_, CotizacionDetallada>(
        "SELECT c.cotizacion_id, c.cotizacion_codigo, c.costo_revision, c.costo_reparacion,
                c.costo_total, c.is_aprobada, c.is_borrador, c.created_by, c.created_at,
                u.usuario_nombre as created_by_nombre
         FROM COTIZACION c
         LEFT JOIN USUARIO u ON c.created_by = u.usuario_id
         ORDER BY c.created_at DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizaciones)
}

/// Obtener una cotización por ID
#[tauri::command]
pub async fn get_cotizacion_by_id(cotizacion_id: i32) -> Result<Option<Cotizacion>, String> {
    let pool = get_db_pool_safe()?;
    
    let cotizacion = sqlx::query_as::<_, Cotizacion>(
        "SELECT cotizacion_id, cotizacion_codigo, costo_revision, costo_reparacion,\
                costo_total, is_aprobada, is_borrador, informe, created_by, created_at \
         FROM COTIZACION \
         WHERE cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizacion)
}

/// Obtener una cotización por código
#[tauri::command]
pub async fn get_cotizacion_by_codigo(cotizacion_codigo: String) -> Result<Option<Cotizacion>, String> {
    let pool = get_db_pool_safe()?;
    
    let cotizacion = sqlx::query_as::<_, Cotizacion>(
        "SELECT cotizacion_id, cotizacion_codigo, costo_revision, costo_reparacion,\
                costo_total, is_aprobada, is_borrador, informe, created_by, created_at \
         FROM COTIZACION \
         WHERE cotizacion_codigo = ?"
    )
    .bind(&cotizacion_codigo)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizacion)
}

/// Crear una nueva cotización
#[tauri::command]
pub async fn create_cotizacion(request: CreateCotizacionRequest) -> Result<Cotizacion, String> {
    let pool = get_db_pool_safe()?;
    // Generar código automático: COT-YYYY-XXXX
    let year = chrono::Utc::now().year();
    // Buscar el mayor número correlativo existente para el año actual
    let last_codigo: Option<String> = sqlx::query_scalar(
        "SELECT cotizacion_codigo FROM COTIZACION WHERE cotizacion_codigo LIKE ? ORDER BY cotizacion_id DESC LIMIT 1"
    )
    .bind(format!("COT-{}-%", year))
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
    let codigo = format!("COT-{}-{:03}", year, next_number);
    // Iniciar transacción
    let mut tx = pool.begin().await.map_err(|e| format!("Database error: {}", e))?;
    // Crear la cotización
    let result = sqlx::query(
        "INSERT INTO COTIZACION (cotizacion_codigo, costo_revision, costo_reparacion, \
                                costo_total, is_aprobada, is_borrador, informe, created_by) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&codigo)
    .bind(request.costo_revision)
    .bind(request.costo_reparacion)
    .bind(request.costo_total)
    .bind(request.is_aprobada.unwrap_or(false))
    .bind(request.is_borrador.unwrap_or(true))
    .bind(&request.informe)
    .bind(request.created_by)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    let cotizacion_id = result.last_insert_id() as i32;
    // Agregar piezas si se proporcionaron
    if let Some(ref piezas) = request.piezas {
        for pieza in piezas {
            sqlx::query(
                "INSERT INTO PIEZAS_COTIZACION (pieza_id, cotizacion_id, cantidad) VALUES (?, ?, ?)"
            )
            .bind(pieza.pieza_id)
            .bind(cotizacion_id)
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
        "CREATE_COTIZACION",
        Some(request.created_by),
        "COTIZACION",
        Some(cotizacion_id),
        None,
        Some(&format!("Cotización creada: {}", codigo))
    ).await;
    // Obtener la cotización recién creada
    get_cotizacion_by_id(cotizacion_id)
        .await?
        .ok_or_else(|| "Failed to retrieve created cotización".to_string())
}

/// Actualizar una cotización existente
#[tauri::command]
pub async fn update_cotizacion(cotizacion_id: i32, request: UpdateCotizacionRequest, updated_by: i32) -> Result<Option<Cotizacion>, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener la cotización actual para logging
    let current_cotizacion = get_cotizacion_by_id(cotizacion_id).await?;
    
    // Verificar que el código no está en uso por otra cotización (si se está actualizando)
    if let Some(ref new_codigo) = request.cotizacion_codigo {
        if let Some(existing_cotizacion) = get_cotizacion_by_codigo(new_codigo.clone()).await? {
            if existing_cotizacion.cotizacion_id != cotizacion_id {
                return Err("Ya existe otra cotización con este código".to_string());
            }
        }
    }
    
    let result = sqlx::query(
        "UPDATE COTIZACION SET \
         cotizacion_codigo = COALESCE(?, cotizacion_codigo),\
         costo_revision = COALESCE(?, costo_revision),\
         costo_reparacion = COALESCE(?, costo_reparacion),\
         costo_total = COALESCE(?, costo_total),\
         is_aprobada = COALESCE(?, is_aprobada),\
         is_borrador = COALESCE(?, is_borrador),\
         informe = COALESCE(?, informe)\
         WHERE cotizacion_id = ?"
    )
    .bind(&request.cotizacion_codigo)
    .bind(request.costo_revision)
    .bind(request.costo_reparacion)
    .bind(request.costo_total)
    .bind(request.is_aprobada)
    .bind(request.is_borrador)
    .bind(&request.informe)
    .bind(cotizacion_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() == 0 {
        return Ok(None);
    }
    
    // Registrar la acción en el log de auditoría
    if let Some(ref cotizacion) = current_cotizacion {
        let prev_data = format!("{}|{}|{}|{}|{}|{}", 
            cotizacion.cotizacion_codigo.as_deref().unwrap_or(""), 
            cotizacion.costo_revision.map_or("".to_string(), |p| p.to_string()),
            cotizacion.costo_reparacion.map_or("".to_string(), |p| p.to_string()),
            cotizacion.costo_total.map_or("".to_string(), |p| p.to_string()),
            cotizacion.is_aprobada.map_or("".to_string(), |p| p.to_string()),
            cotizacion.is_borrador.map_or("".to_string(), |p| p.to_string())
        );
        let new_data = format!("{}|{}|{}|{}|{}|{}", 
            request.cotizacion_codigo.as_deref().unwrap_or(cotizacion.cotizacion_codigo.as_deref().unwrap_or("")),
            request.costo_revision
                .or(cotizacion.costo_revision)
                .map_or("".to_string(), |p| p.to_string()),
            request.costo_reparacion
                .or(cotizacion.costo_reparacion)
                .map_or("".to_string(), |p| p.to_string()),
            request.costo_total
                .or(cotizacion.costo_total)
                .map_or("".to_string(), |p| p.to_string()),
            request.is_aprobada
                .or(cotizacion.is_aprobada)
                .map_or("".to_string(), |p| p.to_string()),
            request.is_borrador
                .or(cotizacion.is_borrador)
                .map_or("".to_string(), |p| p.to_string())
        );
        
        let _ = log_action(
            "UPDATE_COTIZACION",
            Some(updated_by),
            "COTIZACION",
            Some(cotizacion_id),
            Some(&prev_data),
            Some(&new_data)
        ).await;
    }
    
    get_cotizacion_by_id(cotizacion_id).await
}

/// Eliminar una cotización
#[tauri::command]
pub async fn delete_cotizacion(cotizacion_id: i32, deleted_by: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener la cotización antes de eliminarla para logging
    let cotizacion_to_delete = get_cotizacion_by_id(cotizacion_id).await?;
    
    // Verificar si la cotización tiene órdenes de trabajo asociadas
    let has_dependencies = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM ORDEN_TRABAJO WHERE cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Database error checking dependencies: {}", e))?;
    
    if has_dependencies > 0 {
        return Err("No se puede eliminar la cotización porque tiene órdenes de trabajo asociadas".to_string());
    }
    
    // Iniciar transacción
    let mut tx = pool.begin().await.map_err(|e| format!("Database error: {}", e))?;
    
    // Eliminar primero las relaciones con piezas
    sqlx::query("DELETE FROM PIEZAS_COTIZACION WHERE cotizacion_id = ?")
        .bind(cotizacion_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Luego eliminar la cotización
    let result = sqlx::query("DELETE FROM COTIZACION WHERE cotizacion_id = ?")
        .bind(cotizacion_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    let was_deleted = result.rows_affected() > 0;
    
    // Confirmar transacción
    tx.commit().await.map_err(|e| format!("Database error: {}", e))?;
    
    // Registrar la acción en el log de auditoría
    if was_deleted {
        if let Some(ref cotizacion) = cotizacion_to_delete {
            let _ = log_action(
                "DELETE_COTIZACION",
                Some(deleted_by),
                "COTIZACION",
                Some(cotizacion_id),
                Some(&format!("Cotización eliminada: {}", 
                    cotizacion.cotizacion_codigo.as_deref().unwrap_or("N/A")
                )),
                None
            ).await;
        }
    }
    
    Ok(was_deleted)
}

/// Obtener todas las piezas
#[tauri::command]
pub async fn get_piezas() -> Result<Vec<Pieza>, String> {
    let pool = get_db_pool_safe()?;
    let piezas = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, created_at FROM PIEZA ORDER BY pieza_nombre ASC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(piezas)
}

/// Obtener una pieza por ID
#[tauri::command]
pub async fn get_pieza_by_id(pieza_id: i32) -> Result<Option<Pieza>, String> {
    let pool = get_db_pool_safe()?;
    let pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(pieza)
}

/// Crear una nueva pieza
#[tauri::command]
pub async fn create_pieza(request: CreatePiezaRequest) -> Result<Pieza, String> {
    let pool = get_db_pool_safe()?;
    let result = sqlx::query(
        "INSERT INTO PIEZA (pieza_nombre, pieza_marca, pieza_desc, pieza_precio) VALUES (?, ?, ?, ?)"
    )
    .bind(&request.pieza_nombre)
    .bind(&request.pieza_marca)
    .bind(&request.pieza_desc)
    .bind(request.pieza_precio)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    let pieza_id = result.last_insert_id() as i32;
    let pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    // Log de creación de pieza
    let _ = log_action(
        "CREATE_PIEZA",
        None,
        "PIEZA",
        Some(pieza_id),
        None,
        Some(&format!("Pieza creada: {}", request.pieza_nombre))
    ).await;
    Ok(pieza)
}

#[derive(Debug, Deserialize)]
pub struct UpdatePiezaRequest {
    pub pieza_nombre: Option<String>,
    pub pieza_marca: Option<String>,
    pub pieza_desc: Option<String>,
    pub pieza_precio: Option<i32>,
}

/// Actualizar una pieza existente
#[tauri::command]
pub async fn update_pieza(pieza_id: i32, request: UpdatePiezaRequest) -> Result<Option<Pieza>, String> {
    let pool = get_db_pool_safe()?;
    // Obtener datos previos para el log
    let prev_pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    let result = sqlx::query(
        "UPDATE PIEZA SET \
            pieza_nombre = COALESCE(?, pieza_nombre),\
            pieza_marca = COALESCE(?, pieza_marca),\
            pieza_desc = COALESCE(?, pieza_desc),\
            pieza_precio = COALESCE(?, pieza_precio)\
         WHERE pieza_id = ?"
    )
    .bind(&request.pieza_nombre)
    .bind(&request.pieza_marca)
    .bind(&request.pieza_desc)
    .bind(request.pieza_precio)
    .bind(pieza_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    if result.rows_affected() == 0 {
        return Ok(None);
    }
    let pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    // Log de actualización de pieza
    let prev = prev_pieza.as_ref().map(|p| format!("{}|{}|{}|{}", p.pieza_nombre.as_deref().unwrap_or(""), p.pieza_marca.as_deref().unwrap_or(""), p.pieza_desc.as_deref().unwrap_or(""), p.pieza_precio.map_or("".to_string(), |v| v.to_string())));
    let newv = format!("{}|{}|{}|{}", pieza.pieza_nombre.as_deref().unwrap_or(""), pieza.pieza_marca.as_deref().unwrap_or(""), pieza.pieza_desc.as_deref().unwrap_or(""), pieza.pieza_precio.map_or("".to_string(), |v| v.to_string()));
    let _ = log_action(
        "UPDATE_PIEZA",
        None,
        "PIEZA",
        Some(pieza_id),
        prev.as_deref(),
        Some(&newv)
    ).await;
    Ok(Some(pieza))
}

/// Eliminar una pieza
#[tauri::command]
pub async fn delete_pieza(pieza_id: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    // Obtener datos previos para el log
    let prev_pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    let result = sqlx::query("DELETE FROM PIEZA WHERE pieza_id = ?")
        .bind(pieza_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    let deleted = result.rows_affected() > 0;
    // Log de eliminación de pieza
    if deleted {
        if let Some(p) = prev_pieza {
            let prev = format!("{}|{}|{}|{}", p.pieza_nombre.as_deref().unwrap_or(""), p.pieza_marca.as_deref().unwrap_or(""), p.pieza_desc.as_deref().unwrap_or(""), p.pieza_precio.map_or("".to_string(), |v| v.to_string()));
            let _ = log_action(
                "DELETE_PIEZA",
                None,
                "PIEZA",
                Some(pieza_id),
                Some(&prev),
                None
            ).await;
        }
    }
    Ok(deleted)
}

/// Buscar cotizaciones por texto
#[tauri::command]
pub async fn search_cotizaciones(search_term: String) -> Result<Vec<CotizacionDetallada>, String> {
    let pool = get_db_pool_safe()?;
    
    let search_pattern = format!("%{}%", search_term);
    
    let cotizaciones = sqlx::query_as::<_, CotizacionDetallada>(
        "SELECT c.cotizacion_id, c.cotizacion_codigo, c.costo_revision, c.costo_reparacion,\
                c.costo_total, c.is_aprobada, c.is_borrador, c.informe, c.created_by, c.created_at,\
                u.usuario_nombre as created_by_nombre\
         FROM COTIZACION c\
         LEFT JOIN USUARIO u ON c.created_by = u.usuario_id\
         WHERE c.cotizacion_codigo LIKE ? \n         ORDER BY c.created_at DESC"
    )
    .bind(&search_pattern)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizaciones)
}

/// Contar total de cotizaciones
#[tauri::command]
pub async fn count_cotizaciones() -> Result<i64, String> {
    let pool = get_db_pool_safe()?;
    
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM COTIZACION")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(count)
}

/// Obtener cotizaciones con paginación
#[tauri::command]
pub async fn get_cotizaciones_with_pagination(offset: i64, limit: i64) -> Result<Vec<CotizacionDetallada>, String> {
    let pool = get_db_pool_safe()?;
    
    let cotizaciones = sqlx::query_as::<_, CotizacionDetallada>(
        "SELECT c.cotizacion_id, c.cotizacion_codigo, c.costo_revision, c.costo_reparacion,\
                c.costo_total, c.is_aprobada, c.is_borrador, c.informe, c.created_by, c.created_at,\
                u.usuario_nombre as created_by_nombre\
         FROM COTIZACION c\
         LEFT JOIN USUARIO u ON c.created_by = u.usuario_id\
         ORDER BY c.created_at DESC\n         LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizaciones)
}
