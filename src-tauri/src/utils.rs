use bcrypt::{hash, verify, DEFAULT_COST};

/// Encripta una contraseña usando bcrypt
pub fn hash_password(password: &str) -> Result<String, String> {
    hash(password, DEFAULT_COST)
        .map_err(|e| format!("Error hashing password: {}", e))
}

/// Verifica una contraseña contra su hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    verify(password, hash)
        .map_err(|e| format!("Error verifying password: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "test_password_123";
        let hashed = hash_password(password).unwrap();
        
        // Verificar que la contraseña original coincide
        assert!(verify_password(password, &hashed).unwrap());
        
        // Verificar que una contraseña incorrecta no coincide
        assert!(!verify_password("wrong_password", &hashed).unwrap());
    }
}
