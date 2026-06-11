use cadiz12::{
    config::PartidaConfig,
    ui::slint::ejecutar_juego,
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
    
    ejecutar_juego(config)
}
