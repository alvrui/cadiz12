use cadiz12::{
    config::PartidaConfig,
    ui::terminal::ejecutar_juego,
};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Cargar configuración
    let args: Vec<String> = std::env::args().collect();

    let config = if args.len() > 1 {
        // Cargar configuración desde archivo
        PartidaConfig::cargar_desde_json(&args[1])?
    } else {
        // Usar configuración básica por defecto
        println!("Cargando configuración básica...");
        PartidaConfig::cargar_desde_json("configs/basic/completa.json")?
    };

    println!("Iniciando juego: {}", config.nombre);
    println!("Pulsa 'q' para salir");

    // Ejecutar juego en terminal
    if let Err(e) = ejecutar_juego(config) {
        eprintln!("Error al ejecutar el juego: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
