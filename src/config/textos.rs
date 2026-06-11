use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Texto narrativo con variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextoNarrativo {
    /// Texto base con placeholders {variable}
    pub texto: String,
    /// Variables disponibles
    pub variables: HashMap<String, String>,
}

/// Configuracion de textos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextosConfig {
    /// Textos de introduccion
    pub introducciones: HashMap<String, TextoNarrativo>,
    /// Textos de eventos
    pub eventos: HashMap<String, TextoNarrativo>,
    /// Textos de espacios
    pub espacios: HashMap<String, TextoNarrativo>,
    /// Textos de facciones
    pub facciones: HashMap<String, TextoNarrativo>,
    /// Textos de decisiones
    pub decisiones: HashMap<String, TextoNarrativo>,
}

impl Default for TextosConfig {
    fn default() -> Self {
        let mut introducciones = HashMap::new();
        introducciones.insert("inicio".to_string(), TextoNarrativo {
            texto: "Cadiz, {fecha}. La ciudad sitiada respira entre el miedo y la esperanza. Las Cortes se reune en el Oratorio de San Felipe Neri, y tu, {nombre}, llegas con una mision y un nombre que ya susurra en los pasillos del poder.".to_string(),
            variables: HashMap::from([
                ("fecha".to_string(), "24 de septiembre de 1810".to_string()),
                ("nombre".to_string(), "el diputado".to_string()),
            ]),
        });

        Self {
            introducciones,
            eventos: HashMap::new(),
            espacios: HashMap::new(),
            facciones: HashMap::new(),
            decisiones: HashMap::new(),
        }
    }
}