use log::LevelFilter;
use simplelog::{ConfigBuilder, WriteLogger};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

static LOG_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);

#[cfg(windows)]
mod windows_seh {
    use winapi::um::winnt::EXCEPTION_POINTERS;
    use winapi::um::errhandlingapi::SetUnhandledExceptionFilter;
    
    // Constantes de excepción de Windows (valores numéricos)
    const EXCEPTION_ACCESS_VIOLATION: u32 = 0xc0000005;
    const EXCEPTION_DATATYPE_MISALIGNMENT: u32 = 0x80000002;
    const EXCEPTION_BREAKPOINT: u32 = 0x80000003;
    const EXCEPTION_SINGLE_STEP: u32 = 0x80000004;
    const EXCEPTION_ARRAY_BOUNDS_EXCEEDED: u32 = 0xc000008c;
    const EXCEPTION_FLT_DENORMAL_OPERAND: u32 = 0xc000008d;
    const EXCEPTION_FLT_DIVIDE_BY_ZERO: u32 = 0xc000008e;
    const EXCEPTION_FLT_INEXACT_RESULT: u32 = 0xc000008f;
    const EXCEPTION_FLT_INVALID_OPERATION: u32 = 0xc0000090;
    const EXCEPTION_FLT_OVERFLOW: u32 = 0xc0000091;
    const EXCEPTION_FLT_STACK_CHECK: u32 = 0xc0000092;
    const EXCEPTION_FLT_UNDERFLOW: u32 = 0xc0000093;
    const EXCEPTION_INT_DIVIDE_BY_ZERO: u32 = 0xc0000094;
    const EXCEPTION_INT_OVERFLOW: u32 = 0xc0000095;
    const EXCEPTION_PRIV_INSTRUCTION: u32 = 0xc0000096;
    const EXCEPTION_IN_PAGE_ERROR: u32 = 0xc0000006;
    const EXCEPTION_ILLEGAL_INSTRUCTION: u32 = 0xc000001d;
    const EXCEPTION_NONCONTINUABLE_EXCEPTION: u32 = 0xc0000025;
    const EXCEPTION_STACK_OVERFLOW: u32 = 0xc00000fd;
    const EXCEPTION_INVALID_DISPOSITION: u32 = 0xc0000026;
    const EXCEPTION_EXECUTE_HANDLER: i32 = 1;

