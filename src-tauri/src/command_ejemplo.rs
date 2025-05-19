// Aqui habra un ejemplo de comando de rust fuera de lib.rs
#[tauri::command]
pub fn ej_command_externo(name: &str) -> String{
    format!("Mi destino es ser llamado por react {}, estoy fuera de lib.rs",name)
}
// Ahora hay que agregarte en lib.rs