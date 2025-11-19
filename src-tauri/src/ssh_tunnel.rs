use crate::config::BastionConfig;
use std::process::{Command, Child};
use std::time::Duration;
use tokio::time::sleep;
use std::sync::{Arc, Mutex, OnceLock};

pub struct SSHTunnel {
    process: Option<Child>,
    config: BastionConfig,
}

impl SSHTunnel {
    pub fn new(config: BastionConfig) -> Self {
        Self {
            process: None,
            config,
        }
    }
    
    /// Inicia el túnel SSH usando el comando ssh del sistema
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Verificar si ya está ejecutándose
        if self.is_running() {
            return Ok(());
        }
        
        println!("Iniciando túnel SSH a {}:{}...", self.config.host, self.config.port);
        
        // Construir el comando SSH
        let mut cmd = Command::new("ssh");
        
        // Agregar argumentos para el túnel
        cmd.args([
            "-N", // No ejecutar comando remoto
            "-L", // Port forwarding local
            &format!("{}:{}:{}", self.config.local_port, "127.0.0.1", 3306), // localhost:3307:127.0.0.1:3306
            "-o", "StrictHostKeyChecking=no", // No verificar host key (para desarrollo)
            "-o", "UserKnownHostsFile=/dev/null", // No guardar host keys
            "-o", "LogLevel=ERROR", // Reducir verbosidad
        ]);
        
        // Agregar clave privada si está especificada
        if let Some(key_path) = &self.config.private_key_path {
            cmd.args(["-i", key_path]);
        }
        
        // Agregar destino: usuario@host -p puerto
        cmd.args([
            "-p", &self.config.port.to_string(),
            &format!("{}@{}", self.config.username, self.config.host)
        ]);
        
        // Iniciar el proceso
        match cmd.spawn() {
            Ok(child) => {
                self.process = Some(child);
                println!("Túnel SSH iniciado en puerto local {}", self.config.local_port);
                
                // Esperar un momento para que se establezca la conexión
                sleep(Duration::from_secs(3)).await;
                
                // Verificar que el proceso sigue ejecutándose
                if !self.is_running() {
                    return Err("El túnel SSH falló al iniciarse".into());
                }
                
                Ok(())
            }
            Err(e) => {
                Err(format!("Error iniciando túnel SSH: {}. Asegúrate de que ssh esté instalado y disponible en PATH", e).into())
            }
        }
    }
    
    /// Verifica si el túnel está ejecutándose
    pub fn is_running(&mut self) -> bool {
        if let Some(ref mut process) = self.process {
            match process.try_wait() {
                Ok(Some(_)) => {
                    // El proceso terminó
                    self.process = None;
                    false
                }
                Ok(None) => {
                    // El proceso sigue ejecutándose
                    true
                }
                Err(_) => {
                    // Error verificando el estado
                    self.process = None;
                    false
                }
            }
        } else {
            false
        }
    }
    
    /// Detiene el túnel SSH
    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(mut process) = self.process.take() {
            println!("Deteniendo túnel SSH...");
            process.kill()?;
            process.wait()?;
            println!("Túnel SSH detenido");
        }
        Ok(())
    }
}

impl Drop for SSHTunnel {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

// Manager global simplificado para el túnel SSH
static SSH_TUNNEL: OnceLock<Arc<Mutex<Option<SSHTunnel>>>> = OnceLock::new();

fn get_ssh_tunnel() -> &'static Arc<Mutex<Option<SSHTunnel>>> {
    SSH_TUNNEL.get_or_init(|| Arc::new(Mutex::new(None)))
}

/// Inicia el túnel SSH si es necesario - versión simplificada sin async dentro del lock
pub async fn ensure_ssh_tunnel(bastion_config: &BastionConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Verificar primero si necesitamos crear un túnel
    let needs_new_tunnel = {
        let tunnel_guard = get_ssh_tunnel().lock().unwrap();
        tunnel_guard.is_none()
    };
    
    if needs_new_tunnel {
        // Crear túnel fuera del lock
        let mut new_tunnel = SSHTunnel::new(bastion_config.clone());
        new_tunnel.start().await?;
        
        // Guardar el túnel
        let mut tunnel_guard = get_ssh_tunnel().lock().unwrap();
        *tunnel_guard = Some(new_tunnel);
        
        println!("Túnel SSH establecido exitosamente");
    } else {
        // Verificar si el túnel existente está activo
        let needs_restart = {
            let mut tunnel_guard = get_ssh_tunnel().lock().unwrap();
            if let Some(tunnel) = tunnel_guard.as_mut() {
                if !tunnel.is_running() {
                    println!("Túnel SSH no está activo, necesita reiniciarse");
                    tunnel.stop().ok(); // Ignorar errores
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }; // Lock se libera aquí
        
        if needs_restart {
            // Crear un nuevo túnel fuera del lock
            let mut new_tunnel = SSHTunnel::new(bastion_config.clone());
            new_tunnel.start().await?;
            
            // Reemplazar el túnel
            let mut tunnel_guard = get_ssh_tunnel().lock().unwrap();
            *tunnel_guard = Some(new_tunnel);
            
            println!("Túnel SSH reiniciado exitosamente");
        }
    }
    
    Ok(())
}

/// Detiene el túnel SSH si está activo
pub fn stop_ssh_tunnel() -> Result<(), Box<dyn std::error::Error>> {
    let mut tunnel_guard = get_ssh_tunnel().lock().unwrap();
    if let Some(tunnel) = tunnel_guard.as_mut() {
        tunnel.stop()?;
    }
    *tunnel_guard = None;
    Ok(())
}

/// Verifica si el túnel SSH está activo
pub fn is_ssh_tunnel_active() -> bool {
    if let Ok(mut tunnel_guard) = get_ssh_tunnel().try_lock() {
        if let Some(tunnel) = tunnel_guard.as_mut() {
            return tunnel.is_running();
        }
    }
    false
}