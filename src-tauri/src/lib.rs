pub mod commands;
pub mod database;
pub mod utils;
pub mod email;

use database::init_database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Inicializar runtime de Tokio para operaciones async
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    
    // Inicializar la base de datos
    rt.block_on(async {
        if let Err(e) = init_database().await {
            eprintln!("Failed to initialize database: {}", e);
            std::process::exit(1);
        }
    });    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())        .invoke_handler(tauri::generate_handler![
            commands::users::get_usuarios,
            commands::users::get_usuario_by_id,
            commands::users::get_usuario_by_rut,
            commands::users::create_usuario,
            commands::users::update_usuario,
            commands::users::delete_usuario,
            commands::users::authenticate_usuario,
            commands::users::validate_session,            commands::users::logout_user,
            commands::users::cleanup_expired_sessions,
            commands::users::create_admin_user,
            commands::users::request_password_reset,
            commands::users::verify_reset_code,
            commands::users::reset_password_with_code,
            commands::users::cleanup_expired_reset_codes,
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
            commands::clientes::get_clientes_with_pagination
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
