use crate::logger;

/// Obtiene la ruta del archivo de log actual
#[tauri::command]
pub fn get_log_file_path() -> Option<String> {
    logger::get_log_file_path()
        .and_then(|path| path.to_str().map(|s| s.to_string()))
}

