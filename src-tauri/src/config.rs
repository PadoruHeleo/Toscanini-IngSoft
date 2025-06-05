use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{Engine as _, engine::general_purpose};
use dirs;
use keyring::Entry;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

const APP_NAME: &str = "ToscaniniApp";
const CONFIG_FILE: &str = "config.enc";
const KEYRING_SERVICE: &str = "toscanini_db_config";
const KEYRING_USERNAME: &str = "database";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 3306,
            username: "root".to_string(),
            password: "".to_string(),
            database: "toscanini_db".to_string(),
        }
    }
}

impl DatabaseConfig {
    pub fn to_connection_string(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

pub struct SecureConfig {
    config_path: PathBuf,
}

impl SecureConfig {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join(APP_NAME);
        
        // Crear el directorio si no existe
        fs::create_dir_all(&config_dir)?;
        
        let config_path = config_dir.join(CONFIG_FILE);
        
        Ok(Self { config_path })
    }

    /// Genera una clave de encriptación basada en el hardware del sistema
    fn generate_machine_key() -> Result<[u8; 32], Box<dyn std::error::Error>> {
        // Usar información del sistema para generar una clave única por máquina
        let mut hasher = Sha256::new();
        
        // Información del sistema que debería ser única por máquina
        if let Ok(hostname) = std::env::var("COMPUTERNAME").or_else(|_| std::env::var("HOSTNAME")) {
            hasher.update(hostname.as_bytes());
        }
        
        if let Ok(username) = std::env::var("USERNAME").or_else(|_| std::env::var("USER")) {
            hasher.update(username.as_bytes());
        }
        
        // Usar también el path del ejecutable para más entropía
        if let Ok(exe_path) = std::env::current_exe() {
            hasher.update(exe_path.to_string_lossy().as_bytes());
        }
        
        // Añadir una semilla constante específica de la aplicación
        hasher.update(b"ToscaniniApp2024SecureConfig");
        
        let hash = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(&hash[..32]);
        
        Ok(key)
    }

    /// Encripta y guarda la configuración
    pub fn save_config(&self, config: &DatabaseConfig) -> Result<(), Box<dyn std::error::Error>> {
        // Primero intentar guardar en el keyring del sistema
        if let Err(e) = self.save_to_keyring(config) {
            println!("Warning: Could not save to keyring: {}", e);
        }

        // Serializar la configuración
        let config_json = serde_json::to_string(config)?;
        
        // Generar clave de encriptación
        let key = Self::generate_machine_key()?;
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
        
        // Generar nonce aleatorio
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
          // Encriptar
        let ciphertext = cipher.encrypt(nonce, config_json.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;
        
        // Combinar nonce + ciphertext
        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&nonce_bytes);
        encrypted_data.extend_from_slice(&ciphertext);
        
        // Codificar en base64 y guardar
        let encoded = general_purpose::STANDARD.encode(&encrypted_data);
        fs::write(&self.config_path, encoded)?;
        
        println!("Configuration saved securely to: {:?}", self.config_path);
        Ok(())
    }

    /// Carga y desencripta la configuración
    pub fn load_config(&self) -> Result<DatabaseConfig, Box<dyn std::error::Error>> {
        // Primero intentar cargar desde el keyring
        if let Ok(config) = self.load_from_keyring() {
            return Ok(config);
        }

        // Si no está en el keyring, intentar cargar desde el archivo encriptado
        if !self.config_path.exists() {
            return Ok(DatabaseConfig::default());
        }

        // Leer y decodificar
        let encoded_data = fs::read_to_string(&self.config_path)?;
        let encrypted_data = general_purpose::STANDARD.decode(encoded_data.trim())?;
        
        if encrypted_data.len() < 12 {
            return Err("Invalid encrypted data".into());
        }
        
        // Separar nonce y ciphertext
        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        // Generar la misma clave
        let key = Self::generate_machine_key()?;
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
          // Desencriptar
        let decrypted = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        let config_json = String::from_utf8(decrypted)?;
        
        // Deserializar
        let config: DatabaseConfig = serde_json::from_str(&config_json)?;
        
        Ok(config)
    }

    /// Guarda la configuración en el keyring del sistema
    fn save_to_keyring(&self, config: &DatabaseConfig) -> Result<(), Box<dyn std::error::Error>> {
        let entry = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)?;
        let config_json = serde_json::to_string(config)?;
        entry.set_password(&config_json)?;
        Ok(())
    }

