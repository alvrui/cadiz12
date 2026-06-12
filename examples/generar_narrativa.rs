use cadiz12::sdk::{GeneradorNarrativas, ConfiguracionIA, TipoPlantillaGuion};
use cadiz12::config::{FaccionId, EspacioId, EstiloNarrativo};
use serde_json;
use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    println!("Generador de Narrativas - Cádiz 1812");
    println!("====================================");

    // Crear configuración de IA
    let config_ia = ConfiguracionIA {
        modelo: cadiz12::sdk::ModeloIA::MistralSmall,
        temperatura: 0.8,
        max_tokens: 2048,
        top_k: Some(50),
        top_p: Some(0.9),
        frequency_penalty: Some(0.1),
        presence_penalty: Some(0.1),
        n: 1,
        stop_sequences: vec!["\n\n".to_string(), "<|im_end|>".to_string()],
    };

    // Crear generador de narrativas
    let mut generador = GeneradorNarrativas::nuevo(config_ia);

    // Generar narrativa con plantilla DilemaMoral
    println!("\nGenerando narrativa con plantilla: Dilema Moral");
    let narrativa_opt = generador.generar_con_plantilla(TipoPlantillaGuion::DilemaMoral);
    
    if let Some(mut narrativa) = narrativa_opt {
        // Personalizar metadatos
        narrativa.metadatos = narrativa.metadatos
            .con_descripcion("Un dilema moral en la Cádiz de 1812: elección entre lealtad a la patria y lealtad a la familia")
            .con_dificultad(7)
            .con_tags(vec!["dilema", "moral", "lealtad", "familia", "patria"])
            .con_facciones(vec![
                FaccionId::LiberalProgresista,
                FaccionId::AbsolutistaServil,
            ])
            .con_espacios(vec![
                EspacioId::CafeApolo,
                EspacioId::SalonesTeatrosTertulias,
            ])
            .con_estilo(EstiloNarrativo::Detallado);

        // Añadir personajes
        narrativa = narrativa
            .con_personaje("Protagonista")
            .con_personaje("Hermano del Protagonista")
            .con_personaje("Líder Liberal");

        // Añadir objetos
        narrativa = narrativa
            .con_objeto("Carta de lealtad")
            .con_objeto("Sello real");

        // Validar narrativa
        println!("Validando narrativa...");
        match narrativa.validar() {
            Ok(_) => println!("✅ Narrativa válida - sin errores de coherencia"),
            Err(errores) => {
                println!("❌ Errores de coherencia detectados:");
                for error in errores {
                    println!("  - {}", error);
                }
                return Err(anyhow::anyhow!("Errores de validación"));
            }
        }

        // Generar texto contextual usando IA (placeholder)
        println!("\nGenerando texto contextual para el primer evento...");
        let texto = generador.generar_texto_contextual(
            "Primer evento del dilema moral",
            &narrativa.metadatos.estilo_recomendado,
            "Cádiz, 1812. El protagonista debe elegir entre su hermano y su país.",
        );
        println!("Texto generado: {}", texto);

        // Guardar narrativa en JSON
        let ruta_salida = "configs/generated/narrativa_ejemplo.json";
        println!("\nGuardando narrativa en: {}", ruta_salida);
        
        // Crear directorio si no existe
        let path = Path::new(ruta_salida);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        
        let json = serde_json::to_string_pretty(&narrativa)?;
        fs::write(ruta_salida, json)?;
        
        println!("✅ Narrativa guardada exitosamente");

        // Mostrar resumen
        println!("\n=== RESUMEN DE NARRATIVA ===");
        println!("ID: {}", narrativa.metadatos.id);
        println!("Título: {}", narrativa.metadatos.titulo);
        println!("Tipo: {:?}", narrativa.metadatos.tipo_plantilla);
        println!("Dificultad: {}", narrativa.metadatos.dificultad);
        println!("Estilo: {:?}", narrativa.metadatos.estilo_recomendado);
        println!("Facciones: {:?}", narrativa.metadatos.facciones_involucradas);
        println!("Espacios: {:?}", narrativa.metadatos.espacios_relacionados);
        println!("Personajes: {:?}", narrativa.personajes);
        println!("Objetos: {:?}", narrativa.objetos);
        println!("Número de eventos: {}", narrativa.cadena_principal.eventos.len());

        // Mostrar eventos
        println!("\n=== EVENTOS ===");
        for (i, evento) in narrativa.cadena_principal.eventos.iter().enumerate() {
            println!("\n{}. {} ({})", i + 1, evento.titulo, evento.id);
            println!("   Descripción: {}", evento.descripcion);
            println!("   Opciones ({}):", evento.opciones.len());
            for opcion in &evento.opciones {
                println!("     - {}: {}", opcion.id, opcion.texto);
                if opcion.recomendada {
                    println!("       [RECOMENDADA]");
                }
                if opcion.consecuencia.es_final {
                    println!("       [FINAL]");
                }
            }
        }

    } else {
        return Err(anyhow::anyhow!("No se pudo generar la narrativa"));
    }

    // Generar y guardar narrativa con otra plantilla para demostración
    println!("\n\nGenerando segunda narrativa con plantilla: Viaje del Héroe");
    let mut narrativa2 = generador.generar_con_plantilla(TipoPlantillaGuion::HeroesJourney)
        .ok_or_else(|| anyhow::anyhow!("No se pudo generar narrativa 2"))?;
    
    narrativa2.metadatos = narrativa2.metadatos
        .con_descripcion("Viaje del héroe: de aprendiz a líder")
        .con_dificultad(6)
        .con_estilo(EstiloNarrativo::Equilibrado);
    
    let ruta_salida2 = "configs/generated/narrativa_heroe.json";
    fs::create_dir_all(Path::new(ruta_salida2).parent().unwrap())?;
    let json2 = serde_json::to_string_pretty(&narrativa2)?;
    fs::write(ruta_salida2, json2)?;
    println!("✅ Narrativa 'Viaje del Héroe' guardada en: {}", ruta_salida2);

    println!("\n=== FIN DE EJECUCIÓN ===");
    println!("Archivos generados:");
    println!("  - configs/generated/narrativa_ejemplo.json");
    println!("  - configs/generated/narrativa_heroe.json");

    Ok(())
}
