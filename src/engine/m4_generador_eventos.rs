use std::collections::HashMap;
use super::{
    m1_estado_mundo::EstadoMundo,
    m2_estado_protagonista::EstadoProtagonista,
    dtos::EventoDisponibleDto,
};
use crate::config::EventosConfig;

/// Generador de eventos (M4)
#[derive(Debug, Clone)]
pub struct GeneradorEventos {
    pub config: EventosConfig,
}

impl GeneradorEventos {
    pub fn nuevo(config: EventosConfig) -> Self {
        Self { config }
    }

    /// Generar eventos disponibles para la jornada
    pub fn generar_eventos(
        &self,
        mundo: &EstadoMundo,
        protagonista: &EstadoProtagonista,
        presupuesto: u8,
    ) -> Vec<EventoDisponibleDto> {
        let mut eventos = Vec::new();
        let mut coste_acumulado: u8 = 0;

        for evento_fijo in &self.config.eventos_fijos {
            if evento_fijo.jornada == mundo.jornada_absoluta {
                let evento_dto = EventoDisponibleDto {
                    evento_id: evento_fijo.id.clone(),
                    titulo: evento_fijo.nombre.clone(),
                    tipo: "fijo".to_string(),
                    coste_temporal: 4,
                    prioridad: 100,
                    modificador_perfil: 0.0,
                };
                if coste_acumulado + evento_dto.coste_temporal <= presupuesto {
                    eventos.push(evento_dto.clone());
                    coste_acumulado += evento_dto.coste_temporal;
                }
            }
        }

        if let Some(crisis) = &mundo.crisis {
            let eventos_crisis = self.generar_eventos_crisis(&crisis, protagonista);
            for evento in eventos_crisis {
                if coste_acumulado + evento.coste_temporal <= presupuesto {
                    eventos.push(evento.clone());
                    coste_acumulado += evento.coste_temporal;
                }
            }
        }

        let eventos_normales = self.generar_eventos_normales(mundo, protagonista);
        for evento in eventos_normales {
            if coste_acumulado + evento.coste_temporal <= presupuesto {
                eventos.push(evento.clone());
                coste_acumulado += evento.coste_temporal;
            }
        }

        eventos.sort_by(|a, b| {
            let prioridad_a = (b.prioridad as f32 * (1.0 + b.modificador_perfil)) as u32;
            let prioridad_b = (a.prioridad as f32 * (1.0 + a.modificador_perfil)) as u32;
            prioridad_a.cmp(&prioridad_b)
        });

        eventos
    }

    fn generar_eventos_crisis(
        &self,
        crisis: &super::m1_estado_mundo::CrisisActiva,
        protagonista: &EstadoProtagonista,
    ) -> Vec<EventoDisponibleDto> {
        match crisis.fase {
            super::m1_estado_mundo::FaseCrisis::Estallido => {
                vec![EventoDisponibleDto {
                    evento_id: format!("crisis_{}_estallido", crisis.tipo_id),
                    titulo: format!("Crisis! {}", crisis.tipo_id),
                    tipo: "crisis".to_string(),
                    coste_temporal: 3,
                    prioridad: 95,
                    modificador_perfil: 1.5,
                }]
            }
            _ => Vec::new(),
        }
    }

    fn generar_eventos_normales(
        &self,
        mundo: &EstadoMundo,
        protagonista: &EstadoProtagonista,
    ) -> Vec<EventoDisponibleDto> {
        let mut eventos = Vec::new();

        if protagonista.posicion.formal_id == "diputado" ||
           protagonista.posicion.formal_id == "diputado_suplente" {
            eventos.push(EventoDisponibleDto {
                evento_id: "debate_articulo_constitucional".to_string(),
                titulo: "Debate sobre articulo constitucional".to_string(),
                tipo: "sesion_institucional".to_string(),
                coste_temporal: 4,
                prioridad: 85,
                modificador_perfil: match protagonista.perfil.oficio {
                    crate::config::personajes::Oficio::JuristaAbogado => 1.5,
                    crate::config::personajes::Oficio::PeriodistaPublicista => 0.8,
                    _ => 1.0,
                },
            });
        }

        eventos.push(EventoDisponibleDto {
            evento_id: "rumor_en_cafe".to_string(),
            titulo: "Rumor en el cafe".to_string(),
            tipo: "encuentro_urbano".to_string(),
            coste_temporal: 2,
            prioridad: 60,
            modificador_perfil: 1.0,
        });

        if mundo.jornada_absoluta % 10 == 0 {
            eventos.push(EventoDisponibleDto {
                evento_id: "crisis_abastecimiento".to_string(),
                titulo: "Crisis de abastecimiento".to_string(),
                tipo: "crisis_personal".to_string(),
                coste_temporal: 3,
                prioridad: 90,
                modificador_perfil: 1.0,
            });
        }

        if mundo.jornada_absoluta >= 120 {
            eventos.push(EventoDisponibleDto {
                evento_id: "publicar_articulo".to_string(),
                titulo: "Publicar articulo en la prensa".to_string(),
                tipo: "publicacion_prensa".to_string(),
                coste_temporal: 3,
                prioridad: 70,
                modificador_perfil: match protagonista.perfil.oficio {
                    crate::config::personajes::Oficio::PeriodistaPublicista => 1.5,
                    _ => 0.5,
                },
            });
        }

        eventos
    }
}