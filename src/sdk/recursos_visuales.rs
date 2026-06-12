use serde::{Serialize, Deserialize};
use std::path::PathBuf;

/// Tipo de recurso visual
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TipoRecursoVisual {
    /// Imagen de fondo o ilustracion
    Imagen,
    /// Sprite de personaje o elemento
    Sprite,
    /// Icono para UI
    Icono,
    /// Animacion
    Animacion,
    /// Textura 3D
    Textura3D,
    /// Modelo 3D
    Modelo3D,
    /// Fuente tipografica
    Fuente,
    /// Video
    Video,
    /// Audio
    Audio,
}

impl Default for TipoRecursoVisual {
    fn default() -> Self {
        Self::Imagen
    }
}

/// Recurso visual (imagen, sprite, icono, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursoVisual {
    /// Identificador unico del recurso
    pub id: String,
    /// Nombre descriptivo
    pub nombre: String,
    /// Tipo de recurso
    pub tipo: TipoRecursoVisual,
    /// Ruta al archivo del recurso
    pub ruta: PathBuf,
    /// Ancho en pixels (si aplica)
    pub ancho: Option<u32>,
    /// Alto en pixels (si aplica)
    pub alto: Option<u32>,
    /// Formato del archivo
    pub formato: String,
    /// Tags para busqueda y categorizacion
    pub tags: Vec<String>,
    /// Metadatos adicionales
    pub metadatos: serde_json::Value,
}

impl RecursoVisual {
    pub fn nuevo(
        id: &str,
        nombre: &str,
        tipo: TipoRecursoVisual,
        ruta: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            nombre: nombre.to_string(),
            tipo,
            ruta: PathBuf::from(ruta),
            ancho: None,
            alto: None,
            formato: Self::inferir_formato(ruta),
            tags: vec![],
            metadatos: serde_json::Value::Null,
        }
    }

    pub fn con_dimensiones(mut self, ancho: u32, alto: u32) -> Self {
        self.ancho = Some(ancho);
        self.alto = Some(alto);
        self
    }

    pub fn con_tags(mut self, tags: Vec<&str>) -> Self {
        self.tags = tags.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Inferir formato del archivo desde la extension
    fn inferir_formato(ruta: &str) -> String {
        let ruta_lower = ruta.to_lowercase();
        if ruta_lower.ends_with(".png") {
            "png".to_string()
        } else if ruta_lower.ends_with(".jpg") || ruta_lower.ends_with(".jpeg") {
            "jpg".to_string()
        } else if ruta_lower.ends_with(".gif") {
            "gif".to_string()
        } else if ruta_lower.ends_with(".svg") {
            "svg".to_string()
        } else if ruta_lower.ends_with(".webp") {
            "webp".to_string()
        } else if ruta_lower.ends_with(".mp3") {
            "mp3".to_string()
        } else if ruta_lower.ends_with(".wav") {
            "wav".to_string()
        } else if ruta_lower.ends_with(".ogg") {
            "ogg".to_string()
        } else {
            "desconocido".to_string()
        }
    }

    /// Validar que el recurso es accesible
    pub fn validar(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("El ID no puede estar vacio".to_string());
        }
        if self.nombre.is_empty() {
            return Err("El nombre no puede estar vacio".to_string());
        }
        if self.ruta.to_string_lossy().is_empty() {
            return Err("La ruta no puede estar vacia".to_string());
        }
        Ok(())
    }
}

impl Default for RecursoVisual {
    fn default() -> Self {
        Self {
            id: "recurso_vacio".to_string(),
            nombre: "Recurso sin nombre".to_string(),
            tipo: TipoRecursoVisual::Imagen,
            ruta: PathBuf::new(),
            ancho: None,
            alto: None,
            formato: "desconocido".to_string(),
            tags: vec![],
            metadatos: serde_json::Value::Null,
        }
    }
}

/// Evento visual que requiere recursos graficos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventoVisual {
    /// Identificador del evento
    pub evento_id: String,
    /// Titulo del evento
    pub titulo: String,
    /// Descripcion del evento
    pub descripcion: String,
    /// Recurso visual principal (fondo o ilustracion)
    pub recurso_principal: Option<RecursoVisual>,
    /// Recursos adicionales (personajes, objetos, etc.)
    pub recursos_secundarios: Vec<RecursoVisual>,
    /// Posicion de la camara o viewport
    pub configuracion_visual: ConfiguracionVisual,
    /// Transiciones y animaciones
    pub transiciones: Vec<TransicionVisual>,
}

impl EventoVisual {
    pub fn nuevo(evento_id: &str, titulo: &str, descripcion: &str) -> Self {
        Self {
            evento_id: evento_id.to_string(),
            titulo: titulo.to_string(),
            descripcion: descripcion.to_string(),
            recurso_principal: None,
            recursos_secundarios: vec![],
            configuracion_visual: ConfiguracionVisual::default(),
            transiciones: vec![],
        }
    }

    pub fn con_recurso_principal(mut self, recurso: RecursoVisual) -> Self {
        self.recurso_principal = Some(recurso);
        self
    }

    pub fn con_recursos_secundarios(mut self, recursos: Vec<RecursoVisual>) -> Self {
        self.recursos_secundarios = recursos;
        self
    }

    pub fn con_transicion(mut self, transicion: TransicionVisual) -> Self {
        self.transiciones.push(transicion);
        self
    }
}

