use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use super::facciones::FaccionId;

/// Origen geografico del personaje
#[derive(Debug, Clone, PartialEq)]
pub enum Origen {
    Gaditano,
    Peninsular,
    AmericanoVirreinal,
}

impl std::fmt::Display for Origen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Origen::Gaditano => write!(f, "gaditano"),
            Origen::Peninsular => write!(f, "peninsular"),
            Origen::AmericanoVirreinal => write!(f, "americano_virreinal"),
        }
    }
}

impl serde::Serialize for Origen {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            Origen::Gaditano => "gaditano",
            Origen::Peninsular => "peninsular",
            Origen::AmericanoVirreinal => "americano_virreinal",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> serde::Deserialize<'de> for Origen {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "gaditano" => Ok(Origen::Gaditano),
            "peninsular" => Ok(Origen::Peninsular),
            "americano_virreinal" => Ok(Origen::AmericanoVirreinal),
            _ => Err(serde::de::Error::custom(format!("Origen desconocido: {}", s))),
        }
    }
}

/// Clase social
#[derive(Debug, Clone, PartialEq)]
pub enum ClaseSocial {
    EliteMercantilGaditana,
    HidalguiaProfesionLetrada,
    CleroIlustrado,
    CarreraMilitar,
    EliteCriollaAmericana,
    FuncionariadoIlustrado,
}

impl std::fmt::Display for ClaseSocial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClaseSocial::EliteMercantilGaditana => write!(f, "elite_mercantil_gaditana"),
            ClaseSocial::HidalguiaProfesionLetrada => write!(f, "hidalguia_profesion_letrada"),
            ClaseSocial::CleroIlustrado => write!(f, "clero_ilustrado"),
            ClaseSocial::CarreraMilitar => write!(f, "carrera_militar"),
            ClaseSocial::EliteCriollaAmericana => write!(f, "elite_criolla_americana"),
            ClaseSocial::FuncionariadoIlustrado => write!(f, "funcionariado_ilustrado"),
        }
    }
}

impl serde::Serialize for ClaseSocial {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            ClaseSocial::EliteMercantilGaditana => "elite_mercantil_gaditana",
            ClaseSocial::HidalguiaProfesionLetrada => "hidalguia_profesion_letrada",
            ClaseSocial::CleroIlustrado => "clero_ilustrado",
            ClaseSocial::CarreraMilitar => "carrera_militar",
            ClaseSocial::EliteCriollaAmericana => "elite_criolla_americana",
            ClaseSocial::FuncionariadoIlustrado => "funcionariado_ilustrado",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> serde::Deserialize<'de> for ClaseSocial {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "elite_mercantil_gaditana" => Ok(ClaseSocial::EliteMercantilGaditana),
            "hidalguia_profesion_letrada" => Ok(ClaseSocial::HidalguiaProfesionLetrada),
            "clero_ilustrado" => Ok(ClaseSocial::CleroIlustrado),
            "carrera_militar" => Ok(ClaseSocial::CarreraMilitar),
            "elite_criolla_americana" => Ok(ClaseSocial::EliteCriollaAmericana),
            "funcionariado_ilustrado" => Ok(ClaseSocial::FuncionariadoIlustrado),
            _ => Err(serde::de::Error::custom(format!("Clase social desconocida: {}", s))),
        }
    }
}

/// Oficio previo
#[derive(Debug, Clone, PartialEq)]
pub enum Oficio {
    JuristaAbogado,
    PeriodistaPublicista,
    ComercianteAgenteMercantil,
    OficialEjercito,
    ClerigoIlustrado,
    MedicoIlustrado,
}

impl std::fmt::Display for Oficio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Oficio::JuristaAbogado => write!(f, "jurista_abogado"),
            Oficio::PeriodistaPublicista => write!(f, "periodista_publicista"),
            Oficio::ComercianteAgenteMercantil => write!(f, "comerciante_agente_mercantil"),
            Oficio::OficialEjercito => write!(f, "oficial_ejercito"),
            Oficio::ClerigoIlustrado => write!(f, "clerigo_ilustrado"),
            Oficio::MedicoIlustrado => write!(f, "medico_ilustrado"),
        }
    }
}

impl serde::Serialize for Oficio {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            Oficio::JuristaAbogado => "jurista_abogado",
            Oficio::PeriodistaPublicista => "periodista_publicista",
            Oficio::ComercianteAgenteMercantil => "comerciante_agente_mercantil",
            Oficio::OficialEjercito => "oficial_ejercito",
            Oficio::ClerigoIlustrado => "clerigo_ilustrado",
            Oficio::MedicoIlustrado => "medico_ilustrado",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> serde::Deserialize<'de> for Oficio {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "jurista_abogado" => Ok(Oficio::JuristaAbogado),
            "periodista_publicista" => Ok(Oficio::PeriodistaPublicista),
            "comerciante_agente_mercantil" => Ok(Oficio::ComercianteAgenteMercantil),
            "oficial_ejercito" => Ok(Oficio::OficialEjercito),
            "clerigo_ilustrado" => Ok(Oficio::ClerigoIlustrado),
            "medico_ilustrado" => Ok(Oficio::MedicoIlustrado),
            _ => Err(serde::de::Error::custom(format!("Oficio desconocido: {}", s))),
        }
    }
}

/// Temperamento
#[derive(Debug, Clone, PartialEq)]
pub enum Temperamento {
    Retorico,
    Prudente,
    Ambicioso,
    Esceptico,
    Idealista,
    Pragmatico,
}

