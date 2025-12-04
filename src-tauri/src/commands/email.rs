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

#[tauri::command]
pub async fn test_email_send(state: State<'_, AppConfig>, to_email: String) -> Result<String, String> {
    let config = state.email_config.as_ref().ok_or("Configuración de email no encontrada. Verifique variables SMTP en .env")?;
    db_impl::send_test_email(config, to_email).await
}

#[tauri::command]
pub async fn send_orden_trabajo_cliente(state: State<'_, AppConfig>, orden_id: i32, _sent_by: i32) -> Result<String, String> {
    println!("📧 [Command] send_orden_trabajo_cliente called for Orden ID: {}", orden_id);
    
    let config = state.email_config.as_ref().ok_or("Configuración de email no encontrada. Verifique variables SMTP en .env")?;
    println!("📧 [Command] Email config found. Host: {}", config.smtp_server);

    // 1. Fetch Data (Orden, Equipo, Cliente)
    println!("📧 [Command] Fetching data... Mode API: {}", state.use_api);
    let (orden, equipo, cliente) = if state.use_api {
        // Fetch via API
        let orden = api_ot::get_orden_trabajo_by_id(orden_id).await?.ok_or("Orden no encontrada")?;
        let equipo_id = orden.equipo_id.ok_or("La orden no tiene equipo asociado")?;
        let equipo = api_eq::get_equipo_by_id(equipo_id).await?.ok_or("Equipo no encontrado")?;
        let cliente_id = equipo.cliente_id.ok_or("Equipo sin cliente asociado")?;
        let cliente = api_cl::get_cliente_by_id(cliente_id).await?.ok_or("Cliente no encontrado")?;
        (orden, equipo, cliente)
    } else {
        // Fetch via DB
        let orden = db_ot::get_orden_trabajo_by_id(orden_id).await?.ok_or("Orden no encontrada")?;
        let equipo_id = orden.equipo_id.ok_or("La orden no tiene equipo asociado")?;
        let equipo = db_eq::get_equipo_by_id(equipo_id).await?.ok_or("Equipo no encontrado")?;
        let cliente_id = equipo.cliente_id.ok_or("Equipo sin cliente asociado")?;
        let cliente = db_cl::get_cliente_by_id(cliente_id).await?.ok_or("Cliente no encontrado")?;
        (orden, equipo, cliente)
    };
    println!("📧 [Command] Data fetched. Cliente: {:?}, Email: {:?}", cliente.cliente_nombre, cliente.cliente_correo);

    // 2. Validate Email
    let to_email = cliente.cliente_correo.as_ref().ok_or("El cliente no tiene email configurado")?;
    if to_email.trim().is_empty() {
        return Err("El cliente tiene un email vacío".to_string());
    }

    // 3. Send Email using SMTP (always local)
    println!("📧 [Command] Initializing EmailService...");
    let email_service = EmailService::new(config).map_err(|e| format!("Error init EmailService: {}", e))?;
    
    println!("📧 [Command] Sending email to client: {}", to_email);
    // Send to Client
    if let Err(e) = email_service.send_orden_trabajo_cliente(
        to_email,
        cliente.cliente_nombre.as_deref().unwrap_or("Cliente"),
        &orden,
        &equipo
    ).await {
        println!("❌ [Command] Error sending email to client: {}", e);
        return Err(format!("Error enviando email al cliente: {}", e));
    }
    println!("📧 [Command] Email sent to client successfully!");

    // 4. Notify Staff (Admin & Tecnico)
    println!("📧 [Command] Fetching staff users for notification...");
    let users_result = if state.use_api {
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

        println!("📧 [Command] Found {} staff users to notify", staff_users.len());

        for user in staff_users {
            if let Some(email) = &user.usuario_correo {
                if !email.trim().is_empty() {
                    let user_name = user.usuario_nombre.as_deref().unwrap_or("Staff");
                    println!("📧 [Command] Sending notification to staff: {} ({})", user_name, email);
                    
                    if let Err(e) = email_service.send_orden_trabajo_staff_notification(
                        email,
                        user_name,
                        &orden,
                        &equipo,
                        cliente.cliente_nombre.as_deref().unwrap_or("Cliente")
                    ).await {
                        println!("❌ [Command] Failed to notify staff {}: {}", email, e);
                    }
                }
            }
        }
    } else {
        println!("❌ [Command] Failed to fetch users for notification");
    }

    Ok("Emails enviados exitosamente".to_string())
}

#[tauri::command]
pub async fn send_cotizacion_email(state: State<'_, AppConfig>, cotizacion_id: i32, sent_by: i32) -> Result<String, String> {
    let config = state.email_config.as_ref().ok_or("Configuración de email no encontrada. Verifique variables SMTP en .env")?;
    // Por ahora, para cotizaciones y informes (que requieren PDF), seguimos usando db_impl
    // TODO: Refactorizar generación de PDF para soportar API mode
    if state.use_api {
        return Err("El envío de cotizaciones con PDF no está soportado en modo API todavía".to_string());
    }
    db_impl::send_cotizacion_email(config, cotizacion_id, sent_by).await
}

#[tauri::command]
pub async fn send_informe_email(state: State<'_, AppConfig>, orden_id: i32, sent_by: i32) -> Result<String, String> {
    let config = state.email_config.as_ref().ok_or("Configuración de email no encontrada. Verifique variables SMTP en .env")?;
    // Por ahora, para cotizaciones y informes (que requieren PDF), seguimos usando db_impl
    // TODO: Refactorizar generación de PDF para soportar API mode
    if state.use_api {
        return Err("El envío de informes con PDF no está soportado en modo API todavía".to_string());
    }
    db_impl::send_informe_email(config, orden_id, sent_by).await
}
