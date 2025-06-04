use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_unchecked;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub log_id: i32,
    pub log_accion: Option<String>,
    pub log_usuario_id: Option<i32>,
    pub log_entidad_tabla: Option<String>,
    pub log_entidad_id: Option<i32>,
    pub log_prev_v: Option<String>,
    pub log_new_v: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AuditLogWithUser {
    pub log_id: i32,
    pub log_accion: Option<String>,
    pub log_usuario_id: Option<i32>,
    pub log_entidad_tabla: Option<String>,
    pub log_entidad_id: Option<i32>,
    pub log_prev_v: Option<String>,
    pub log_new_v: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub usuario_nombre: Option<String>,
    pub usuario_correo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAuditLogRequest {
    pub log_accion: String,
    pub log_usuario_id: Option<i32>,
    pub log_entidad_tabla: String,
    pub log_entidad_id: Option<i32>,
    pub log_prev_v: Option<String>,
    pub log_new_v: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LogFilters {
    pub usuario_id: Option<i32>,
    pub entidad_tabla: Option<String>,
    pub accion: Option<String>,
    pub fecha_desde: Option<String>,
    pub fecha_hasta: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Crear un nuevo registro de auditoría
#[tauri::command]
pub async fn create_audit_log(request: CreateAuditLogRequest) -> Result<AuditLog, String> {
    let pool = get_db_pool_unchecked();
    
    let result = sqlx::query(
        "INSERT INTO AUDIT_LOG (log_accion, log_usuario_id, log_entidad_tabla, log_entidad_id, log_prev_v, log_new_v) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&request.log_accion)
    .bind(&request.log_usuario_id)
    .bind(&request.log_entidad_tabla)
    .bind(&request.log_entidad_id)
    .bind(&request.log_prev_v)
    .bind(&request.log_new_v)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let log_id = result.last_insert_id() as i32;
    
    // Obtener el log recién creado
    get_audit_log_by_id(log_id)
        .await?
        .ok_or_else(|| "Failed to retrieve created audit log".to_string())
}

/// Obtener un registro de auditoría por ID
#[tauri::command]
pub async fn get_audit_log_by_id(log_id: i32) -> Result<Option<AuditLog>, String> {
    let pool = get_db_pool_unchecked();
    
    let log = sqlx::query_as::<_, AuditLog>(
        "SELECT log_id, log_accion, log_usuario_id, log_entidad_tabla, log_entidad_id, log_prev_v, log_new_v, created_at 
         FROM AUDIT_LOG 
         WHERE log_id = ?"
    )
    .bind(log_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(log)
}

/// Obtener todos los registros de auditoría con filtros opcionales
#[tauri::command]
pub async fn get_audit_logs(filters: Option<LogFilters>) -> Result<Vec<AuditLogWithUser>, String> {
    let pool = get_db_pool_unchecked();
    
    let mut query = String::from(
        "SELECT a.log_id, a.log_accion, a.log_usuario_id, a.log_entidad_tabla, a.log_entidad_id, 
                a.log_prev_v, a.log_new_v, a.created_at, u.usuario_nombre, u.usuario_correo
         FROM AUDIT_LOG a
         LEFT JOIN USUARIO u ON a.log_usuario_id = u.usuario_id"
    );
    
    let mut conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();
    
    if let Some(filters) = filters {
        if let Some(usuario_id) = filters.usuario_id {
            conditions.push("a.log_usuario_id = ?");
            params.push(usuario_id.to_string());
        }
        
        if let Some(entidad_tabla) = filters.entidad_tabla {
            conditions.push("a.log_entidad_tabla = ?");
            params.push(entidad_tabla);
        }
        
        if let Some(accion) = filters.accion {
            conditions.push("a.log_accion LIKE ?");
            params.push(format!("%{}%", accion));
        }
        
        if let Some(fecha_desde) = filters.fecha_desde {
            conditions.push("a.created_at >= ?");
            params.push(fecha_desde);
        }
        
        if let Some(fecha_hasta) = filters.fecha_hasta {
            conditions.push("a.created_at <= ?");
            params.push(fecha_hasta);
        }
        
        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }
        
        query.push_str(" ORDER BY a.created_at DESC");
        
        if let Some(limit) = filters.limit {
            query.push_str(&format!(" LIMIT {}", limit));
            
            if let Some(offset) = filters.offset {
                query.push_str(&format!(" OFFSET {}", offset));
            }
        }
    } else {
        query.push_str(" ORDER BY a.created_at DESC LIMIT 100");
    }
    
    let mut sqlx_query = sqlx::query_as::<_, AuditLogWithUser>(&query);
    
    for param in params {
        sqlx_query = sqlx_query.bind(param);
    }
    
    let logs = sqlx_query
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(logs)
}

/// Obtener registros de auditoría por usuario
#[tauri::command]
pub async fn get_audit_logs_by_user(usuario_id: i32, limit: Option<i32>) -> Result<Vec<AuditLogWithUser>, String> {
    let pool = get_db_pool_unchecked();
    
    let limit_clause = limit.map(|l| format!(" LIMIT {}", l)).unwrap_or_else(|| " LIMIT 50".to_string());
    
    let query = format!(
        "SELECT a.log_id, a.log_accion, a.log_usuario_id, a.log_entidad_tabla, a.log_entidad_id, 
                a.log_prev_v, a.log_new_v, a.created_at, u.usuario_nombre, u.usuario_correo
         FROM AUDIT_LOG a
         LEFT JOIN USUARIO u ON a.log_usuario_id = u.usuario_id
         WHERE a.log_usuario_id = ?
         ORDER BY a.created_at DESC{}",
        limit_clause
    );
    
    let logs = sqlx::query_as::<_, AuditLogWithUser>(&query)
        .bind(usuario_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(logs)
}

/// Obtener registros de auditoría por entidad
#[tauri::command]
pub async fn get_audit_logs_by_entity(entidad_tabla: String, entidad_id: Option<i32>) -> Result<Vec<AuditLogWithUser>, String> {
    let pool = get_db_pool_unchecked();
    
    let mut query = String::from(
        "SELECT a.log_id, a.log_accion, a.log_usuario_id, a.log_entidad_tabla, a.log_entidad_id, 
                a.log_prev_v, a.log_new_v, a.created_at, u.usuario_nombre, u.usuario_correo
         FROM AUDIT_LOG a
         LEFT JOIN USUARIO u ON a.log_usuario_id = u.usuario_id
         WHERE a.log_entidad_tabla = ?"
    );
    
    let logs = if let Some(id) = entidad_id {
        query.push_str(" AND a.log_entidad_id = ? ORDER BY a.created_at DESC LIMIT 100");
        sqlx::query_as::<_, AuditLogWithUser>(&query)
            .bind(&entidad_tabla)
            .bind(id)
            .fetch_all(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?
    } else {
        query.push_str(" ORDER BY a.created_at DESC LIMIT 100");
        sqlx::query_as::<_, AuditLogWithUser>(&query)
            .bind(&entidad_tabla)
            .fetch_all(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?
    };
    
    Ok(logs)
}

/// Eliminar registros de auditoría antiguos (cleanup)
#[tauri::command]
pub async fn cleanup_old_audit_logs(days_old: i32) -> Result<u64, String> {
    let pool = get_db_pool_unchecked();
    
    let result = sqlx::query(
        "DELETE FROM AUDIT_LOG WHERE created_at < DATE_SUB(NOW(), INTERVAL ? DAY)"
    )
    .bind(days_old)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(result.rows_affected())
}

/// Contar total de registros de auditoría
#[tauri::command]
pub async fn count_audit_logs() -> Result<i64, String> {
    let pool = get_db_pool_unchecked();
    
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM AUDIT_LOG")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(count.0)
}

/// Función de utilidad para registrar acciones automáticamente
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
    
    create_audit_log(request).await?;
    Ok(())
}

/// Obtener estadísticas de actividad
#[tauri::command]
pub async fn get_audit_stats() -> Result<serde_json::Value, String> {
    let pool = get_db_pool_unchecked();
    
    // Contar acciones por tipo
    let actions_count: Vec<(String, i64)> = sqlx::query_as(
        "SELECT log_accion, COUNT(*) as count 
         FROM AUDIT_LOG 
         GROUP BY log_accion 
         ORDER BY count DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    // Contar actividad por usuario
    let user_activity: Vec<(String, String, i64)> = sqlx::query_as(
        "SELECT COALESCE(u.usuario_nombre, 'Sistema') as nombre, 
                COALESCE(u.usuario_correo, 'sistema@toscanini.com') as correo, 
                COUNT(*) as count
         FROM AUDIT_LOG a
         LEFT JOIN USUARIO u ON a.log_usuario_id = u.usuario_id
         GROUP BY a.log_usuario_id, u.usuario_nombre, u.usuario_correo
         ORDER BY count DESC
         LIMIT 10"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    // Actividad por tabla
    let table_activity: Vec<(String, i64)> = sqlx::query_as(
        "SELECT log_entidad_tabla, COUNT(*) as count 
         FROM AUDIT_LOG 
         GROUP BY log_entidad_tabla 
         ORDER BY count DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let stats = serde_json::json!({
        "actions_count": actions_count,
        "user_activity": user_activity,
        "table_activity": table_activity,
        "total_logs": count_audit_logs().await?
    });
    
    Ok(stats)
}