use tauri::State;
use crate::config::AppConfig;
use crate::models::cotizacion::{
    Cotizacion, Pieza, PiezaCotizacion, CotizacionDetallada, 
    CreateCotizacionRequest, UpdateCotizacionRequest, 
    CreatePiezaRequest, PiezaCotizacionRequest, UpdatePiezaRequest,
    InventarioEquipo, InventarioEquipoRequest, SalidaEquipo, RegistrarSalidaRequest
};
use crate::infrastructure::db::cotizacion as db_impl;
use crate::infrastructure::api::cotizacion as api_impl;

#[tauri::command]
pub async fn get_cotizaciones(state: State<'_, AppConfig>) -> Result<Vec<Cotizacion>, String> {
    if state.use_api { api_impl::get_cotizaciones().await } else { db_impl::get_cotizaciones().await }
}

#[tauri::command]
pub async fn get_cotizaciones_detalladas(state: State<'_, AppConfig>) -> Result<Vec<CotizacionDetallada>, String> {
    if state.use_api { api_impl::get_cotizaciones_detalladas().await } else { db_impl::get_cotizaciones_detalladas().await }
}

#[tauri::command]
pub async fn get_cotizacion_by_id(state: State<'_, AppConfig>, cotizacion_id: i32) -> Result<Option<Cotizacion>, String> {
    if state.use_api { api_impl::get_cotizacion_by_id(cotizacion_id).await } else { db_impl::get_cotizacion_by_id(cotizacion_id).await }
}

#[tauri::command]
pub async fn get_cotizacion_by_codigo(state: State<'_, AppConfig>, cotizacion_codigo: String) -> Result<Option<Cotizacion>, String> {
    if state.use_api { api_impl::get_cotizacion_by_codigo(cotizacion_codigo).await } else { db_impl::get_cotizacion_by_codigo(cotizacion_codigo).await }
}

#[tauri::command]
pub async fn create_cotizacion(state: State<'_, AppConfig>, request: CreateCotizacionRequest) -> Result<Cotizacion, String> {
    if state.use_api { api_impl::create_cotizacion(request).await } else { db_impl::create_cotizacion(request).await }
}

#[tauri::command]
pub async fn update_cotizacion(state: State<'_, AppConfig>, cotizacion_id: i32, request: UpdateCotizacionRequest, updated_by: i32) -> Result<Option<Cotizacion>, String> {
    if state.use_api { api_impl::update_cotizacion(cotizacion_id, request, updated_by).await } else { db_impl::update_cotizacion(cotizacion_id, request, updated_by).await }
}

#[tauri::command]
pub async fn delete_cotizacion(state: State<'_, AppConfig>, cotizacion_id: i32, deleted_by: i32) -> Result<bool, String> {
    if state.use_api { api_impl::delete_cotizacion(cotizacion_id, deleted_by).await } else { db_impl::delete_cotizacion(cotizacion_id, deleted_by).await }
}

#[tauri::command]
pub async fn get_piezas(state: State<'_, AppConfig>) -> Result<Vec<Pieza>, String> {
    if state.use_api { api_impl::get_piezas().await } else { db_impl::get_piezas().await }
}

#[tauri::command]
pub async fn get_pieza_by_id(state: State<'_, AppConfig>, pieza_id: i32) -> Result<Option<Pieza>, String> {
    if state.use_api { api_impl::get_pieza_by_id(pieza_id).await } else { db_impl::get_pieza_by_id(pieza_id).await }
}

#[tauri::command]
pub async fn create_pieza(state: State<'_, AppConfig>, request: CreatePiezaRequest) -> Result<Pieza, String> {
    if state.use_api { api_impl::create_pieza(request).await } else { db_impl::create_pieza(request).await }
}

#[tauri::command]
pub async fn update_pieza(state: State<'_, AppConfig>, pieza_id: i32, request: UpdatePiezaRequest) -> Result<Option<Pieza>, String> {
    if state.use_api { 
        api_impl::update_pieza(pieza_id, request).await 
    } else { 
        // Convertimos el resultado de DB (Pieza) a Option<Pieza>
        db_impl::update_pieza(pieza_id, request).await.map(Some) 
    }
}

#[tauri::command]
pub async fn delete_pieza(state: State<'_, AppConfig>, pieza_id: i32) -> Result<bool, String> {
    if state.use_api { api_impl::delete_pieza(pieza_id).await } else { db_impl::delete_pieza(pieza_id).await }
}

#[tauri::command]
pub async fn search_cotizaciones(state: State<'_, AppConfig>, search_term: String) -> Result<Vec<CotizacionDetallada>, String> {
    if state.use_api { api_impl::search_cotizaciones(search_term).await } else { db_impl::search_cotizaciones(search_term).await }
}

#[tauri::command]
pub async fn count_cotizaciones(state: State<'_, AppConfig>) -> Result<i64, String> {
    if state.use_api { api_impl::count_cotizaciones().await } else { db_impl::count_cotizaciones().await }
}

