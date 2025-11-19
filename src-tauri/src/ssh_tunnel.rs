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
        println!("[SSH TUNNEL] Iniciando creación del túnel SSH...");
        
        let ssh_host = config.ssh_host.as_ref()
            .ok_or("SSH_HOST no está configurado")?;
        let ssh_port = config.ssh_port.unwrap_or(22);
        let ssh_user = config.ssh_user.as_ref()
            .ok_or("SSH_USER no está configurado")?;
        
        let remote_host = config.ssh_remote_host.as_deref()
            .unwrap_or("localhost");
        let remote_port = config.ssh_remote_port.unwrap_or(3306);
        
        println!("[SSH TUNNEL] Configuración SSH:");
        println!("  - Host SSH: {}:{}", ssh_host, ssh_port);
        println!("  - Usuario SSH: {}", ssh_user);
        println!("  - Destino remoto: {}:{}", remote_host, remote_port);
        println!("[SSH TUNNEL] Conectando a servidor SSH: {}@{}:{}", ssh_user, ssh_host, ssh_port);
        
        // Conectar al servidor SSH
        println!("[SSH TUNNEL] Estableciendo conexión TCP...");
        let tcp = TcpStream::connect(format!("{}:{}", ssh_host, ssh_port))?;
        println!("[SSH TUNNEL] Conexión TCP establecida exitosamente");
        
        tcp.set_read_timeout(Some(Duration::from_secs(10)))?;
        tcp.set_write_timeout(Some(Duration::from_secs(10)))?;
        println!("[SSH TUNNEL] Timeouts configurados (10 segundos)");
        
        println!("[SSH TUNNEL] Creando sesión SSH...");
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        println!("[SSH TUNNEL] Iniciando handshake SSH...");
        session.handshake()?;
        println!("[SSH TUNNEL] Handshake SSH completado");
        
        // Autenticación
        println!("[SSH TUNNEL] Iniciando autenticación SSH...");
        if let Some(ref password) = config.ssh_password {
            println!("[SSH TUNNEL] Usando autenticación por contraseña");
            session.userauth_password(ssh_user, password)?;
        } else if let Some(ref key_path) = config.ssh_key_path {
            println!("[SSH TUNNEL] Usando autenticación por clave privada: {}", key_path);
            session.userauth_pubkey_file(ssh_user, None, std::path::Path::new(key_path), None)?;
        } else {
            println!("[SSH TUNNEL] Intentando autenticación con agente SSH");
            session.userauth_agent(ssh_user)?;
        }
        
        if !session.authenticated() {
            println!("[SSH TUNNEL] ERROR: Autenticación SSH fallida");
            return Err("Autenticación SSH fallida".into());
        }
        
        println!("[SSH TUNNEL] ✓ Autenticación SSH exitosa");
        
        // Encontrar un puerto local disponible
        println!("[SSH TUNNEL] Buscando puerto local disponible...");
        let local_port = if let Some(port) = config.ssh_local_port {
            println!("[SSH TUNNEL] Usando puerto local especificado: {}", port);
            port
        } else {
            let port = find_free_port().await?;
            println!("[SSH TUNNEL] Puerto local asignado automáticamente: {}", port);
            port
        };
        
        println!("[SSH TUNNEL] Configuración del túnel:");
        println!("  - Puerto local: {}", local_port);
        println!("  - Destino remoto: {}:{}", remote_host, remote_port);
        println!("[SSH TUNNEL] Creando túnel SSH: localhost:{} -> {}:{}", local_port, remote_host, remote_port);
        
        // Crear el túnel
        println!("[SSH TUNNEL] Preparando sesión SSH para el túnel...");
        let session_arc = Arc::new(Mutex::new(session));
        
        // Iniciar el listener del túnel en un task separado
        let session_clone = session_arc.clone();
        let remote_host_clone = remote_host.to_string();
        let remote_port_clone = remote_port;
        
        println!("[SSH TUNNEL] Iniciando task de mantenimiento del túnel...");
        let keep_alive_handle = task::spawn(async move {
            println!("[SSH TUNNEL] Task de túnel iniciado, ejecutando run_tunnel...");
            if let Err(e) = Self::run_tunnel(session_clone, local_port, remote_host_clone, remote_port_clone).await {
                eprintln!("[SSH TUNNEL] ERROR en túnel SSH: {}", e);
            }
        });
        
        println!("[SSH TUNNEL] ✓ Túnel SSH creado exitosamente");
        println!("[SSH TUNNEL] Túnel escuchando en localhost:{}", local_port);
        
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
        println!("[SSH TUNNEL] run_tunnel: Iniciando listener en 127.0.0.1:{}", local_port);
        let listener = TcpListener::bind(format!("127.0.0.1:{}", local_port)).await?;
        println!("[SSH TUNNEL] ✓ Listener TCP creado exitosamente en 127.0.0.1:{}", local_port);
        println!("[SSH TUNNEL] Túnel SSH escuchando conexiones en 127.0.0.1:{}", local_port);
        println!("[SSH TUNNEL] Redirigiendo conexiones a {}:{}", remote_host, remote_port);
        
        loop {
            println!("[SSH TUNNEL] Esperando conexión entrante...");
            match listener.accept().await {
                Ok((stream, addr)) => {
                    println!("[SSH TUNNEL] ✓ Nueva conexión aceptada desde {}", addr);
                    let session_clone = session.clone();
                    let remote_host_clone = remote_host.clone();
                    let remote_port_clone = remote_port;
                    
                    task::spawn(async move {
                        println!("[SSH TUNNEL] Manejando conexión del túnel...");
                        if let Err(e) = Self::handle_connection(session_clone, stream, remote_host_clone, remote_port_clone).await {
                            eprintln!("[SSH TUNNEL] ERROR manejando conexión del túnel: {}", e);
                        } else {
                            println!("[SSH TUNNEL] Conexión del túnel cerrada normalmente");
                        }
                    });
                }
                Err(e) => {
                    eprintln!("[SSH TUNNEL] ERROR aceptando conexión en túnel SSH: {}", e);
                    break;
                }
            }
        }
        
        println!("[SSH TUNNEL] run_tunnel: Loop terminado");
        Ok(())
    }
    
    /// Maneja una conexión individual a través del túnel
    async fn handle_connection(
        session: Arc<Mutex<Session>>,
        local_stream: tokio::net::TcpStream,
        remote_host: String,
        remote_port: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("[SSH TUNNEL] handle_connection: Iniciando manejo de conexión");
        println!("[SSH TUNNEL] handle_connection: Destino remoto {}:{}", remote_host, remote_port);
        
        // Convertir el stream de Tokio a std::net::TcpStream para ssh2
        println!("[SSH TUNNEL] handle_connection: Convirtiendo stream de Tokio a std::net::TcpStream");
        let mut local_stream_std = local_stream.into_std()?;
        let mut local_stream_clone = local_stream_std.try_clone()?;
        println!("[SSH TUNNEL] handle_connection: Stream convertido exitosamente");
        
        // Crear dos canales SSH: uno para lectura y otro para escritura
        // (aunque técnicamente un canal es bidireccional, esto simplifica el manejo)
        println!("[SSH TUNNEL] handle_connection: Creando canal SSH direct_tcpip");
        let (mut channel_write, mut channel_read) = {
            let sess = session.lock().unwrap();
            let channel = sess.channel_direct_tcpip(&remote_host, remote_port, None)?;
            println!("[SSH TUNNEL] handle_connection: ✓ Canal SSH creado exitosamente");
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

