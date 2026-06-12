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

/// Tipo de placeholder visual
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TipoPlaceholder {
    /// Fondo con color sólido y texto descriptivo
    Fondo,
    /// Localización con icono simple + texto
    Localizacion,
    /// Personaje con silueta + texto
    Personaje,
}

impl Default for TipoPlaceholder {
    fn default() -> Self {
        Self::Fondo
    }
}

/// Configuración para generación de placeholders
#[derive(Debug, Clone)]
pub struct ConfiguracionPlaceholder {
    /// Ancho de la imagen en pixels
    pub ancho: u32,
    /// Alto de la imagen en pixels
    pub alto: u32,
    /// Color de fondo (hex: #RRGGBB)
    pub color_fondo: String,
    /// Color del texto (hex: #RRGGBB)
    pub color_texto: String,
    /// Fuente a usar
    pub fuente: String,
}

impl Default for ConfiguracionPlaceholder {
    fn default() -> Self {
        Self {
            ancho: 800,
            alto: 600,
            color_fondo: "#1a1a2e".to_string(),
            color_texto: "#ffffff".to_string(),
            fuente: "DejaVu Sans".to_string(),
        }
    }
}

/// Cache de imágenes para evitar regeneración
pub struct CacheImagenes {
    /// Directorio base para almacenar imágenes
    pub directorio: std::path::PathBuf,
    /// Tiempo de vida (TTL) en segundos
    pub ttl: std::time::Duration,
    /// Tamaño máximo del cache en bytes
    pub max_tamano: usize,
}

impl CacheImagenes {
    /// Crear un nuevo cache de imágenes
    pub fn nuevo(directorio: &str, ttl_seconds: u64, max_tamano_mb: usize) -> Self {
        Self {
            directorio: std::path::PathBuf::from(directorio),
            ttl: std::time::Duration::from_secs(ttl_seconds),
            max_tamano: max_tamano_mb * 1024 * 1024, // Convertir MB a bytes
        }
    }

    /// Obtener una imagen del cache o generarla si no existe
    pub fn obtener_o_generar<F, E>(
        &self,
        clave: &str,
        generador: F,
    ) -> Result<std::path::PathBuf, E>
    where
        F: FnOnce() -> Result<Vec<u8>, E>,
        E: std::fmt::Debug + From<std::io::Error>,
    {
        let ruta_archivo = self.directorio.join(format!("{}.png", clave));
        
        // Verificar si el archivo existe y no ha expirado
        if ruta_archivo.exists() {
            // Verificar TTL
            let metadata = std::fs::metadata(&ruta_archivo)?;
            let modificado = metadata.modified()?;
            let ahora = std::time::SystemTime::now();
            let duracion = ahora.duration_since(modificado).unwrap_or_default();
            
            if duracion < self.ttl {
                return Ok(ruta_archivo);
            }
        }
        
        // Generar la imagen
        let imagen_bytes = generador()?;
        
        // Asegurar que el directorio existe
        std::fs::create_dir_all(&self.directorio)?;
        
        // Guardar la imagen
        std::fs::write(&ruta_archivo, imagen_bytes)?;
        
        Ok(ruta_archivo)
    }

    /// Limpiar el cache
    pub fn limpiar(&self) -> Result<usize, std::io::Error> {
        let mut eliminado = 0;
        
        if self.directorio.exists() {
            for entry in std::fs::read_dir(&self.directorio)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() {
                    let metadata = std::fs::metadata(&path)?;
                    let modificado = metadata.modified()?;
                    let ahora = std::time::SystemTime::now();
                    let duracion = ahora.duration_since(modificado).unwrap_or_default();
                    
                    if duracion >= self.ttl {
                        std::fs::remove_file(&path)?;
                        eliminado += 1;
                    }
                }
            }
        }
        
        Ok(eliminado)
    }

    /// Obtener tamaño actual del cache en bytes
    pub fn tamano_actual(&self) -> Result<usize, std::io::Error> {
        let mut tamano_total: usize = 0;
        
        if self.directorio.exists() {
            for entry in std::fs::read_dir(&self.directorio)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() {
                    tamano_total += std::fs::metadata(&path)?.len() as usize;
                }
            }
        }
        
        Ok(tamano_total)
    }
}

