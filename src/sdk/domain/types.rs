// Domain types for Cádiz 1812 SDK v3.0
// This file contains all closed enum types forming the formal SDK contract

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Error type for domain parsing operations
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DomainParseError {
    #[error("Unknown variant: {0}")]
    UnknownVariant(String),
    #[error("Empty string")]
    EmptyString,
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}

/// Unique identifier for a faction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum FaccionId {
    Liberal,
    Absolutista,
    Clero,
    Militar,
    Pueblo,
    Nobleza,
    Burguesia,
    Extranjero,
}

impl FaccionId {
    pub fn variants() -> &'static [Self] {
        &[
            Self::Liberal,
            Self::Absolutista,
            Self::Clero,
            Self::Militar,
            Self::Pueblo,
            Self::Nobleza,
            Self::Burguesia,
            Self::Extranjero,
        ]
    }
}

/// Character origin/background
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Origen {
    Local,
    Foraneo,
    Noble,
    Plebeyo,
    Militar,
    Clerigo,
    Comerciante,
    Artesano,
}

/// Social class of a character
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ClaseSocial {
    AltaNobleza,
    BajaNobleza,
    Burguesia,
    CleroAlto,
    CleroBajo,
    MilitarAlto,
    MilitarBajo,
    Artesano,
    Comerciante,
    Campesino,
    Obrero,
    Mendigo,
}

/// Profession/occupation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Oficio {
    Politico,
    Militar,
    Clerigo,
    Comerciante,
    Artesano,
    Abogado,
    Medico,
    Periodista,
    Escritor,
    Obrero,
    Campesino,
    Sirviente,
    Espia,
    Contrabandista,
    Noble,
}

/// Political affiliation/alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Adscripcion {
    Liberal,
    Absolutista,
    Moderado,
    Radical,
    Conservador,
    Reformista,
    Neutral,
    Oportunista,
}

/// Character temperament
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Temperamento {
    Impulsivo,
    Reflexivo,
    Agresivo,
    Pacifista,
    Honesto,
    Astuto,
    Leal,
    Traitor,
    Optimista,
    Pesimista,
    Carismatico,
    Timido,
}

/// Formal position identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PosicionFormalId {
    Diputado,
    Senador,
    Ministro,
    General,
    Almirante,
    Obispo,
    Alcalde,
    Juez,
    Abogado,
    Periodista,
    Comerciante,
    Artesano,
    Campesino,
    Noble,
    Sirviente,
}

/// Visibility level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Visibilidad {
    Publico,
    Privado,
    Secreto,
    Oculto,
}

/// Moral trajectory/alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TrayectoriaMoral {
    Heroico,
    Villano,
    Antiheroe,
    Neutral,
    Oportunista,
    Idealista,
    Pragmatico,
    Corrupto,
    Redimido,
    Caido,
}

/// Event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TipoEvento {
    Politico,
    Personal,
    Militar,
    Social,
    Economico,
    Religioso,
    Judicial,
    Diplomatico,
    Cultural,
    Urgente,
}

/// Crisis phase
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum FaseCrisis {
    Inicio,
    Desarrollo,
    Climax,
    Resolucion,
    Consecuencia,
}

/// Crisis type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TipoCrisis {
    Politica,
    Social,
    Economica,
    Militar,
    Religiosa,
    Personal,
    Institucional,
    Internacional,
}

/// Space/location identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum EspacioId {
    Cortes,
    Calle,
    Taberna,
    Iglesia,
    Cuartel,
    Puerto,
    Mercado,
    Palacio,
    Casa,
    Prision,
    Hospital,
    Universidad,
    Plaza,
}

/// Relationship status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum EstadoRelacion {
    Aliado,
    Amigo,
    Conocido,
    Neutral,
    Desconfiado,
    Enemigo,
    Rival,
    Mentor,
    Protegido,
    Familiar,
}

/// Meter/gauge identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum MedidorId {
    Influencia,
    Reputacion,
    Moral,
    Salud,
    Riqueza,
    Conocimiento,
    Lealtad,
    Miedo,
    Esperanza,
    Ira,
}

/// Section/segment identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TramoId {
    Primero,
    Segundo,
    Tercero,
    Cuarto,
    Quinto,
}

/// Act type alias - represents a formal act or action in the narrative
pub type Acto = String;

