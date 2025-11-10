use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_safe;
use crate::commands::logs::log_action;
use crate::commands::terminos_condiciones::apply_default_terminos_to_cotizacion;
use chrono::{DateTime, Utc};
use chrono::Datelike;
use sqlx::Row;

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
    pub pieza_stock: Option<i32>,
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

#[derive(Debug)]
struct OrdenInfoRow {
    // orden_id: i32,  // Campo no utilizado
    orden_codigo: Option<String>,
    estado: String,
    // equipo_nombre: Option<String>,  // Campo no utilizado
    // cliente_nombre: Option<String>,  // Campo no utilizado
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
    .fetch_all(&*pool)
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
    .fetch_all(&*pool)
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
    .fetch_optional(&*pool)
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
    .fetch_optional(&*pool)
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
    
    // Aplicar términos y condiciones por defecto automáticamente
    let _ = apply_default_terminos_to_cotizacion(cotizacion_id, request.created_by).await;
    
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
    
    let current_cotizacion = get_cotizacion_by_id(cotizacion_id).await?;
    if let Some(ref cotizacion) = current_cotizacion {
        // Si estaba en borrador y ahora se rechaza
        if cotizacion.is_borrador == Some(true) && request.is_aprobada == Some(false) {
            // Guardar el motivo si corresponde (ya lo haces)
            // Desvincular la cotización de la orden de trabajo
            sqlx::query("UPDATE ORDEN_TRABAJO SET cotizacion_id = NULL WHERE cotizacion_id = ?")
                .bind(cotizacion_id)
                .execute(&*pool)
                .await
                .map_err(|e| format!("Database error al desvincular cotización: {}", e))?;

            // AuditLog para rechazo de borrador
            let motivo = request.informe.as_deref().unwrap_or(""); // O usa request.motivo_rechazo si tienes ese campo
            let _ = log_action(
                "ELIMINAR_BORRADOR_COTIZACION",
                Some(updated_by),
                "COTIZACION",
                Some(cotizacion_id),
                Some("Cotización en borrador rechazada"),
                Some(motivo)
            ).await;
        }
    }
    
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
    .execute(&*pool)
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
    .fetch_one(&*pool)
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
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at FROM PIEZA ORDER BY pieza_nombre ASC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(piezas)
}

/// Obtener una pieza por ID
#[tauri::command]
pub async fn get_pieza_by_id(pieza_id: i32) -> Result<Option<Pieza>, String> {
    let pool = get_db_pool_safe()?;
    let pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_optional(&*pool)
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
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    let pieza_id = result.last_insert_id() as i32;
    let pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_one(&*pool)
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
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_optional(&*pool)
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
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    if result.rows_affected() == 0 {
        return Ok(None);
    }
    let pieza = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_one(&*pool)
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
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    let result = sqlx::query("DELETE FROM PIEZA WHERE pieza_id = ?")
        .bind(pieza_id)
        .execute(&*pool)
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
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizaciones)
}

/// Contar total de cotizaciones
#[tauri::command]
pub async fn count_cotizaciones() -> Result<i64, String> {
    let pool = get_db_pool_safe()?;
    
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM COTIZACION")
        .fetch_one(&*pool)
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
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(cotizaciones)
}

