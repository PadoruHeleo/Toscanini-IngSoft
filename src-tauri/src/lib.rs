// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// Ejemplo de comando a invocar en el frontend, si los define aca NO se definen publicos con pub
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        // Dentro del generate handler agregas el nombre de las funciones que programes en este archivo
        // Ademas, agregas otros comandos de otros archivos .rs
        .invoke_handler(tauri::generate_handler![greet,
            command_ejemplo::ej_command_externo
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
