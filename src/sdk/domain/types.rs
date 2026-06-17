// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Alvaro Ruiz
// Contrato formal del SDK v3.0 - Tipos cerrados del dominio

use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

// ============================================================================
// ERRORES DE PARSEO
// ============================================================================

#[derive(Debug, Error, PartialEq)]
pub enum DomainParseError {
    #[error("Valor desconocido para {type_name}: {value}")]
    UnknownValue { type_name: &'static str, value: String },
}

// ============================================================================
// FACCIONES POLITICAS
// ============================================================================

/// Identificadores de facciones politicas en las Cortes de Cadiz 1812
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaccionId {
    LiberalProgresista,
    LiberalModerado,
    AbsolutistaServil,
    IndependienteOportunista,
    Americanista,
}

impl fmt::Display for FaccionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FaccionId::LiberalProgresista => write!(f, "liberal_progresista"),
            FaccionId::LiberalModerado => write!(f, "liberal_moderado"),
            FaccionId::AbsolutistaServil => write!(f, "absolutista_servil"),
            FaccionId::IndependienteOportunista => write!(f, "independiente_oportunista"),
            FaccionId::Americanista => write!(f, "americanista"),
        }
    }
}

impl FromStr for FaccionId {
    type Err = DomainParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "liberal_progresista" => Ok(FaccionId::LiberalProgresista),
            "liberal_moderado" => Ok(FaccionId::LiberalModerado),
            "absolutista_servil" => Ok(FaccionId::AbsolutistaServil),
            "independiente_oportunista" => Ok(FaccionId::IndependienteOportunista),
            "americanista" => Ok(FaccionId::Americanista),
            _ => Err(DomainParseError::UnknownValue {
                type_name: "FaccionId",
                value: s.to_string(),
            }),
        }
    }
}

impl FaccionId {
    pub fn all() -> &'static [FaccionId] {
        &[
            FaccionId::LiberalProgresista,
            FaccionId::LiberalModerado,
            FaccionId::AbsolutistaServil,
            FaccionId::IndependienteOportunista,
            FaccionId::Americanista,
        ]
    }
    pub fn nombre(&self) -> &'static str {
        match self {
            FaccionId::LiberalProgresista => "Liberales Progresistas",
            FaccionId::LiberalModerado => "Liberales Moderados",
            FaccionId::AbsolutistaServil => "Absolutistas/Serviles",
            FaccionId::IndependienteOportunista => "Independientes Oportunistas",
            FaccionId::Americanista => "Americanistas",
        }
    }
    pub fn descripcion(&self) -> &'static str {
        match self {
            FaccionId::LiberalProgresista => "Partidarios de reformas profundas y rapidas",
            FaccionId::LiberalModerado => "Reformistas graduales",
            FaccionId::AbsolutistaServil => "Defensores del Antiguo Regimen",
            FaccionId::IndependienteOportunista => "Politicos sin adscripcion fija",
            FaccionId::Americanista => "Partidarios de los intereses americanos",
        }
    }
}