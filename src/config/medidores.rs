use serde::{Serialize, Deserialize};

/// Umbrales para los 6 medidores del protagonista
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedidorConfig {
    pub umbral_bajo: u8,
    pub umbral_alto: u8,
    /// Tasa de decaimiento por jornada (0.0 = no decae)
    pub decaimiento_tasa: f32,
    /// Jornada en la que empieza el decaimiento (0 = desde el inicio)
    pub decaimiento_inicio: u32,
}

/// Configuración de los 6 medidores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedidoresConfig {
    pub influencia: MedidorConfig,
    pub relacional: MedidorConfig,
    pub reputacion: MedidorConfig,
    pub coherencia: MedidorConfig,
    pub recursos: MedidorConfig,
    pub aguante: MedidorConfig,
}

impl Default for MedidoresConfig {
    fn default() -> Self {
        Self {
            influencia: MedidorConfig { umbral_bajo: 30, umbral_alto: 80, decaimiento_tasa: 0.0, decaimiento_inicio: 0 },
            relacional: MedidorConfig { umbral_bajo: 25, umbral_alto: 75, decaimiento_tasa: 0.0, decaimiento_inicio: 0 },
            reputacion: MedidorConfig { umbral_bajo: 35, umbral_alto: 85, decaimiento_tasa: 0.0, decaimiento_inicio: 0 },
            coherencia: MedidorConfig { umbral_bajo: 40, umbral_alto: 90, decaimiento_tasa: 0.0, decaimiento_inicio: 0 },
            recursos: MedidorConfig { umbral_bajo: 20, umbral_alto: 70, decaimiento_tasa: 0.0, decaimiento_inicio: 0 },
            aguante: MedidorConfig { umbral_bajo: 25, umbral_alto: 80, decaimiento_tasa: 0.0, decaimiento_inicio: 0 },
        }
    }
}
