use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::database::get_db_pool_safe;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TipoAccesorio {
    pub tipo_id: i32,
    pub nombre: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrdenAccesorio {
    pub id: i32,
    pub orden_id: i32,
    pub tipo_accesorio_id: i32,
    pub estado: Option<String>,
    pub observaciones: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTipoAccesorioRequest {
    pub nombre: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrdenAccesorioRequest {
    pub tipo_accesorio_id: i32,
    pub estado: Option<String>,
    pub observaciones: Option<String>,
}

#[tauri::command]
pub async fn get_tipos_accesorios() -> Result<Vec<TipoAccesorio>, String> {
    let pool = get_db_pool_safe()?;
    let tipos = sqlx::query_as::<_, TipoAccesorio>(
        "SELECT tipo_id, nombre, created_at FROM tipos_accesorios ORDER BY nombre",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error cargando tipos de accesorios: {}", e))?;

    Ok(tipos)
}

#[tauri::command]
pub async fn create_tipo_accesorio(request: CreateTipoAccesorioRequest) -> Result<TipoAccesorio, String> {
    let pool = get_db_pool_safe()?;

    // Evitar duplicados por nombre
    let exists: Option<TipoAccesorio> = sqlx::query_as::<_, TipoAccesorio>(
        "SELECT tipo_id, nombre, created_at FROM tipos_accesorios WHERE nombre = ? LIMIT 1",
    )
    .bind(&request.nombre)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("DB error: {}", e))?;

    if let Some(t) = exists {
        return Ok(t);
    }

    let res = sqlx::query("INSERT INTO tipos_accesorios (nombre) VALUES (?)")
        .bind(&request.nombre)
        .execute(pool)
        .await
        .map_err(|e| format!("Error creando tipo de accesorio: {}", e))?;

    let tipo_id = res.last_insert_id() as i32;

    let tipo = sqlx::query_as::<_, TipoAccesorio>(
        "SELECT tipo_id, nombre, created_at FROM tipos_accesorios WHERE tipo_id = ?",
    )
    .bind(tipo_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error recuperando tipo creado: {}", e))?;

    Ok(tipo)
}

#[tauri::command]
pub async fn get_accesorios_orden(orden_id: i32) -> Result<Vec<OrdenAccesorio>, String> {
    let pool = get_db_pool_safe()?;
    let accesorios = sqlx::query_as::<_, OrdenAccesorio>(
        "SELECT id, orden_id, tipo_accesorio_id, estado, observaciones, created_at FROM orden_accesorios WHERE orden_id = ? ORDER BY id",
    )
    .bind(orden_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error cargando accesorios de orden: {}", e))?;

    Ok(accesorios)
}

#[tauri::command]
pub async fn create_accesorio_orden(orden_id: i32, request: CreateOrdenAccesorioRequest) -> Result<OrdenAccesorio, String> {
    let pool = get_db_pool_safe()?;
    let estado = request.estado.unwrap_or_else(|| "presente".to_string());

    let res = sqlx::query("INSERT INTO orden_accesorios (orden_id, tipo_accesorio_id, estado, observaciones) VALUES (?, ?, ?, ?)")
        .bind(orden_id)
        .bind(request.tipo_accesorio_id)
        .bind(estado)
        .bind(request.observaciones)
        .execute(pool)
        .await
        .map_err(|e| format!("Error creando accesorio en orden: {}", e))?;

    let id = res.last_insert_id() as i32;

    let accesorio = sqlx::query_as::<_, OrdenAccesorio>(
        "SELECT id, orden_id, tipo_accesorio_id, estado, observaciones, created_at FROM orden_accesorios WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error recuperando accesorio creado: {}", e))?;

    Ok(accesorio)
}

#[tauri::command]
pub async fn update_accesorios_orden(orden_id: i32, accesorios: Vec<CreateOrdenAccesorioRequest>) -> Result<(), String> {
    let pool = get_db_pool_safe()?;

    // Eliminar existentes
    sqlx::query("DELETE FROM orden_accesorios WHERE orden_id = ?")
        .bind(orden_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Error eliminando accesorios anteriores: {}", e))?;

    for acc in accesorios {
        let estado = acc.estado.unwrap_or_else(|| "presente".to_string());
        sqlx::query("INSERT INTO orden_accesorios (orden_id, tipo_accesorio_id, estado, observaciones) VALUES (?, ?, ?, ?)")
            .bind(orden_id)
            .bind(acc.tipo_accesorio_id)
            .bind(estado)
            .bind(acc.observaciones)
            .execute(pool)
            .await
            .map_err(|e| format!("Error insertando accesorio: {}", e))?;
    }

    Ok(())
}
