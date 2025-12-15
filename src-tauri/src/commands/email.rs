use std::sync::RwLock;
use tauri::State;
use crate::config::AppConfig;
use crate::infrastructure::db::email as db_impl;
use crate::infrastructure::api::ordenes_trabajo as api_ot;
use crate::infrastructure::db::ordenes_trabajo as db_ot;
use crate::infrastructure::api::equipos as api_eq;
use crate::infrastructure::db::equipos as db_eq;
use crate::infrastructure::api::clientes as api_cl;
use crate::infrastructure::db::clientes as db_cl;
use crate::infrastructure::api::users as api_users;
use crate::infrastructure::db::users as db_users;
use crate::infrastructure::db::email::EmailService;

use crate::infrastructure::api::cotizacion as api_cot;
use crate::infrastructure::db::cotizacion as db_cot;
use crate::infrastructure::api::informe as api_inf;
use crate::infrastructure::db::informe as db_inf;

use crate::pdf::api_data;
use crate::pdf::db_data;
use crate::pdf::CotizacionPdfGenerator;
use crate::pdf::InformePdfGenerator;

#[tauri::command]
pub async fn test_email_send(state: State<'_, RwLock<AppConfig>>, to_email: String) -> Result<String, String> {
    let email_config = state.read().map_err(|_| "Error de lectura de configuración")?.email_config.clone();
    let config = email_config.ok_or("Configuración de email no encontrada. Verifique variables SMTP en .env")?;
    db_impl::send_test_email(&config, to_email).await
}

#[tauri::command]
pub async fn send_orden_trabajo_cliente(state: State<'_, RwLock<AppConfig>>, orden_id: i32, _sent_by: i32) -> Result<String, String> {
    println!("📧 [Command] send_orden_trabajo_cliente called for Orden ID: {}", orden_id);
    
    let (email_config, use_api) = {
        let config_guard = state.read().map_err(|_| "Error de lectura de configuración")?;
        (config_guard.email_config.clone(), config_guard.use_api)
    };

    let config = email_config.ok_or("Configuración de email no encontrada. Verifique variables SMTP en .env")?;

    // 1. Fetch Data
    let (orden, equipo, cliente) = if use_api {
        let orden = api_ot::get_orden_trabajo_by_id(orden_id).await?.ok_or("Orden no encontrada")?;
        let equipo_id = orden.equipo_id.ok_or("La orden no tiene equipo asociado")?;
        let equipo = api_eq::get_equipo_by_id(equipo_id).await?.ok_or("Equipo no encontrado")?;
        let cliente_id = equipo.cliente_id.ok_or("Equipo sin cliente asociado")?;
        let cliente = api_cl::get_cliente_by_id(cliente_id).await?.ok_or("Cliente no encontrado")?;
        (orden, equipo, cliente)
    } else {
        let orden = db_ot::get_orden_trabajo_by_id(orden_id).await?.ok_or("Orden no encontrada")?;
        let equipo_id = orden.equipo_id.ok_or("La orden no tiene equipo asociado")?;
        let equipo = db_eq::get_equipo_by_id(equipo_id).await?.ok_or("Equipo no encontrado")?;
        let cliente_id = equipo.cliente_id.ok_or("Equipo sin cliente asociado")?;
        let cliente = db_cl::get_cliente_by_id(cliente_id).await?.ok_or("Cliente no encontrado")?;
        (orden, equipo, cliente)
    };

    // 2. Validate Email
    let to_email = cliente.cliente_correo.as_ref().ok_or("El cliente no tiene email configurado")?;
    if to_email.trim().is_empty() { return Err("El cliente tiene un email vacío".to_string()); }

    // 3. Send Email
    let email_service = EmailService::new(&config).map_err(|e| format!("Error init EmailService: {}", e))?;
    
    email_service.send_orden_trabajo_cliente(
        to_email,
        cliente.cliente_nombre.as_deref().unwrap_or("Cliente"),
        &orden,
        &equipo
    ).await.map_err(|e| format!("Error enviando email: {}", e))?;

    // 4. Notify Staff
    let users_result = if use_api {
        api_users::get_usuarios().await
    } else {
        db_users::get_usuarios().await
    };

    if let Ok(users) = users_result {
        let staff_users: Vec<_> = users.into_iter()
            .filter(|u| {
                let role = u.usuario_rol.as_deref().unwrap_or("").to_lowercase();
                (role == "admin" || role == "tecnico") && u.is_active.unwrap_or(false)
            })
            .collect();

        for user in staff_users {
            if let Some(email) = &user.usuario_correo {
                if !email.trim().is_empty() {
                    let _ = email_service.send_orden_trabajo_staff_notification(
                        email,
                        user.usuario_nombre.as_deref().unwrap_or("Staff"),
                        &orden,
                        &equipo,
                        cliente.cliente_nombre.as_deref().unwrap_or("Cliente")
                    ).await;
                }
            }
        }
    }

    Ok("Emails enviados exitosamente".to_string())
}

