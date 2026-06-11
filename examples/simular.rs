use cadiz12::config::PartidaConfig;
use cadiz12::engine::Motor;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Cargar configuracion
    let args: Vec<String> = std::env::args().collect();
    let ruta_config = if args.len() > 1 {
        &args[1]
    } else {
        "configs/basic/completa.json"
    };

    let config = PartidaConfig::cargar_desde_json(ruta_config)?;
    println!("Cargada configuracion: {}", config.nombre);

    // Crear motor
    let motor = Motor::nuevo(config);

    // Simular 10 jornadas
    for i in 0..10 {
        let estado = motor.api.estado_jornada();
        println!("Jornada {}: Acto {} - {} eventos disponibles",
                 estado.tiempo.jornada, estado.tiempo.acto, estado.eventos_disponibles.len());

        // Resolver primer evento disponible
        if !estado.eventos_disponibles.is_empty() {
            let evento = &estado.eventos_disponibles[0];
            let resultado = motor.api.resolver_evento(&evento.evento_id, "apoyar");
            match resultado {
                Ok(_) => println!("  -> Resuelto evento: {}", evento.titulo),
                Err(e) => println!("  -> Error: {}", e),
            }
        }
    }

    Ok(())
}