use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Tipo de evento
#[derive(Debug, Clone, PartialEq)]
pub enum TipoEvento {
    /// Sesion institucional (A)
    SesionInstitucional,
    /// Encuentro urbano (B)
    EncuentroUrbano,
    /// Crisis personal (C)
    CrisisPersonal,
    /// Lectura de expediente (D)
    LecturaExpediente,
    /// Publicacion en prensa (S1)
    PublicacionPrensa,
    /// Edicion de prensa (S2)
    EdicionPrensa,
}

impl std::fmt::Display for TipoEvento {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TipoEvento::SesionInstitucional => write!(f, "sesion_institucional"),
            TipoEvento::EncuentroUrbano => write!(f, "encuentro_urbano"),
            TipoEvento::CrisisPersonal => write!(f, "crisis_personal"),
            TipoEvento::LecturaExpediente => write!(f, "lectura_expediente"),
            TipoEvento::PublicacionPrensa => write!(f, "publicacion_prensa"),
            TipoEvento::EdicionPrensa => write!(f, "edicion_prensa"),
        }
    }
}

impl serde::Serialize for TipoEvento {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for TipoEvento {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "sesion_institucional" => Ok(TipoEvento::SesionInstitucional),
            "encuentro_urbano" => Ok(TipoEvento::EncuentroUrbano),
            "crisis_personal" => Ok(TipoEvento::CrisisPersonal),
            "lectura_expediente" => Ok(TipoEvento::LecturaExpediente),
            "publicacion_prensa" => Ok(TipoEvento::PublicacionPrensa),
            "edicion_prensa" => Ok(TipoEvento::EdicionPrensa),
            _ => Err(serde::de::Error::custom(format!("Tipo de evento desconocido: {}", s))),
        }
    }
}

/// Fase de una crisis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FaseCrisis {
    Senal,
    Estallido,
    PeriodoReaccion,
    Resolucion,
    Resaca,
}

/// Evento historico fijo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventoFijo {
    pub id: String,
    pub nombre: String,
    pub fecha: String,
    pub descripcion: String,
    pub impacto: String,
    /// Jornadas desde el inicio
    pub jornada: u32,
}

/// Plantilla de evento generado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventoPlantilla {
    pub id: String,
    pub titulo: String,
    pub tipo: TipoEvento,
    pub descripcion: String,
    /// Coste temporal en unidades
    pub coste_temporal: u8,
    /// Prioridad base (0-100)
    pub prioridad_base: u8,
    /// Familias de eventos a las que pertenece
    pub familias: Vec<String>,
    /// Requerimientos para que aparezca
    pub requerimientos: HashMap<String, String>,
    /// Consecuencias posibles
    pub consecuencias: HashMap<String, i16>,
    /// Opciones disponibles
    pub opciones: Vec<OpcionEvento>,
}

/// Opcion de respuesta a un evento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpcionEvento {
    pub id: String,
    pub texto: String,
    /// Modificador de prioridad segun perfil
    pub modificador_perfil: f32,
    /// Coste temporal adicional
    pub coste_adicional: u8,
    /// Consecuencias directas
    pub consecuencias: HashMap<String, i16>,
    /// Requerimientos para estar disponible
    pub requerimientos: Vec<String>,
}

/// Configuracion de eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventosConfig {
    /// Eventos historicos fijos (no modificables)
    pub eventos_fijos: Vec<EventoFijo>,
    /// Plantillas de eventos generados
    pub plantillas: HashMap<String, EventoPlantilla>,
    /// Familias de eventos
    pub familias: HashMap<String, Vec<String>>,
}

impl Default for EventosConfig {
    fn default() -> Self {
        let mut plantillas = HashMap::new();
        let mut familias = HashMap::new();

        familias.insert("sesion_institucional".to_string(), vec![
            "debate_constitucional".to_string(),
            "votacion_ley".to_string(),
        ]);

        plantillas.insert("debate_constitucional".to_string(), EventoPlantilla {
            id: "debate_constitucional".to_string(),
            titulo: "Debate sobre articulo constitucional".to_string(),
            tipo: TipoEvento::SesionInstitucional,
            descripcion: "Participar en el debate sobre un articulo clave de la Constitucion".to_string(),
            coste_temporal: 4,
            prioridad_base: 80,
            familias: vec!["sesion_institucional".to_string()],
            requerimientos: HashMap::from([("ser_diputado".to_string(), "true".to_string())]),
            consecuencias: HashMap::from([
                ("influencia".to_string(), 10),
                ("coherencia".to_string(), 5),
            ]),
            opciones: vec![
                OpcionEvento {
                    id: "apoyar".to_string(),
                    texto: "Apoyar la propuesta liberal".to_string(),
                    modificador_perfil: 1.5,
                    coste_adicional: 0,
                    consecuencias: HashMap::from([
                        ("reputacion_liberales".to_string(), 15),
                        ("reputacion_absolutistas".to_string(), -10),
                    ]),
                    requerimientos: vec![],
                },
                OpcionEvento {
                    id: "oponerse".to_string(),
                    texto: "Oponerse con argumentos juridicos".to_string(),
                    modificador_perfil: 1.2,
                    coste_adicional: 1,
                    consecuencias: HashMap::from([
                        ("reputacion_liberales".to_string(), -5),
                        ("influencia".to_string(), 8),
                    ]),
                    requerimientos: vec!["oficio_jurista".to_string()],
                },
            ],
        });

        Self {
            eventos_fijos: vec![
                EventoFijo {
                    id: "trafalgar".to_string(),
                    nombre: "Batalla de Trafalgar".to_string(),
                    fecha: "1805-10-21".to_string(),
                    descripcion: "Derrota de la flota franco-espanola".to_string(),
                    impacto: "Aumento del aislamiento de Cadiz".to_string(),
                    jornada: 0,
                },
                EventoFijo {
                    id: "inauguracion_cortes".to_string(),
                    nombre: "Inauguracion de las Cortes".to_string(),
                    fecha: "1810-09-24".to_string(),
                    descripcion: "Primera sesion de las Cortes de Cadiz".to_string(),
                    impacto: "Inicio del proceso constituyente".to_string(),
                    jornada: 100,
                },
            ],
            plantillas,
            familias,
        }
    }
}
