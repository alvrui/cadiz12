use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Identificador de medidor
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MedidorId {
    Influencia,
    Coherencia,
    Legitimidad,
    ApoyoPopular,
    RedDeContactos,
    CapacidadDeMando,
    RecursosEconomicos,
    PrestigioPersonal,
}

impl std::fmt::Display for MedidorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MedidorId::Influencia => write!(f, "influencia"),
            MedidorId::Coherencia => write!(f, "coherencia"),
            MedidorId::Legitimidad => write!(f, "legitimidad"),
            MedidorId::ApoyoPopular => write!(f, "apoyo_popular"),
            MedidorId::RedDeContactos => write!(f, "red_de_contactos"),
            MedidorId::CapacidadDeMando => write!(f, "capacidad_de_mando"),
            MedidorId::RecursosEconomicos => write!(f, "recursos_economicos"),
            MedidorId::PrestigioPersonal => write!(f, "prestigio_personal"),
        }
    }
}

impl serde::Serialize for MedidorId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for MedidorId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "influencia" => Ok(MedidorId::Influencia),
            "coherencia" => Ok(MedidorId::Coherencia),
            "legitimidad" => Ok(MedidorId::Legitimidad),
            "apoyo_popular" => Ok(MedidorId::ApoyoPopular),
            "red_de_contactos" => Ok(MedidorId::RedDeContactos),
            "capacidad_de_mando" => Ok(MedidorId::CapacidadDeMando),
            "recursos_economicos" => Ok(MedidorId::RecursosEconomicos),
            "prestigio_personal" => Ok(MedidorId::PrestigioPersonal),
            _ => Err(serde::de::Error::custom(format!("Medidor desconocido: {}", s))),
        }
    }
}

/// Configuracion de un medidor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedidorConfig {
    pub id: MedidorId,
    pub nombre: String,
    pub descripcion: String,
    pub valor_inicial: u8,
    pub umbral_bajo: u8,
    pub umbral_alto: u8,
    /// Importancia para la puntuacion final (0-100)
    pub peso: u8,
}

/// Tendencia de un medidor
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tendencia {
    SubiendoRapido,
    Subiendo,
    Estable,
    Bajando,
    BajandoRapido,
}

impl std::fmt::Display for Tendencia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tendencia::SubiendoRapido => write!(f, "subiendo_rapido"),
            Tendencia::Subiendo => write!(f, "subiendo"),
            Tendencia::Estable => write!(f, "estable"),
            Tendencia::Bajando => write!(f, "bajando"),
            Tendencia::BajandoRapido => write!(f, "bajando_rapido"),
        }
    }
}

/// Coleccion de medidores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedidoresConfig {
    pub medidores: HashMap<MedidorId, MedidorConfig>,
}

impl Default for MedidoresConfig {
    fn default() -> Self {
        let mut medidores = HashMap::new();

        medidores.insert(MedidorId::Influencia, MedidorConfig {
            id: MedidorId::Influencia,
            nombre: "Influencia".to_string(),
            descripcion: "Capacidad para convencer y liderar".to_string(),
            valor_inicial: 50,
            umbral_bajo: 30,
            umbral_alto: 80,
            peso: 25,
        });

        medidores.insert(MedidorId::Coherencia, MedidorConfig {
            id: MedidorId::Coherencia,
            nombre: "Coherencia".to_string(),
            descripcion: "Consistencia entre acciones y principios".to_string(),
            valor_inicial: 70,
            umbral_bajo: 40,
            umbral_alto: 90,
            peso: 20,
        });

        Self { medidores }
    }
}