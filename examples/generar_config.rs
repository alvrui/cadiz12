use cadiz12::config::{PartidaConfig, PerfilProtagonista, Compromiso, Origen, ClaseSocial, Oficio, AdscripcionPolitica, Temperamento};
use cadiz12::sdk::GeneradorConfig;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("=== SDK CADIZ 1812 - Generador de Configuraciones ===\n");

    // Crear generador
    let generador = GeneradorConfig::nuevo();

    // Crear perfil del protagonista
    let perfil = PerfilProtagonista {
        origen: Origen::Peninsular,
        clase_social: ClaseSocial::HidalguiaProfesionLetrada,
        oficio: Oficio::JuristaAbogado,
        adscripcion: AdscripcionPolitica::LiberalProgresista,
        temperamento: Temperamento::Prudente,
        compromisos: vec![
            Compromiso {
                id: "deuda_consulado".to_string(),
                tipo: "material".to_string(),
                descripcion: "Deuda con el Consulado de Cádiz".to_string(),
                impacto: "Presión económica constante".to_string(),
            },
            Compromiso {
                id: "promesa_reforma".to_string(),
                tipo: "emocional".to_string(),
                descripcion: "Prometiste luchar por la abolición de la Inquisición".to_string(),
                impacto: "Si no cumples, perderás reputación entre los liberales".to_string(),
            },
        ],
    };

    // Generar configuración completa
    println!("Generando configuración de partida...");
    let config = generador.generar_partida(
        "Partida Generada por SDK",
        "Configuración generada automáticamente usando el SDK",
        perfil,
    )?;

    println!("✓ Configuración generada");
    println!("  - Facciones: {}", config.facciones.facciones.len());
    println!("  - Espacios: {}", config.espacios.espacios.len());
    println!("  - Personajes: {}", config.personajes.npcs.len());
    println!("  - Eventos fijos: {}", config.eventos.eventos_fijos.len());
    println!("  - Plantillas: {}", config.eventos.plantillas.len());

    // Guardar configuración
    let ruta = "configs/generated/partida_sdk.json";
    std::fs::create_dir_all(Path::new(ruta).parent().unwrap())?;
    config.guardar_como_json(ruta)?;
    println!("\n✓ Configuración guardada en: {}", ruta);

    // Validar configuración cargándola
    println!("\nValidando configuración...");
    let config_cargada = PartidaConfig::cargar_desde_json(ruta)?;
    assert_eq!(config.nombre, config_cargada.nombre);
    println!("✓ Configuración válida y cargable");

    // Mostrar ejemplo de uso con Mistral (si hay API key)
    if let Ok(api_key) = std::env::var("MISTRAL_API_KEY") {
        println!("\nGenerando personaje con Mistral API...");
        
        use cadiz12::config::{FaccionId, Oficio};
        
        // Crear cliente Mistral directamente para el ejemplo
        use cadiz12::sdk::ClienteMistral;
        let cliente = ClienteMistral::nuevo(api_key);
        
        match cliente.generar_personaje(
                "Crea un personaje para la facción liberal progresista, un jurista que sea diputado en las Cortes",
                &FaccionId::LiberalProgresista,
                &Oficio::JuristaAbogado,
            ).await {
            Ok(npc) => {
                println!("✓ Personaje generado por IA:");
                println!("  - ID: {}", npc.id);
                println!("  - Nombre: {}", npc.nombre);
                println!("  - Descripción: {}", npc.descripcion);
                println!("  - Facción: {:?}", npc.faccion);
                println!("  - Oficio: {:?}", npc.oficio);
            }
            Err(e) => {
                println!("⚠ Error generando personaje con IA: {}", e);
            }
        }
    } else {
        println!("\n⚠ MISTRAL_API_KEY no configurada. Saltando generación con IA.");
        println!("  Para usar la generación con IA, configura la variable de entorno MISTRAL_API_KEY");
    }

    Ok(())
}
