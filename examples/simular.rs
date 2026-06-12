use cadiz12::config::PartidaConfig;
use cadiz12::engine::Motor;

fn main() {
    // Cargar configuración
    let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
        .expect("Error cargando configuración");
    
    println!("=== SIMULACIÓN DE PARTIDA: {} ===", config.nombre);
    println!("Período: {} a {}", config.periodo_inicio, config.periodo_fin);
    println!("Protagonista: {:?}", config.perfil_inicial.origen);
    println!();
    
    let mut motor = Motor::nuevo(config);
    
    // Simular 10 jornadas
    for _ in 0..10 {
        let estado = motor.api.estado_jornada();
        
        println!("--- Jornada {} (Acto {}) ---", estado.tiempo.jornada, estado.tiempo.acto);
        println!("Tramo: {}", estado.tiempo.tramo_id);
        
        if let Some(crisis) = &estado.crisis_activa {
            println!("CRISIS ACTIVA: {} (fase: {})", crisis.tipo_id, crisis.fase);
        }
        
        println!("Presupuesto temporal: {}", estado.presupuesto_temporal);
        println!("Eventos disponibles: {}", estado.eventos_disponibles.len());
        
        for evento in &estado.eventos_disponibles {
            println!("  - {} ({}, coste: {}, prioridad: {})",
                evento.titulo, evento.tipo, evento.coste_temporal, evento.prioridad);
        }
        
        // Mostrar medidores
        println!("\nMedidores:");
        for medidor in &estado.protagonista.medidores {
            let tendencia_str = match medidor.tendencia {
                t if t > 0 => format!("+{}", t),
                t => format!("{}", t),
            };
            println!("  {}: {} ({}, umbrales: {}-{})",
                medidor.nombre, medidor.valor, tendencia_str,
                medidor.umbral_bajo, medidor.umbral_alto);
        }
        
        println!();
        
        // Resolver el primer evento con la primera opción
        if let Some(evento) = estado.eventos_disponibles.first() {
            let opciones = ["apoyar_liberal", "apoyar_moderado", "abstenerse", "escuchar_atentamente"];
            for opcion in &opciones {
                let resultado = motor.api.resolver_evento(&evento.evento_id, opcion);
                match resultado {
                    Ok(_) => {
                        println!("  → Evento '{}' resuelto con opción '{}'", evento.evento_id, opcion);
                        break;
                    }
                    Err(_) => {
                        // Continuar probando otras opciones
                    }
                }
            }
        }
    }
    
    // Estado final
    let estado_final = motor.api.estado_jornada();
    let personaje_final = motor.api.estado_personaje();
    
    println!("\n=== ESTADO FINAL (Jornada {}) ===", estado_final.tiempo.jornada);
    println!("Eventos resueltos: {}", personaje_final.relaciones.len());
    println!("\nMedidores finales:");
    for medidor in &personaje_final.medidores {
        println!("  {}: {}", medidor.nombre, medidor.valor);
    }
}
