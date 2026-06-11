use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::config::personajes::*;

/// Estado completo del protagonista (M2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstadoProtagonista {
    /// Perfil fijo desde creacion
    pub perfil: crate::config::personajes::PerfilProtagonista,
    /// Posicion y visibilidad dinamica
    pub posicion: PosicionDinamica,
    /// Los 6 medidores centrales
    pub medidores: Medidores,
    /// Reputaciones segmentadas por grupo
    pub reputaciones: HashMap<String, Reputacion>,
    /// Red de relaciones con NPCs
    pub relaciones: HashMap<String, Relacion>,
    /// Memoria de decisiones tomadas
    pub historial: Vec<Decision>,
}

/// Posicion dinamica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosicionDinamica {
    pub formal_id: String,
    pub visibilidad: Visibilidad,
    pub trayectoria_moral: TrayectoriaMoral,
}

/// Visibilidad publica
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Visibilidad {
    Desconocido,
    Emergente,
    FiguraReconocible,
    MuyExpuesto,
}

impl Default for Visibilidad {
    fn default() -> Self {
        Self::Desconocido
    }
}

impl std::fmt::Display for Visibilidad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibilidad::Desconocido => write!(f, "desconocido"),
            Visibilidad::Emergente => write!(f, "emergente"),
            Visibilidad::FiguraReconocible => write!(f, "figura_reconocible"),
            Visibilidad::MuyExpuesto => write!(f, "muy_expuesto"),
        }
    }
}

/// Trayectoria moral acumulada
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrayectoriaMoral {
    Oportunista,
    Coherente,
    Ambiguo,
    Fiable,
    Temido,
    Imprescindible,
}

impl Default for TrayectoriaMoral {
    fn default() -> Self {
        Self::Ambiguo
    }
}

impl std::fmt::Display for TrayectoriaMoral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrayectoriaMoral::Oportunista => write!(f, "oportunista"),
            TrayectoriaMoral::Coherente => write!(f, "coherente"),
            TrayectoriaMoral::Ambiguo => write!(f, "ambiguo"),
            TrayectoriaMoral::Fiable => write!(f, "fiable"),
            TrayectoriaMoral::Temido => write!(f, "temido"),
            TrayectoriaMoral::Imprescindible => write!(f, "imprescindible"),
        }
    }
}

/// Los 6 medidores con su estado completo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Medidores {
    pub influencia: Medidor,
    pub relacional: Medidor,
    pub reputacion: Medidor,
    pub coherencia: Medidor,
    pub recursos: Medidor,
    pub aguante: Medidor,
}

/// Un medidor individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Medidor {
    pub valor: u8,
    pub tendencia: i8,
    pub umbral_bajo: u8,
    pub umbral_alto: u8,
}

impl Medidor {
    pub fn nuevo(valor: u8, umbral_bajo: u8, umbral_alto: u8) -> Self {
        Self {
            valor,
            tendencia: 0,
            umbral_bajo,
            umbral_alto,
        }
    }

    /// Aplicar delta al valor
    pub fn aplicar_delta(&mut self, delta: i16) {
        let nuevo_valor = (self.valor as i16) + delta;
        self.valor = nuevo_valor.clamp(0, 100) as u8;

        if delta > 0 {
            self.tendencia = (self.tendencia + 1).min(3);
        } else if delta < 0 {
            self.tendencia = (self.tendencia - 1).max(-3);
        }
    }
}

/// Reputacion segmentada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reputacion {
    pub valor: u8,
    pub tendencia: i8,
}

impl Reputacion {
    pub fn nueva(valor: u8) -> Self {
        Self { valor, tendencia: 0 }
    }
}

/// Relacion con un NPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relacion {
    pub estado: String,
    pub confianza: u8,
    pub deuda: i16,
    pub cooldown: u8,
}

impl Relacion {
    pub fn nueva(confianza: u8) -> Self {
        Self {
            estado: "neutral".to_string(),
            confianza,
            deuda: 0,
            cooldown: 0,
        }
    }
}

/// Decision tomada (para historial)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub jornada: u32,
    pub evento_id: String,
    pub opcion_id: String,
    pub consecuencias: HashMap<String, i16>,
}

