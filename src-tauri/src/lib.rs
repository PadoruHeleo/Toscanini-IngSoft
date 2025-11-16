pub mod commands;
pub mod database;
pub mod utils;
pub mod email;
pub mod config;
pub mod pdf;
pub mod logger;

use database::{init_database, start_auto_reconnect_task, start_periodic_connection_check};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Inicializar el sistema de logging PRIMERO (antes de cualquier otra operación)
    // Esto capturará todos los errores que ocurran después
    if let Err(e) = logger::init_logger() {
        // Si falla la inicialización del logger, intentar escribir a archivo de emergencia
        logger::write_to_log_file(&format!("ERROR CRÍTICO: Fallo al inicializar el sistema de logging: {}", e));
        eprintln!("Error al inicializar el sistema de logging: {}", e);
    }
    
    // Configurar el handler de panics para capturar errores críticos
    logger::setup_panic_hook();
    
    // Configurar handler de excepciones de Windows (SEH) para capturar errores como 0xc0000005
    #[cfg(windows)]
    {
        logger::setup_windows_exception_handler();
        log::info!("Handler de excepciones de Windows (SEH) configurado");
    }
    
    // Registrar información crítica ANTES de cualquier otra operación
    log::info!("=== Iniciando aplicación Toscanini ===");
    log::info!("Paso 1: Sistema de logging inicializado correctamente");
    
    // Cargar variables de entorno desde .env
    log::info!("Paso 2: Cargando variables de entorno...");
    dotenv::dotenv().ok();
    log::info!("Paso 2 completado: Variables de entorno cargadas");
    
    // Crear runtime de Tokio con mejor manejo de errores
    log::info!("Paso 3: Creando runtime de Tokio...");
    let rt = match tokio::runtime::Runtime::new() {
        Ok(runtime) => {
            log::info!("Runtime de Tokio creado exitosamente");
            runtime
        },
        Err(e) => {
            log::error!("Error crítico: Fallo al crear el runtime de Tokio: {}", e);
            log::error!("Esto puede deberse a incompatibilidad del sistema o limitaciones de recursos.");
            // Intentar crear un runtime más básico
            tokio::runtime::Runtime::new().unwrap_or_else(|e2| {
                log::error!("Error fatal: Fallo al crear el runtime de respaldo: {}", e2);
                logger::write_to_log_file(&format!("FATAL: No se pudo crear ningún runtime de Tokio. Error original: {}, Error de respaldo: {}", e, e2));
                panic!("Failed to create fallback Tokio runtime: {}", e2);
            })
        }
    };
    
    // Inicializar la base de datos con manejo robusto de errores
    log::info!("Paso 4: Inicializando base de datos...");
    rt.block_on(async {
        log::info!("Inicializando conexión a la base de datos...");
        // Inicializar base de datos con manejo de errores robusto
        if let Err(e) = init_database().await {
            log::warn!("Advertencia: Fallo al inicializar la base de datos: {}", e);
            log::warn!("La aplicación continuará, pero las funciones de base de datos pueden no estar disponibles.");
        } else {
            log::info!("Base de datos inicializada correctamente");
        }
        
        // Iniciar la tarea de reconexión automática (solo cuando no está conectada)
        // Estas funciones ya tienen manejo interno de errores
        start_auto_reconnect_task();
        
        // Iniciar verificación periódica cada 60 segundos (incluso cuando está conectada)
        start_periodic_connection_check(10);
    });
    
    // Mantener el runtime vivo usando el handle en lugar de moverlo
    // Esto evita problemas de acceso a memoria en sistemas de 32 bits
    log::info!("Paso 4.1: Creando hilo de mantenimiento del runtime...");
    let rt_handle = rt.handle().clone();
    log::info!("Paso 4.2: Handle del runtime clonado exitosamente");
    
    match std::thread::Builder::new()
        .name("tokio-runtime-keeper".to_string())
        .spawn(move || {
            log::info!("Hilo de mantenimiento del runtime iniciado");
            // Usar el handle del runtime en lugar de mover el runtime completo
            rt_handle.block_on(async {
                log::info!("Runtime keeper: Entrando en loop de mantenimiento");
                // Mantener el runtime corriendo indefinidamente
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
                }
            });
        }) {
        Ok(_) => {
            log::info!("Paso 4.3: Hilo de mantenimiento del runtime creado exitosamente");
        },
        Err(e) => {
            log::error!("Error crítico: Fallo al crear el hilo de mantenimiento del runtime: {}", e);
            logger::write_to_log_file(&format!("FATAL: No se pudo crear el hilo de mantenimiento del runtime: {}", e));
            panic!("Failed to spawn runtime keeper thread: {}", e);
        }
    }
    
    log::info!("Paso 5: Inicializando aplicación Tauri...");
    log::info!("Paso 5.1: Creando Builder de Tauri...");
    let builder = tauri::Builder::default();
    log::info!("Paso 5.2: Builder creado exitosamente");
    
    // Plugin opener - comentado temporalmente para Tauri 1.x
    // En Tauri 1.x, el plugin opener puede requerir configuración diferente
    // log::info!("Paso 5.3: Inicializando plugin opener...");
    // let builder = builder.plugin(tauri_plugin_opener::init());
    // log::info!("Paso 5.4: Plugin opener inicializado");
    
    log::info!("Paso 5.5: Configurando handlers de comandos...");
    let builder = builder.invoke_handler(tauri::generate_handler![
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
            commands::cotizacion::update_inventario_equipo_stock,
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
            commands::logger::get_log_file_path,
            email::send_orden_trabajo_cliente,
            email::send_cotizacion_email,
            email::send_informe_email,
            pdf::commands::generate_cotizacion_pdf_command,
            pdf::commands::generate_informe_pdf_command,
            pdf::commands::generate_orden_trabajo_pdf_command,
            commands::cotizacion::update_cotizacion_piezas,
        ]);
    log::info!("Paso 5.6: Handlers de comandos configurados");
    
    log::info!("Paso 5.7: Generando contexto de Tauri...");
    let context = tauri::generate_context!();
    log::info!("Paso 5.8: Contexto generado exitosamente");
    
    log::info!("Paso 5.9: Ejecutando aplicación Tauri...");
    builder.run(context)
        .unwrap_or_else(|e| {
            log::error!("Error fatal al ejecutar la aplicación Tauri: {}", e);
            logger::write_to_log_file(&format!("FATAL: Error al ejecutar la aplicación Tauri: {}", e));
            panic!("error while running tauri application: {}", e);
        });
    
    log::info!("=== Aplicación Toscanini finalizada ===");
}
