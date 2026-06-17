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

// ============================================================================
// ORIGENES GEOGRAFICOS
// ============================================================================

/// Origen geografico del personaje
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Origen {
    Gaditano,
    Peninsular,
    AmericanoVirreinal,
}

impl fmt::Display for Origen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Origen::Gaditano => write!(f, "gaditano"),
            Origen::Peninsular => write!(f, "peninsular"),
            Origen::AmericanoVirreinal => write!(f, "americano_virreinal"),
        }
    }
}

impl FromStr for Origen {
    type Err = DomainParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gaditano" => Ok(Origen::Gaditano),
            "peninsular" => Ok(Origen::Peninsular),
            "americano_virreinal" => Ok(Origen::AmericanoVirreinal),
            _ => Err(DomainParseError::UnknownValue {
                type_name: "Origen",
                value: s.to_string(),
            }),
        }
    }
}

impl Origen {
    pub fn all() -> &'static [Origen] {
        &[Origen::Gaditano, Origen::Peninsular, Origen::AmericanoVirreinal]
    }
}

// ============================================================================
// CLASES SOCIALES
// ============================================================================

/// Clase social del personaje
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaseSocial {
    EliteMercantilGaditana,
    HidalguiaProfesionLetrada,
    CleroIlustrado,
    CarreraMilitar,
    EliteCriollaAmericana,
    FuncionariadoIlustrado,
}

impl fmt::Display for ClaseSocial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl FromStr for ClaseSocial {
    type Err = DomainParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "elite_mercantil_gaditana" => Ok(ClaseSocial::EliteMercantilGaditana),
            "hidalguia_profesion_letrada" => Ok(ClaseSocial::HidalguiaProfesionLetrada),
            "clero_ilustrado" => Ok(ClaseSocial::CleroIlustrado),
            "carrera_militar" => Ok(ClaseSocial::CarreraMilitar),
            "elite_criolla_americana" => Ok(ClaseSocial::EliteCriollaAmericana),
            "funcionariado_ilustrado" => Ok(ClaseSocial::FuncionariadoIlustrado),
            _ => Err(DomainParseError::UnknownValue {
                type_name: "ClaseSocial",
                value: s.to_string(),
            }),
        }
    }
}

impl ClaseSocial {
    pub fn all() -> &'static [ClaseSocial] {
        &[
            ClaseSocial::EliteMercantilGaditana,
            ClaseSocial::HidalguiaProfesionLetrada,
            ClaseSocial::CleroIlustrado,
            ClaseSocial::CarreraMilitar,
            ClaseSocial::EliteCriollaAmericana,
            ClaseSocial::FuncionariadoIlustrado,
        ]
    }
}
