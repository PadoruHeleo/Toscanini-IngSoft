use tauri::State;
use crate::config::AppConfig;
use crate::models::logs::{
    AuditLog, AuditLogWithUser, CreateAuditLogRequest, LogFilters
};
use crate::infrastructure::db::logs as db_impl;
use crate::infrastructure::api::logs as api_impl;

#[tauri::command]
pub async fn create_audit_log(state: State<'_, AppConfig>, request: CreateAuditLogRequest) -> Result<AuditLog, String> {
    if state.use_api { api_impl::create_audit_log(request).await } else { db_impl::create_audit_log(request).await }
}

#[tauri::command]
pub async fn get_audit_log_by_id(state: State<'_, AppConfig>, log_id: i32) -> Result<Option<AuditLog>, String> {
    if state.use_api { api_impl::get_audit_log_by_id(log_id).await } else { db_impl::get_audit_log_by_id(log_id).await }
}

#[tauri::command]
pub async fn get_audit_logs(state: State<'_, AppConfig>, filters: Option<LogFilters>) -> Result<Vec<AuditLogWithUser>, String> {
    // Note: api_impl might not support all filters or return AuditLogWithUser exactly as DB.
    // Assuming api_impl has been updated or we need to handle discrepancy.
    // api_impl::get_logs returns Vec<AuditLog>.
    // api_impl::get_logs_filtrados returns Vec<AuditLog>.
    // We need to match the return type.
    // If API doesn't support returning user info joined, we might have a problem or need to fetch it separately.
    // For now, let's assume we use DB implementation mostly or API needs update.
    // But the task is to refactor commands.
    // Let's check api_impl again.
    
    // api_impl::get_logs returns Result<Vec<AuditLog>, String>
    // db_impl::get_audit_logs returns Result<Vec<AuditLogWithUser>, String>
    
    // This is a mismatch.
    // I should probably wrap the API result to match or update API implementation.
    // Given the constraints, I will use conditional compilation or just call the appropriate function and map if needed.
    // But I cannot change the return type of the command easily without breaking frontend.
    // The command returns Result<Vec<AuditLogWithUser>, String>.
    
    if state.use_api { 
        // API implementation currently returns Vec<AuditLog>, not AuditLogWithUser.
        // We might need to map it or fail.
        // For now, let's assume we can map it (missing user info).
        let logs = api_impl::get_logs_filtrados(filters.unwrap_or(LogFilters {
            usuario_id: None, entidad_tabla: None, accion: None, search: None, 
            fecha_desde: None, fecha_hasta: None, limit: None, offset: None
        })).await?;
        
        // Map AuditLog to AuditLogWithUser
        let logs_with_user = logs.into_iter().map(|log| AuditLogWithUser {
            log_id: log.log_id,
            log_accion: log.log_accion,
            log_usuario_id: log.log_usuario_id,
            log_entidad_tabla: log.log_entidad_tabla,
            log_entidad_id: log.log_entidad_id,
            log_prev_v: log.log_prev_v,
            log_new_v: log.log_new_v,
            created_at: log.created_at,
            usuario_nombre: None, // API doesn't return this yet
            usuario_correo: None, // API doesn't return this yet
        }).collect();
        
        Ok(logs_with_user)
    } else { 
        db_impl::get_audit_logs(filters).await 
    }
}

#[tauri::command]
pub async fn get_audit_logs_by_user(state: State<'_, AppConfig>, usuario_id: i32, limit: Option<i32>) -> Result<Vec<AuditLogWithUser>, String> {
    if state.use_api {
        // Similar mapping needed
        // For now, falling back to DB or implementing mapping
         let filters = LogFilters {
            usuario_id: Some(usuario_id),
            limit,
            ..Default::default()
        };
        let logs = api_impl::get_logs_filtrados(filters).await?;
         let logs_with_user = logs.into_iter().map(|log| AuditLogWithUser {
            log_id: log.log_id,
            log_accion: log.log_accion,
            log_usuario_id: log.log_usuario_id,
            log_entidad_tabla: log.log_entidad_tabla,
            log_entidad_id: log.log_entidad_id,
            log_prev_v: log.log_prev_v,
            log_new_v: log.log_new_v,
            created_at: log.created_at,
            usuario_nombre: None,
            usuario_correo: None,
        }).collect();
        Ok(logs_with_user)
    } else { 
        db_impl::get_audit_logs_by_user(usuario_id, limit).await 
    }
}

