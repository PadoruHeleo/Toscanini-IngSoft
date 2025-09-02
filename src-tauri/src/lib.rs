pub mod commands;
pub mod database;
pub mod utils;
pub mod email;
pub mod config;

use database::init_database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Cargar variables de entorno desde .env
    dotenv::dotenv().ok();
    
    // Inicializar runtime de Tokio para operaciones async
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
      // Inicializar la base de datos
    rt.block_on(async {
        if let Err(e) = init_database().await {
            eprintln!("Warning: Failed to initialize database: {}", e);
            // No terminar la aplicación, solo mostrar advertencia
        }
    });tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())        
        .invoke_handler(tauri::generate_handler![
            commands::users::get_usuarios,
            commands::users::get_usuario_by_id,
            commands::users::get_usuario_by_rut,
            commands::users::create_usuario,
            commands::users::update_usuario,
            commands::users::delete_usuario,
            commands::users::authenticate_usuario,
            commands::users::validate_session,            
            commands::users::logout_user,
            commands::users::cleanup_expired_sessions,
            commands::users::create_admin_user,
            commands::users::request_password_reset,            
            commands::users::verify_reset_code,
            commands::users::reset_password_with_code,
            commands::users::cleanup_expired_reset_codes,
            commands::users::change_user_password,
            commands::users::change_user_email,
            commands::logs::create_audit_log,
            commands::logs::get_audit_log_by_id,
            commands::logs::get_audit_logs,
            commands::logs::get_audit_logs_by_user,
            commands::logs::get_audit_logs_by_entity,
            commands::logs::cleanup_old_audit_logs,
            commands::logs::count_audit_logs,
            commands::logs::get_audit_stats,            
            commands::clientes::get_clientes,
            commands::clientes::get_cliente_by_id,
            commands::clientes::get_cliente_by_rut,
            commands::clientes::get_clientes_by_created_by,
            commands::clientes::search_clientes,
            commands::clientes::create_cliente,
            commands::clientes::update_cliente,
            commands::clientes::delete_cliente,
            commands::clientes::count_clientes,
            commands::clientes::get_clientes_with_pagination,
            commands::clientes::get_clientes_filtrados,
            commands::clientes::get_correos_clientes,
            commands::equipos::get_equipos,
            commands::equipos::get_equipo_by_id,
            commands::equipos::get_equipo_by_numero_serie,
            commands::equipos::get_equipos_by_cliente,
            commands::equipos::get_equipos_by_tipo,
            commands::equipos::get_equipos_by_created_by,
            commands::equipos::search_equipos,
            commands::equipos::create_equipo,
            commands::equipos::update_equipo,
            commands::equipos::delete_equipo,
            commands::equipos::count_equipos,
            commands::equipos::get_equipos_with_pagination,            
            commands::equipos::get_equipos_stats_by_tipo,
            commands::equipos::get_equipos_by_price_range,            
            commands::equipos::get_equipos_with_cliente,
            commands::equipos::get_equipos_marcas,
            commands::equipos::get_equipos_modelos_by_marca,
            commands::equipos::get_equipos_ubicaciones,
            commands::equipos::transfer_equipo_to_cliente,            commands::ordenes_trabajo::get_ordenes_trabajo,
            commands::ordenes_trabajo::get_ordenes_trabajo_filtradas,
            commands::ordenes_trabajo::get_modelos_disponibles,
            commands::ordenes_trabajo::get_marcas_disponibles,
            commands::ordenes_trabajo::get_clientes_disponibles,
            commands::ordenes_trabajo::get_orden_trabajo_by_id,
            commands::ordenes_trabajo::get_orden_trabajo_by_codigo,
            commands::ordenes_trabajo::get_ordenes_trabajo_by_equipo,
            commands::ordenes_trabajo::get_ordenes_trabajo_by_estado,
            commands::ordenes_trabajo::get_ordenes_trabajo_by_prioridad,
            commands::ordenes_trabajo::get_ordenes_trabajo_by_usuario,
            commands::ordenes_trabajo::get_ordenes_trabajo_detalladas,
            commands::ordenes_trabajo::get_orden_trabajo_detallada_by_id,
            commands::ordenes_trabajo::create_orden_trabajo,
            commands::ordenes_trabajo::update_orden_trabajo,
            commands::ordenes_trabajo::cambiar_estado_orden_trabajo,
            commands::ordenes_trabajo::asignar_cotizacion_orden_trabajo,
            commands::ordenes_trabajo::asignar_informe_orden_trabajo,            
            commands::ordenes_trabajo::delete_orden_trabajo,
            commands::ordenes_trabajo::get_ordenes_trabajo_stats,
            commands::ordenes_trabajo::search_ordenes_trabajo,
            commands::ordenes_trabajo::send_orden_trabajo_notification,
            commands::cotizacion::get_cotizaciones,
            commands::cotizacion::get_cotizacion_by_id,
            commands::cotizacion::get_cotizacion_by_codigo,
            commands::cotizacion::get_cotizaciones_detalladas,
            commands::cotizacion::create_cotizacion,
            commands::cotizacion::update_cotizacion,
            commands::cotizacion::delete_cotizacion,
            commands::cotizacion::search_cotizaciones,
            commands::cotizacion::count_cotizaciones,
            commands::cotizacion::get_cotizaciones_with_pagination,
            commands::cotizacion::get_piezas,
            commands::cotizacion::get_pieza_by_id,
            commands::cotizacion::create_pieza,
            commands::cotizacion::update_pieza,
            commands::cotizacion::delete_pieza,              
            commands::cotizacion::get_piezas_cotizacion,           
            commands::informe::get_informes,
            commands::informe::get_informe_by_id,
            commands::informe::get_informe_by_codigo,
            commands::informe::get_informes_detallados,
            commands::informe::create_informe,
            commands::informe::update_informe,
            commands::informe::delete_informe,
            commands::informe::search_informes,
            commands::informe::count_informes,
            commands::informe::get_informes_with_pagination,
            commands::informe::get_piezas_informe,
            commands::informe::send_informe_to_client,
            commands::database::get_database_status,
            commands::database::check_database_connection,
            commands::database::retry_database_connection,
            commands::config::check_database_config,
            commands::config::save_database_config,
            commands::config::load_database_config,
            commands::config::test_database_connection,
            commands::config::delete_database_config,
            commands::config::get_default_database_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
