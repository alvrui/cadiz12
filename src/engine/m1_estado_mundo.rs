use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::config::{
    facciones::FaccionId,
    espacios::EspacioId,
};

/// Estado global del mundo (M1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstadoMundo {
    /// Identificador del tramo narrativo actual
    pub tramo_id: String,
    /// Fase del arco principal (1-5)
    pub acto_narrativo: u32,
    /// Contador absoluto de jornadas
    pub jornada_absoluta: u32,
    /// Jornadas hasta el proximo pivote historico
    pub distancia_pivote_proximo: i32,
    /// Estado global del tablero
    pub estado_global: EstadoGlobal,
    /// Grado de polarizacion (1-5)
    pub polarizacion: u8,
    /// Atencion publica concentrada (1-5)
    pub visibilidad_tablero: u8,
    /// Tema dominante actual
    pub tema_caliente_id: Option<String>,
    /// Estado de cada faccion
    pub facciones: HashMap<FaccionId, EstadoFaccion>,
    /// Estado de cada espacio
    pub espacios: HashMap<EspacioId, EstadoEspacio>,
    /// Crisis activa (si la hay)
    pub crisis: Option<CrisisActiva>,
}

/// Estados globales posibles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EstadoGlobal {
    NormalidadTensa,
    PreCrisis,
    CrisisAbierta,
    ResacaCrisis,
    CelebracionPublica,
    RepresionLatente,
}

impl Default for EstadoGlobal {
    fn default() -> Self {
        Self::NormalidadTensa
    }
}

/// Estado de una faccion en el mundo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstadoFaccion {
    pub fuerza: u8,
    pub necesidad_del_jugador: u8,
    pub cohesion_interna: u8,
    pub linea_roja_activa: bool,
    pub vigilancia_sobre_jugador: u8,
}

/// Estado de un espacio en el mundo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstadoEspacio {
    pub disponible: bool,
    pub nivel_riesgo: u8,
    pub clima: ClimaEspacio,
    pub npcs_presentes: Vec<String>,
    pub coste_temporal: u8,
}

/// Clima de un espacio
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClimaEspacio {
    Tranquilo,
    Saturado,
    Nervioso,
    Vigilado,
    Vacio,
    Efervescente,
}

impl Default for ClimaEspacio {
    fn default() -> Self {
        Self::Tranquilo
    }
}

/// Crisis activa en el tablero
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrisisActiva {
    pub tipo_id: String,
    pub fase: FaseCrisis,
    pub tablero_permeable: bool,
    pub ventana_activa: bool,
}

/// Fases de una crisis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FaseCrisis {
    Senal,
    Estallido,
    PeriodoReaccion,
    Resolucion,
    Resaca,
}

impl Default for FaseCrisis {
    fn default() -> Self {
        Self::Senal
    }
}

impl std::fmt::Display for FaseCrisis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FaseCrisis::Senal => write!(f, "senal"),
            FaseCrisis::Estallido => write!(f, "estallido"),
            FaseCrisis::PeriodoReaccion => write!(f, "periodo_reaccion"),
            FaseCrisis::Resolucion => write!(f, "resolucion"),
            FaseCrisis::Resaca => write!(f, "resaca"),
        }
    }
}

impl EstadoMundo {
    /// Crear estado inicial desde configuracion
    pub fn nuevo(config: &crate::config::PartidaConfig) -> Self {
        let mut facciones = HashMap::new();
        for (id, fac_config) in &config.facciones.facciones {
            facciones.insert(
                id.clone(),
                EstadoFaccion {
                    fuerza: fac_config.estado_inicial.fuerza,
                    necesidad_del_jugador: fac_config.estado_inicial.necesidad_del_jugador,
                    cohesion_interna: fac_config.estado_inicial.cohesion_interna,
                    linea_roja_activa: fac_config.estado_inicial.linea_roja_activa,
                    vigilancia_sobre_jugador: fac_config.estado_inicial.vigilancia_sobre_jugador,
                },
            );
        }

        let mut espacios = HashMap::new();
        for (id, esp_config) in &config.espacios.espacios {
            let clima = match esp_config.clima_inicial {
                crate::config::espacios::ClimaEspacio::Tranquilo => ClimaEspacio::Tranquilo,
                crate::config::espacios::ClimaEspacio::Saturado => ClimaEspacio::Saturado,
                crate::config::espacios::ClimaEspacio::Nervioso => ClimaEspacio::Nervioso,
                crate::config::espacios::ClimaEspacio::Vigilado => ClimaEspacio::Vigilado,
                crate::config::espacios::ClimaEspacio::Vacio => ClimaEspacio::Vacio,
                crate::config::espacios::ClimaEspacio::Efervescente => ClimaEspacio::Efervescente,
            };
            espacios.insert(
                id.clone(),
                EstadoEspacio {
                    disponible: true,
                    nivel_riesgo: esp_config.nivel_riesgo,
                    clima,
                    npcs_presentes: esp_config.npcs_iniciales.clone(),
                    coste_temporal: esp_config.coste_temporal,
                },
            );
        }

        Self {
            tramo_id: "inicio_1810".to_string(),
            acto_narrativo: 1,
            jornada_absoluta: 0,
            distancia_pivote_proximo: 100,
            estado_global: EstadoGlobal::NormalidadTensa,
            polarizacion: 2,
            visibilidad_tablero: 3,
            tema_caliente_id: Some("inauguracion_cortes".to_string()),
            facciones,
            espacios,
            crisis: None,
        }
    }

    /// Avanzar una jornada
    pub fn avanzar_jornada(&mut self) {
        self.jornada_absoluta += 1;
        self.distancia_pivote_proximo -= 1;

        for (_, espacio) in self.espacios.iter_mut() {
            espacio.clima = match espacio.clima {
                ClimaEspacio::Tranquilo => ClimaEspacio::Nervioso,
                ClimaEspacio::Nervioso => ClimaEspacio::Tranquilo,
                ClimaEspacio::Saturado => ClimaEspacio::Vigilado,
                ClimaEspacio::Vigilado => ClimaEspacio::Saturado,
                _ => ClimaEspacio::Tranquilo,
            };
        }
    }
}