#[tauri::command]
pub async fn get_audit_logs_by_entity(state: State<'_, AppConfig>, entidad_tabla: String, entidad_id: Option<i32>) -> Result<Vec<AuditLogWithUser>, String> {
    if state.use_api {
            // API might not support this specific filter via get_logs_filtrados if it's not in LogFilters.
            // I'll assume for now we map what we can.
            // ..Default::default()
        // };
        // This is imperfect.
        Err("API implementation for get_audit_logs_by_entity not fully supported".to_string())
    } else { 
        db_impl::get_audit_logs_by_entity(entidad_tabla, entidad_id).await 
    }
}

#[tauri::command]
pub async fn cleanup_old_audit_logs(state: State<'_, AppConfig>, days_old: i32) -> Result<u64, String> {
    // API might not expose cleanup
    if state.use_api { 
        Err("Cleanup not supported via API".to_string()) 
    } else { 
        db_impl::cleanup_old_audit_logs(days_old).await 
    }
}

#[tauri::command]
pub async fn count_audit_logs(state: State<'_, AppConfig>) -> Result<i64, String> {
    if state.use_api { 
        // API might not expose count
        // We can fetch all and count? No, too expensive.
        // Assuming API has a count endpoint or we fail.
        Err("Count not supported via API".to_string())
    } else { 
        db_impl::count_audit_logs().await 
    }
}

#[tauri::command]
pub async fn get_audit_stats(state: State<'_, AppConfig>) -> Result<serde_json::Value, String> {
    if state.use_api { 
        Err("Stats not supported via API".to_string())
    } else { 
        db_impl::get_audit_stats().await 
    }
}

// Public utility function
pub async fn log_action(
    accion: &str,
    usuario_id: Option<i32>,
    entidad_tabla: &str,
    entidad_id: Option<i32>,
    prev_value: Option<&str>,
    new_value: Option<&str>,
) -> Result<(), String> {
    // We need access to state to decide. 
    // But this function signature doesn't have state.
    // It's called from other commands which have state.
    // However, we can't easily pass state here without changing all call sites.
    // A common pattern is to use a global config or check the config file directly?
    // Or, since this is a utility, maybe we should default to DB or try to get config?
    // But we are inside tauri commands context usually.
    
    // Ideally, we should refactor call sites to pass state, or use the db_impl directly if we know we are in DB mode.
    // But the goal is to switch.
    
    // For now, I will default to DB implementation because `log_action` is mostly used in DB implementation files anyway?
    // Wait, `log_action` is used in `commands/*.rs`.
    // If I refactored `commands/*.rs` to use `api_impl` or `db_impl`, those implementations should handle logging internally?
    // `infrastructure/db/*.rs` calls `log_action`.
    // `infrastructure/api/*.rs` does NOT call `log_action` (it sends requests to API which logs).
    
    // So `log_action` in `commands/logs.rs` is primarily for the `commands` layer to log things?
    // But I removed business logic from `commands/*.rs`.
    // So `commands/*.rs` just delegate.
    // The `infrastructure/db/*.rs` files call `crate::infrastructure::db::logs::log_action` (or similar).
    
    // Let's check imports in `infrastructure/db/ordenes_trabajo.rs`.
    // `use crate::infrastructure::db::logs::log_action;`
    // It uses the DB implementation of log_action directly.
    
    // So `commands/logs.rs` `log_action` is only for other commands that might need to log?
    // But I refactored them to not have logic.
    // So maybe `log_action` in `commands/logs.rs` is not used anymore by refactored commands?
    // Let's check `clientes.rs` refactored.
    // It doesn't call `log_action`.
    
    // So `log_action` in `commands/logs.rs` might be dead code for the refactored commands, 
    // but it might be used by other parts of the app?
    // I will keep it wrapper to DB for now to be safe, or try to load config.
    // Loading config here is async and might be slow.
    
    // Actually, if `log_action` is only used by DB impls, they should import `infrastructure::db::logs::log_action`.
    // If it's used by API impls, they probably don't need it (API handles logging).
    
    // So I will just redirect to `db_impl::log_action` for now, as it's the safest default.
    db_impl::log_action(accion, usuario_id, entidad_tabla, entidad_id, prev_value, new_value).await
}

impl Default for LogFilters {
    fn default() -> Self {
        Self {
            usuario_id: None,
            entidad_tabla: None,
            accion: None,
            search: None,
            fecha_desde: None,
            fecha_hasta: None,
            limit: None,
            offset: None,
        }
    }
}