    /// Handler de excepciones no manejadas de Windows (SEH)
    /// Captura errores como 0xc0000005 antes de que Windows los maneje
    pub unsafe extern "system" fn exception_filter(exception_info: *mut EXCEPTION_POINTERS) -> i32 {
        if exception_info.is_null() {
            return EXCEPTION_EXECUTE_HANDLER;
        }

        let record = (*exception_info).ExceptionRecord;
        if record.is_null() {
            return EXCEPTION_EXECUTE_HANDLER;
        }

        let exception_code = (*record).ExceptionCode;
        let exception_address = (*record).ExceptionAddress as usize;

        // Obtener información del error
        let exception_name = match exception_code {
            EXCEPTION_ACCESS_VIOLATION => "EXCEPTION_ACCESS_VIOLATION (0xc0000005)",
            EXCEPTION_ARRAY_BOUNDS_EXCEEDED => "EXCEPTION_ARRAY_BOUNDS_EXCEEDED",
            EXCEPTION_BREAKPOINT => "EXCEPTION_BREAKPOINT",
            EXCEPTION_DATATYPE_MISALIGNMENT => "EXCEPTION_DATATYPE_MISALIGNMENT",
            EXCEPTION_FLT_DENORMAL_OPERAND => "EXCEPTION_FLT_DENORMAL_OPERAND",
            EXCEPTION_FLT_DIVIDE_BY_ZERO => "EXCEPTION_FLT_DIVIDE_BY_ZERO",
            EXCEPTION_FLT_INEXACT_RESULT => "EXCEPTION_FLT_INEXACT_RESULT",
            EXCEPTION_FLT_INVALID_OPERATION => "EXCEPTION_FLT_INVALID_OPERATION",
            EXCEPTION_FLT_OVERFLOW => "EXCEPTION_FLT_OVERFLOW",
            EXCEPTION_FLT_STACK_CHECK => "EXCEPTION_FLT_STACK_CHECK",
            EXCEPTION_FLT_UNDERFLOW => "EXCEPTION_FLT_UNDERFLOW",
            EXCEPTION_ILLEGAL_INSTRUCTION => "EXCEPTION_ILLEGAL_INSTRUCTION",
            EXCEPTION_IN_PAGE_ERROR => "EXCEPTION_IN_PAGE_ERROR",
            EXCEPTION_INT_DIVIDE_BY_ZERO => "EXCEPTION_INT_DIVIDE_BY_ZERO",
            EXCEPTION_INT_OVERFLOW => "EXCEPTION_INT_OVERFLOW",
            EXCEPTION_INVALID_DISPOSITION => "EXCEPTION_INVALID_DISPOSITION",
            EXCEPTION_NONCONTINUABLE_EXCEPTION => "EXCEPTION_NONCONTINUABLE_EXCEPTION",
            EXCEPTION_PRIV_INSTRUCTION => "EXCEPTION_PRIV_INSTRUCTION",
            EXCEPTION_SINGLE_STEP => "EXCEPTION_SINGLE_STEP",
            EXCEPTION_STACK_OVERFLOW => "EXCEPTION_STACK_OVERFLOW",
            _ => "EXCEPTION_UNKNOWN",
        };

        // Información adicional para ACCESS_VIOLATION
        let access_info = if exception_code == EXCEPTION_ACCESS_VIOLATION && (*record).NumberParameters >= 2 {
            let read_write = if (*record).ExceptionInformation[0] == 0 {
                "READ"
            } else {
                "WRITE"
            };
            let address = (*record).ExceptionInformation[1];
            format!("Operación: {}, Dirección: 0x{:X}", read_write, address)
        } else {
            String::new()
        };

        let error_message = format!(
            "\n=== EXCEPCIÓN DE WINDOWS NO MANEJADA (SEH) ===\n\
            Código de excepción: 0x{:08X}\n\
            Tipo: {}\n\
            Dirección de excepción: 0x{:X}\n\
            {}\n\
            Thread ID: {:?}\n\
            ============================================\n\
            Esta excepción ocurrió ANTES de que Rust/Tauri pudiera manejarla.\n\
            Esto es común con errores de acceso a memoria (0xc0000005).\n\
            Ver: https://github.com/tauri-apps/tauri/issues/13482\n\
            ============================================\n",
            exception_code,
            exception_name,
            exception_address,
            access_info,
            std::thread::current().id()
        );

        // Escribir al log de emergencia
        super::write_to_log_file(&error_message);

        // Intentar escribir también al log principal si está disponible
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("toscanini_seh_error.log")
        {
            use std::io::Write;
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let _ = writeln!(file, "[{}] {}", timestamp, error_message);
            let _ = file.flush();
        }

        // Devolver EXCEPTION_EXECUTE_HANDLER para que Windows muestre el diálogo de error
        // pero ya tenemos la información en el log
        EXCEPTION_EXECUTE_HANDLER
    }

    /// Configura el handler de excepciones de Windows
    pub fn setup_windows_exception_handler() {
        unsafe {
            let old_handler = SetUnhandledExceptionFilter(Some(exception_filter));
            if old_handler.is_some() {
                // Handler configurado exitosamente
            }
        }
    }
}

#[cfg(windows)]
pub use windows_seh::setup_windows_exception_handler;

#[cfg(not(windows))]
pub fn setup_windows_exception_handler() {
    // No-op en sistemas no-Windows
}