impl Default for EventoVisual {
    fn default() -> Self {
        Self {
            evento_id: "evento_vacio".to_string(),
            titulo: "Evento sin titulo".to_string(),
            descripcion: "".to_string(),
            recurso_principal: None,
            recursos_secundarios: vec![],
            configuracion_visual: ConfiguracionVisual::default(),
            transiciones: vec![],
        }
    }
}

/// Configuracion visual para eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfiguracionVisual {
    /// Posicion X de la camara (0.0 a 1.0)
    pub camara_x: f32,
    /// Posicion Y de la camara (0.0 a 1.0)
    pub camara_y: f32,
    /// Zoom de la camara
    pub zoom: f32,
    /// Color de fondo (hex: #RRGGBB o nombre)
    pub color_fondo: String,
    /// Brillo (0.0 a 1.0)
    pub brillo: f32,
    /// Contraste (0.0 a 2.0)
    pub contraste: f32,
    /// Saturacion (0.0 a 1.0)
    pub saturacion: f32,
}

impl Default for ConfiguracionVisual {
    fn default() -> Self {
        Self {
            camara_x: 0.5,
            camara_y: 0.5,
            zoom: 1.0,
            color_fondo: "#000000".to_string(),
            brillo: 1.0,
            contraste: 1.0,
            saturacion: 1.0,
        }
    }
}

/// Transicion visual entre estados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransicionVisual {
    /// Tipo de transicion
    pub tipo: TipoTransicion,
    /// Duracion en milisegundos
    pub duracion_ms: u32,
    /// Easing function (ej: "linear", "ease_in", "ease_out", "ease_in_out")
    pub easing: String,
    /// Color de transicion (opcional)
    pub color: Option<String>,
    /// Opacidad inicial (0.0 a 1.0)
    pub opacidad_inicial: f32,
    /// Opacidad final (0.0 a 1.0)
    pub opacidad_final: f32,
}

impl TransicionVisual {
    pub fn nueva(tipo: TipoTransicion, duracion_ms: u32) -> Self {
        Self {
            tipo,
            duracion_ms,
            easing: "linear".to_string(),
            color: None,
            opacidad_inicial: 0.0,
            opacidad_final: 1.0,
        }
    }

    pub fn con_easing(mut self, easing: &str) -> Self {
        self.easing = easing.to_string();
        self
    }

    pub fn con_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn con_opacidad(mut self, inicial: f32, final_: f32) -> Self {
        self.opacidad_inicial = inicial;
        self.opacidad_final = final_;
        self
    }
}

impl Default for TransicionVisual {
    fn default() -> Self {
        Self {
            tipo: TipoTransicion::Fade,
            duracion_ms: 300,
            easing: "linear".to_string(),
            color: None,
            opacidad_inicial: 0.0,
            opacidad_final: 1.0,
        }
    }
}

/// Tipo de transicion visual
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TipoTransicion {
    /// Fundido de entrada/salida
    Fade,
    /// Deslizamiento horizontal
    SlideHorizontal,
    /// Deslizamiento vertical
    SlideVertical,
    /// Zoom
    Zoom,
    /// Rotacion
    Rotate,
    /// Fundido a color
    FadeToColor,
    /// Cortinilla
    Wipe,
    /// Desvanecimiento con color
    CrossFade,
}

impl Default for TipoTransicion {
    fn default() -> Self {
        Self::Fade
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurso_visual_creacion() {
        let recurso = RecursoVisual::nuevo("img1", "Imagen 1", TipoRecursoVisual::Imagen, "assets/img1.png");
        assert_eq!(recurso.id, "img1");
        assert_eq!(recurso.nombre, "Imagen 1");
        assert_eq!(recurso.tipo, TipoRecursoVisual::Imagen);
        assert_eq!(recurso.formato, "png");
    }

    #[test]
    fn test_recurso_visual_con_dimensiones() {
        let recurso = RecursoVisual::nuevo("img1", "Imagen 1", TipoRecursoVisual::Imagen, "assets/img1.png")
            .con_dimensiones(800, 600);
        assert_eq!(recurso.ancho, Some(800));
        assert_eq!(recurso.alto, Some(600));
    }

    #[test]
    fn test_evento_visual_creacion() {
        let evento = EventoVisual::nuevo("evento1", "Evento 1", "Descripcion del evento");
        assert_eq!(evento.evento_id, "evento1");
        assert_eq!(evento.titulo, "Evento 1");
        assert!(evento.recurso_principal.is_none());
        assert!(evento.recursos_secundarios.is_empty());
    }

    #[test]
    fn test_configuracion_visual_default() {
        let config = ConfiguracionVisual::default();
        assert_eq!(config.camara_x, 0.5);
        assert_eq!(config.camara_y, 0.5);
        assert_eq!(config.zoom, 1.0);
        assert_eq!(config.color_fondo, "#000000");
    }

    #[test]
    fn test_transicion_visual_default() {
        let trans = TransicionVisual::default();
        assert_eq!(trans.tipo, TipoTransicion::Fade);
        assert_eq!(trans.duracion_ms, 300);
        assert_eq!(trans.easing, "linear");
    }

    #[test]
    fn test_inferir_formato() {
        assert_eq!(RecursoVisual::inferir_formato("image.png"), "png");
        assert_eq!(RecursoVisual::inferir_formato("image.jpg"), "jpg");
        assert_eq!(RecursoVisual::inferir_formato("sound.mp3"), "mp3");
        assert_eq!(RecursoVisual::inferir_formato("unknown.xyz"), "desconocido");
    }
}
