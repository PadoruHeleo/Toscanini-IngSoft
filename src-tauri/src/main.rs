// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Intentar escribir un log de emergencia ANTES de cualquier otra cosa
    // Esto capturará errores que ocurran antes de que se inicialice el logger principal
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let emergency_log = exe_dir.join("toscanini_startup.log");
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&emergency_log)
            {
                use std::io::Write;
                // Usar formato simple de fecha/hora sin dependencias externas
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let _ = writeln!(file, "[{}] INICIO: main() llamado", now);
                let _ = writeln!(file, "[{}] Ejecutable: {:?}", now, exe_path);
                let _ = writeln!(file, "[{}] Directorio: {:?}", now, exe_dir);
                let _ = writeln!(file, "[{}] Arquitectura: {}", now, std::env::consts::ARCH);
                let _ = file.flush();
            }
        }
    }
    
    tauri_app_lib::run()
}