/// Escribe información del sistema al archivo de log
fn log_system_info() {
    use std::env;
    
    // Obtener información de Windows si está disponible
    #[cfg(windows)]
    let os_info = format!("Windows - {}", env::consts::OS);
    #[cfg(not(windows))]
    let os_info = format!("{}", env::consts::OS);
    
    // Información de memoria (si está disponible)
    let memory_info = get_memory_info();
    
    // Información de threads
    let thread_info = format!("Threads activos: {}", std::thread::available_parallelism()
        .map(|n| n.get().to_string())
        .unwrap_or_else(|_| "Desconocido".to_string()));
    
    let system_info = format!(
        "\n=== INFORMACIÓN DEL SISTEMA ===\n\
        Sistema operativo: {}\n\
        Arquitectura de compilación: {}\n\
        Plataforma objetivo: {}\n\
        Modo de compilación: {}\n\
        Ejecutable: {:?}\n\
        Directorio de ejecución: {:?}\n\
        {}\n\
        {}\n\
        Variables de entorno relevantes:\n\
        - CARGO_BUILD_TARGET: {:?}\n\
        - TARGET: {:?}\n\
        - PATH contiene Rust: {}\n\
        ================================\n",
        os_info,
        env::consts::ARCH,
        env::var("CARGO_BUILD_TARGET").unwrap_or_else(|_| "default".to_string()),
        if cfg!(debug_assertions) { "DEBUG" } else { "RELEASE" },
        env::current_exe().ok(),
        env::current_dir().ok(),
        memory_info,
        thread_info,
        env::var("CARGO_BUILD_TARGET").ok(),
        env::var("TARGET").ok(),
        env::var("PATH").ok()
            .map(|p| p.contains("rust") || p.contains("cargo"))
            .unwrap_or(false)
    );
    
    write_to_log_file(&system_info);
}

/// Obtiene información de memoria del sistema
fn get_memory_info() -> String {
    #[cfg(windows)]
    {
        use std::mem;
        use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
        
        unsafe {
            let mut mem_status: MEMORYSTATUSEX = mem::zeroed();
            mem_status.dwLength = mem::size_of::<MEMORYSTATUSEX>() as u32;
            
            if GlobalMemoryStatusEx(&mut mem_status) != 0 {
                format!(
                    "Memoria física total: {} MB\n\
                    Memoria física disponible: {} MB\n\
                    Memoria virtual total: {} MB\n\
                    Memoria virtual disponible: {} MB",
                    mem_status.ullTotalPhys / (1024 * 1024),
                    mem_status.ullAvailPhys / (1024 * 1024),
                    mem_status.ullTotalVirtual / (1024 * 1024),
                    mem_status.ullAvailVirtual / (1024 * 1024)
                )
            } else {
                "Información de memoria no disponible".to_string()
            }
        }
    }
    
    #[cfg(not(windows))]
    {
        "Información de memoria no disponible en esta plataforma".to_string()
    }
}

/// Inicializa el sistema de logging
/// En modo release (build), escribe los logs a un archivo
/// En modo debug, también muestra en consola
pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    // Determinar el nivel de log según el modo de compilación
    let log_level = if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    // En modo release, guardar logs en archivo
    if cfg!(not(debug_assertions)) {
        let log_dir = get_log_directory()?;
        std::fs::create_dir_all(&log_dir)?;

        let log_file_path = log_dir.join(get_log_filename());
        
        // Guardar la ruta del archivo de log para uso posterior
        *LOG_FILE.lock().unwrap() = Some(log_file_path.clone());

        // Configuración del logger con formato detallado
        let mut config_builder = ConfigBuilder::new();
        config_builder.set_time_format_rfc3339();
        if let Err(_) = config_builder.set_time_offset_to_local() {
            // Si falla, usar configuración por defecto
        }
        let config = config_builder.build();

        // Crear el archivo de log
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file_path)?;

        // Inicializar el logger solo con escritura a archivo en release
        WriteLogger::init(log_level, config, log_file)?;

        // Registrar el inicio del sistema de logging
        log::info!("=== Sistema de logging inicializado ===");
        log::info!("Archivo de log: {:?}", log_file_path);
        log::info!("Nivel de log: {:?}", log_level);
        
        // Registrar información del sistema inmediatamente
        log_system_info();
    } else {
        // En modo debug, usar configuración simple para consola
        let mut config_builder = ConfigBuilder::new();
        config_builder.set_time_format_rfc3339();
        if let Err(_) = config_builder.set_time_offset_to_local() {
            // Si falla, usar configuración por defecto
        }
        let config = config_builder.build();

        // En debug, también escribir a archivo para tener historial
        let log_dir = get_log_directory().ok();
        if let Some(ref dir) = log_dir {
            std::fs::create_dir_all(dir).ok();
            let log_file_path = dir.join(get_log_filename());
            *LOG_FILE.lock().unwrap() = Some(log_file_path.clone());
            
            if let Ok(log_file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_file_path)
            {
                WriteLogger::init(log_level, config.clone(), log_file)?;
                log::info!("=== Sistema de logging inicializado (DEBUG) ===");
                log::info!("Archivo de log: {:?}", log_file_path);
                
                // Registrar información del sistema inmediatamente
                log_system_info();
            }
        }
    }

    Ok(())
}

