// Comandos de Tauri para generar PDFs
use tauri::command;
use crate::database::get_db_pool_safe;
use crate::pdf::{CotizacionPdfData, CotizacionPdfGenerator, InformePdfData, InformePdfGenerator, EmpresaInfo, ClienteInfo, EquipoInfo, PiezaPdf, TerminoPdf};
use sqlx::Row;

#[command]
pub async fn generate_cotizacion_pdf_command(
    cotizacion_id: i32
) -> Result<Vec<u8>, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener datos de la cotización
    let cotizacion_row = sqlx::query(
        "SELECT c.cotizacion_codigo, c.costo_revision, c.costo_reparacion, c.costo_total, 
                c.is_aprobada, c.informe, c.created_at,
                ot.orden_codigo,
                cl.cliente_nombre, cl.cliente_correo, cl.cliente_telefono, cl.cliente_direccion,
                e.equipo_marca, e.equipo_modelo, e.equipo_tipo, e.numero_serie, e.equipo_ubicacion
         FROM COTIZACION c
         LEFT JOIN ORDEN_TRABAJO ot ON c.cotizacion_id = ot.cotizacion_id
         LEFT JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         LEFT JOIN CLIENTE cl ON e.cliente_id = cl.cliente_id
         WHERE c.cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo cotización: {}", e))?
    .ok_or_else(|| format!("Cotización con ID {} no encontrada", cotizacion_id))?;

    // Acceder a los campos por nombre
    let cotizacion_codigo: Option<String> = cotizacion_row.try_get("cotizacion_codigo").ok();
    let costo_revision: Option<i32> = cotizacion_row.try_get("costo_revision").ok();
    let costo_reparacion: Option<i32> = cotizacion_row.try_get("costo_reparacion").ok();
    let costo_total: Option<i32> = cotizacion_row.try_get("costo_total").ok();
    let is_aprobada: Option<i32> = cotizacion_row.try_get("is_aprobada").ok();
    let informe: Option<String> = cotizacion_row.try_get("informe").ok();
    let created_at: chrono::DateTime<chrono::Utc> = cotizacion_row.try_get("created_at").ok().unwrap();
    let orden_codigo: Option<String> = cotizacion_row.try_get("orden_codigo").ok();
    let cliente_nombre: Option<String> = cotizacion_row.try_get("cliente_nombre").ok();
    let cliente_correo: Option<String> = cotizacion_row.try_get("cliente_correo").ok();
    let cliente_telefono: Option<String> = cotizacion_row.try_get("cliente_telefono").ok();
    let cliente_direccion: Option<String> = cotizacion_row.try_get("cliente_direccion").ok();
    let equipo_marca: Option<String> = cotizacion_row.try_get("equipo_marca").ok();
    let equipo_modelo: Option<String> = cotizacion_row.try_get("equipo_modelo").ok();
    let equipo_tipo: Option<String> = cotizacion_row.try_get("equipo_tipo").ok();
    let numero_serie: Option<String> = cotizacion_row.try_get("numero_serie").ok();
    let equipo_ubicacion: Option<String> = cotizacion_row.try_get("equipo_ubicacion").ok();

    // Obtener piezas de la cotización
    let piezas_rows = sqlx::query(
        "SELECT p.pieza_nombre, p.pieza_marca, p.pieza_precio, pc.cantidad
         FROM PIEZAS_COTIZACION pc
         INNER JOIN PIEZA p ON pc.pieza_id = p.pieza_id
         WHERE pc.cotizacion_id = ?"
    )
    .bind(cotizacion_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo piezas: {}", e))?;

    // Obtener términos y condiciones de la cotización
    let terminos_rows = sqlx::query(
        "SELECT tc.termino_nombre, tc.termino_descripcion
         FROM TERMINOS_COTIZACION tcot
         INNER JOIN TERMINOS_CONDICIONES tc ON tcot.termino_id = tc.termino_id
         WHERE tcot.cotizacion_id = ? AND tcot.aplicado = TRUE AND tc.is_active = TRUE
         ORDER BY tc.termino_nombre"
    )
    .bind(cotizacion_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo términos y condiciones: {}", e))?;
    
    println!("DEBUG: Encontrados {} términos para cotización {}", terminos_rows.len(), cotizacion_id);

    let piezas: Vec<PiezaPdf> = piezas_rows.iter().map(|row| {
        let pieza_nombre: String = row.try_get("pieza_nombre")
            .ok()
            .flatten()
            .unwrap_or_else(|| "Pieza sin nombre".to_string());
        let pieza_marca: Option<String> = row.try_get("pieza_marca").ok().flatten();
        let pieza_precio: Option<i32> = row.try_get("pieza_precio").ok().flatten();
        let cantidad: Option<i32> = row.try_get("cantidad").ok().flatten();
        
        PiezaPdf {
            nombre: pieza_nombre,
            marca: pieza_marca,
            cantidad: cantidad.unwrap_or(1),
            precio_unitario: pieza_precio.unwrap_or(0),
            subtotal: (pieza_precio.unwrap_or(0) * cantidad.unwrap_or(1)),
        }
    }).collect();

    let terminos_condiciones: Vec<TerminoPdf> = terminos_rows.iter().map(|row| {
        let termino_nombre: String = row.try_get("termino_nombre").unwrap();
        let termino_descripcion: String = row.try_get("termino_descripcion").unwrap();
        TerminoPdf {
            nombre: termino_nombre,
            descripcion: termino_descripcion,
        }
    }).collect();

    // Crear estructura de datos para el PDF
    let pdf_data = CotizacionPdfData {
        cotizacion_codigo: cotizacion_codigo.unwrap_or_else(|| "COT-0000".to_string()),
        fecha: created_at,
        empresa: EmpresaInfo {
            nombre: "Toscanini".to_string(),
            direccion: Some("Dirección de la empresa".to_string()),
            telefono: Some("+56 9 1234 5678".to_string()),
            email: Some("contacto@toscanini.cl".to_string()),
            website: Some("www.toscanini.cl".to_string()),
        },
        cliente: ClienteInfo {
            nombre: cliente_nombre.unwrap_or_else(|| "Cliente sin nombre".to_string()),
            email: cliente_correo,
            telefono: cliente_telefono,
            direccion: cliente_direccion,
        },
        equipo: EquipoInfo {
            marca: equipo_marca,
            modelo: equipo_modelo,
            tipo: equipo_tipo,
            numero_serie: numero_serie,
            ubicacion: equipo_ubicacion,
        },
        informe_tecnico: informe.unwrap_or_else(|| "Sin informe técnico".to_string()),
        costo_revision: costo_revision,
        costo_reparacion: costo_reparacion,
        costo_total: costo_total.unwrap_or(0),
        piezas,
        is_aprobada: is_aprobada.unwrap_or(0) == 1,
        orden_codigo: orden_codigo,
        terminos_condiciones,
    };

    // Generar PDF
    let generator = CotizacionPdfGenerator::new();
    generator.generate_cotizacion_pdf(pdf_data).await
}

#[command]  
pub async fn generate_informe_pdf_command(
    informe_id: i32
) -> Result<Vec<u8>, String> {
    let pool = get_db_pool_safe()?;
    
    // Obtener datos del informe
    let informe_row = sqlx::query(
        "SELECT i.informe_codigo, i.diagnostico, i.recomendaciones, i.solucion_aplicada, 
                i.tecnico_responsable, i.created_at,
                ot.orden_codigo, ot.has_garantia,
                cl.cliente_nombre, cl.cliente_correo, cl.cliente_telefono, cl.cliente_direccion,
                e.equipo_marca, e.equipo_modelo, e.equipo_tipo, e.numero_serie, e.equipo_ubicacion
         FROM INFORME i
         LEFT JOIN ORDEN_TRABAJO ot ON i.informe_id = ot.informe_id
         LEFT JOIN EQUIPO e ON ot.equipo_id = e.equipo_id
         LEFT JOIN CLIENTE cl ON e.cliente_id = cl.cliente_id
         WHERE i.informe_id = ?"
    )
    .bind(informe_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo informe: {}", e))?
    .ok_or_else(|| format!("Informe con ID {} no encontrado", informe_id))?;

    // Acceder a los campos por nombre
    let informe_codigo: Option<String> = informe_row.try_get("informe_codigo").ok();
    let diagnostico: Option<String> = informe_row.try_get("diagnostico").ok();
    let recomendaciones: Option<String> = informe_row.try_get("recomendaciones").ok();
    let solucion_aplicada: Option<String> = informe_row.try_get("solucion_aplicada").ok();
    let tecnico_responsable: Option<String> = informe_row.try_get("tecnico_responsable").ok();
    let created_at: chrono::DateTime<chrono::Utc> = informe_row.try_get("created_at").ok().unwrap();
    let orden_codigo: Option<String> = informe_row.try_get("orden_codigo").ok();
    let has_garantia: Option<i32> = informe_row.try_get("has_garantia").ok();
    let cliente_nombre: Option<String> = informe_row.try_get("cliente_nombre").ok();
    let cliente_correo: Option<String> = informe_row.try_get("cliente_correo").ok();
    let cliente_telefono: Option<String> = informe_row.try_get("cliente_telefono").ok();
    let cliente_direccion: Option<String> = informe_row.try_get("cliente_direccion").ok();
    let equipo_marca: Option<String> = informe_row.try_get("equipo_marca").ok();
    let equipo_modelo: Option<String> = informe_row.try_get("equipo_modelo").ok();
    let equipo_tipo: Option<String> = informe_row.try_get("equipo_tipo").ok();
    let numero_serie: Option<String> = informe_row.try_get("numero_serie").ok();
    let equipo_ubicacion: Option<String> = informe_row.try_get("equipo_ubicacion").ok();

    // Obtener términos y condiciones del informe
    let terminos_rows = sqlx::query(
        "SELECT tc.termino_nombre, tc.termino_descripcion
         FROM TERMINOS_INFORME ti
         INNER JOIN TERMINOS_CONDICIONES tc ON ti.termino_id = tc.termino_id
         WHERE ti.informe_id = ? AND ti.aplicado = TRUE AND tc.is_active = TRUE
         ORDER BY tc.termino_nombre"
    )
    .bind(informe_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo términos y condiciones del informe: {}", e))?;
    
    println!("DEBUG: Encontrados {} términos para informe {}", terminos_rows.len(), informe_id);

    // Obtener piezas del informe
    let piezas_rows = sqlx::query(
        "SELECT p.pieza_nombre, p.pieza_marca, p.pieza_precio, pi.cantidad
         FROM PIEZAS_INFORME pi
         INNER JOIN PIEZA p ON pi.pieza_id = p.pieza_id
         WHERE pi.informe_id = ?"
    )
    .bind(informe_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo piezas del informe: {}", e))?;

    let piezas: Vec<PiezaPdf> = piezas_rows.iter().map(|row| {
        let pieza_nombre: String = row.try_get("pieza_nombre")
            .ok()
            .flatten()
            .unwrap_or_else(|| "Pieza sin nombre".to_string());
        let pieza_marca: Option<String> = row.try_get("pieza_marca").ok();
        let pieza_precio: Option<i32> = row.try_get("pieza_precio").ok();
        let cantidad: Option<i32> = row.try_get("cantidad").ok();
        PiezaPdf {
            nombre: pieza_nombre,
            marca: pieza_marca,
            cantidad: cantidad.unwrap_or(1),
            precio_unitario: pieza_precio.unwrap_or(0),
            subtotal: (pieza_precio.unwrap_or(0) * cantidad.unwrap_or(1)),
        }
    }).collect();

    let terminos_condiciones: Vec<TerminoPdf> = terminos_rows.iter().map(|row| {
        let termino_nombre: String = row.try_get("termino_nombre").unwrap();
        let termino_descripcion: String = row.try_get("termino_descripcion").unwrap();
        TerminoPdf {
            nombre: termino_nombre,
            descripcion: termino_descripcion,
        }
    }).collect();

    // Crear estructura de datos para el PDF
    let pdf_data = InformePdfData {
        informe_codigo: informe_codigo.unwrap_or_else(|| "INF-0000".to_string()),
        fecha: created_at,
        empresa: EmpresaInfo {
            nombre: "Toscanini".to_string(),
            direccion: Some("Dirección de la empresa".to_string()),
            telefono: Some("+56 9 1234 5678".to_string()),
            email: Some("contacto@toscanini.cl".to_string()),
            website: Some("www.toscanini.cl".to_string()),
        },
        cliente: ClienteInfo {
            nombre: cliente_nombre.unwrap_or_else(|| "Cliente sin nombre".to_string()),
            email: cliente_correo,
            telefono: cliente_telefono,
            direccion: cliente_direccion,
        },
        equipo: EquipoInfo {
            marca: equipo_marca,
            modelo: equipo_modelo,
            tipo: equipo_tipo,
            numero_serie: numero_serie,
            ubicacion: equipo_ubicacion,
        },
        diagnostico: diagnostico.unwrap_or_else(|| "Sin diagnóstico".to_string()),
        recomendaciones: recomendaciones,
        solucion_aplicada: solucion_aplicada,
        tecnico_responsable: tecnico_responsable.unwrap_or_else(|| "No especificado".to_string()),
        piezas,
        orden_codigo: orden_codigo,
        tiene_garantia: has_garantia.unwrap_or(0) == 1,
        terminos_condiciones,
    };

    // Generar PDF
    let generator = InformePdfGenerator::new();
    generator.generate_informe_pdf(pdf_data).await
}

