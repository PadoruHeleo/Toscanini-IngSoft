use serde::{Deserialize, Serialize};

// Estructuras compartidas para datos del PDF
#[derive(Debug, Serialize, Deserialize)]
pub struct EmpresaInfo {
    pub nombre: String,
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClienteInfo {
    pub nombre: String,
    pub email: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquipoInfo {
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub tipo: Option<String>,
    pub numero_serie: Option<String>,
    pub ubicacion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PiezaPdf {
    pub nombre: String,
    pub marca: Option<String>,
    pub cantidad: i32,
    pub precio_unitario: i32,
    pub subtotal: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerminoPdf {
    pub nombre: String,
    pub descripcion: String,
}

/// Función auxiliar para dividir texto en líneas
pub fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if current_line.len() + word.len() + 1 > max_chars {
            if !current_line.is_empty() {
                lines.push(current_line.trim().to_string());
                current_line = String::new();
            }
        }
        
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    
    if !current_line.is_empty() {
        lines.push(current_line.trim().to_string());
    }
    
    if lines.is_empty() {
        lines.push(String::new());
    }
    
    lines
}

/// Función para compilar todos los términos y condiciones en un solo texto
pub fn compile_terminos_text(terminos: &[TerminoPdf]) -> String {
    if terminos.is_empty() {
        return "No se han definido términos y condiciones específicos para este documento.".to_string();
    }

    let mut texto_completo = String::new();
    
    for (i, termino) in terminos.iter().enumerate() {
        // Agregar el nombre del término con numeración
        texto_completo.push_str(&format!("{}. {}", i + 1, termino.nombre));
        
        // Agregar la descripción si existe
        if !termino.descripcion.is_empty() {
            texto_completo.push_str(&format!("- {}", termino.descripcion));
        }
        
        // Agregar separador entre términos
        if i < terminos.len() - 1 {
            texto_completo.push_str(" || ");
        }
    }
    
    println!("DEBUG: Texto compilado de {} términos: {}", terminos.len(), &texto_completo[..texto_completo.len().min(100)]);
    println!("DEBUG: Longitud total del texto: {} caracteres", texto_completo.len());
    texto_completo
}