/// Obtiene el directorio donde se guardarán los logs
fn get_log_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Intentar usar el directorio de datos de la aplicación
    if let Some(data_dir) = dirs::data_dir() {
        let app_log_dir = data_dir.join("Toscanini").join("logs");
        return Ok(app_log_dir);
    }

    // Fallback: usar el directorio actual si no se puede obtener el directorio de datos
    Ok(PathBuf::from(".").join("logs"))
}

/// Genera el nombre del archivo de log con fecha
fn get_log_filename() -> String {
    use chrono::Local;
    let now = Local::now();
    format!("toscanini_{}.log", now.format("%Y%m%d"))
}

/// Obtiene la ruta del archivo de log actual
pub fn get_log_file_path() -> Option<PathBuf> {
    LOG_FILE.lock().unwrap().clone()
}

/// Escribe un mensaje directamente al archivo de log (útil para panics)
pub fn write_to_log_file(message: &str) {
    // Intentar escribir al archivo de log si está configurado
    if let Some(ref log_path) = *LOG_FILE.lock().unwrap() {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
        {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let _ = writeln!(file, "[{}] {}", timestamp, message);
            let _ = file.flush(); // Asegurar que se escriba inmediatamente
        }
    } else {
        // Si no hay archivo de log configurado, intentar crear uno de emergencia
        // en el directorio actual o junto al ejecutable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let emergency_log = exe_dir.join("toscanini_error.log");
                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&emergency_log)
                {
                    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
                    let _ = writeln!(file, "[{}] EMERGENCY LOG: {}", timestamp, message);
                    let _ = file.flush();
                }
            }
        }
    }
}

/// Configura el handler de panics para capturar errores críticos
pub fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        use std::env;
        
        let panic_message = panic_info.payload().downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| panic_info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "Unknown panic".to_string());
        
        let location = panic_info.location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "Unknown location".to_string());
        
        let backtrace = std::backtrace::Backtrace::capture();
        
        // Información adicional para diagnóstico
        let diagnostic_info = format!(
            "\n=== ERROR CRÍTICO (PANIC) ===\n\
            Mensaje: {}\n\
            Ubicación: {}\n\
            Arquitectura: {}\n\
            Modo: {}\n\
            Ejecutable: {:?}\n\
            Backtrace:\n{}\n\
            =============================\n\
            NOTA: Error 0xc0000005 (ACCESS_VIOLATION) puede ser causado por:\n\
            1. Incompatibilidad de arquitectura (32-bit vs 64-bit)\n\
            2. DLLs faltantes o incompatibles\n\
            3. Problemas con WebView2 en sistemas antiguos\n\
            4. Problemas de memoria en sistemas de 32 bits\n\
            5. Problemas con el runtime de Tokio\n\
            =============================\n",
            panic_message,
            location,
            env::consts::ARCH,
            if cfg!(debug_assertions) { "DEBUG" } else { "RELEASE" },
            env::current_exe().ok(),
            backtrace
        );

        // Escribir al archivo de log
        write_to_log_file(&diagnostic_info);
        
        // También usar el logger si está disponible
        log::error!("{}", diagnostic_info);
        
        // En modo debug, también mostrar en consola
        if cfg!(debug_assertions) {
            eprintln!("{}", diagnostic_info);
        }
    }));
}

