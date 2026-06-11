use super::{
    m5_bucle_jornada::BucleJornada,
    m1_estado_mundo::EstadoMundo,
    m2_estado_protagonista::EstadoProtagonista,
    dtos::*,
};
use crate::config::PartidaConfig;

/// API del motor (M7)
#[derive(Debug, Clone)]
pub struct MotorApi {
    mundo: EstadoMundo,
    protagonista: EstadoProtagonista,
    bucle: BucleJornada,
    config: PartidaConfig,
}

impl MotorApi {
    pub fn nuevo(config: PartidaConfig) -> Self {
        let mundo = EstadoMundo::nuevo(&config);
        let protagonista = EstadoProtagonista::nuevo(config.perfil_inicial.clone(), &config);

        Self {
            mundo,
            protagonista,
            bucle: BucleJornada::nuevo(config.eventos.clone(), config.medidores.clone()),
            config,
        }
    }

    /// GET /estado_jornada
    pub fn estado_jornada(&mut self) -> EstadoJornadaDto {
        self.bucle.iniciar_jornada(&mut self.mundo, &mut self.protagonista, &self.config)
    }

    /// GET /evento/{evento_id}
    pub fn obtener_evento(&self, evento_id: &str) -> Option<EventoDisponibleDto> {
        self.config.eventos.plantillas.get(evento_id).map(|plantilla| {
            EventoDisponibleDto {
                evento_id: plantilla.id.clone(),
                titulo: plantilla.titulo.clone(),
                tipo: plantilla.tipo.to_string(),
                coste_temporal: plantilla.coste_temporal,
                prioridad: plantilla.prioridad_base,
                modificador_perfil: 0.0,
            }
        })
    }

    /// POST /resolver_evento
    pub fn resolver_evento(&mut self, evento_id: &str, opcion_id: &str) -> anyhow::Result<()> {
        self.bucle.resolver_evento(&mut self.mundo, &mut self.protagonista, evento_id, opcion_id)
    }

    /// GET /estado_personaje
    pub fn estado_personaje(&self) -> EstadoProtagonistaDto {
        EstadoProtagonistaDto {
            perfil: PerfilDto {
                origen: self.protagonista.perfil.origen.to_string(),
                clase_social: self.protagonista.perfil.clase_social.to_string(),
                oficio: self.protagonista.perfil.oficio.to_string(),
                adscripcion: self.protagonista.perfil.adscripcion.to_string(),
                temperamento: self.protagonista.perfil.temperamento.to_string(),
            },
            posicion: PosicionDto {
                formal_id: self.protagonista.posicion.formal_id.clone(),
                visibilidad: self.protagonista.posicion.visibilidad.to_string(),
                trayectoria_moral: self.protagonista.posicion.trayectoria_moral.to_string(),
            },
            medidores: vec![
                MedidorDto {
                    nombre: "influencia".to_string(),
                    valor: self.protagonista.medidores.influencia.valor,
                    tendencia: self.protagonista.medidores.influencia.tendencia,
                    umbral_bajo: self.protagonista.medidores.influencia.umbral_bajo,
                    umbral_alto: self.protagonista.medidores.influencia.umbral_alto,
                },
                MedidorDto {
                    nombre: "relacional".to_string(),
                    valor: self.protagonista.medidores.relacional.valor,
                    tendencia: self.protagonista.medidores.relacional.tendencia,
                    umbral_bajo: self.protagonista.medidores.relacional.umbral_bajo,
                    umbral_alto: self.protagonista.medidores.relacional.umbral_alto,
                },
                MedidorDto {
                    nombre: "reputacion".to_string(),
                    valor: self.protagonista.medidores.reputacion.valor,
                    tendencia: self.protagonista.medidores.reputacion.tendencia,
                    umbral_bajo: self.protagonista.medidores.reputacion.umbral_bajo,
                    umbral_alto: self.protagonista.medidores.reputacion.umbral_alto,
                },
                MedidorDto {
                    nombre: "coherencia".to_string(),
                    valor: self.protagonista.medidores.coherencia.valor,
                    tendencia: self.protagonista.medidores.coherencia.tendencia,
                    umbral_bajo: self.protagonista.medidores.coherencia.umbral_bajo,
                    umbral_alto: self.protagonista.medidores.coherencia.umbral_alto,
                },
                MedidorDto {
                    nombre: "recursos".to_string(),
                    valor: self.protagonista.medidores.recursos.valor,
                    tendencia: self.protagonista.medidores.recursos.tendencia,
                    umbral_bajo: self.protagonista.medidores.recursos.umbral_bajo,
                    umbral_alto: self.protagonista.medidores.recursos.umbral_alto,
                },
                MedidorDto {
                    nombre: "aguante".to_string(),
                    valor: self.protagonista.medidores.aguante.valor,
                    tendencia: self.protagonista.medidores.aguante.tendencia,
                    umbral_bajo: self.protagonista.medidores.aguante.umbral_bajo,
                    umbral_alto: self.protagonista.medidores.aguante.umbral_alto,
                },
            ],
            reputaciones: self.protagonista.reputaciones.iter()
                .map(|(k, v)| (k.clone(), ReputacionDto { valor: v.valor, tendencia: v.tendencia }))
                .collect(),
            relaciones: self.protagonista.relaciones.iter()
                .map(|(k, v)| RelacionDto {
                    npc_id: k.clone(),
                    estado: v.estado.clone(),
                    confianza: v.confianza,
                    deuda: v.deuda,
                    cooldown: v.cooldown,
                })
                .collect(),
        }
    }
}
