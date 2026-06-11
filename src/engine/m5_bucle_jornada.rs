use super::{
    m1_estado_mundo::EstadoMundo,
    m2_estado_protagonista::EstadoProtagonista,
    m3_medidores::SistemaMedidores,
    m4_generador_eventos::GeneradorEventos,
    m6_memoria::MemoriaSistema,
    dtos::{EstadoJornadaDto, TiempoDto},
};

/// Bucle de jornada (M5)
#[derive(Debug, Clone)]
pub struct BucleJornada {
    pub generador_eventos: GeneradorEventos,
    pub sistema_medidores: SistemaMedidores,
    pub memoria: MemoriaSistema,
}

impl BucleJornada {
    pub fn nuevo(
        config_eventos: crate::config::EventosConfig,
        _config_medidores: crate::config::MedidoresConfig,
    ) -> Self {
        Self {
            generador_eventos: GeneradorEventos::nuevo(config_eventos),
            sistema_medidores: SistemaMedidores,
            memoria: MemoriaSistema::nueva(),
        }
    }

    /// Iniciar una nueva jornada
    pub fn iniciar_jornada(
        &mut self,
        mundo: &mut EstadoMundo,
        protagonista: &mut EstadoProtagonista,
        config: &crate::config::PartidaConfig,
    ) -> EstadoJornadaDto {
        mundo.avanzar_jornada();

        super::m3_medidores::SistemaMedidores::aplicar_decaimiento(protagonista, &config.medidores, mundo.jornada_absoluta);

        let _eventos_umbral = super::m3_medidores::SistemaMedidores::verificar_umbrales(&self.sistema_medidores, protagonista);

        self.memoria.limpiar_cooldowns(mundo.jornada_absoluta);

        let eventos_disponibles = self.generador_eventos.generar_eventos(
            mundo,
            protagonista,
            config.presupuesto_temporal,
        );

        EstadoJornadaDto {
            tiempo: TiempoDto {
                tramo_id: mundo.tramo_id.clone(),
                acto: mundo.acto_narrativo,
                jornada: mundo.jornada_absoluta,
            },
            protagonista: protagonista.a_protagonista_dto(),
            crisis_activa: mundo.crisis.as_ref().map(|c| super::dtos::CrisisActivaDto {
                tipo_id: c.tipo_id.clone(),
                fase: c.fase.to_string(),
            }),
            eventos_disponibles,
            presupuesto_temporal: config.presupuesto_temporal,
        }
    }

    /// Resolver un evento
    pub fn resolver_evento(
        &mut self,
        mundo: &mut EstadoMundo,
        protagonista: &mut EstadoProtagonista,
        evento_id: &str,
        opcion_id: &str,
    ) -> anyhow::Result<()> {
        if let Some(plantilla) = self.generador_eventos.config.plantillas.get(evento_id) {
            if let Some(opcion) = plantilla.opciones.iter().find(|o| o.id == opcion_id) {
                super::m3_medidores::SistemaMedidores::aplicar_consecuencias(protagonista, &plantilla.consecuencias);
                super::m3_medidores::SistemaMedidores::aplicar_consecuencias(protagonista, &opcion.consecuencias);

                self.memoria.registrar_evento(evento_id, opcion_id, mundo.jornada_absoluta);

                protagonista.historial.push(super::m2_estado_protagonista::Decision {
                    jornada: mundo.jornada_absoluta,
                    evento_id: evento_id.to_string(),
                    opcion_id: opcion_id.to_string(),
                    consecuencias: opcion.consecuencias.clone(),
                });

                Ok(())
            } else {
                anyhow::bail!("Opcion {} no encontrada en evento {}", opcion_id, evento_id);
            }
        } else {
            anyhow::bail!("Evento {} no encontrado en plantillas", evento_id);
        }
    }
}