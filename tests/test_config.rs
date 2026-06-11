#[cfg(test)]
mod tests {
    use cadiz12::config::PartidaConfig;
    use std::path::Path;

    #[test]
    fn test_cargar_configuracion_basica() {
        let ruta = Path::new("configs/basic/partida_basica.json");
        let result = PartidaConfig::cargar_desde_json(ruta.to_str().unwrap());
        assert!(result.is_ok(), "Error cargando configuracion: {:?}", result.err());
        let config = result.unwrap();
        assert_eq!(config.nombre, "Partida Basica");
        assert_eq!(config.perfil_inicial.origen, cadiz12::config::personajes::Origen::Peninsular);
    }

    #[test]
    fn test_guardar_y_cargar_configuracion() {
        let config = PartidaConfig::default();
        let ruta = "target/test_config_temp.json";
        config.guardar_como_json(ruta).expect("Error guardando");
        let cargada = PartidaConfig::cargar_desde_json(ruta).expect("Error cargando");
        assert_eq!(config.nombre, cargada.nombre);
        std::fs::remove_file(ruta).ok();
    }
}