    /// Carga la configuración desde el keyring del sistema
    fn load_from_keyring(&self) -> Result<DatabaseConfig, Box<dyn std::error::Error>> {
        let entry = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)?;
        let config_json = entry.get_password()?;
        let config: DatabaseConfig = serde_json::from_str(&config_json)?;
        Ok(config)
    }

    /// Elimina la configuración (tanto del archivo como del keyring)
    pub fn delete_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Eliminar del keyring
        if let Ok(entry) = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME) {
            let _ = entry.delete_password(); // Ignorar errores si no existe
        }

        // Eliminar archivo
        if self.config_path.exists() {
            fs::remove_file(&self.config_path)?;
        }

        Ok(())
    }

    /// Verifica si existe una configuración guardada
    pub fn config_exists(&self) -> bool {
        // Verificar en keyring primero
        if let Ok(entry) = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME) {
            if entry.get_password().is_ok() {
                return true;
            }
        }

        // Verificar archivo encriptado
        self.config_path.exists()
    }
}

/// Función de conveniencia para cargar la configuración de la base de datos
pub fn load_database_config() -> Result<DatabaseConfig, Box<dyn std::error::Error>> {
    let secure_config = SecureConfig::new()?;
    
    // Si no existe configuración guardada, usar variables de entorno o valores por defecto
    if !secure_config.config_exists() {
        return Ok(load_from_env_or_default());
    }

    secure_config.load_config()
}

/// Carga configuración desde variables de entorno o usa valores por defecto
fn load_from_env_or_default() -> DatabaseConfig {
    if let Ok(database_url) = std::env::var("DATABASE_URL") {
        // Parsear URL de conexión si está disponible
        if let Ok(config) = parse_database_url(&database_url) {
            return config;
        }
    }

    // Usar variables individuales o valores por defecto
    DatabaseConfig {
        host: std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
        port: std::env::var("DB_PORT")
            .unwrap_or_else(|_| "3306".to_string())
            .parse()
            .unwrap_or(3306),
        username: std::env::var("DB_USERNAME").unwrap_or_else(|_| "root".to_string()),
        password: std::env::var("DB_PASSWORD").unwrap_or_else(|_| "".to_string()),
        database: std::env::var("DB_DATABASE").unwrap_or_else(|_| "toscanini_db".to_string()),
    }
}

/// Parsea una URL de conexión MySQL
fn parse_database_url(url: &str) -> Result<DatabaseConfig, Box<dyn std::error::Error>> {
    // Formato esperado: mysql://username:password@host:port/database
    if !url.starts_with("mysql://") {
        return Err("Invalid MySQL URL format".into());
    }

    let url = &url[8..]; // Remover "mysql://"
    
    let parts: Vec<&str> = url.split('@').collect();
    if parts.len() != 2 {
        return Err("Invalid MySQL URL format".into());
    }

    let auth_part = parts[0];
    let host_db_part = parts[1];

    // Parsear usuario:contraseña
    let auth_parts: Vec<&str> = auth_part.split(':').collect();
    let username = auth_parts[0].to_string();
    let password = if auth_parts.len() > 1 {
        auth_parts[1].to_string()
    } else {
        "".to_string()
    };

    // Parsear host:puerto/base_de_datos
    let host_db_parts: Vec<&str> = host_db_part.split('/').collect();
    if host_db_parts.len() != 2 {
        return Err("Invalid MySQL URL format".into());
    }

    let host_port = host_db_parts[0];
    let database = host_db_parts[1].to_string();

    // Parsear host:puerto
    let host_port_parts: Vec<&str> = host_port.split(':').collect();
    let host = host_port_parts[0].to_string();
    let port = if host_port_parts.len() > 1 {
        host_port_parts[1].parse().unwrap_or(3306)
    } else {
        3306
    };

    Ok(DatabaseConfig {
        host,
        port,
        username,
        password,
        database,
    })
}
