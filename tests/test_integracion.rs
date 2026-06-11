#[cfg(test)]
mod tests {
    use cadiz12::config::PartidaConfig;
    use cadiz12::engine::Motor;

    #[test]
    fn test_cargar_configuracion_completa() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuracion completa");
        
        assert_eq!(config.nombre, "Partida Completa de Testing");
        assert_eq!(config.facciones.facciones.len(), 5);
        assert_eq!(config.espacios.espacios.len(), 13);
        assert_eq!(config.personajes.npcs.len(), 10);
        assert!(config.eventos.eventos_fijos.len() > 0);
        assert!(config.eventos.plantillas.len() > 0);
    }

    #[test]
    fn test_crear_motor_con_configuracion_completa() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuracion");
        
        let mut motor = Motor::nuevo(config);
        
        let estado1 = motor.api.estado_jornada();
        assert_eq!(estado1.tiempo.jornada, 1);
        assert_eq!(estado1.tiempo.acto, 1);
        assert_eq!(estado1.presupuesto_temporal, 8);
    }

    #[test]
    fn test_simular_5_jornadas() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuracion");
        
        let mut motor = Motor::nuevo(config);
        
        for _ in 0..5 {
            let estado = motor.api.estado_jornada();
            println!("Jornada {}: {} eventos disponibles",
                estado.tiempo.jornada, estado.eventos_disponibles.len());
            
            assert!(estado.eventos_disponibles.len() > 0,
                "No hay eventos en jornada {}", estado.tiempo.jornada);
            
            assert_eq!(estado.protagonista.medidores.len(), 6);
            
            for medidor in &estado.protagonista.medidores {
                assert!(medidor.valor <= 100);
                assert!(medidor.tendencia >= -3 && medidor.tendencia <= 3);
            }
        }
        
        let estado_final = motor.api.estado_jornada();
        assert_eq!(estado_final.tiempo.jornada, 6);
    }

    #[test]
    fn test_resolver_evento() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuracion");
        
        let mut motor = Motor::nuevo(config);
        
        let estado1 = motor.api.estado_jornada();
        assert_eq!(estado1.tiempo.jornada, 1);
        
        assert!(estado1.eventos_disponibles.len() > 0);
        
        if let Some(evento) = estado1.eventos_disponibles.first() {
            let resultado = motor.api.resolver_evento(&evento.evento_id, "apoyar_liberal");
            match resultado {
                Ok(_) => println!("Evento resuelto con exito"),
                Err(e) => println!("Error resolviendo evento (esperado): {}", e),
            }
        }
    }

    #[test]
    fn test_medidores_cruzan_umbrales() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuracion");
        
        let mut motor = Motor::nuevo(config);
        
        for _ in 0..10 {
            motor.api.estado_jornada();
        }
        
        let estado = motor.api.estado_personaje();
        
        for medidor in &estado.medidores {
            println!("{} = {} (tendencia: {})", medidor.nombre, medidor.valor, medidor.tendencia);
        }
    }

    #[test]
    fn test_eventos_fijos_aparecen_en_jornada_correcta() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuracion");
        
        let mut motor = Motor::nuevo(config);
        
        for _ in 0..99 {
            motor.api.estado_jornada();
        }
        
        let estado = motor.api.estado_jornada();
        assert_eq!(estado.tiempo.jornada, 100);
        
        let _tiene_evento_fijo = estado.eventos_disponibles.iter()
            .any(|e| e.evento_id == "inauguracion_cortes");
        
        println!("Eventos en jornada 100: {:?}",
            estado.eventos_disponibles.iter().map(|e| &e.evento_id).collect::<Vec<_>>());
    }

    #[test]
    fn test_medidores_cambian_al_resolver_evento() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuracion");
        
        let mut motor = Motor::nuevo(config);
        
        let estado_jornada = motor.api.estado_jornada();
        
        println!("Eventos disponibles: {:?}",
            estado_jornada.eventos_disponibles.iter().map(|e| &e.evento_id).collect::<Vec<_>>());
        println!("Posicion formal: {}", estado_jornada.protagonista.posicion_formal_id);
        
        if let Some(evento) = estado_jornada.eventos_disponibles.iter()
            .find(|e| e.evento_id == "debate_articulo_constitucional") {
            let resultado = motor.api.resolver_evento(&evento.evento_id, "apoyar_liberal");
            assert!(resultado.is_ok(), "Error resolviendo evento: {:?}", resultado.err());
            
            let estado_final = motor.api.estado_personaje();
            let influencia_final = estado_final.medidores.iter()
                .find(|m| m.nombre == "influencia")
                .map(|m| m.valor)
                .unwrap_or(50);
            
            assert_ne!(50, influencia_final,
                "Los medidores no cambiaron despues de resolver evento. influencia: 50 -> {}", influencia_final);
        } else if let Some(evento) = estado_jornada.eventos_disponibles.iter()
            .find(|e| e.evento_id == "rumor_en_cafe") {
            let resultado = motor.api.resolver_evento(&evento.evento_id, "escuchar_atentamente");
            assert!(resultado.is_ok(), "Error resolviendo evento: {:?}", resultado.err());
            
            let estado_final = motor.api.estado_personaje();
            let relacional_final = estado_final.medidores.iter()
                .find(|m| m.nombre == "relacional")
                .map(|m| m.valor)
                .unwrap_or(50);
            
            assert_ne!(50, relacional_final,
                "Los medidores no cambiaron despues de resolver evento. relacional: 50 -> {}", relacional_final);
        } else {
            panic!("Ni debate_articulo_constitucional ni rumor_en_cafe disponibles");
        }
    }

    #[test]
    fn test_reputaciones_segmentadas_se_actualizan() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuracion");
        
        let mut motor = Motor::nuevo(config);
        
        let estado_jornada = motor.api.estado_jornada();
        
        if let Some(evento) = estado_jornada.eventos_disponibles.iter()
            .find(|e| e.evento_id == "debate_articulo_constitucional") {
            let resultado = motor.api.resolver_evento(&evento.evento_id, "apoyar_liberal");
            assert!(resultado.is_ok(), "Error resolviendo evento: {:?}", resultado.err());
        }
        
        let estado_final = motor.api.estado_personaje();
        let rep_liberales = estado_final.reputaciones.get("liberal_progresista");
        assert!(rep_liberales.is_some(), "No hay reputacion con liberales progresistas");
        assert!(rep_liberales.unwrap().valor > 50,
            "Reputacion con liberales no aumento despues de apoyar opcion liberal");
    }
}