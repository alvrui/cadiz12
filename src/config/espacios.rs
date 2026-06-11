use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Identificadores de espacios
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EspacioId {
    OratorioSanFelipeNeri,
    ComisionesParlamentarias,
    CafeApolo,
    CafeOrta,
    MuellesAduana,
    CalleNuevaConsulado,
    CatedralRedEclesiastica,
    ImprentasPublicaciones,
    BarrioSanCarlosCapitania,
    SalonesTeatrosTertulias,
    BarrioLaVina,
    BarrioElPopulo,
    BarrioSantaMaria,
}

impl std::fmt::Display for EspacioId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EspacioId::OratorioSanFelipeNeri => write!(f, "oratorio_san_felipe_neri"),
            EspacioId::ComisionesParlamentarias => write!(f, "comisiones_parlamentarias"),
            EspacioId::CafeApolo => write!(f, "cafe_apolo"),
            EspacioId::CafeOrta => write!(f, "cafe_orta"),
            EspacioId::MuellesAduana => write!(f, "muelles_aduana"),
            EspacioId::CalleNuevaConsulado => write!(f, "calle_nueva_consulado"),
            EspacioId::CatedralRedEclesiastica => write!(f, "catedral_red_eclesiastica"),
            EspacioId::ImprentasPublicaciones => write!(f, "imprentas_publicaciones"),
            EspacioId::BarrioSanCarlosCapitania => write!(f, "barrio_san_carlos_capitania"),
            EspacioId::SalonesTeatrosTertulias => write!(f, "salones_teatros_tertulias"),
            EspacioId::BarrioLaVina => write!(f, "barrio_la_vina"),
            EspacioId::BarrioElPopulo => write!(f, "barrio_el_populo"),
            EspacioId::BarrioSantaMaria => write!(f, "barrio_santa_maria"),
        }
    }
}

impl serde::Serialize for EspacioId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for EspacioId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "oratorio_san_felipe_neri" => Ok(EspacioId::OratorioSanFelipeNeri),
            "comisiones_parlamentarias" => Ok(EspacioId::ComisionesParlamentarias),
            "cafe_apolo" => Ok(EspacioId::CafeApolo),
            "cafe_orta" => Ok(EspacioId::CafeOrta),
            "muelles_aduana" => Ok(EspacioId::MuellesAduana),
            "calle_nueva_consulado" => Ok(EspacioId::CalleNuevaConsulado),
            "catedral_red_eclesiastica" => Ok(EspacioId::CatedralRedEclesiastica),
            "imprentas_publicaciones" => Ok(EspacioId::ImprentasPublicaciones),
            "barrio_san_carlos_capitania" => Ok(EspacioId::BarrioSanCarlosCapitania),
            "salones_teatros_tertulias" => Ok(EspacioId::SalonesTeatrosTertulias),
            "barrio_la_vina" => Ok(EspacioId::BarrioLaVina),
            "barrio_el_populo" => Ok(EspacioId::BarrioElPopulo),
            "barrio_santa_maria" => Ok(EspacioId::BarrioSantaMaria),
            _ => Err(serde::de::Error::custom(format!("Espacio desconocido: {}", s))),
        }
    }
}

/// Clima del espacio
#[derive(Debug, Clone, PartialEq)]
pub enum ClimaEspacio {
    Tranquilo,
    Saturado,
    Nervioso,
    Vigilado,
    Vacio,
    Efervescente,
}

impl serde::Serialize for ClimaEspacio {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            ClimaEspacio::Tranquilo => "tranquilo",
            ClimaEspacio::Saturado => "saturado",
            ClimaEspacio::Nervioso => "nervioso",
            ClimaEspacio::Vigilado => "vigilado",
            ClimaEspacio::Vacio => "vacio",
            ClimaEspacio::Efervescente => "efervescente",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> serde::Deserialize<'de> for ClimaEspacio {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "tranquilo" => Ok(ClimaEspacio::Tranquilo),
            "saturado" => Ok(ClimaEspacio::Saturado),
            "nervioso" => Ok(ClimaEspacio::Nervioso),
            "vigilado" => Ok(ClimaEspacio::Vigilado),
            "vacio" => Ok(ClimaEspacio::Vacio),
            "efervescente" => Ok(ClimaEspacio::Efervescente),
            _ => Err(serde::de::Error::custom(format!("Clima desconocido: {}", s))),
        }
    }
}

/// Configuracion de un espacio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EspacioConfig {
    pub id: EspacioId,
    pub nombre: String,
    pub descripcion: String,
    /// Funcion en el juego
    pub funcion: String,
    /// Tipo de interaccion
    pub tipo_interaccion: String,
    /// Coste temporal en unidades de jornada
    pub coste_temporal: u8,
    /// Nivel de riesgo (1-4)
    pub nivel_riesgo: u8,
    /// Clima inicial
    pub clima_inicial: ClimaEspacio,
    /// NPCs disponibles inicialmente
    pub npcs_iniciales: Vec<String>,
    /// Requerimientos para acceder
    pub requerimientos: Vec<String>,
}

/// Coleccion de todos los espacios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EspaciosConfig {
    pub espacios: HashMap<EspacioId, EspacioConfig>,
}

impl Default for EspaciosConfig {
    fn default() -> Self {
        let mut espacios = HashMap::new();

        espacios.insert(EspacioId::OratorioSanFelipeNeri, EspacioConfig {
            id: EspacioId::OratorioSanFelipeNeri,
            nombre: "Oratorio de San Felipe Neri".to_string(),
            descripcion: "Sede de las Cortes desde febrero de 1811".to_string(),
            funcion: "Debate formal, votacion, enmienda, procedimiento".to_string(),
            tipo_interaccion: "Institucional".to_string(),
            coste_temporal: 4,
            nivel_riesgo: 2,
            clima_inicial: ClimaEspacio::Nervioso,
            npcs_iniciales: vec!["arguelles".to_string(), "munoz_torrero".to_string()],
            requerimientos: vec!["ser_diputado".to_string()],
        });

        espacios.insert(EspacioId::CafeApolo, EspacioConfig {
            id: EspacioId::CafeApolo,
            nombre: "Cafe de Apolo".to_string(),
            descripcion: "El filtro popular de la informacion impresa".to_string(),
            funcion: "Rumor, opinion, reclutamiento de apoyos, provocacion".to_string(),
            tipo_interaccion: "Social".to_string(),
            coste_temporal: 2,
            nivel_riesgo: 1,
            clima_inicial: ClimaEspacio::Efervescente,
            npcs_iniciales: vec!["periodista_1".to_string(), "ciudadano_1".to_string()],
            requerimientos: vec![],
        });

        Self { espacios }
    }
}