/// Obtener las piezas asociadas a una cotización
#[tauri::command]
pub async fn get_piezas_cotizacion(cotizacion_id: i32) -> Result<Vec<PiezaCotizacion>, String> {
    let pool = get_db_pool_safe()?;
    let piezas = sqlx::query_as::<_, PiezaCotizacion>(
        "SELECT pc.pieza_id, pc.cotizacion_id, COALESCE(pc.cantidad, 1) as cantidad, \
                p.pieza_nombre, p.pieza_marca, p.pieza_desc, p.pieza_precio \
         FROM PIEZAS_COTIZACION pc \
         LEFT JOIN PIEZA p ON pc.pieza_id = p.pieza_id \
         WHERE pc.cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(piezas)
}

#[tauri::command]
pub async fn get_cotizaciones_by_cliente(cliente_id: i32) -> Result<Vec<Cotizacion>, String> {
    let pool = get_db_pool_safe()?;
    let cotizaciones = sqlx::query_as::<_, Cotizacion>(
        "SELECT c.cotizacion_id, c.cotizacion_codigo, c.costo_revision, c.costo_reparacion, \
                c.costo_total, c.is_aprobada, c.is_borrador, c.informe, c.created_by, c.created_at \
         FROM COTIZACION c \
         INNER JOIN ORDEN_TRABAJO ot ON c.cotizacion_id = ot.cotizacion_id \
         INNER JOIN EQUIPO e ON ot.equipo_id = e.equipo_id \
         WHERE e.cliente_id = ? \
         GROUP BY c.cotizacion_id \
         ORDER BY c.created_at DESC"
    )
    .bind(cliente_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error al obtener cotizaciones del cliente: {}", e))?;
    Ok(cotizaciones)
}

/// Obtener todas las piezas con información de inventario
#[tauri::command]
pub async fn get_piezas_inventario() -> Result<Vec<Pieza>, String> {
    let pool = get_db_pool_safe()?;
    let piezas = sqlx::query_as::<_, Pieza>(
        "SELECT pieza_id, pieza_nombre, pieza_marca, pieza_desc, pieza_precio, pieza_stock, created_at 
         FROM PIEZA 
         ORDER BY pieza_nombre ASC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(piezas)
}

/// Actualizar el stock de una pieza
#[tauri::command]
pub async fn update_pieza_stock(pieza_id: i32, cantidad: i32, tipo: String) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener el stock actual
    let current_stock = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT pieza_stock FROM PIEZA WHERE pieza_id = ?"
    )
    .bind(pieza_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Database error getting current stock: {}", e))?
    .unwrap_or(0);
    
    // Calcular el nuevo stock
    let new_stock = match tipo.as_str() {
        "entrada" => current_stock + cantidad,
        "salida" => std::cmp::max(0, current_stock - cantidad),
        _ => return Err("Tipo de operación inválido. Use 'entrada' o 'salida'".to_string()),
    };
    
    // Actualizar el stock
    let result = sqlx::query(
        "UPDATE PIEZA SET pieza_stock = ? WHERE pieza_id = ?"
    )
    .bind(new_stock)
    .bind(pieza_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error updating stock: {}", e))?;
    
    if result.rows_affected() > 0 {
        // Log de la operación
        let _ = log_action(
            "UPDATE_STOCK",
            None,
            "PIEZA",
            Some(pieza_id),
            Some(&format!("Stock anterior: {}", current_stock)),
            Some(&format!("Stock nuevo: {} ({})", new_stock, if tipo == "entrada" { format!("+{}", cantidad) } else { format!("-{}", cantidad) }))
        ).await;
        Ok(true)
    } else {
        Ok(false)
    }
}

// Estructuras para inventario de equipos
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InventarioEquipo {
    pub inventario_equipo_id: i32,
    pub equipo_codigo: Option<String>,
    pub equipo_nombre: Option<String>,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: Option<String>,
    pub equipo_descripcion: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_stock: Option<i32>,
    pub equipo_estado: Option<String>,
    pub equipo_ubicacion: Option<String>,
    pub fecha_adquisicion: Option<String>,
    pub proveedor: Option<String>,
    pub numero_serie: Option<String>,
    pub garantia_vencimiento: Option<String>,
    pub observaciones: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventarioEquipoRequest {
    pub equipo_codigo: String,
    pub equipo_nombre: String,
    pub equipo_marca: Option<String>,
    pub equipo_modelo: Option<String>,
    pub equipo_tipo: String,
    pub equipo_descripcion: Option<String>,
    pub equipo_precio: Option<i32>,
    pub equipo_stock: Option<i32>,
    pub equipo_ubicacion: Option<String>,
    pub proveedor: Option<String>,
    pub numero_serie: Option<String>,
    pub observaciones: Option<String>,
}

// Funciones para inventario de equipos
#[tauri::command]
pub async fn get_inventario_equipos() -> Result<Vec<InventarioEquipo>, String> {
    let pool = get_db_pool_safe()?;
    
    println!("Ejecutando query para obtener inventario de equipos...");
    
    let equipos = sqlx::query_as::<_, InventarioEquipo>(
        "SELECT * FROM INVENTARIO_EQUIPO ORDER BY equipo_nombre"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| {
        println!("Error en query INVENTARIO_EQUIPO: {}", e);
        format!("Database error: {}", e)
    })?;
    
    println!("Equipos encontrados: {}", equipos.len());
    Ok(equipos)
}

#[tauri::command]
pub async fn create_inventario_equipo(request: InventarioEquipoRequest) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query(
        "INSERT INTO INVENTARIO_EQUIPO (
            equipo_codigo, equipo_nombre, equipo_marca, equipo_modelo, 
            equipo_tipo, equipo_descripcion, equipo_precio, equipo_stock,
            equipo_estado, equipo_ubicacion, proveedor, numero_serie, observaciones
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'disponible', ?, ?, ?, ?)"
    )
    .bind(&request.equipo_codigo)
    .bind(&request.equipo_nombre)
    .bind(&request.equipo_marca)
    .bind(&request.equipo_modelo)
    .bind(&request.equipo_tipo)
    .bind(&request.equipo_descripcion)
    .bind(&request.equipo_precio)
    .bind(&request.equipo_stock.unwrap_or(0))
    .bind(&request.equipo_ubicacion)
    .bind(&request.proveedor)
    .bind(&request.numero_serie)
    .bind(&request.observaciones)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() > 0 {
        let _ = log_action(
            "CREATE",
            None,
            "INVENTARIO_EQUIPO",
            None,
            None,
            Some(&format!("Creado equipo: {} - {}", request.equipo_codigo, request.equipo_nombre))
        ).await;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn update_inventario_equipo(equipo_id: i32, request: InventarioEquipoRequest) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    let result = sqlx::query(
        "UPDATE INVENTARIO_EQUIPO SET 
            equipo_codigo = ?, equipo_nombre = ?, equipo_marca = ?, equipo_modelo = ?,
            equipo_tipo = ?, equipo_descripcion = ?, equipo_precio = ?, equipo_stock = ?,
            equipo_ubicacion = ?, proveedor = ?, numero_serie = ?, observaciones = ?
        WHERE inventario_equipo_id = ?"
    )
    .bind(&request.equipo_codigo)
    .bind(&request.equipo_nombre)
    .bind(&request.equipo_marca)
    .bind(&request.equipo_modelo)
    .bind(&request.equipo_tipo)
    .bind(&request.equipo_descripcion)
    .bind(&request.equipo_precio)
    .bind(&request.equipo_stock.unwrap_or(0))
    .bind(&request.equipo_ubicacion)
    .bind(&request.proveedor)
    .bind(&request.numero_serie)
    .bind(&request.observaciones)
    .bind(equipo_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() > 0 {
        let _ = log_action(
            "UPDATE",
            None,
            "INVENTARIO_EQUIPO",
            Some(equipo_id),
            None,
            Some(&format!("Actualizado equipo: {} - {}", request.equipo_codigo, request.equipo_nombre))
        ).await;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn delete_inventario_equipo(equipo_id: i32) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Primero obtenemos la información del equipo para el log
    let equipo_info = sqlx::query_as::<_, InventarioEquipo>(
        "SELECT * FROM INVENTARIO_EQUIPO WHERE inventario_equipo_id = ?"
    )
    .bind(equipo_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let result = sqlx::query(
        "DELETE FROM INVENTARIO_EQUIPO WHERE inventario_equipo_id = ?"
    )
    .bind(equipo_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    if result.rows_affected() > 0 {
        if let Some(equipo) = equipo_info {
            let _ = log_action(
                "DELETE",
                None,
                "INVENTARIO_EQUIPO",
                Some(equipo_id),
                None,
                Some(&format!("Eliminado equipo: {} - {}", 
                    equipo.equipo_codigo.unwrap_or_default(), 
                    equipo.equipo_nombre.unwrap_or_default()))
            ).await;
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn update_inventario_equipo_stock(equipo_id: i32, cantidad: i32, tipo: String) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Primero obtenemos el stock actual
    let current_stock: i32 = sqlx::query_scalar(
        "SELECT equipo_stock FROM INVENTARIO_EQUIPO WHERE inventario_equipo_id = ?"
    )
    .bind(equipo_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?
    .unwrap_or(0);
    
    let new_stock = match tipo.as_str() {
        "entrada" => current_stock + cantidad,
        "salida" => std::cmp::max(0, current_stock - cantidad),
        _ => return Err("Tipo de operación no válido. Use 'entrada' o 'salida'".to_string()),
    };
    
    let result = sqlx::query(
        "UPDATE INVENTARIO_EQUIPO SET equipo_stock = ? WHERE inventario_equipo_id = ?"
    )
    .bind(new_stock)
    .bind(equipo_id)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Database error updating stock: {}", e))?;
    
    if result.rows_affected() > 0 {
        // Log de la operación
        let _ = log_action(
            "UPDATE_STOCK",
            None,
            "INVENTARIO_EQUIPO",
            Some(equipo_id),
            Some(&format!("Stock anterior: {}", current_stock)),
            Some(&format!("Stock nuevo: {} ({})", new_stock, if tipo == "entrada" { format!("+{}", cantidad) } else { format!("-{}", cantidad) }))
        ).await;
        Ok(true)
    } else {
        Ok(false)
    }
}

// ===============================
// STRUCTS Y COMANDOS PARA SALIDAS DE EQUIPOS
// ===============================

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SalidaEquipo {
    pub salida_id: i32,
    pub orden_trabajo_id: i32,
    pub motivo_salida: String,
    pub fecha_salida: Option<DateTime<Utc>>,
    pub usuario_id: Option<i32>,
    pub observaciones: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    // Campos adicionales para JOINs
    pub orden_codigo: Option<String>,
    pub equipo_nombre: Option<String>,
    pub cliente_nombre: Option<String>,
    pub usuario_nombre: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrarSalidaRequest {
    pub orden_trabajo_id: i32,
    pub motivo_salida: String,
    pub observaciones: Option<String>,
    pub usuario_id: i32,
}

/// Registrar salida de equipo en tabla específica (NUEVA IMPLEMENTACIÓN)
#[tauri::command]
pub async fn registrar_salida_equipo_v2(request: RegistrarSalidaRequest) -> Result<bool, String> {
    let pool = get_db_pool_safe()?;
    
    // Verificar que la orden existe y está en estado válido
    let orden_info = sqlx::query(
        "SELECT ot.orden_id, ot.orden_codigo, ot.estado, 
                CONCAT(e.equipo_marca, ' ', e.equipo_modelo) as equipo_nombre, 
                c.cliente_nombre
         FROM ORDEN_TRABAJO ot
         JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         JOIN CLIENTE c ON e.cliente_id = c.cliente_id
         WHERE ot.orden_id = ?"
    )
    .bind(request.orden_trabajo_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error al verificar orden: {}", e))?;
    
    let orden_row = orden_info.ok_or("Orden de trabajo no encontrada")?;
    
    // Extraer los campos usando try_get
    // let orden_id: i32 = orden_row.try_get("orden_id")  // Campo no utilizado
    //     .map_err(|e| format!("Error obteniendo orden_id: {}", e))?;
    let orden_codigo: Option<String> = orden_row.try_get("orden_codigo").ok().flatten();
    let estado: String = orden_row.try_get("estado")
        .map_err(|e| format!("Error obteniendo estado: {}", e))?;
    // let equipo_nombre: Option<String> = orden_row.try_get("equipo_nombre").ok().flatten();  // Campo no utilizado
    // let cliente_nombre: Option<String> = orden_row.try_get("cliente_nombre").ok().flatten();  // Campo no utilizado

    // Crear una estructura para usar en el resto del código
    let orden = OrdenInfoRow {
        // orden_id,  // Campo no utilizado
        orden_codigo,
        estado,
        // equipo_nombre,  // Campo no utilizado
        // cliente_nombre,  // Campo no utilizado
    };
    
    // Estados que permiten salida
    let estados_validos = vec![
        "recibido", "cotizacion_enviada", "aprobacion_pendiente",
        "en_reparacion", "espera_de_retiro", "cotizacion_rechazada"
    ];
    
    if !estados_validos.contains(&orden.estado.as_str()) {
        return Err(format!("No se puede registrar salida. Estado actual: {}", orden.estado));
    }
    
    // Verificar que no hay salida previa registrada
    let salida_existente = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM SALIDA_EQUIPO WHERE orden_trabajo_id = ?"
    )
    .bind(request.orden_trabajo_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al verificar salida existente: {}", e))?;
    
    if salida_existente > 0 {
        return Err("Ya se registró una salida para esta orden de trabajo".to_string());
    }
    
    // Determinar nuevo estado según motivo
    let nuevo_estado = match request.motivo_salida.as_str() {
        "entregado_cliente" => "entregado",
        "retirado_sin_reparacion" | "abandonado" => "abandonado",
        "baja_definitiva" => "equipo_no_reparable",
        _ => return Err("Motivo de salida no válido".to_string()),
    };
    
    // Iniciar transacción
    let mut tx = pool.begin().await.map_err(|e| format!("Error iniciando transacción: {}", e))?;
    
    // Registrar la salida
    let result = sqlx::query(
        "INSERT INTO SALIDA_EQUIPO (orden_trabajo_id, motivo_salida, usuario_id, observaciones)
         VALUES (?, ?, ?, ?)"
    )
    .bind(request.orden_trabajo_id)
    .bind(&request.motivo_salida)
    .bind(request.usuario_id)
    .bind(&request.observaciones)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Error registrando salida: {}", e))?;
    
    let salida_id = result.last_insert_id() as i32;
    
    // Actualizar estado de la orden
    sqlx::query("UPDATE ORDEN_TRABAJO SET estado = ? WHERE orden_id = ?")
        .bind(&nuevo_estado)
        .bind(request.orden_trabajo_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Error actualizando estado orden: {}", e))?;
    
    // Confirmar transacción
    tx.commit().await.map_err(|e| format!("Error confirmando transacción: {}", e))?;
    
    // Log de auditoría
    let _ = log_action(
        "REGISTRAR_SALIDA_EQUIPO",
        Some(request.usuario_id),
        "SALIDA_EQUIPO",
        Some(salida_id),
        None,
        Some(&format!("Salida registrada para orden {} - Motivo: {}", 
            orden.orden_codigo.unwrap_or_default(), 
            request.motivo_salida))
    ).await;
    
    Ok(true)
}

/// Obtener historial completo de salidas
#[tauri::command]
pub async fn get_salidas_equipo() -> Result<Vec<SalidaEquipo>, String> {
    let pool = get_db_pool_safe()?;
    
    let salidas = sqlx::query_as::<_, SalidaEquipo>(
        "SELECT s.salida_id, s.orden_trabajo_id, s.motivo_salida, s.fecha_salida,
                s.usuario_id, s.observaciones, s.created_at,
                ot.orden_codigo, 
                CONCAT(e.equipo_marca, ' ', e.equipo_modelo) as equipo_nombre, 
                c.cliente_nombre, u.usuario_nombre
         FROM SALIDA_EQUIPO s
         JOIN ORDEN_TRABAJO ot ON s.orden_trabajo_id = ot.orden_id
         JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         JOIN CLIENTE c ON e.cliente_id = c.cliente_id
         LEFT JOIN USUARIO u ON s.usuario_id = u.usuario_id
         ORDER BY s.fecha_salida DESC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo salidas: {}", e))?;
    
    Ok(salidas)
}

/// Verificar si una orden puede registrar salida (NUEVA VALIDACIÓN)
#[tauri::command]
pub async fn puede_registrar_salida_v2(orden_trabajo_id: i32) -> Result<(bool, String), String> {
    let pool = get_db_pool_safe()?;
    
    // Verificar estado de la orden
    let estado = sqlx::query_scalar::<_, Option<String>>(
        "SELECT estado FROM ORDEN_TRABAJO WHERE orden_id = ?"
    )
    .bind(orden_trabajo_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error verificando estado: {}", e))?
    .flatten();
    
    let estado = estado.ok_or("Orden no encontrada")?;
    
    // Verificar que no hay salida registrada
    let salida_existente = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM SALIDA_EQUIPO WHERE orden_trabajo_id = ?"
    )
    .bind(orden_trabajo_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error verificando salida: {}", e))?;
    
    let estados_validos = vec![
        "recibido", "cotizacion_enviada", "aprobacion_pendiente",
        "en_reparacion", "espera_de_retiro", "cotizacion_rechazada"
    ];
    
    let puede_registrar = estados_validos.contains(&estado.as_str()) && salida_existente == 0;
    
    let mensaje = if salida_existente > 0 {
        format!("Ya se registró salida para esta orden")
    } else if !estados_validos.contains(&estado.as_str()) {
        format!("Estado '{}' no permite registro de salida", estado)
    } else {
        format!("Puede registrar salida - Estado: {}", estado)
    };
    
    Ok((puede_registrar, mensaje))
}

/// Obtener salida específica por orden de trabajo
#[tauri::command]
pub async fn get_salida_by_orden(orden_trabajo_id: i32) -> Result<Option<SalidaEquipo>, String> {
    let pool = get_db_pool_safe()?;
    
    let salida = sqlx::query_as::<_, SalidaEquipo>(
        "SELECT s.salida_id, s.orden_trabajo_id, s.motivo_salida, s.fecha_salida,
                s.usuario_id, s.observaciones, s.created_at,
                ot.orden_codigo, 
                CONCAT(e.equipo_marca, ' ', e.equipo_modelo) as equipo_nombre, 
                c.cliente_nombre, u.usuario_nombre
         FROM SALIDA_EQUIPO s
         JOIN ORDEN_TRABAJO ot ON s.orden_trabajo_id = ot.orden_id
         JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         JOIN CLIENTE c ON e.cliente_id = c.cliente_id
         LEFT JOIN USUARIO u ON s.usuario_id = u.usuario_id
         WHERE s.orden_trabajo_id = ?"
    )
    .bind(orden_trabajo_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo salida: {}", e))?;
    
    Ok(salida)
}