/// Generar un placeholder visual
pub fn generar_placeholder(
    tipo: TipoPlaceholder,
    texto: &str,
    config: &ConfiguracionPlaceholder,
) -> Result<Vec<u8>, image::ImageError> {
    use image::RgbImage;
    
    // Convertir color hex a RGB
    let color_fondo = parsear_color_hex(&config.color_fondo);
    let color_texto = parsear_color_hex(&config.color_texto);
    
    // Crear imagen con fondo
    let mut imagen = RgbImage::new(config.ancho, config.alto);
    
    // Rellenar con color de fondo
    for pixel in imagen.pixels_mut() {
        *pixel = color_fondo;
    }
    
    // Dibujar icono simple según el tipo
    match tipo {
        TipoPlaceholder::Fondo => {
            // Solo texto centrado
        }
        TipoPlaceholder::Localizacion => {
            // Dibujar un círculo simple como icono de localización
            let center_x = config.ancho / 2;
            let center_y = config.alto / 2;
            let radius = (config.ancho.min(config.alto) / 4) as i32;
            
            // Dibujar círculo
            for y in (center_y as i32 - radius)..(center_y as i32 + radius) {
                for x in (center_x as i32 - radius)..(center_x as i32 + radius) {
                    if x >= 0 && x < config.ancho as i32 && y >= 0 && y < config.alto as i32 {
                        let dx = x - center_x as i32;
                        let dy = y - center_y as i32;
                        if dx * dx + dy * dy <= radius * radius {
                            imagen.put_pixel(x as u32, y as u32, color_texto);
                        }
                    }
                }
            }
        }
        TipoPlaceholder::Personaje => {
            // Dibujar silueta de persona (cabeza y cuerpo)
            let head_radius = (config.ancho.min(config.alto) / 8) as i32;
            let body_height = (config.alto / 3) as i32;
            let center_x = config.ancho / 2;
            let head_y = (config.alto / 3) as i32;
            
            // Cabeza
            for y in (head_y - head_radius)..(head_y + head_radius) {
                for x in (center_x as i32 - head_radius)..(center_x as i32 + head_radius) {
                    if x >= 0 && x < config.ancho as i32 && y >= 0 && y < config.alto as i32 {
                        let dx = x - center_x as i32;
                        let dy = y - head_y;
                        if dx * dx + dy * dy <= head_radius * head_radius {
                            imagen.put_pixel(x as u32, y as u32, color_texto);
                        }
                    }
                }
            }
            
            // Cuerpo (línea vertical)
            for y in head_y..(head_y + body_height) {
                if y >= 0 && y < config.alto as i32 {
                    imagen.put_pixel(center_x, y as u32, color_texto);
                    imagen.put_pixel(center_x + 1, y as u32, color_texto);
                }
            }
        }
    }
    
    // Dibujar texto
    // Para simplificar, dibujamos el texto como pixels (aproximación)
    dibujar_texto_simple(&mut imagen, texto, config.ancho / 2, config.alto * 3 / 4, color_texto);
    
    // Convertir a PNG
    use std::io::Cursor;
    let mut bytes = Cursor::new(Vec::new());
    imagen.write_to(&mut bytes, image::ImageOutputFormat::Png)?;
    Ok(bytes.into_inner())
}

/// Dibujar texto de forma simple (sin dependencia de font)
fn dibujar_texto_simple(imagen: &mut image::RgbImage, texto: &str, x: u32, y: u32, color: image::Rgb<u8>) {
    // Dibujar cada carácter como un bloque simple
    let char_width = 8;
    let char_height = 12;
    let mut current_x = x as i32;
    
    for _c in texto.chars() {
        // Dibujar carácter como un rectángulo simple
        for dy in 0..char_height as i32 {
            for dx in 0..char_width as i32 {
                let px = current_x + dx;
                let py = y as i32 + dy;
                if px >= 0 && px < imagen.width() as i32 && py >= 0 && py < imagen.height() as i32 {
                    imagen.put_pixel(px as u32, py as u32, color);
                }
            }
        }
        current_x += char_width as i32 + 2; // Espacio entre caracteres
    }
}

/// Parsear color hex #RRGGBB a Rgb<u8>
fn parsear_color_hex(color: &str) -> image::Rgb<u8> {
    use image::Rgb;
    let color = color.trim_start_matches('#');
    let r = u8::from_str_radix(&color[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&color[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&color[4..6], 16).unwrap_or(0);
    Rgb([r, g, b])
}

/// Generar placeholder para fondo
pub fn generar_placeholder_fondo(texto: &str) -> Result<Vec<u8>, image::ImageError> {
    generar_placeholder(TipoPlaceholder::Fondo, texto, &ConfiguracionPlaceholder::default())
}

/// Generar placeholder para localización
pub fn generar_placeholder_localizacion(texto: &str) -> Result<Vec<u8>, image::ImageError> {
    generar_placeholder(TipoPlaceholder::Localizacion, texto, &ConfiguracionPlaceholder::default())
}

/// Generar placeholder para personaje
pub fn generar_placeholder_personaje(texto: &str) -> Result<Vec<u8>, image::ImageError> {
    generar_placeholder(TipoPlaceholder::Personaje, texto, &ConfiguracionPlaceholder::default())
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

    #[test]
    fn test_configuracion_placeholder_default() {
        let config = ConfiguracionPlaceholder::default();
        assert_eq!(config.ancho, 800);
        assert_eq!(config.alto, 600);
        assert_eq!(config.color_fondo, "#1a1a2e");
        assert_eq!(config.color_texto, "#ffffff");
    }

    #[test]
    fn test_parsear_color_hex() {
        let color = parsear_color_hex("#ff0000");
        assert_eq!(color[0], 255);
        assert_eq!(color[1], 0);
        assert_eq!(color[2], 0);
        
        let color = parsear_color_hex("#00ff00");
        assert_eq!(color[0], 0);
        assert_eq!(color[1], 255);
        assert_eq!(color[2], 0);
    }

    #[test]
    fn test_generar_placeholder_fondo() {
        let resultado = generar_placeholder_fondo("Fondo: Cortes de Cadiz");
        assert!(resultado.is_ok());
        let bytes = resultado.unwrap();
        assert!(bytes.len() > 0);
    }

    #[test]
    fn test_generar_placeholder_localizacion() {
        let resultado = generar_placeholder_localizacion("Localizacion: Mesa de Debate");
        assert!(resultado.is_ok());
        let bytes = resultado.unwrap();
        assert!(bytes.len() > 0);
    }

    #[test]
    fn test_generar_placeholder_personaje() {
        let resultado = generar_placeholder_personaje("Personaje: Jurista");
        assert!(resultado.is_ok());
        let bytes = resultado.unwrap();
        assert!(bytes.len() > 0);
    }

    #[test]
    fn test_cache_imagenes_nuevo() {
        let cache = CacheImagenes::nuevo("cache", 3600, 100);
        assert_eq!(cache.directorio, std::path::PathBuf::from("cache"));
        assert_eq!(cache.ttl, std::time::Duration::from_secs(3600));
        assert_eq!(cache.max_tamano, 100 * 1024 * 1024);
    }
}
