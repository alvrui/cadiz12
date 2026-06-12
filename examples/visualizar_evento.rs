use cadiz12::sdk::{generador_narrativas::Narrativa, recursos_visuales::{generar_placeholder_fondo, generar_placeholder_localizacion, generar_placeholder_personaje, CacheImagenes, TipoPlaceholder}};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Generando placeholders visuales para narrativa ===");
    
    // Cargar una narrativa de ejemplo (usar la generada por el ejemplo generar_narrativa)
    let ruta_narrativa = "configs/generated/narrativa_ejemplo.json";
    
    let narrativa_json = fs::read_to_string(ruta_narrativa)?;
    let narrativa: Narrativa = serde_json::from_str(&narrativa_json)?;
    
    println!("Narrativa cargada: {}", narrativa.metadatos.titulo);
    println!("Número de eventos: {}", narrativa.cadena_principal.eventos.len());
    
    // Crear cache de imágenes
    let cache = CacheImagenes::nuevo("configs/generated/visuales", 86400, 10); // 1 día TTL, 10MB max
    fs::create_dir_all(&cache.directorio)?;
    
    let mut resumen = Vec::new();
    
    // Para cada evento, generar placeholders si no hay recursos definidos
    for (i, evento) in narrativa.cadena_principal.eventos.iter().enumerate() {
        println!("\n--- Evento {}: {} ---", i + 1, evento.titulo);
        
        // Determinar tipo de placeholder según el evento
        let tipo_placeholder = if evento.titulo.contains("Cortes") || evento.titulo.contains("Constitución") {
            TipoPlaceholder::Fondo
        } else if evento.titulo.contains("café") || evento.titulo.contains("local") || evento.titulo.contains("lugar") {
            TipoPlaceholder::Localizacion
        } else if evento.titulo.contains("personaje") || evento.titulo.contains("PNJ") {
            TipoPlaceholder::Personaje
        } else {
            TipoPlaceholder::Fondo
        };
        
        // Generar placeholder
        let texto_placeholder = match tipo_placeholder {
            TipoPlaceholder::Fondo => format!("Fondo: {}", evento.titulo),
            TipoPlaceholder::Localizacion => format!("Localización: {}", evento.titulo),
            TipoPlaceholder::Personaje => format!("PNJ: {}", evento.titulo),
        };
        
        // Usar cache para obtener o generar
        let clave = format!("placeholder_{}_{}", i, evento.id);
        
        let ruta_archivo = cache.obtener_o_generar(&clave, || {
            match tipo_placeholder {
                TipoPlaceholder::Fondo => generar_placeholder_fondo(&texto_placeholder),
                TipoPlaceholder::Localizacion => generar_placeholder_localizacion(&texto_placeholder),
                TipoPlaceholder::Personaje => generar_placeholder_personaje(&texto_placeholder),
            }
        })?;
        
        println!("  → Placeholder generado: {}", ruta_archivo.display());
        
        // Añadir al resumen
        resumen.push((
            evento.id.clone(),
            evento.titulo.clone(),
            ruta_archivo.display().to_string(),
            texto_placeholder,
        ));
    }
    
    // Guardar resumen
    let ruta_resumen = cache.directorio.join("resumen_placeholders.txt");
    let mut contenido_resumen = String::new();
    contenido_resumen.push_str("=== RESUMEN DE PLACEHOLDERS GENERADOS ===\n\n");
    contenido_resumen.push_str(&format!("Narrativa: {}\n", narrativa.metadatos.titulo));
    contenido_resumen.push_str(&format!("Fecha: {}\n\n", narrativa.metadatos.fecha_creacion));
    
    for (id, titulo, ruta, texto) in &resumen {
        contenido_resumen.push_str(&format!("- Evento '{}' ({}):\n", titulo, id));
        contenido_resumen.push_str(&format!("  Placeholder: {}\n", texto));
        contenido_resumen.push_str(&format!("  Archivo: {}\n\n", ruta));
    }
    
    fs::write(&ruta_resumen, contenido_resumen)?;
    println!("\n✅ Resumen guardado en: {}", ruta_resumen.display());
    
    // Mostrar tamaño del cache
    let tamano = cache.tamano_actual()?;
    println!("Tamaño total del cache: {} bytes ({:.2} KB)", tamano, tamano as f64 / 1024.0);
    
    println!("\n✅ Todos los placeholders generados con éxito!");
    
    Ok(())
}
