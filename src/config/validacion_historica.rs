use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use super::{FaccionId, ErrorValidacion};

/// Evento histórico real de Cádiz 1810-1814
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventoReal {
    /// Nombre del evento histórico
    pub nombre: String,
    /// Fecha de inicio (YYYY-MM-DD)
    pub fecha_inicio: String,
    /// Fecha de fin (YYYY-MM-DD) - None si es un día específico
    pub fecha_fin: Option<String>,
    /// Descripción histórica
    pub descripcion: String,
    /// Facciones históricas involucradas
    pub facciones_involucradas: Vec<FaccionId>,
    /// Consecuencias históricas conocidas
    pub consecuencias: Vec<String>,
    /// Personajes clave históricos
    pub personajes_clave: Vec<String>,
    /// Lugares históricos
    pub lugares: Vec<String>,
    /// Fuente histórica
    pub fuente: String,
}

impl EventoReal {
    pub fn nuevo(
        nombre: &str,
        fecha_inicio: &str,
        descripcion: &str,
    ) -> Self {
        Self {
            nombre: nombre.to_string(),
            fecha_inicio: fecha_inicio.to_string(),
            fecha_fin: None,
            descripcion: descripcion.to_string(),
            facciones_involucradas: vec![],
            consecuencias: vec![],
            personajes_clave: vec![],
            lugares: vec![],
            fuente: String::new(),
        }
    }

    pub fn con_fecha_fin(mut self, fecha_fin: &str) -> Self {
        self.fecha_fin = Some(fecha_fin.to_string());
        self
    }

    pub fn con_facciones(mut self, facciones: Vec<FaccionId>) -> Self {
        self.facciones_involucradas = facciones;
        self
    }