impl std::fmt::Display for Temperamento {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Temperamento::Retorico => write!(f, "retorico"),
            Temperamento::Prudente => write!(f, "prudente"),
            Temperamento::Ambicioso => write!(f, "ambicioso"),
            Temperamento::Esceptico => write!(f, "esceptico"),
            Temperamento::Idealista => write!(f, "idealista"),
            Temperamento::Pragmatico => write!(f, "pragmatico"),
        }
    }
}

impl serde::Serialize for Temperamento {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            Temperamento::Retorico => "retorico",
            Temperamento::Prudente => "prudente",
            Temperamento::Ambicioso => "ambicioso",
            Temperamento::Esceptico => "esceptico",
            Temperamento::Idealista => "idealista",
            Temperamento::Pragmatico => "pragmatico",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> serde::Deserialize<'de> for Temperamento {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "retorico" => Ok(Temperamento::Retorico),
            "prudente" => Ok(Temperamento::Prudente),
            "ambicioso" => Ok(Temperamento::Ambicioso),
            "esceptico" => Ok(Temperamento::Esceptico),
            "idealista" => Ok(Temperamento::Idealista),
            "pragmatico" => Ok(Temperamento::Pragmatico),
            _ => Err(serde::de::Error::custom(format!("Temperamento desconocido: {}", s))),
        }
    }
}

/// Adscripcion politica
#[derive(Debug, Clone, PartialEq)]
pub enum AdscripcionPolitica {
    LiberalProgresista,
    LiberalModerado,
    AbsolutistaServil,
    IndependienteOportunista,
    Americanista,
}

impl std::fmt::Display for AdscripcionPolitica {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdscripcionPolitica::LiberalProgresista => write!(f, "liberal_progresista"),
            AdscripcionPolitica::LiberalModerado => write!(f, "liberal_moderado"),
            AdscripcionPolitica::AbsolutistaServil => write!(f, "absolutista_servil"),
            AdscripcionPolitica::IndependienteOportunista => write!(f, "independiente_oportunista"),
            AdscripcionPolitica::Americanista => write!(f, "americanista"),
        }
    }
}

impl serde::Serialize for AdscripcionPolitica {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            AdscripcionPolitica::LiberalProgresista => "liberal_progresista",
            AdscripcionPolitica::LiberalModerado => "liberal_moderado",
            AdscripcionPolitica::AbsolutistaServil => "absolutista_servil",
            AdscripcionPolitica::IndependienteOportunista => "independiente_oportunista",
            AdscripcionPolitica::Americanista => "americanista",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> serde::Deserialize<'de> for AdscripcionPolitica {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "liberal_progresista" => Ok(AdscripcionPolitica::LiberalProgresista),
            "liberal_moderado" => Ok(AdscripcionPolitica::LiberalModerado),
            "absolutista_servil" => Ok(AdscripcionPolitica::AbsolutistaServil),
            "independiente_oportunista" => Ok(AdscripcionPolitica::IndependienteOportunista),
            "americanista" => Ok(AdscripcionPolitica::Americanista),
            _ => Err(serde::de::Error::custom(format!("Adscripcion politica desconocida: {}", s))),
        }
    }
}

/// Compromiso personal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compromiso {
    pub id: String,
    pub tipo: String,
    pub descripcion: String,
    pub impacto: String,
}

/// Perfil del protagonista (fijo desde creacion)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfilProtagonista {
    pub origen: Origen,
    pub clase_social: ClaseSocial,
    pub oficio: Oficio,
    pub adscripcion: AdscripcionPolitica,
    pub temperamento: Temperamento,
    pub compromisos: Vec<Compromiso>,
}

/// Habilidad narrativa de partida
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabilidadNarrativa {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub escenas_determinantes: Vec<String>,
}

/// Configuracion de un NPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcConfig {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub faccion: FaccionId,
    pub origen: Origen,
    pub clase_social: ClaseSocial,
    pub oficio: Oficio,
    pub adscripcion: AdscripcionPolitica,
    pub temperamento: Temperamento,
    /// Relacion inicial con el jugador (-100 a 100)
    pub relacion_inicial: i16,
    /// Confianza inicial (0-100)
    pub confianza_inicial: u8,
    /// Deuda inicial (puede ser negativa = favor)
    pub deuda_inicial: i16,
    /// Habilidades narrativas
    pub habilidades: Vec<HabilidadNarrativa>,
    /// Espacios donde aparece
    pub espacios: Vec<String>,
}

/// Coleccion de todos los NPCs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonajesConfig {
    pub npcs: HashMap<String, NpcConfig>,
}

impl Default for PersonajesConfig {
    fn default() -> Self {
        let mut npcs = HashMap::new();

        npcs.insert("arguelles".to_string(), NpcConfig {
            id: "arguelles".to_string(),
            nombre: "Agustin Arguelles".to_string(),
            descripcion: "Lider liberal progresista, jurista brillante".to_string(),
            faccion: FaccionId::LiberalProgresista,
            origen: Origen::Peninsular,
            clase_social: ClaseSocial::HidalguiaProfesionLetrada,
            oficio: Oficio::JuristaAbogado,
            adscripcion: AdscripcionPolitica::LiberalProgresista,
            temperamento: Temperamento::Retorico,
            relacion_inicial: 0,
            confianza_inicial: 30,
            deuda_inicial: 0,
            habilidades: vec![
                HabilidadNarrativa {
                    id: "lectura_tecnica".to_string(),
                    nombre: "Lectura tecnica de borradores".to_string(),
                    descripcion: "Puede analizar textos legales complejos".to_string(),
                    escenas_determinantes: vec!["comisiones".to_string()],
                }
            ],
            espacios: vec!["oratorio_san_felipe_neri".to_string(), "comisiones_parlamentarias".to_string()],
        });

        Self { npcs }
    }
}