/// Day/journey type alias - represents a game day or journey
pub type Jornada = u32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faccion_id_parsing() {
        assert_eq!("liberal".parse::<FaccionId>().unwrap(), FaccionId::Liberal);
        assert_eq!("absolutista".parse::<FaccionId>().unwrap(), FaccionId::Absolutista);
    }

    #[test]
    fn test_origen_parsing() {
        assert_eq!("local".parse::<Origen>().unwrap(), Origen::Local);
        assert_eq!("foraneo".parse::<Origen>().unwrap(), Origen::Foraneo);
    }

    #[test]
    fn test_clase_social_parsing() {
        assert_eq!("alta_nobleza".parse::<ClaseSocial>().unwrap(), ClaseSocial::AltaNobleza);
        assert_eq!("burguesia".parse::<ClaseSocial>().unwrap(), ClaseSocial::Burguesia);
    }

    #[test]
    fn test_oficio_parsing() {
        assert_eq!("politico".parse::<Oficio>().unwrap(), Oficio::Politico);
        assert_eq!("militar".parse::<Oficio>().unwrap(), Oficio::Militar);
    }

    #[test]
    fn test_adscripcion_parsing() {
        assert_eq!("liberal".parse::<Adscripcion>().unwrap(), Adscripcion::Liberal);
        assert_eq!("neutral".parse::<Adscripcion>().unwrap(), Adscripcion::Neutral);
    }

    #[test]
    fn test_temperamento_parsing() {
        assert_eq!("impulsivo".parse::<Temperamento>().unwrap(), Temperamento::Impulsivo);
        assert_eq!("reflexivo".parse::<Temperamento>().unwrap(), Temperamento::Reflexivo);
    }

    #[test]
    fn test_posicion_formal_id_parsing() {
        assert_eq!("diputado".parse::<PosicionFormalId>().unwrap(), PosicionFormalId::Diputado);
        assert_eq!("general".parse::<PosicionFormalId>().unwrap(), PosicionFormalId::General);
    }

    #[test]
    fn test_visibilidad_parsing() {
        assert_eq!("publico".parse::<Visibilidad>().unwrap(), Visibilidad::Publico);
        assert_eq!("secreto".parse::<Visibilidad>().unwrap(), Visibilidad::Secreto);
    }

    #[test]
    fn test_trayectoria_moral_parsing() {
        assert_eq!("heroico".parse::<TrayectoriaMoral>().unwrap(), TrayectoriaMoral::Heroico);
        assert_eq!("villano".parse::<TrayectoriaMoral>().unwrap(), TrayectoriaMoral::Villano);
    }

    #[test]
    fn test_tipo_evento_parsing() {
        assert_eq!("politico".parse::<TipoEvento>().unwrap(), TipoEvento::Politico);
        assert_eq!("urgente".parse::<TipoEvento>().unwrap(), TipoEvento::Urgente);
    }

    #[test]
    fn test_fase_crisis_parsing() {
        assert_eq!("inicio".parse::<FaseCrisis>().unwrap(), FaseCrisis::Inicio);
        assert_eq!("climax".parse::<FaseCrisis>().unwrap(), FaseCrisis::Climax);
    }

    #[test]
    fn test_tipo_crisis_parsing() {
        assert_eq!("politica".parse::<TipoCrisis>().unwrap(), TipoCrisis::Politica);
        assert_eq!("social".parse::<TipoCrisis>().unwrap(), TipoCrisis::Social);
    }

    #[test]
    fn test_espacio_id_parsing() {
        assert_eq!("cortes".parse::<EspacioId>().unwrap(), EspacioId::Cortes);
        assert_eq!("taberna".parse::<EspacioId>().unwrap(), EspacioId::Taberna);
    }

    #[test]
    fn test_estado_relacion_parsing() {
        assert_eq!("aliado".parse::<EstadoRelacion>().unwrap(), EstadoRelacion::Aliado);
        assert_eq!("enemigo".parse::<EstadoRelacion>().unwrap(), EstadoRelacion::Enemigo);
    }

    #[test]
    fn test_medidor_id_parsing() {
        assert_eq!("influencia".parse::<MedidorId>().unwrap(), MedidorId::Influencia);
        assert_eq!("reputacion".parse::<MedidorId>().unwrap(), MedidorId::Reputacion);
    }

    #[test]
    fn test_tramo_id_parsing() {
        assert_eq!("primero".parse::<TramoId>().unwrap(), TramoId::Primero);
        assert_eq!("segundo".parse::<TramoId>().unwrap(), TramoId::Segundo);
    }

    #[test]
    fn test_type_aliases() {
        let acto: Acto = "Declaración de Independencia".to_string();
        let jornada: Jornada = 1;
        assert_eq!(acto, "Declaración de Independencia");
        assert_eq!(jornada, 1);
    }

    #[test]
    fn test_serialization() {
        let faccion = FaccionId::Liberal;
        let json = serde_json::to_string(&faccion).unwrap();
        assert_eq!(json, "\"liberal\"");

        let origen = Origen::Local;
        let json = serde_json::to_string(&origen).unwrap();
        assert_eq!(json, "\"local\"");
    }

    #[test]
    fn test_deserialization() {
        let json = "\"liberal\"";
        let faccion: FaccionId = serde_json::from_str(json).unwrap();
        assert_eq!(faccion, FaccionId::Liberal);

        let json = "\"local\"";
        let origen: Origen = serde_json::from_str(json).unwrap();
        assert_eq!(origen, Origen::Local);
    }
}