impl EstadoProtagonista {
    /// Crear estado inicial desde perfil
    pub fn nuevo(perfil: PerfilProtagonista, config: &crate::config::PartidaConfig) -> Self {
        let medidores = Medidores {
            influencia: Medidor::nuevo(50, config.medidores.influencia.umbral_bajo, config.medidores.influencia.umbral_alto),
            relacional: Medidor::nuevo(50, config.medidores.relacional.umbral_bajo, config.medidores.relacional.umbral_alto),
            reputacion: Medidor::nuevo(50, config.medidores.reputacion.umbral_bajo, config.medidores.reputacion.umbral_alto),
            coherencia: Medidor::nuevo(50, config.medidores.coherencia.umbral_bajo, config.medidores.coherencia.umbral_alto),
            recursos: Medidor::nuevo(50, config.medidores.recursos.umbral_bajo, config.medidores.recursos.umbral_alto),
            aguante: Medidor::nuevo(50, config.medidores.aguante.umbral_bajo, config.medidores.aguante.umbral_alto),
        };

        let mut relaciones = HashMap::new();
        for (npc_id, npc_config) in &config.personajes.npcs {
            relaciones.insert(npc_id.clone(), Relacion::nueva(npc_config.confianza_inicial));
        }

        let formal_id = match perfil.oficio {
            crate::config::personajes::Oficio::JuristaAbogado => "diputado".to_string(),
            crate::config::personajes::Oficio::PeriodistaPublicista => "periodista".to_string(),
            crate::config::personajes::Oficio::ComercianteAgenteMercantil => "comerciante".to_string(),
            crate::config::personajes::Oficio::ClerigoIlustrado => "clerigo".to_string(),
            crate::config::personajes::Oficio::OficialEjercito => "militar".to_string(),
            crate::config::personajes::Oficio::MedicoIlustrado => "medico".to_string(),
        };

        Self {
            perfil,
            posicion: PosicionDinamica {
                formal_id,
                visibilidad: Visibilidad::Desconocido,
                trayectoria_moral: TrayectoriaMoral::Ambiguo,
            },
            medidores,
            reputaciones: HashMap::new(),
            relaciones,
            historial: Vec::new(),
        }
    }

    /// Obtener DTO para la interfaz
    pub fn a_protagonista_dto(&self) -> super::dtos::ProtagonistaDto {
        super::dtos::ProtagonistaDto {
            posicion_formal_id: self.posicion.formal_id.clone(),
            visibilidad: self.posicion.visibilidad.to_string(),
            medidores: vec![
                super::dtos::MedidorResumenDto {
                    nombre: "influencia".to_string(),
                    valor: self.medidores.influencia.valor,
                    tendencia: self.medidores.influencia.tendencia,
                    umbral_bajo: self.medidores.influencia.umbral_bajo,
                    umbral_alto: self.medidores.influencia.umbral_alto,
                },
                super::dtos::MedidorResumenDto {
                    nombre: "relacional".to_string(),
                    valor: self.medidores.relacional.valor,
                    tendencia: self.medidores.relacional.tendencia,
                    umbral_bajo: self.medidores.relacional.umbral_bajo,
                    umbral_alto: self.medidores.relacional.umbral_alto,
                },
                super::dtos::MedidorResumenDto {
                    nombre: "reputacion".to_string(),
                    valor: self.medidores.reputacion.valor,
                    tendencia: self.medidores.reputacion.tendencia,
                    umbral_bajo: self.medidores.reputacion.umbral_bajo,
                    umbral_alto: self.medidores.reputacion.umbral_alto,
                },
                super::dtos::MedidorResumenDto {
                    nombre: "coherencia".to_string(),
                    valor: self.medidores.coherencia.valor,
                    tendencia: self.medidores.coherencia.tendencia,
                    umbral_bajo: self.medidores.coherencia.umbral_bajo,
                    umbral_alto: self.medidores.coherencia.umbral_alto,
                },
                super::dtos::MedidorResumenDto {
                    nombre: "recursos".to_string(),
                    valor: self.medidores.recursos.valor,
                    tendencia: self.medidores.recursos.tendencia,
                    umbral_bajo: self.medidores.recursos.umbral_bajo,
                    umbral_alto: self.medidores.recursos.umbral_alto,
                },
                super::dtos::MedidorResumenDto {
                    nombre: "aguante".to_string(),
                    valor: self.medidores.aguante.valor,
                    tendencia: self.medidores.aguante.tendencia,
                    umbral_bajo: self.medidores.aguante.umbral_bajo,
                    umbral_alto: self.medidores.aguante.umbral_alto,
                },
            ],
        }
    }
}