pub mod facciones;
pub mod espacios;
pub mod personajes;
pub mod eventos;
pub mod textos;
pub mod partida;
pub mod medidores;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub use facciones::*;
pub use espacios::*;
pub use personajes::*;
pub use eventos::*;
pub use textos::*;
pub use partida::*;
pub use medidores::*;
