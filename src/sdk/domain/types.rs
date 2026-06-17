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
