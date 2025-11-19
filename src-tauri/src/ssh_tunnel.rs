use crate::config::DatabaseConfig;
use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::task;

/// Estructura que maneja el túnel SSH
pub struct SshTunnel {
    session: Arc<Mutex<Session>>,
    local_port: u16,
    keep_alive_handle: Option<task::JoinHandle<()>>,
}

impl SshTunnel {
    /// Crea un nuevo túnel SSH basado en la configuración
    pub async fn create(config: &DatabaseConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let ssh_host = config.ssh_host.as_ref()
            .ok_or("SSH_HOST no está configurado")?;
        let ssh_port = config.ssh_port.unwrap_or(22);
        let ssh_user = config.ssh_user.as_ref()
            .ok_or("SSH_USER no está configurado")?;
        
        let remote_host = config.ssh_remote_host.as_deref()
            .unwrap_or("localhost");
        let remote_port = config.ssh_remote_port.unwrap_or(3306);
        
        println!("Conectando a servidor SSH: {}@{}:{}", ssh_user, ssh_host, ssh_port);
        
        // Conectar al servidor SSH
        let tcp = TcpStream::connect(format!("{}:{}", ssh_host, ssh_port))?;
        tcp.set_read_timeout(Some(Duration::from_secs(10)))?;
        tcp.set_write_timeout(Some(Duration::from_secs(10)))?;
        
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        
        // Autenticación
        if let Some(ref password) = config.ssh_password {
            session.userauth_password(ssh_user, password)?;
        } else if let Some(ref key_path) = config.ssh_key_path {
            session.userauth_pubkey_file(ssh_user, None, std::path::Path::new(key_path), None)?;
        } else {
            // Intentar autenticación con agente SSH
            session.userauth_agent(ssh_user)?;
        }
        
        if !session.authenticated() {
            return Err("Autenticación SSH fallida".into());
        }
        
        println!("Autenticación SSH exitosa");
        
        // Encontrar un puerto local disponible
        let local_port = if let Some(port) = config.ssh_local_port {
            port
        } else {
            find_free_port().await?
        };
        
        println!("Creando túnel SSH: localhost:{} -> {}:{}", local_port, remote_host, remote_port);
        
        // Crear el túnel
        let session_arc = Arc::new(Mutex::new(session));
        
        // Iniciar el listener del túnel en un task separado
        let session_clone = session_arc.clone();
        let remote_host_clone = remote_host.to_string();
        let remote_port_clone = remote_port;
        
        let keep_alive_handle = task::spawn(async move {
            if let Err(e) = Self::run_tunnel(session_clone, local_port, remote_host_clone, remote_port_clone).await {
                eprintln!("Error en túnel SSH: {}", e);
            }
        });
        
        Ok(Self {
            session: session_arc,
            local_port,
            keep_alive_handle: Some(keep_alive_handle),
        })
    }
    
    /// Ejecuta el túnel SSH escuchando en el puerto local
    async fn run_tunnel(
        session: Arc<Mutex<Session>>,
        local_port: u16,
        remote_host: String,
        remote_port: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", local_port)).await?;
        println!("Túnel SSH escuchando en 127.0.0.1:{}", local_port);
        
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let session_clone = session.clone();
                    let remote_host_clone = remote_host.clone();
                    let remote_port_clone = remote_port;
                    
                    task::spawn(async move {
                        if let Err(e) = Self::handle_connection(session_clone, stream, remote_host_clone, remote_port_clone).await {
                            eprintln!("Error manejando conexión del túnel: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Error aceptando conexión en túnel SSH: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Maneja una conexión individual a través del túnel
    async fn handle_connection(
        session: Arc<Mutex<Session>>,
        local_stream: tokio::net::TcpStream,
        remote_host: String,
        remote_port: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Convertir el stream de Tokio a std::net::TcpStream para ssh2
        let mut local_stream_std = local_stream.into_std()?;
        let mut local_stream_clone = local_stream_std.try_clone()?;
        
        // Crear dos canales SSH: uno para lectura y otro para escritura
        // (aunque técnicamente un canal es bidireccional, esto simplifica el manejo)
        let (mut channel_write, mut channel_read) = {
            let sess = session.lock().unwrap();
            let channel = sess.channel_direct_tcpip(&remote_host, remote_port, None)?;
            let reader = channel.stream(0);
            (channel, reader)
        };
        
        // Leer desde local y escribir a remoto
        let read_task = task::spawn_blocking(move || {
            let mut buf = [0u8; 8192];
            loop {
                match local_stream_std.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if let Err(e) = channel_write.write_all(&buf[..n]) {
                            eprintln!("Error escribiendo a canal SSH: {}", e);
                            break;
                        }
                        if let Err(e) = channel_write.flush() {
                            eprintln!("Error haciendo flush a canal SSH: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error leyendo desde local: {}", e);
                        break;
                    }
                }
            }
            Ok::<(), String>(())
        });
        
        // Leer desde remoto y escribir a local
        let write_task = task::spawn_blocking(move || {
            let mut buf = [0u8; 8192];
            loop {
                match channel_read.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if let Err(e) = local_stream_clone.write_all(&buf[..n]) {
                            eprintln!("Error escribiendo a local: {}", e);
                            break;
                        }
                        if let Err(e) = local_stream_clone.flush() {
                            eprintln!("Error haciendo flush a local: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error leyendo desde canal SSH: {}", e);
                        break;
                    }
                }
            }
            Ok::<(), String>(())
        });
        
        // Esperar a que termine cualquiera de las tareas
        tokio::select! {
            _ = read_task => {}
            _ = write_task => {}
        }
        
        Ok(())
    }
    
    /// Retorna el puerto local del túnel
    pub fn local_port(&self) -> u16 {
        self.local_port
    }
    
    /// Verifica si el túnel está activo
    pub fn is_active(&self) -> bool {
        if let Ok(session) = self.session.lock() {
            session.authenticated()
        } else {
            false
        }
    }
    
    /// Cierra el túnel SSH
    pub fn close(self) {
        // El handle se cancelará automáticamente cuando se dropee
        if let Some(handle) = self.keep_alive_handle {
            handle.abort();
        }
    }
}

/// Encuentra un puerto local disponible
async fn find_free_port() -> Result<u16, Box<dyn std::error::Error>> {
    use std::net::TcpListener;
    
    // Intentar puertos desde 3307 hasta 3399
    for port in 3307..3400 {
        if TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok() {
            return Ok(port);
        }
    }
    
    Err("No se pudo encontrar un puerto local disponible".into())
}

/// Verifica si SSH está configurado en la configuración
pub fn is_ssh_configured(config: &DatabaseConfig) -> bool {
    config.ssh_host.is_some() && config.ssh_user.is_some()
}

