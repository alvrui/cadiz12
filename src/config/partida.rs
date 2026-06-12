use serde::{Serialize, Deserialize};
use super::{espacios::EspacioId, validacion::ContextoHistorico};

/// Preferencias de juego del jugador
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferenciasJuego {
    /// Dificultad (Facil, Normal, Dificil, Experto)
    pub dificultad: Dificultad,
    /// Velocidad de juego (Lento, Normal, Rapido)
    pub velocidad: VelocidadJuego,
    /// Estilo narrativo preferido
    pub estilo_narrativo: EstiloNarrativo,
    /// Mostrar tutoriales
    pub mostrar_tutoriales: bool,
    /// Sonido habilitado
    pub sonido_habilitado: bool,
    /// Animaciones habilitadas
    pub animaciones_habilitadas: bool,
}

impl Default for PreferenciasJuego {
    fn default() -> Self {
        Self {
            dificultad: Dificultad::Normal,
            velocidad: VelocidadJuego::Normal,
            estilo_narrativo: EstiloNarrativo::Equilibrado,
            mostrar_tutoriales: true,
            sonido_habilitado: true,
            animaciones_habilitadas: true,
        }
    }
}

/// Nivel de dificultad
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Dificultad {
    Facil,
    Normal,
    Dificil,
    Experto,
}

impl Default for Dificultad {
    fn default() -> Self {
        Self::Normal
    }
}

/// Velocidad de juego
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VelocidadJuego {
    Lento,
    Normal,
    Rapido,
}

impl Default for VelocidadJuego {
    fn default() -> Self {
        Self::Normal
    }
}

/// Estilo narrativo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EstiloNarrativo {
    /// Narrativa detallada y descriptiva
    Detallado,
    /// Narrativa equilibrada
    Equilibrado,
    /// Narrativa concisa y directa
    Conciso,
}

impl Default for EstiloNarrativo {
    fn default() -> Self {
        Self::Equilibrado
    }
}

/// Configuracion de progresion del jugador
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfiguracionProgresion {
    /// Experiencia requerida por nivel
    pub experiencia_por_nivel: Vec<u32>,
    /// Nivel inicial
    pub nivel_inicial: u32,
    /// Multiplicador de experiencia
    pub multiplicador_experiencia: f32,
    /// Desbloquear todo desde el inicio
    pub desbloquear_todo: bool,
    /// Logros habilitados
    pub logros_habilitados: bool,
}

impl Default for ConfiguracionProgresion {
    fn default() -> Self {
        Self {
            experiencia_por_nivel: vec![100, 250, 500, 1000, 2000, 4000, 8000, 16000, 32000, 64000],
            nivel_inicial: 1,
            multiplicador_experiencia: 1.0,
            desbloquear_todo: false,
            logros_habilitados: true,
        }
    }
}

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
    /// Preferencias de juego del jugador
    #[serde(default)]
    pub preferencias: PreferenciasJuego,
    /// Configuracion de progresion
    #[serde(default)]
    pub progresion: ConfiguracionProgresion,
    /// Contexto historico inicial
    #[serde(default)]
    pub contexto_inicial: ContextoHistorico,
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
            preferencias: PreferenciasJuego::default(),
            progresion: ConfiguracionProgresion::default(),
            contexto_inicial: ContextoHistorico::default(),
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
