use crate::config::DatabaseConfig;
use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::task;
use std::fs;

// CARGAR LA LLAVE EN MEMORIA AL COMPILAR
const EMBEDDED_SSH_KEY: &str = include_str!("../../certs/clave_gcp");

pub struct SshTunnel {
    session: Arc<Mutex<Session>>,
    local_port: u16,
    keep_alive_handle: Option<task::JoinHandle<()>>,
}

impl SshTunnel {
    pub async fn create(config: &DatabaseConfig) -> Result<Self, Box<dyn std::error::Error>> {
        println!("[SSH DEBUG] Iniciando creación de túnel...");

        let ssh_host = config.ssh_host.as_ref().ok_or("SSH_HOST no configurado")?;
        let ssh_port = config.ssh_port.unwrap_or(22);
        let ssh_user = config.ssh_user.as_ref().ok_or("SSH_USER no configurado")?;
        
        let remote_host = config.ssh_remote_host.as_deref().unwrap_or("localhost");
        let remote_port = config.ssh_remote_port.unwrap_or(3306);
        
        println!("[SSH DEBUG] Conectando TCP al Bastion...");
        let tcp = TcpStream::connect(format!("{}:{}", ssh_host, ssh_port))?;
        
        // Configurar timeouts básicos
        tcp.set_read_timeout(Some(Duration::from_secs(10)))?;
        tcp.set_write_timeout(Some(Duration::from_secs(10)))?;
        
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        
        // Autenticación
        if let Some(ref password) = config.ssh_password {
            session.userauth_password(ssh_user, password)?;
        } else {
            // Usar llave embebida (memoria)
            // NOTA: session.userauth_pubkey_memory es más limpio que crear archivos temporales
            // Si tu versión de ssh2 lo soporta, úsalo. Si no, mantén tu lógica de archivo temporal.
            // Aquí uso tu lógica de archivo temporal que ya sabemos que funciona:
            
            let temp_dir = std::env::temp_dir();
            let temp_key_path = temp_dir.join(format!("ssh_key_{}.tmp", uuid::Uuid::new_v4()));
            fs::write(&temp_key_path, EMBEDDED_SSH_KEY)?;
            
            let auth_result = session.userauth_pubkey_file(ssh_user, None, &temp_key_path, None);
            let _ = fs::remove_file(&temp_key_path);
            
            auth_result?;
        }
        
        if !session.authenticated() {
            return Err("Autenticación SSH fallida".into());
        }
        
        println!("[SSH DEBUG] ¡Autenticación SSH exitosa!");
        
        // Puerto local
        let local_port = if let Some(port) = config.ssh_local_port {
            port
        } else {
            find_free_port().await?
        };
        
        println!("[SSH DEBUG] Tunnel listener: localhost:{}", local_port);
        
        let session_arc = Arc::new(Mutex::new(session));
        let session_clone = session_arc.clone();
        let remote_host = remote_host.to_string();
        
        let keep_alive_handle = task::spawn(async move {
            if let Err(e) = Self::run_tunnel(session_clone, local_port, remote_host, remote_port).await {
                eprintln!("[SSH DEBUG] Error en loop del túnel: {}", e);
            }
        });
        
        Ok(Self {
            session: session_arc,
            local_port,
            keep_alive_handle: Some(keep_alive_handle),
        })
    }
    
    async fn run_tunnel(
        session: Arc<Mutex<Session>>,
        local_port: u16,
        remote_host: String,
        remote_port: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", local_port)).await?;
        
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let session_clone = session.clone();
                    let r_host = remote_host.clone();
                    
                    task::spawn(async move {
                        if let Err(e) = Self::handle_connection(session_clone, stream, r_host, remote_port).await {
                            eprintln!("[SSH DEBUG] Error conexión: {}", e);
                        }
                    });
                }
                Err(e) => eprintln!("Error accept: {}", e),
            }
        }
    }
    
    async fn handle_connection(
        session: Arc<Mutex<Session>>,
        local_stream: tokio::net::TcpStream,
        remote_host: String,
        remote_port: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Convertir stream a estándar y configurar para "Polling" (Non-blocking)
        let local_stream_std = local_stream.into_std()?;
        local_stream_std.set_nonblocking(true)?; // Modo no bloqueante para poder alternar
        local_stream_std.set_nodelay(true)?;

        println!("[SSH] Solicitando canal hacia {}:{}", remote_host, remote_port);

        // 2. Abrir canal SSH
        // Bloqueamos la sesión solo lo justo para abrir el canal
        let mut channel = {
            let sess = session.lock().unwrap();
            sess.channel_direct_tcpip(&remote_host, remote_port, None)?
        };
        
        // Configurar el canal SSH también como no bloqueante
        // Esto es crucial para que channel.read no congele todo el programa
        {
            let sess = session.lock().unwrap();
            sess.set_blocking(false);
        }
        println!("[SSH] Canal abierto. Iniciando bucle de relevo único...");

        // 3. Bucle de Relevo (Single Thread Relay)
        // Manejamos ambos sentidos en el mismo hilo para evitar conflictos de la librería ssh2
        task::spawn_blocking(move || {
            let mut buf = [0u8; 8192];
            let mut local_stream = local_stream_std;
            
            loop {
                let mut did_work = false;

                // --- SENTIDO 1: Local (App) -> Remoto (MySQL) ---
                match local_stream.read(&mut buf) {
                    Ok(0) => break, // Cierre de conexión
                    Ok(n) => {
                        // Escribir al canal SSH
                        if let Err(e) = channel.write_all(&buf[..n]) {
                            eprintln!("[SSH] Error escritura remota: {}", e);
                            break;
                        }
                        let _ = channel.flush();
                        did_work = true;
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // No hay datos, continuamos
                    },
                    Err(e) => {
                        eprintln!("[SSH] Error lectura local: {}", e);
                        break;
                    }
                }

                // --- SENTIDO 2: Remoto (MySQL) -> Local (App) ---
                match channel.read(&mut buf) {
                    Ok(0) => break, // Cierre remoto
                    Ok(n) => {
                        // Escribir al socket local
                        if let Err(e) = local_stream.write_all(&buf[..n]) {
                            eprintln!("[SSH] Error escritura local: {}", e);
                            break;
                        }
                        let _ = local_stream.flush();
                        did_work = true;
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // No hay datos, continuamos
                    },
                    Err(e) => {
                         eprintln!("[SSH] Error lectura remota: {}", e);
                         break;
                    }
                }

                // Si no hubo actividad en ninguno de los dos lados, dormimos un micro-instante
                // para no quemar el 100% de la CPU
                if !did_work {
                    std::thread::sleep(Duration::from_millis(1));
                }
            }

            println!("[SSH] Conexión cerrada.");
        }).await?;
        
        Ok(())
    }

    pub fn local_port(&self) -> u16 { self.local_port }
    
    pub fn is_active(&self) -> bool {
        self.session.lock().map(|s| s.authenticated()).unwrap_or(false)
    }
    
    pub fn close(self) {
        if let Some(handle) = self.keep_alive_handle { handle.abort(); }
    }
}

async fn find_free_port() -> Result<u16, Box<dyn std::error::Error>> {
    use std::net::TcpListener;
    for port in 3307..3400 {
        if TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok() { return Ok(port); }
    }
    Err("No puertos libres".into())
}

pub fn is_ssh_configured(config: &DatabaseConfig) -> bool {
    config.ssh_host.is_some() && config.ssh_user.is_some()
}