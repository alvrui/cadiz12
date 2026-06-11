use serde::{Serialize, Deserialize};
use super::{facciones::FaccionId, espacios::EspacioId, personajes::*, eventos::*};

/// Configuracion completa de una partida
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartidaConfig {
    /// Nombre de la partida
    pub nombre: String,
    /// Descripcion
    pub descripcion: String,
    /// Periodo historico (1810-1814)
    pub periodo_inicio: String,
    pub periodo_fin: String,
    /// Jornadas totales estimadas
    pub jornadas_totales: u32,
    /// Configuracion de medidores
    pub medidores: super::medidores::MedidoresConfig,
    /// Configuracion de facciones
    pub facciones: super::facciones::FaccionesConfig,
    /// Configuracion de espacios
    pub espacios: super::espacios::EspaciosConfig,
    /// Configuracion de personajes/NPCs
    pub personajes: super::personajes::PersonajesConfig,
    /// Configuracion de eventos
    pub eventos: super::eventos::EventosConfig,
    /// Perfil inicial del protagonista
    pub perfil_inicial: super::personajes::PerfilProtagonista,
    /// Espacio inicial
    pub espacio_inicial: EspacioId,
    /// Presupuesto temporal por jornada
    pub presupuesto_temporal: u8,
}

impl Default for PartidaConfig {
    fn default() -> Self {
        Self {
            nombre: "Partida Basica".to_string(),
            descripcion: "Configuracion basica para testing".to_string(),
            periodo_inicio: "1810-09-24".to_string(),
            periodo_fin: "1814-05-01".to_string(),
            jornadas_totales: 500,
            medidores: super::medidores::MedidoresConfig::default(),
            facciones: super::facciones::FaccionesConfig::default(),
            espacios: super::espacios::EspaciosConfig::default(),
            personajes: super::personajes::PersonajesConfig::default(),
            eventos: super::eventos::EventosConfig::default(),
            perfil_inicial: super::personajes::PerfilProtagonista {
                origen: super::personajes::Origen::Peninsular,
                clase_social: super::personajes::ClaseSocial::HidalguiaProfesionLetrada,
                oficio: super::personajes::Oficio::JuristaAbogado,
                adscripcion: super::personajes::AdscripcionPolitica::LiberalProgresista,
                temperamento: super::personajes::Temperamento::Prudente,
                compromisos: vec![
                    super::personajes::Compromiso {
                        id: "deuda_consulado".to_string(),
                        tipo: "material".to_string(),
                        descripcion: "Deuda con el Consulado de Cadiz".to_string(),
                        impacto: "Presion economica constante".to_string(),
                    }
                ],
            },
            espacio_inicial: super::espacios::EspacioId::CafeApolo,
            presupuesto_temporal: 8,
        }
    }
}

/// Cargador de configuracion
impl PartidaConfig {
    pub fn cargar_desde_json(ruta: &str) -> anyhow::Result<Self> {
        let contenido = std::fs::read_to_string(ruta)?;
        let config: Self = serde_json::from_str(&contenido)?;
        Ok(config)
    }

    pub fn guardar_como_json(&self, ruta: &str) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(ruta, json)?;
        Ok(())
    }
}