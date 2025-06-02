pub mod commands;
pub mod database;
pub mod utils;

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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::users::get_usuarios,
            commands::users::get_usuario_by_id,
            commands::users::get_usuario_by_rut,
            commands::users::create_usuario,
            commands::users::update_usuario,
            commands::users::delete_usuario,
            commands::users::authenticate_usuario
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
