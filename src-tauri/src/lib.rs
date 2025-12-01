pub mod commands;
pub mod database;
pub mod utils;

pub mod config;
pub mod pdf;
pub mod ssh_tunnel;
pub mod infrastructure;
pub mod models;

use database::{init_database, start_auto_reconnect_task, start_periodic_connection_check};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // En modo release, cargar primero el .env embebido
    #[cfg(not(debug_assertions))]
    {
        config::parse_embedded_env();
    }
    
    // Cargar variables de entorno desde .env (tiene prioridad en debug, fallback en release)
    dotenv::dotenv().ok();
    
    // Cargar AppConfig para determinar el modo de operación
    let app_config = config::AppConfig::default();
    
    // Solo inicializar DB/SSH si NO estamos en modo API
    if !app_config.use_api {
        println!("🔌 Inicializando en modo DATABASE (local)...");
        
        // Crear runtime de Tokio que se mantenga vivo
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        
        // Inicializar la base de datos
        rt.block_on(async {
            if let Err(e) = init_database().await {
                eprintln!("Warning: Failed to initialize database: {}", e);
            }
            
            // Iniciar la tarea de reconexión automática (solo cuando no está conectada)
            start_auto_reconnect_task();
            
            // Iniciar verificación periódica cada 10 segundos
            start_periodic_connection_check(10);
        });
        
        // Mantener el runtime vivo usando spawn_blocking
        std::thread::spawn(move || {
            rt.block_on(async {
                // Mantener el runtime corriendo indefinidamente
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
                }
            });
        });
    } else {
        println!("🌐 Inicializando en modo API-ONLY (sin base de datos local)...");
        println!("   Las consultas se realizarán mediante llamadas a la API remota");
    }
    
    tauri::Builder::default()
        .manage(app_config)  // Hacer AppConfig disponible para todos los comandos
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
            commands::users::verify_phone,
            commands::users::request_password_reset,            
            commands::users::verify_reset_code,
            commands::users::reset_password_with_code,
            commands::users::cleanup_expired_reset_codes,
            commands::users::change_user_password,
            commands::users::change_user_email,
            commands::users::change_user_phone,
            commands::users::send_password_email,
            commands::users::verify_phone,
            commands::users::verify_email_in_use,
            commands::users::verify_rut_in_use,
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
            commands::clientes::reactivate_cliente,
            commands::clientes::count_clientes,
            commands::clientes::get_clientes_with_pagination,
            commands::clientes::get_clientes_filtrados,
            commands::clientes::get_correos_clientes,
            commands::clientes::get_ruts_clientes,
            commands::clientes::get_ciudades_clientes,
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
            commands::equipos::transfer_equipo_to_cliente,
            commands::equipos::registrar_salida_equipo,
            commands::equipos::puede_registrar_salida_equipo,
            commands::equipos::equipo_esta_en_sistema,
            commands::equipos::get_equipos_en_sistema,
            commands::equipos::get_equipos_fuera_sistema,
            commands::equipos::get_estadisticas_equipos_sistema,
            commands::equipos::get_equipos_filtrados,
            commands::equipos::get_equipos_con_estado,
            commands::equipos::get_clientes_con_equipos,
            commands::equipos::get_tipos_equipos,
            commands::equipos::get_estados_ordenes_trabajo,
            commands::equipos::get_estadisticas_equipos_por_estado,            
            commands::ordenes_trabajo::get_ordenes_trabajo,
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
            commands::ordenes_trabajo::get_ordenes_trabajo_by_cliente,
            commands::ordenes_trabajo::get_ordenes_trabajo_detalladas,
            commands::ordenes_trabajo::get_orden_trabajo_detallada_by_id,
            commands::ordenes_trabajo::create_orden_trabajo,
            commands::ordenes_trabajo::update_orden_trabajo,
            commands::ordenes_trabajo::remove_cotizacion_from_ordenes,
            commands::ordenes_trabajo::remove_informe_from_ordenes,
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
            commands::cotizacion::get_cotizaciones_by_cliente,
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
            commands::cotizacion::get_piezas_inventario,
            commands::cotizacion::update_pieza_stock,
            commands::cotizacion::get_piezas_cotizacion,
            commands::cotizacion::get_inventario_equipos,
            commands::cotizacion::create_inventario_equipo,
            commands::cotizacion::update_inventario_equipo,
            commands::cotizacion::delete_inventario_equipo,

            // Comandos para salidas de equipos (NUEVA FUNCIONALIDAD)
            commands::cotizacion::registrar_salida_equipo_v2,
            commands::cotizacion::get_salidas_equipo,
            commands::cotizacion::puede_registrar_salida_v2,
            commands::cotizacion::get_salida_by_orden,           
            commands::informe::get_informes,
            commands::informe::get_informe_by_id,
            commands::informe::get_informe_by_codigo,
            commands::informe::get_informes_by_cliente,
            commands::informe::get_informes_detallados,
            commands::informe::create_informe,
            commands::informe::update_informe,
            commands::informe::delete_informe,
            commands::informe::search_informes,
            commands::informe::count_informes,
            commands::informe::get_informes_with_pagination,
            commands::informe::get_piezas_informe,
            commands::informe::send_informe_to_client,
            commands::terminos_condiciones::get_terminos_condiciones,
            commands::terminos_condiciones::get_terminos_condiciones_activos,
            commands::terminos_condiciones::get_terminos_condiciones_by_tipo,
            commands::terminos_condiciones::get_terminos_condiciones_default,
            commands::terminos_condiciones::get_termino_condicion_by_id,
            commands::terminos_condiciones::create_termino_condicion,
            commands::terminos_condiciones::update_termino_condicion,
            commands::terminos_condiciones::delete_termino_condicion,
            commands::terminos_condiciones::reactivate_termino_condicion,
            commands::terminos_condiciones::toggle_termino_default,
            commands::terminos_condiciones::get_terminos_by_informe,
            commands::terminos_condiciones::get_terminos_by_cotizacion,
            commands::terminos_condiciones::apply_terminos_to_informe,
            commands::terminos_condiciones::apply_terminos_to_cotizacion,
            commands::terminos_condiciones::apply_default_terminos_to_informe,
            commands::terminos_condiciones::apply_default_terminos_to_cotizacion,
            commands::informe::rechazar_informe_borrador,
            commands::database::get_database_status,
            commands::database::check_database_connection,
            commands::database::retry_database_connection,
            commands::database::force_run_migrations,
            commands::database::insert_test_data,
            commands::database::check_equipo_ids,
            commands::config::check_database_config,
            commands::config::save_database_config,
            commands::config::load_database_config,
            commands::config::test_database_connection,
            commands::config::delete_database_config,
            commands::config::get_default_database_config,
            commands::email::send_orden_trabajo_cliente,
            commands::email::send_cotizacion_email,
            commands::email::send_informe_email,
            commands::email::test_email_send,
            pdf::commands::generate_cotizacion_pdf_command,
            pdf::commands::generate_informe_pdf_command,
            pdf::commands::generate_orden_trabajo_pdf_command,
            commands::cotizacion::update_cotizacion_piezas,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
