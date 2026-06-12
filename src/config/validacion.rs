use serde::{Serialize, Deserialize};
use std::fmt;

/// Contexto historico para validacion de eventos y personajes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ContextoHistorico {
    /// Periodo historico (ej: "1810-1814")
    pub periodo: String,
    /// Evento historico de referencia (ej: "Sitio de Cadiz")
    pub evento_referencia: String,
    /// Faccion dominante en este contexto
    pub faccion_dominante: String,
    /// Nivel de tension politica (0-100)
    pub tension_politica: u8,
    /// Nivel de tension social (0-100)
    pub tension_social: u8,
}

impl ContextoHistorico {
    pub fn nuevo(
        periodo: &str,
        evento_referencia: &str,
        faccion_dominante: &str,
        tension_politica: u8,
        tension_social: u8,
    ) -> Self {
        Self {
            periodo: periodo.to_string(),
            evento_referencia: evento_referencia.to_string(),
            faccion_dominante: faccion_dominante.to_string(),
            tension_politica: tension_politica.min(100),
            tension_social: tension_social.min(100),
        }
    }

    /// Validar que el contexto es coherente
    pub fn validar(&self) -> Result<(), ErrorValidacion> {
        if self.tension_politica > 100 {
            return Err(ErrorValidacion::RangoInvalido(
                "tension_politica debe ser <= 100".to_string(),
            ));
        }
        if self.tension_social > 100 {
            return Err(ErrorValidacion::RangoInvalido(
                "tension_social debe ser <= 100".to_string(),
            ));
        }
        if self.periodo.is_empty() {
            return Err(ErrorValidacion::CampoRequerido("periodo".to_string()));
        }
        Ok(())
    }
}

impl Default for ContextoHistorico {
    fn default() -> Self {
        Self {
            periodo: "1810-1814".to_string(),
            evento_referencia: "Sitio de Cadiz".to_string(),
            faccion_dominante: "Cortes de Cadiz".to_string(),
            tension_politica: 75,
            tension_social: 60,
        }
    }
}

/// Relacion historica entre entidades (personajes, facciones, espacios)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelacionHistorica {
    /// Identificador de la primera entidad
    pub entidad_a: String,
    /// Identificador de la segunda entidad
    pub entidad_b: String,
    /// Tipo de relacion (ej: "aliado", "enemigo", "neutral", "familiar")
    pub tipo: TipoRelacion,
    /// Fuerza de la relacion (-100 a 100)
    pub fuerza: i16,
    /// Descripcion de la relacion
    pub descripcion: String,
    /// Contexto en el que esta relacion es valida
    pub contexto: ContextoHistorico,
}

impl RelacionHistorica {
    pub fn nueva(
        entidad_a: &str,
        entidad_b: &str,
        tipo: TipoRelacion,
        fuerza: i16,
        descripcion: &str,
        contexto: ContextoHistorico,
    ) -> Self {
        Self {
            entidad_a: entidad_a.to_string(),
            entidad_b: entidad_b.to_string(),
            tipo,
            fuerza: fuerza.max(-100).min(100),
            descripcion: descripcion.to_string(),
            contexto,
        }
    }

    /// Validar que la relacion es coherente
    pub fn validar(&self) -> Result<(), ErrorValidacion> {
        if self.entidad_a.is_empty() {
            return Err(ErrorValidacion::CampoRequerido("entidad_a".to_string()));
        }
        if self.entidad_b.is_empty() {
            return Err(ErrorValidacion::CampoRequerido("entidad_b".to_string()));
        }
        if self.fuerza < -100 || self.fuerza > 100 {
            return Err(ErrorValidacion::RangoInvalido(
                "fuerza debe estar entre -100 y 100".to_string(),
            ));
        }
        self.contexto.validar()?;
        Ok(())
    }

    /// Obtener el signo de la fuerza (positivo, negativo, neutro)
    pub fn signo(&self) -> SignoRelacion {
        if self.fuerza > 10 {
            SignoRelacion::Positivo
        } else if self.fuerza < -10 {
            SignoRelacion::Negativo
        } else {
            SignoRelacion::Neutro
        }
    }
}

/// Tipo de relacion entre entidades
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TipoRelacion {
    /// Relacion de alianza o cooperacion
    Aliado,
    /// Relacion de rivalidad o conflicto
    Enemigo,
    /// Relacion neutral o indiferente
    Neutral,
    /// Relacion familiar
    Familiar,
    /// Relacion de mentor/discipulo
    Mentoria,
    /// Relacion de lealtad
    Lealtad,
    /// Relacion de desconfianza
    Desconfianza,
    /// Relacion commercial
    Commercial,
    /// Relacion politica
    Politica,
    /// Otra tipo de relacion
    Otra(String),
}