#[tauri::command]
pub async fn send_cotizacion_email(state: State<'_, RwLock<AppConfig>>, cotizacion_id: i32, sent_by: i32) -> Result<String, String> {
    let (email_config, use_api) = {
        let config_guard = state.read().map_err(|_| "Error de lectura de configuración")?;
        (config_guard.email_config.clone(), config_guard.use_api)
    };

    let config = email_config.ok_or("Configuración de email no encontrada. Verifique variables SMTP en .env")?;

    // 1. Fetch Data
    let (cotizacion, orden_trabajo, equipo, cliente) = if use_api {
        let cotizacion = api_cot::get_cotizacion_by_id(cotizacion_id).await?.ok_or("Cotización no encontrada")?;
        let ordenes = api_ot::get_ordenes_trabajo().await?;
        let orden = ordenes.into_iter().find(|o| o.cotizacion_id == Some(cotizacion_id))
            .ok_or("La cotización no está asociada a ninguna orden de trabajo")?;
        let equipo_id = orden.equipo_id.ok_or("La orden no tiene equipo asociado")?;
        let equipo = api_eq::get_equipo_by_id(equipo_id).await?.ok_or("Equipo no encontrado")?;
        let cliente_id = equipo.cliente_id.ok_or("Equipo sin cliente asociado")?;
        let cliente = api_cl::get_cliente_by_id(cliente_id).await?.ok_or("Cliente no encontrado")?;
        (cotizacion, orden, equipo, cliente)
    } else {
        let cotizacion = db_cot::get_cotizacion_by_id(cotizacion_id).await?.ok_or("Cotización no encontrada")?;
        
        // Manual SQL
        use crate::database::get_db_pool_safe;
        let pool = get_db_pool_safe()?;
        let orden_id: Option<i32> = sqlx::query_scalar("SELECT orden_id FROM ORDEN_TRABAJO WHERE cotizacion_id = ? LIMIT 1")
            .bind(cotizacion_id)
            .fetch_optional(&*pool).await.map_err(|e| e.to_string())?;
            
        let orden_id = orden_id.ok_or("La cotización no está asociada a ninguna orden de trabajo")?;
        let orden = db_ot::get_orden_trabajo_by_id(orden_id).await?.ok_or("Orden no encontrada")?;
        let equipo_id = orden.equipo_id.ok_or("La orden no tiene equipo asociado")?;
        let equipo = db_eq::get_equipo_by_id(equipo_id).await?.ok_or("Equipo no encontrado")?;
        let cliente_id = equipo.cliente_id.ok_or("Equipo sin cliente asociado")?;
        let cliente = db_cl::get_cliente_by_id(cliente_id).await?.ok_or("Cliente no encontrado")?;
        (cotizacion, orden, equipo, cliente)
    };

    let to_email = cliente.cliente_correo.as_ref().ok_or("El cliente no tiene email configurado")?;
    if to_email.trim().is_empty() { return Err("Email cliente vacío".to_string()); }

    // 2. Generate PDF
    let pdf_bytes = if use_api {
        let data = api_data::get_cotizacion_pdf_data(cotizacion_id).await?;
        CotizacionPdfGenerator::new().generate_cotizacion_pdf(data).await?
    } else {
        let data = db_data::get_cotizacion_pdf_data(cotizacion_id).await?;
        CotizacionPdfGenerator::new().generate_cotizacion_pdf(data).await?
    };

    // 3. Send Email
    let email_service = EmailService::new(&config).map_err(|e| format!("{}", e))?;
    email_service.send_cotizacion_email_with_pdf(
        to_email, 
        cliente.cliente_nombre.as_deref().unwrap_or("Cliente"), 
        &cotizacion, 
        &orden_trabajo, 
        &equipo, 
        &pdf_bytes
    ).await.map_err(|e| format!("Error enviando email: {}", e))?;

    // 4. Update Status
    if use_api {
        let _ = api_cot::update_cotizacion(cotizacion_id, crate::models::cotizacion::UpdateCotizacionRequest {
             cotizacion_codigo: None, costo_revision: None, costo_reparacion: None, 
             costo_total: None, is_aprobada: None, is_borrador: Some(false), informe: None, piezas: None
        }, sent_by).await;
        
        if orden_trabajo.estado.as_deref() == Some("recibido") {
             let _ = api_ot::cambiar_estado_orden_trabajo(orden_trabajo.orden_id, "cotizacion_enviada".to_string(), sent_by).await;
        }
    } else {
        let _ = db_cot::update_cotizacion(cotizacion_id, crate::models::cotizacion::UpdateCotizacionRequest {
             cotizacion_codigo: None, costo_revision: None, costo_reparacion: None, 
             costo_total: None, is_aprobada: None, is_borrador: Some(false), informe: None, piezas: None
        }, sent_by).await;

         if orden_trabajo.estado.as_deref() == Some("recibido") {
             let _ = db_ot::cambiar_estado_orden_trabajo(orden_trabajo.orden_id, "cotizacion_enviada".to_string(), sent_by).await;
        }
    }

    Ok("Email enviado con éxito".to_string())
}

