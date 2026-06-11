use cadiz12::{
    config::PartidaConfig,
    ui::gtk::ejecutar_juego,
};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();

    let config = if args.len() > 1 {
        PartidaConfig::cargar_desde_json(&args[1])?
    } else {
        println!("Cargando configuracion basica...");
        PartidaConfig::cargar_desde_json("configs/basic/completa.json")?
    };

    println!("Iniciando juego: {}", config.nombre);
    println!("Pulsa 'q' para salir");

    ejecutar_juego(config)
}