#[tauri::command]
pub async fn get_cotizaciones_with_pagination(state: State<'_, AppConfig>, offset: i64, limit: i64) -> Result<Vec<CotizacionDetallada>, String> {
    if state.use_api { api_impl::get_cotizaciones_with_pagination(offset, limit).await } else { db_impl::get_cotizaciones_with_pagination(offset, limit).await }
}

#[tauri::command]
pub async fn get_piezas_cotizacion(state: State<'_, AppConfig>, cotizacion_id: i32) -> Result<Vec<PiezaCotizacion>, String> {
    if state.use_api { api_impl::get_piezas_cotizacion(cotizacion_id).await } else { db_impl::get_piezas_cotizacion(cotizacion_id).await }
}

#[tauri::command]
pub async fn get_cotizaciones_by_cliente(state: State<'_, AppConfig>, cliente_id: i32) -> Result<Vec<Cotizacion>, String> {
    if state.use_api { api_impl::get_cotizaciones_by_cliente(cliente_id).await } else { db_impl::get_cotizaciones_by_cliente(cliente_id).await }
}

#[tauri::command]
pub async fn get_piezas_inventario(state: State<'_, AppConfig>) -> Result<Vec<Pieza>, String> {
    if state.use_api { api_impl::get_piezas_inventario().await } else { db_impl::get_piezas_inventario().await }
}

#[tauri::command]
pub async fn update_pieza_stock(state: State<'_, AppConfig>, pieza_id: i32, cantidad: i32, tipo: String) -> Result<bool, String> {
    if state.use_api { api_impl::update_pieza_stock(pieza_id, cantidad, tipo).await } else { db_impl::update_pieza_stock(pieza_id, cantidad, tipo).await }
}

#[tauri::command]
pub async fn registrar_salida_equipo_v2(state: State<'_, AppConfig>, request: RegistrarSalidaRequest) -> Result<bool, String> {
    if state.use_api { 
        api_impl::registrar_salida_equipo_v2(request).await 
    } else { 
        // Map to the existing function in db_impl which returns Result<SalidaEquipo, String>
        db_impl::registrar_salida_equipo(request).await.map(|_| true) 
    }
}

#[tauri::command]
pub async fn get_salidas_equipo(state: State<'_, AppConfig>) -> Result<Vec<SalidaEquipo>, String> {
    if state.use_api { api_impl::get_salidas_equipo().await } else { db_impl::get_salidas_equipo().await }
}

#[tauri::command]
pub async fn puede_registrar_salida_v2(state: State<'_, AppConfig>, orden_trabajo_id: i32) -> Result<(bool, String), String> {
    if state.use_api { api_impl::puede_registrar_salida_v2(orden_trabajo_id).await } else { db_impl::puede_registrar_salida_v2(orden_trabajo_id).await }
}

#[tauri::command]
pub async fn get_salida_by_orden(state: State<'_, AppConfig>, orden_trabajo_id: i32) -> Result<Option<SalidaEquipo>, String> {
    if state.use_api { api_impl::get_salida_by_orden(orden_trabajo_id).await } else { db_impl::get_salida_by_orden(orden_trabajo_id).await }
}

#[tauri::command]
pub async fn update_cotizacion_piezas(state: State<'_, AppConfig>, cotizacion_id: i32, piezas: Vec<PiezaCotizacionRequest>, updated_by: i32) -> Result<bool, String> {
    if state.use_api { api_impl::update_cotizacion_piezas(cotizacion_id, piezas, updated_by).await } else { db_impl::update_cotizacion_piezas(cotizacion_id, piezas, updated_by).await }
}

#[tauri::command]
pub async fn get_inventario_equipos(state: State<'_, AppConfig>) -> Result<Vec<InventarioEquipo>, String> {
    if state.use_api { api_impl::get_inventario_equipos().await } else { db_impl::get_inventario_equipos().await }
}

#[tauri::command]
pub async fn create_inventario_equipo(state: State<'_, AppConfig>, request: InventarioEquipoRequest) -> Result<bool, String> {
    if state.use_api { 
        api_impl::create_inventario_equipo(request).await.map(|_| true) 
    } else { 
        db_impl::create_inventario_equipo(request).await.map(|_| true) 
    }
}

#[tauri::command]
pub async fn update_inventario_equipo(state: State<'_, AppConfig>, equipo_id: i32, request: InventarioEquipoRequest) -> Result<bool, String> {
    if state.use_api { 
        api_impl::update_inventario_equipo(equipo_id, request).await.map(|_| true) 
    } else { 
        db_impl::update_inventario_equipo(equipo_id, request).await.map(|_| true) 
    }
}

#[tauri::command]
pub async fn delete_inventario_equipo(state: State<'_, AppConfig>, equipo_id: i32) -> Result<bool, String> {
    if state.use_api { api_impl::delete_inventario_equipo(equipo_id).await } else { db_impl::delete_inventario_equipo(equipo_id).await }
}