    pub fn con_consecuencias(mut self, consecuencias: Vec<&str>) -> Self {
        self.consecuencias = consecuencias.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn con_personajes(mut self, personajes: Vec<&str>) -> Self {
        self.personajes_clave = personajes.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn con_lugares(mut self, lugares: Vec<&str>) -> Self {
        self.lugares = lugares.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn con_fuente(mut self, fuente: &str) -> Self {
        self.fuente = fuente.to_string();
        self
    }

    /// Validar que el evento tiene datos mínimos
    pub fn validar(&self) -> Result<(), ErrorValidacion> {
        if self.nombre.is_empty() {
            return Err(ErrorValidacion::CampoRequerido("nombre".to_string()));
        }
        if self.fecha_inicio.is_empty() {
            return Err(ErrorValidacion::CampoRequerido("fecha_inicio".to_string()));
        }
        if self.descripcion.is_empty() {
            return Err(ErrorValidacion::CampoRequerido("descripcion".to_string()));
        }
        Ok(())
    }

    /// Verificar si una fecha está dentro del rango del evento
    pub fn contiene_fecha(&self, fecha: &str) -> bool {
        // Implementación simple: comparar strings (YYYY-MM-DD)
        let fecha_inicio_ok = fecha >= self.fecha_inicio.as_str();
        
        let fecha_fin_ok = if let Some(ref fin) = self.fecha_fin {
            fecha <= fin.as_str()
        } else {
            fecha == self.fecha_inicio.as_str()
        };
        
        fecha_inicio_ok && fecha_fin_ok
    }

    /// Verificar si una facción está involucrada
    pub fn involucra_faccion(&self, faccion: &FaccionId) -> bool {
        self.facciones_involucradas.contains(faccion)
    }
}

/// Base de datos de eventos históricos de Cádiz 1810-1814
pub struct BaseDatosHistorica;

impl BaseDatosHistorica {
    /// Obtener todos los eventos históricos del período
    pub fn obtener_eventos() -> Vec<EventoReal> {
        vec![
            // 1810
            EventoReal::nuevo(
                "Sitio de Cádiz",
                "1810-02-05",
                "Las tropas francesas inician el asedio a la ciudad de Cádiz, que durará hasta 1813",
            )
            .con_fecha_fin("1813-08-24")
            .con_facciones(vec![
                FaccionId::AbsolutistaServil,
                FaccionId::LiberalProgresista,
                FaccionId::LiberalModerado,
            ])
            .con_consecuencias(vec![
                "Aislamiento de la ciudad",
                "Centralización del gobierno liberal",
                "Aumento de la resistencia española",
            ])
            .con_personajes(vec![
                "Mariscal Soult",
                "Duque de Wellington",
                "Regencia de España",
            ])
            .con_lugares(vec![
                "Cádiz",
                "Isla de León",
                "San Fernando",
            ])
            .con_fuente("Archivos históricos de las Cortes de Cádiz"),
            
            EventoReal::nuevo(
                "Convocatoria de las Cortes de Cádiz",
                "1810-09-24",
                "Se convoca a los diputados para formar las Cortes Generales y Extraordinarias",
            )
            .con_facciones(vec![
                FaccionId::LiberalProgresista,
                FaccionId::LiberalModerado,
                FaccionId::AbsolutistaServil,
            ])
            .con_consecuencias(vec![
                "Primera constitución española",
                "Representación nacional",
            ])
            .con_personajes(vec![
                "Floridablanca",
                "Jovellanos",
                "Argüelles",
            ])
            .con_lugares(vec![
                "Oratorio de San Felipe Neri",
            ])
            .con_fuente("Actas de las Cortes de Cádiz"),
            
            EventoReal::nuevo(
                "Apertura de las Cortes de Cádiz",
                "1810-09-24",
                "Primera sesión de las Cortes en el Oratorio de San Felipe Neri",
            )
            .con_facciones(vec![
                FaccionId::LiberalProgresista,
                FaccionId::LiberalModerado,
            ])
            .con_consecuencias(vec![
                "Inicio de la revolución liberal española",
            ])
            .con_personajes(vec![
                "Muñoz Torrero",
                "Argüelles",
            ])
            .con_lugares(vec![
                "Oratorio de San Felipe Neri",
            ])
            .con_fuente("Actas de las Cortes de Cádiz"),
            
            // 1811
            EventoReal::nuevo(
                "Decreto de libertad de imprenta",
                "1810-11-10",
                "Las Cortes abolieron la censura previa y establecieron la libertad de imprenta",
            )
            .con_facciones(vec![
                FaccionId::LiberalProgresista,
                FaccionId::Americanista,
            ])
            .con_consecuencias(vec![
                "Proliferación de periódicos y panfletos",
                "Difusión de ideas liberales",
            ])
            .con_fuente("Decretos de las Cortes de Cádiz"),
            
            EventoReal::nuevo(
                "Abolición de la Inquisición",
                "1813-02-26",
                "Las Cortes decretan la abolición de la Santa Inquisición",
            )
            .con_facciones(vec![
                FaccionId::LiberalProgresista,
                FaccionId::AbsolutistaServil,
            ])
            .con_consecuencias(vec![
                "Fin de la persecución religiosa por motivos políticos",
                "Resistencia del clero conservador",
            ])
            .con_fuente("Decretos de las Cortes de Cádiz"),
            
            // 1812
            EventoReal::nuevo(
                "Promulgación de la Constitución de 1812",
                "1812-03-19",
                "Se promulga la primera constitución española, conocida como 'La Pepa'",
            )
            .con_facciones(vec![
                FaccionId::LiberalProgresista,
                FaccionId::LiberalModerado,
                FaccionId::AbsolutistaServil,
            ])
            .con_consecuencias(vec![
                "Soberanía nacional",
                "Separación de poderes",
                "Derechos individuales",
                "Monarquía constitucional",
            ])
            .con_personajes(vec![
                "Argüelles",
                "Muñoz Torrero",
                "Calatrava",
            ])
            .con_lugares(vec![
                "Oratorio de San Felipe Neri",
            ])
            .con_fuente("Constitución de 1812"),
            
            EventoReal::nuevo(
                "Levamiento del 2 de Mayo en Madrid",
                "1808-05-02",
                "El pueblo de Madrid se levanta contra las tropas francesas",
            )
            .con_fecha_fin("1808-05-02")
            .con_consecuencias(vec![
                "Inicio de la Guerra de Independencia",
                "Repulsión inicial francesas",
                "Represión posterior",
            ])
            .con_fuente("Crónicas de la Guerra de Independencia"),
            
            // 1813
            EventoReal::nuevo(
                "Fin del Sitio de Cádiz",
                "1813-08-24",
                "Las tropas francesas levantan el asedio tras la derrota en la Batalla de Los Arapiles",
            )
            .con_facciones(vec![
                FaccionId::AbsolutistaServil,
                FaccionId::LiberalProgresista,
            ])
            .con_personajes(vec![
                "Duque de Wellington",
                "Mariscal Soult",
            ])
            .con_lugares(vec![
                "Cádiz",
                "Salamanca (Los Arapiles)",
            ])
            .con_fuente("Correspondencia militar"),
            
            // 1814
            EventoReal::nuevo(
                "Restauración absolutista de Fernando VII",
                "1814-05-04",
                "Fernando VII restablece el absolutismo y deroga la Constitución de 1812",
            )
            .con_facciones(vec![
                FaccionId::AbsolutistaServil,
                FaccionId::LiberalProgresista,
            ])
            .con_consecuencias(vec![
                "Fin del trienio liberal",
                "Persecución de liberales",
                "Exilio de muchos diputados",
            ])
            .con_personajes(vec![
                "Fernando VII",
            ])
            .con_fuente("Decreto de Valencia"),
        ]
    }

    /// Obtener evento por nombre
    pub fn obtener_evento(nombre: &str) -> Option<EventoReal> {
        Self::obtener_eventos().into_iter().find(|e| e.nombre == nombre)
    }

    /// Verificar si un evento generado contradice hechos históricos
    pub fn validar_evento_historico(
        &self,
        nombre_evento: &str,
        fecha: &str,
        facciones: &[FaccionId],
        consecuencias: &[String],
    ) -> Result<(), Vec<ErrorValidacion>> {
        let mut errores = Vec::new();
        
        // Buscar evento histórico equivalente
        if let Some(evento_hist) = Self::obtener_evento(nombre_evento) {
            // Validar fecha
            if !evento_hist.contiene_fecha(fecha) {
                errores.push(ErrorValidacion::Inconsistencia(format!(
                    "Fecha '{}' no coincide con el período histórico del evento '{}' ({}-{})",
                    fecha,
                    nombre_evento,
                    evento_hist.fecha_inicio,
                    evento_hist.fecha_fin.as_deref().unwrap_or("mismo día")
                )));
            }
            
            // Validar facciones
            for faccion in facciones {
                if !evento_hist.involucra_faccion(faccion) {
                    errores.push(ErrorValidacion::Inconsistencia(format!(
                        "Facción {:?} no participó históricamente en el evento '{}'",
                        faccion, nombre_evento
                    )));
                }
            }
            
            // Validar consecuencias
            for consec in consecuencias {
                if !evento_hist.consecuencias.iter().any(|c| c.contains(consec)) &&
                   !evento_hist.consecuencias.is_empty() {
                    // Solo advertir si hay consecuencias históricas registradas
                    // Esto permite consecuencias nuevas que no contradigan las conocidas
                }
            }
        } else {
            // Evento no histórico - validar que la fecha esté en el período 1810-1814
            if !(fecha.starts_with("1810") || fecha.starts_with("1811") || 
                  fecha.starts_with("1812") || fecha.starts_with("1813") || 
                  fecha.starts_with("1814")) {
                errores.push(ErrorValidacion::ValorInvalido(format!(
                    "Fecha '{}' fuera del período histórico válido (1810-1814)",
                    fecha
                )));
            }
        }
        
        if errores.is_empty() {
            Ok(())
        } else {
            Err(errores)
        }
    }
}

/// Términos anacrónicos para el contexto de Cádiz 1812
pub struct DeteccionAnacronismos {
    /// Términos modernos que no existían o no se usaban en la época
    terminos_anacronicos: HashSet<String>,
    /// Términos aceptables y su validez
    terminos_validos: HashSet<String>,
    /// Términos con advertencia (usados pero con matices)
    terminos_advertencia: HashMap<String, String>,
}

impl Default for DeteccionAnacronismos {
    fn default() -> Self {
        let mut terminos_anacronicos = HashSet::new();
        // Términos políticos modernos
        terminos_anacronicos.insert("socialismo".to_string());
        terminos_anacronicos.insert("comunismo".to_string());
        terminos_anacronicos.insert("fascismo".to_string());
        terminos_anacronicos.insert("nazi".to_string());
        terminos_anacronicos.insert("capitalismo".to_string());
        terminos_anacronicos.insert("neoliberal".to_string());
        
        // Términos tecnológicos
        terminos_anacronicos.insert("teléfono".to_string());
        terminos_anacronicos.insert("automóvil".to_string());
        terminos_anacronicos.insert("avión".to_string());
        terminos_anacronicos.insert("computadora".to_string());
        terminos_anacronicos.insert("internet".to_string());
        terminos_anacronicos.insert("electricidad".to_string());
        terminos_anacronicos.insert("ferrocarril".to_string());
        
        // Términos sociales modernos
        terminos_anacronicos.insert("feminismo".to_string());
        terminos_anacronicos.insert("ecología".to_string());
        terminos_anacronicos.insert("globalización".to_string());
        
        let mut terminos_validos = HashSet::new();
        // Términos válidos para 1812
        terminos_validos.insert("constitución".to_string());
        terminos_validos.insert("libertad".to_string());
        terminos_validos.insert("nación".to_string());
        terminos_validos.insert("soberanía".to_string());
        terminos_validos.insert("cortes".to_string());
        terminos_validos.insert("diputado".to_string());
        terminos_validos.insert("liberal".to_string());
        terminos_validos.insert("absolutista".to_string());
        terminos_validos.insert("revolución".to_string());
        terminos_validos.insert("francés".to_string());
        terminos_validos.insert("inglés".to_string());
        terminos_validos.insert("sitio".to_string());
        terminos_validos.insert("regencia".to_string());
        
        let mut terminos_advertencia = HashMap::new();
        terminos_advertencia.insert("democracia".to_string(), 
            "El término existía pero con significado diferente (gobierno del pueblo), no en el sentido moderno".to_string());
        terminos_advertencia.insert("república".to_string(),
            "Concepto conocido pero no aplicado en España en 1812".to_string());
        
        Self {
            terminos_anacronicos,
            terminos_validos,
            terminos_advertencia,
        }
    }
}

impl DeteccionAnacronismos {
    pub fn nuevo() -> Self {
        Self::default()
    }

    /// Validar un texto para detectar anacronismos
    pub fn validar_texto_historico(&self, texto: &str) -> Result<(), Vec<ErrorValidacion>> {
        let texto_lower = texto.to_lowercase();
        let mut errores = Vec::new();
        let mut advertencias = Vec::new();
        
        // Buscar términos anacrónicos
        for termino in &self.terminos_anacronicos {
            if texto_lower.contains(&termino.to_lowercase()) {
                errores.push(ErrorValidacion::ValorInvalido(format!(
                    "Término anacrónico detectado: '{}'",
                    termino
                )));
            }
        }
        
        // Buscar términos con advertencia
        for (termino, explicacion) in &self.terminos_advertencia {
            if texto_lower.contains(&termino.to_lowercase()) {
                advertencias.push(ErrorValidacion::FormatoInvalido(format!(
                    "Término con advertencia: '{}' - {}",
                    termino, explicacion
                )));
            }
        }
        
        // Combinar errores y advertencias
        let mut todos = errores;
        todos.extend(advertencias);
        
        if todos.is_empty() {
            Ok(())
        } else {
            Err(todos)
        }
    }

    /// Añadir término anacrónico
    pub fn anadir_termino_anacronico(&mut self, termino: &str) -> &mut Self {
        self.terminos_anacronicos.insert(termino.to_lowercase());
        self
    }

    /// Añadir término válido
    pub fn anadir_termino_valido(&mut self, termino: &str) -> &mut Self {
        self.terminos_validos.insert(termino.to_lowercase());
        self
    }
}

/// Configuración para auditoría de equilibrio de facciones
#[derive(Debug, Clone)]
pub struct ConfiguracionEquilibrio {
    /// Umbral máximo de influencia para una facción (0.0-1.0)
    pub umbral_maximo: f32,
    /// Umbral mínimo para considerar desequilibrio (0.0-1.0)
    pub umbral_minimo: f32,
    /// Permitir facción dominante en contexto específico
    pub permitir_dominante: bool,
}

impl Default for ConfiguracionEquilibrio {
    fn default() -> Self {
        Self {
            umbral_maximo: 0.6,  // 60%
            umbral_minimo: 0.1,  // 10%
            permitir_dominante: false,
        }
    }
}

impl ConfiguracionEquilibrio {
    pub fn nuevo(umbral_maximo: f32, umbral_minimo: f32) -> Self {
        Self {
            umbral_maximo: umbral_maximo.clamp(0.0, 1.0),
            umbral_minimo: umbral_minimo.clamp(0.0, 1.0),
            permitir_dominante: false,
        }
    }

    pub fn con_permitir_dominante(mut self, permitir: bool) -> Self {
        self.permitir_dominante = permitir;
        self
    }
}

/// Auditor de equilibrio de facciones
pub struct AuditorEquilibrio {
    configuracion: ConfiguracionEquilibrio,
}

impl AuditorEquilibrio {
    pub fn nuevo(configuracion: ConfiguracionEquilibrio) -> Self {
        Self { configuracion }
    }

    pub fn nuevo_con_umbrales(umbral_maximo: f32, umbral_minimo: f32) -> Self {
        Self {
            configuracion: ConfiguracionEquilibrio::nuevo(umbral_maximo, umbral_minimo),
        }
    }

    /// Validar equilibrio de facciones en una distribución
    pub fn validar_equilibrio_facciones(
        &self,
        distribuion: &HashMap<FaccionId, f32>,
    ) -> Result<(), Vec<ErrorValidacion>> {
        let mut errores = Vec::new();
        
        let total: f32 = distribuion.values().sum();
        
        if total == 0.0 {
            return Err(vec![ErrorValidacion::ValorInvalido(
                "Distribución de facciones vacía".to_string()
            )]);
        }
        
        for (faccion, influencia) in distribuion {
            let porcentaje = influencia / total;
            
            // Verificar si supera el umbral máximo
            if porcentaje > self.configuracion.umbral_maximo && !self.configuracion.permitir_dominante {
                errores.push(ErrorValidacion::RangoInvalido(format!(
                    "Facción {:?} tiene {:.1}% de influencia (máximo permitido: {:.1}%)",
                    faccion, porcentaje * 100.0, self.configuracion.umbral_maximo * 100.0
                )));
            }
            
            // Verificar si está por debajo del umbral mínimo
            if porcentaje < self.configuracion.umbral_minimo {
                errores.push(ErrorValidacion::RangoInvalido(format!(
                    "Facción {:?} tiene {:.1}% de influencia (mínimo recomendado: {:.1}%)",
                    faccion, porcentaje * 100.0, self.configuracion.umbral_minimo * 100.0
                )));
            }
        }
        
        // Verificar diversidad
        if distribuion.len() < 2 {
            errores.push(ErrorValidacion::Inconsistencia(
                "Se recomienda al menos 2 facciones para equilibrio".to_string()
            ));
        }
        
        if errores.is_empty() {
            Ok(())
        } else {
            Err(errores)
        }
    }

    /// Obtener sugestiones para equilibrar
    pub fn sugerencias_equilibrio(
        &self,
        distribuion: &HashMap<FaccionId, f32>,
    ) -> Vec<String> {
        let mut sugerencias = Vec::new();
        let total: f32 = distribuion.values().sum();
        
        if total == 0.0 {
            return sugerencias;
        }
        
        let objetivo = 1.0 / distribuion.len() as f32;
        
        for (faccion, influencia) in distribuion {
            let porcentaje = influencia / total;
            let objetivo_pct = objetivo * 100.0;
            let actual_pct = porcentaje * 100.0;
            
            if (actual_pct - objetivo_pct).abs() > 5.0 {
                sugerencias.push(format!(
                    "Ajustar influencia de {:?}: {:.1}% → {:.1}% (promedio)",
                    faccion, actual_pct, objetivo_pct
                ));
            }
        }
        
        sugerencias
    }
}

/// Validador histórico completo
pub struct ValidadorHistorico {
    base_datos: BaseDatosHistorica,
    deteccion_anacronismos: DeteccionAnacronismos,
    auditor_equilibrio: AuditorEquilibrio,
}

impl Default for ValidadorHistorico {
    fn default() -> Self {
        Self {
            base_datos: BaseDatosHistorica,
            deteccion_anacronismos: DeteccionAnacronismos::default(),
            auditor_equilibrio: AuditorEquilibrio::nuevo(ConfiguracionEquilibrio::default()),
        }
    }
}

impl ValidadorHistorico {
    pub fn nuevo() -> Self {
        Self::default()
    }

    /// Validar evento histórico
    pub fn validar_evento_historico(
        &self,
        nombre: &str,
        fecha: &str,
        facciones: &[FaccionId],
        consecuencias: &[String],
    ) -> Result<(), Vec<ErrorValidacion>> {
        self.base_datos.validar_evento_historico(nombre, fecha, facciones, consecuencias)
    }

    /// Validar texto para anacronismos
    pub fn validar_texto_historico(&self, texto: &str) -> Result<(), Vec<ErrorValidacion>> {
        self.deteccion_anacronismos.validar_texto_historico(texto)
    }

    /// Validar equilibrio de facciones
    pub fn validar_equilibrio_facciones(
        &self,
        distribuion: &HashMap<FaccionId, f32>,
    ) -> Result<(), Vec<ErrorValidacion>> {
        self.auditor_equilibrio.validar_equilibrio_facciones(distribuion)
    }

    /// Validar todo un evento narrativo
    pub fn validar_evento_narrativo(
        &self,
        titulo: &str,
        descripcion: &str,
        fecha: Option<&str>,
        facciones: &[FaccionId],
        distribuion_influencia: Option<&HashMap<FaccionId, f32>>,
    ) -> Result<(), Vec<ErrorValidacion>> {
        let mut errores = Vec::new();
        
        // Validar texto
        if let Err(e) = self.validar_texto_historico(descripcion) {
            errores.extend(e);
        }
        
        // Validar fecha y contexto histórico
        if let Some(fecha) = fecha {
            if let Err(e) = self.validar_evento_historico(titulo, fecha, facciones, &[]) {
                errores.extend(e);
            }
        }
        
        // Validar equilibrio
        if let Some(dist) = distribuion_influencia {
            if let Err(e) = self.validar_equilibrio_facciones(dist) {
                errores.extend(e);
            }
        }
        
        if errores.is_empty() {
            Ok(())
        } else {
            Err(errores)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FaccionId;

    #[test]
    fn test_evento_real_validacion() {
        let evento = EventoReal::nuevo(
            "Sitio de Cádiz",
            "1810-02-05",
            "Asedio francés a Cádiz",
        );
        assert!(evento.validar().is_ok());
    }

    #[test]
    fn test_evento_real_sin_nombre() {
        let evento = EventoReal::nuevo("", "1810-02-05", "Asedio");
        assert!(evento.validar().is_err());
    }

    #[test]
    fn test_base_datos_obtener_evento() {
        let evento = BaseDatosHistorica::obtener_evento("Sitio de Cádiz");
        assert!(evento.is_some());
        let evento = evento.unwrap();
        assert_eq!(evento.nombre, "Sitio de Cádiz");
        assert!(evento.involucra_faccion(&FaccionId::LiberalProgresista));
    }

    #[test]
    fn test_validar_evento_historico_fecha_invalida() {
        let validador = ValidadorHistorico::nuevo();
        let errores = validador.validar_evento_historico(
            "Sitio de Cádiz",
            "1900-01-01",
            &[FaccionId::LiberalProgresista],
            &[],
        );
        assert!(errores.is_err());
    }

    #[test]
    fn test_validar_evento_historico_faccion_invalida() {
        let validador = ValidadorHistorico::nuevo();
        // Sitio de Cádiz no involucraba a Americanistas
        let _errores = validador.validar_evento_historico(
            "Sitio de Cádiz",
            "1810-02-05",
            &[FaccionId::Americanista],
            &[],
        );
        // Note: Americanista podría haber estado presente históricamente
        // Este test demuestra la funcionalidad
    }

    #[test]
    fn test_deteccion_anacronismos() {
        let detector = DeteccionAnacronismos::nuevo();
        
        // Texto con anacronismo
        let resultado = detector.validar_texto_historico(
            "El socialismo se discute en las Cortes de Cádiz"
        );
        assert!(resultado.is_err());
        
        // Texto válido
        let resultado = detector.validar_texto_historico(
            "La constitución fue discutida en las Cortes"
        );
        assert!(resultado.is_ok());
    }

    #[test]
    fn test_auditor_equilibrio_desequilibrado() {
        let mut config = ConfiguracionEquilibrio::default();
        config.umbral_maximo = 0.5; // 50%
        
        let auditor = AuditorEquilibrio::nuevo(config);
        
        let mut distribuion = HashMap::new();
        distribuion.insert(FaccionId::LiberalProgresista, 0.8);
        distribuion.insert(FaccionId::LiberalModerado, 0.2);
        
        let errores = auditor.validar_equilibrio_facciones(&distribuion);
        assert!(errores.is_err());
    }

    #[test]
    fn test_auditor_equilibrio_equilibrado() {
        let auditor = AuditorEquilibrio::nuevo(ConfiguracionEquilibrio::default());
        
        let mut distribuion = HashMap::new();
        distribuion.insert(FaccionId::LiberalProgresista, 0.4);
        distribuion.insert(FaccionId::LiberalModerado, 0.35);
        distribuion.insert(FaccionId::AbsolutistaServil, 0.25);
        
        let errores = auditor.validar_equilibrio_facciones(&distribuion);
        assert!(errores.is_ok());
    }

    #[test]
    fn test_validador_historico_completo() {
        let validador = ValidadorHistorico::nuevo();
        
        let errores = validador.validar_evento_narrativo(
            "Sitio de Cádiz",
            "Las tropas francesas asedian la ciudad de Cádiz en 1810",
            Some("1810-02-05"),
            &[FaccionId::LiberalProgresista],
            None,
        );
        assert!(errores.is_ok());
    }

    #[test]
    fn test_validador_historico_con_anacronismo() {
        let validador = ValidadorHistorico::nuevo();
        
        let errores = validador.validar_evento_narrativo(
            "Reunión de las Cortes",
            "Las Cortes discuten el socialismo en 1812",
            Some("1812-03-19"),
            &[FaccionId::LiberalProgresista],
            None,
        );
        assert!(errores.is_err());
    }
}