#[tauri::command]
pub async fn send_informe_email(state: State<'_, RwLock<AppConfig>>, orden_id: i32, _sent_by: i32) -> Result<String, String> {
    let (email_config, use_api) = {
        let config_guard = state.read().map_err(|_| "Error de lectura de configuración")?;
        (config_guard.email_config.clone(), config_guard.use_api)
    };

    let config = email_config.ok_or("Configuración de email no encontrada. Verifique variables SMTP en .env")?;

    // 1. Fetch Data
    let (informe, orden, equipo, cliente) = if use_api {
        let orden = api_ot::get_orden_trabajo_by_id(orden_id).await?.ok_or("Orden no encontrada")?;
        let informe_id = orden.informe_id.ok_or("La orden no tiene informe asociado")?;
        let informe = api_inf::get_informe_by_id(informe_id).await?.ok_or("Informe no encontrado")?;
        let equipo_id = orden.equipo_id.ok_or("La orden no tiene equipo asociado")?;
        let equipo = api_eq::get_equipo_by_id(equipo_id).await?.ok_or("Equipo no encontrado")?;
        let cliente_id = equipo.cliente_id.ok_or("Equipo sin cliente asociado")?;
        let cliente = api_cl::get_cliente_by_id(cliente_id).await?.ok_or("Cliente no encontrado")?;
        (informe, orden, equipo, cliente)
    } else {
        let orden = db_ot::get_orden_trabajo_by_id(orden_id).await?.ok_or("Orden no encontrada")?;
        let informe_id = orden.informe_id.ok_or("La orden no tiene informe asociado")?;
        let informe = db_inf::get_informe_by_id(informe_id).await?.ok_or("Informe no encontrado")?;
        let equipo_id = orden.equipo_id.ok_or("La orden no tiene equipo asociado")?;
        let equipo = db_eq::get_equipo_by_id(equipo_id).await?.ok_or("Equipo no encontrado")?;
        let cliente_id = equipo.cliente_id.ok_or("Equipo sin cliente asociado")?;
        let cliente = db_cl::get_cliente_by_id(cliente_id).await?.ok_or("Cliente no encontrado")?;
        (informe, orden, equipo, cliente)
    };

    let to_email = cliente.cliente_correo.as_ref().ok_or("El cliente no tiene email configurado")?;
    if to_email.trim().is_empty() { return Err("Email cliente vacío".to_string()); }

    // 2. Generate PDF
    let pdf_bytes = if use_api {
        let data = api_data::get_informe_pdf_data(informe.informe_id).await?;
        InformePdfGenerator::new().generate_informe_pdf(data).await?
    } else {
        let data = db_data::get_informe_pdf_data(informe.informe_id).await?;
        InformePdfGenerator::new().generate_informe_pdf(data).await?
    };

    // 3. Send Email
    let email_service = EmailService::new(&config).map_err(|e| format!("{}", e))?;
    email_service.send_informe_email_with_pdf(
        to_email, 
        cliente.cliente_nombre.as_deref().unwrap_or("Cliente"), 
        &informe, 
        &orden, 
        &equipo, 
        &pdf_bytes
    ).await.map_err(|e| format!("Error enviando email: {}", e))?;

    Ok("Email enviado con éxito".to_string())
}
