use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Identificadores de facciones políticas
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FaccionId {
    LiberalProgresista,
    LiberalModerado,
    AbsolutistaServil,
    IndependienteOportunista,
    Americanista,
}

impl std::fmt::Display for FaccionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FaccionId::LiberalProgresista => write!(f, "liberal_progresista"),
            FaccionId::LiberalModerado => write!(f, "liberal_moderado"),
            FaccionId::AbsolutistaServil => write!(f, "absolutista_servil"),
            FaccionId::IndependienteOportunista => write!(f, "independiente_oportunista"),
            FaccionId::Americanista => write!(f, "americanista"),
        }
    }
}

impl serde::Serialize for FaccionId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for FaccionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "liberal_progresista" => Ok(FaccionId::LiberalProgresista),
            "liberal_moderado" => Ok(FaccionId::LiberalModerado),
            "absolutista_servil" => Ok(FaccionId::AbsolutistaServil),
            "independiente_oportunista" => Ok(FaccionId::IndependienteOportunista),
            "americanista" => Ok(FaccionId::Americanista),
            _ => Err(serde::de::Error::custom(format!("Faccion desconocida: {}", s))),
        }
    }
}

/// Estado dinamico de una faccion en el juego
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaccionEstado {
    /// Fuerza relativa (1-5)
    pub fuerza: u8,
    /// Cuanto necesita al jugador (1-3)
    pub necesidad_del_jugador: u8,
    /// Cohesion interna (1-3)
    pub cohesion_interna: u8,
    /// Si la linea roja esta activa
    pub linea_roja_activa: bool,
    /// Vigilancia sobre el jugador (1-3)
    pub vigilancia_sobre_jugador: u8,
}

/// Personajes historicos clave de una faccion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaccionLider {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub influencia: u8,
}

/// Configuracion de una faccion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaccionConfig {
    pub id: FaccionId,
    pub nombre: String,
    pub descripcion: String,
    pub lideres: Vec<FaccionLider>,
    pub estado_inicial: FaccionEstado,
    /// Temas que preocupan a esta faccion
    pub temas_sensibles: Vec<String>,
    /// Alianzas naturales
    pub alianzas: Vec<FaccionId>,
    /// Conflictos estructurales
    pub conflictos: Vec<FaccionId>,
}

/// Coleccion de todas las facciones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaccionesConfig {
    pub facciones: HashMap<FaccionId, FaccionConfig>,
}

impl Default for FaccionesConfig {
    fn default() -> Self {
        let mut facciones = HashMap::new();

        facciones.insert(FaccionId::LiberalProgresista, FaccionConfig {
            id: FaccionId::LiberalProgresista,
            nombre: "Liberales Progresistas".to_string(),
            descripcion: "Partidarios de reformas profundas y rapidas".to_string(),
            lideres: vec![
                FaccionLider { id: "arguelles".to_string(), nombre: "Agustin Arguelles".to_string(), descripcion: "Lider intelectual".to_string(), influencia: 95 },
                FaccionLider { id: "munoz_torrero".to_string(), nombre: "Diego Munoz Torrero".to_string(), descripcion: "Estratega".to_string(), influencia: 90 },
            ],
            estado_inicial: FaccionEstado {
                fuerza: 4,
                necesidad_del_jugador: 2,
                cohesion_interna: 3,
                linea_roja_activa: false,
                vigilancia_sobre_jugador: 1,
            },
            temas_sensibles: vec!["soberania_nacional".to_string(), "libertades_individuales".to_string()],
            alianzas: vec![FaccionId::LiberalModerado, FaccionId::Americanista],
            conflictos: vec![FaccionId::AbsolutistaServil],
        });

        facciones.insert(FaccionId::LiberalModerado, FaccionConfig {
            id: FaccionId::LiberalModerado,
            nombre: "Liberales Moderados".to_string(),
            descripcion: "Reformistas graduales".to_string(),
            lideres: vec![
                FaccionLider { id: "martinez_rosa".to_string(), nombre: "Francisco de Paula Martinez de la Rosa".to_string(), descripcion: "Mediador".to_string(), influencia: 90 },
            ],
            estado_inicial: FaccionEstado {
                fuerza: 3,
                necesidad_del_jugador: 3,
                cohesion_interna: 2,
                linea_roja_activa: false,
                vigilancia_sobre_jugador: 2,
            },
            temas_sensibles: vec!["equilibrio_poder".to_string(), "estabilidad".to_string()],
            alianzas: vec![FaccionId::LiberalProgresista, FaccionId::AbsolutistaServil],
            conflictos: vec![FaccionId::IndependienteOportunista],
        });

        facciones.insert(FaccionId::AbsolutistaServil, FaccionConfig {
            id: FaccionId::AbsolutistaServil,
            nombre: "Absolutistas/Serviles".to_string(),
            descripcion: "Defensores del Antiguo Regimen".to_string(),
            lideres: vec![
                FaccionLider { id: "ostolaza".to_string(), nombre: "Fernando Ostolaza".to_string(), descripcion: "Conservador".to_string(), influencia: 85 },
            ],
            estado_inicial: FaccionEstado {
                fuerza: 2,
                necesidad_del_jugador: 1,
                cohesion_interna: 3,
                linea_roja_activa: true,
                vigilancia_sobre_jugador: 3,
            },
            temas_sensibles: vec!["tradicion".to_string(), "autoridad_real".to_string()],
            alianzas: vec![FaccionId::LiberalModerado],
            conflictos: vec![FaccionId::LiberalProgresista, FaccionId::Americanista],
        });

        Self { facciones }
    }
}