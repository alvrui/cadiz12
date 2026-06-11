pub mod dtos;
pub mod m1_estado_mundo;
pub mod m2_estado_protagonista;
pub mod m3_medidores;
pub mod m4_generador_eventos;
pub mod m5_bucle_jornada;
pub mod m6_memoria;
pub mod m7_api;

pub use dtos::*;
pub use m1_estado_mundo::*;
pub use m2_estado_protagonista::*;
pub use m3_medidores::*;
pub use m4_generador_eventos::*;
pub use m5_bucle_jornada::*;
pub use m6_memoria::*;
pub use m7_api::*;

use crate::config::PartidaConfig;

/// Motor principal del juego
pub struct Motor {
    pub api: MotorApi,
}

impl Motor {
    pub fn nuevo(config: PartidaConfig) -> Self {
        Self {
            api: MotorApi::nuevo(config),
        }
    }
}
