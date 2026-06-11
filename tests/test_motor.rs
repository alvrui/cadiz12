#[cfg(test)]
mod tests {
    use cadiz12::config::PartidaConfig;
    use cadiz12::engine::Motor;

    #[test]
    fn test_motor_crear() {
        let config = PartidaConfig::default();
        let mut motor = Motor::nuevo(config);
        // Primera llamada a estado_jornada avanza a jornada 1
        let estado = motor.api.estado_jornada();
        assert_eq!(estado.tiempo.jornada, 1);
    }

    #[test]
    fn test_avanzar_jornada() {
        let config = PartidaConfig::default();
        let mut motor = Motor::nuevo(config);

        // Primera llamada avanza a jornada 1
        let estado1 = motor.api.estado_jornada();
        assert_eq!(estado1.tiempo.jornada, 1);

        // Segunda llamada avanza a jornada 2
        let estado2 = motor.api.estado_jornada();
        assert_eq!(estado2.tiempo.jornada, 2);
    }

    #[test]
    fn test_estado_protagonista_inicial() {
        let config = PartidaConfig::default();
        let motor = Motor::nuevo(config);

        let estado = motor.api.estado_personaje();
        assert_eq!(estado.medidores.len(), 6);
        assert_eq!(estado.medidores[0].nombre, "influencia");
        assert_eq!(estado.medidores[0].valor, 50);
    }
}