impl Default for TipoRelacion {
    fn default() -> Self {
        Self::Neutral
    }
}

impl fmt::Display for TipoRelacion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Aliado => write!(f, "aliado"),
            Self::Enemigo => write!(f, "enemigo"),
            Self::Neutral => write!(f, "neutral"),
            Self::Familiar => write!(f, "familiar"),
            Self::Mentoria => write!(f, "mentoria"),
            Self::Lealtad => write!(f, "lealtad"),
            Self::Desconfianza => write!(f, "desconfianza"),
            Self::Commercial => write!(f, "commercial"),
            Self::Politica => write!(f, "politica"),
            Self::Otra(s) => write!(f, "{}", s),
        }
    }
}

/// Signo de una relacion (positivo, negativo, neutro)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SignoRelacion {
    Positivo,
    Negativo,
    Neutro,
}

/// Error de validacion
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorValidacion {
    /// Campo requerido no proporcionado
    CampoRequerido(String),
    /// Valor fuera de rango
    RangoInvalido(String),
    /// Valor invalido
    ValorInvalido(String),
    /// Inconsistencia entre campos
    Inconsistencia(String),
    /// Error de formato
    FormatoInvalido(String),
}

impl fmt::Display for ErrorValidacion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CampoRequerido(campo) => write!(f, "Campo requerido no proporcionado: {}", campo),
            Self::RangoInvalido(msg) => write!(f, "Valor fuera de rango: {}", msg),
            Self::ValorInvalido(msg) => write!(f, "Valor invalido: {}", msg),
            Self::Inconsistencia(msg) => write!(f, "Inconsistencia: {}", msg),
            Self::FormatoInvalido(msg) => write!(f, "Formato invalido: {}", msg),
        }
    }
}

impl std::error::Error for ErrorValidacion {}

impl ErrorValidacion {
    pub fn mensaje(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contexto_historico_valido() {
        let ctx = ContextoHistorico::nuevo("1810-1814", "Sitio de Cadiz", "Cortes", 75, 60);
        assert!(ctx.validar().is_ok());
    }

    #[test]
    fn test_contexto_historico_periodo_vacio() {
        let ctx = ContextoHistorico {
            periodo: "".to_string(),
            evento_referencia: "Sitio".to_string(),
            faccion_dominante: "Cortes".to_string(),
            tension_politica: 50,
            tension_social: 60,
        };
        assert!(ctx.validar().is_err());
    }

    #[test]
    fn test_relacion_historica_valida() {
        let ctx = ContextoHistorico::default();
        let rel = RelacionHistorica::nueva("personaje1", "personaje2", TipoRelacion::Aliado, 50, "Aliados políticos", ctx);
        assert!(rel.validar().is_ok());
    }

    #[test]
    fn test_relacion_historica_entidad_vacia() {
        let ctx = ContextoHistorico::default();
        let rel = RelacionHistorica {
            entidad_a: "".to_string(),
            entidad_b: "p2".to_string(),
            tipo: TipoRelacion::Enemigo,
            fuerza: 50,
            descripcion: "Enemigos".to_string(),
            contexto: ctx,
        };
        assert!(rel.validar().is_err());
    }

    #[test]
    fn test_signo_relacion() {
        let ctx = ContextoHistorico::default();
        
        let rel_pos = RelacionHistorica::nueva("a", "b", TipoRelacion::Aliado, 50, "Positiva", ctx.clone());
        assert_eq!(rel_pos.signo(), SignoRelacion::Positivo);

        let rel_neg = RelacionHistorica::nueva("a", "b", TipoRelacion::Enemigo, -50, "Negativa", ctx.clone());
        assert_eq!(rel_neg.signo(), SignoRelacion::Negativo);

        let rel_neutro = RelacionHistorica::nueva("a", "b", TipoRelacion::Neutral, 5, "Neutra", ctx);
        assert_eq!(rel_neutro.signo(), SignoRelacion::Neutro);
    }

    #[test]
    fn test_error_validacion_display() {
        let err = ErrorValidacion::CampoRequerido("nombre".to_string());
        assert_eq!(err.to_string(), "Campo requerido no proporcionado: nombre");
    }
}
