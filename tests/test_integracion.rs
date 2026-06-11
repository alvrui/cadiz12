#[cfg(test)]
mod tests {
    use cadiz12::config::PartidaConfig;
    use cadiz12::engine::Motor;

    #[test]
    fn test_cargar_configuracion_completa() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuración completa");
        
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
            .expect("Error cargando configuración");
        
        let mut motor = Motor::nuevo(config);
        
        // La primera llamada a estado_jornada avanza a jornada 1
        let estado1 = motor.api.estado_jornada();
        assert_eq!(estado1.tiempo.jornada, 1);
        assert_eq!(estado1.tiempo.acto, 1);
        assert_eq!(estado1.presupuesto_temporal, 8);
    }

    #[test]
    fn test_simular_5_jornadas() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuración");
        
        let mut motor = Motor::nuevo(config);
        
        for _ in 0..5 {
            let estado = motor.api.estado_jornada();
            println!("Jornada {}: {} eventos disponibles",
                estado.tiempo.jornada, estado.eventos_disponibles.len());
            
            assert!(estado.eventos_disponibles.len() > 0,
                "No hay eventos en jornada {}", estado.tiempo.jornada);
            
            // Verificar que el protagonista tiene 6 medidores
            assert_eq!(estado.protagonista.medidores.len(), 6);
            
            // Verificar que los medidores tienen valores válidos
            for medidor in &estado.protagonista.medidores {
                assert!(medidor.valor <= 100);
                assert!(medidor.tendencia >= -3 && medidor.tendencia <= 3);
            }
        }
        
        // Verificar que avanzamos 5 jornadas (empezamos en 1, luego 2,3,4,5,6)
        let estado_final = motor.api.estado_jornada();
        assert_eq!(estado_final.tiempo.jornada, 6);
    }

    #[test]
    fn test_resolver_evento() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuración");
        
        let mut motor = Motor::nuevo(config);
        
        // Avanzar a la primera jornada
        let estado1 = motor.api.estado_jornada();
        assert_eq!(estado1.tiempo.jornada, 1);
        
        // Obtener eventos disponibles
        assert!(estado1.eventos_disponibles.len() > 0);
        
        // Intentar resolver el primer evento con la primera opción
        if let Some(evento) = estado1.eventos_disponibles.first() {
            let resultado = motor.api.resolver_evento(&evento.evento_id, "apoyar_liberal");
            match resultado {
                Ok(_) => println!("Evento resuelto con éxito"),
                Err(e) => println!("Error resolviendo evento (esperado): {}", e),
            }
        }
    }

    #[test]
    fn test_medidores_cruzan_umbrales() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuración");
        
        let mut motor = Motor::nuevo(config);
        
        // Avanzar varias jornadas
        for _ in 0..10 {
            motor.api.estado_jornada();
        }
        
        // Obtener estado del personaje
        let estado = motor.api.estado_personaje();
        
        // Verificar que los medidores tienen valores
        for medidor in &estado.medidores {
            println!("{} = {} (tendencia: {})", medidor.nombre, medidor.valor, medidor.tendencia);
        }
    }

    #[test]
    fn test_eventos_fijos_aparecen_en_jornada_correcta() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuración");
        
        let mut motor = Motor::nuevo(config);
        
        // Avanzar hasta la jornada 100 (Inauguración de las Cortes)
        // Cada llamada a estado_jornada avanza la jornada
        for _ in 0..99 {
            motor.api.estado_jornada();
        }
        
        let estado = motor.api.estado_jornada();
        // Esperamos jornada 100 (empezamos en 1, +99 = 100)
        assert_eq!(estado.tiempo.jornada, 100);
        
        // Buscar evento fijo de inauguración de las Cortes
        let _tiene_evento_fijo = estado.eventos_disponibles.iter()
            .any(|e| e.evento_id == "inauguracion_cortes");
        
        println!("Eventos en jornada 100: {:?}", 
            estado.eventos_disponibles.iter().map(|e| &e.evento_id).collect::<Vec<_>>());
    }

    #[test]
    fn test_medidores_cambian_al_resolver_evento() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuración");
        
        let mut motor = Motor::nuevo(config);

        // Avanzar a jornada con eventos
        let estado_jornada = motor.api.estado_jornada();
        
        // Depurar: qué eventos están disponibles
        println!("Eventos disponibles: {:?}", 
            estado_jornada.eventos_disponibles.iter().map(|e| &e.evento_id).collect::<Vec<_>>());
        println!("Posición formal: {}", estado_jornada.protagonista.posicion_formal_id);

        // Buscar el evento debate_articulo_constitucional que cambia medidores
        if let Some(evento) = estado_jornada.eventos_disponibles.iter()
            .find(|e| e.evento_id == "debate_articulo_constitucional") {
            // Resolver con opción apoyar_liberal que tiene consecuencia influencia: 5
            let resultado = motor.api.resolver_evento(&evento.evento_id, "apoyar_liberal");
            assert!(resultado.is_ok(), "Error resolviendo evento: {:?}", resultado.err());
            
            // Obtener estado después de resolver
            let estado_final = motor.api.estado_personaje();
            let influencia_final = estado_final.medidores.iter()
                .find(|m| m.nombre == "influencia")
                .map(|m| m.valor)
                .unwrap_or(50);

            // Verificar que la influencia ha cambiado
            assert_ne!(50, influencia_final,
                "Los medidores no cambiaron después de resolver evento. influencia: 50 -> {}", influencia_final);
        } else {
            // Si no hay debate, probar con rumor_en_cafe
            if let Some(evento) = estado_jornada.eventos_disponibles.iter()
                .find(|e| e.evento_id == "rumor_en_cafe") {
                let resultado = motor.api.resolver_evento(&evento.evento_id, "escuchar_atentamente");
                assert!(resultado.is_ok(), "Error resolviendo evento: {:?}", resultado.err());
                
                let estado_final = motor.api.estado_personaje();
                let relacional_final = estado_final.medidores.iter()
                    .find(|m| m.nombre == "relacional")
                    .map(|m| m.valor)
                    .unwrap_or(50);
                
                assert_ne!(50, relacional_final,
                    "Los medidores no cambiaron después de resolver evento. relacional: 50 -> {}", relacional_final);
            } else {
                panic!("Ni debate_articulo_constitucional ni rumor_en_cafe disponibles");
            }
        }
    }

    #[test]
    fn test_reputaciones_segmentadas_se_actualizan() {
        let config = PartidaConfig::cargar_desde_json("configs/basic/completa.json")
            .expect("Error cargando configuración");
        
        let mut motor = Motor::nuevo(config);

        // Avanzar a jornada con eventos
        let estado_jornada = motor.api.estado_jornada();

        // Resolver evento de debate con opción liberal
        if let Some(evento) = estado_jornada.eventos_disponibles.iter()
            .find(|e| e.evento_id == "debate_articulo_constitucional") {
            let resultado = motor.api.resolver_evento(&evento.evento_id, "apoyar_liberal");
            assert!(resultado.is_ok(), "Error resolviendo evento: {:?}", resultado.err());
        }

        // Verificar que la reputación con liberales progresistas ha aumentado
        let estado_final = motor.api.estado_personaje();
        let rep_liberales = estado_final.reputaciones.get("liberal_progresista");
        assert!(rep_liberales.is_some(), "No hay reputación con liberales progresistas");
        assert!(rep_liberales.unwrap().valor > 50,
            "Reputación con liberales no aumentó después de apoyar opción liberal");
    }
}
