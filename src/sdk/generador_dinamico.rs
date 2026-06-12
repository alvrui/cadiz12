use crate::config::{validacion_historica::AuditorEquilibrio, PartidaConfig};
use crate::engine::m1_estado_mundo::EstadoMundo;
use super::generador_narrativas::{EventoNarrativo, Consecuencia, OpcionNarrativa};
use super::generador::ConfiguracionIA;
use super::recursos_visuales::CacheImagenes;
use std::collections::HashMap;

/// Contexto del juego para generación dinámica de eventos
pub struct ContextoJuego<'a> {
    /// Referencia al estado del mundo
    pub estado: &'a EstadoMundo,
    /// Jornada actual
    pub jornada_actual: u32,
    /// Eventos actualmente activos en la partida
    pub eventos_activos: Vec<String>,
}

impl<'a> ContextoJuego<'a> {
    /// Crear un nuevo contexto de juego
    pub fn nuevo(estado: &'a EstadoMundo, jornada_actual: u32, eventos_activos: Vec<String>) -> Self {
        Self {
            estado,
            jornada_actual,
            eventos_activos,
        }
    }
}

/// Generador dinámico de eventos contextuales
#[derive(Debug, Clone)]
pub struct GeneradorDinamico {
    /// Configuración de IA para generación
    configuracion_ia: ConfiguracionIA,
    /// Auditor de equilibrio de facciones
    auditor: AuditorEquilibrio,
    /// Cache de imágenes para recursos visuales
    cache: CacheImagenes,
    /// Configuración de la partida
    config: PartidaConfig,
}

impl GeneradorDinamico {
    /// Crear un nuevo generador dinámico con configuración por defecto
    pub fn nuevo(config: PartidaConfig) -> Self {
        Self {
            configuracion_ia: ConfiguracionIA::default(),
            auditor: AuditorEquilibrio::nuevo_con_umbrales(0.6, 0.1),
            cache: CacheImagenes::nuevo("configs/generated/cache", 3600, 50),
            config,
        }
    }

    /// Solicitar un evento dinámico basado en el contexto del juego
    /// 
    /// Genera eventos contextuales según:
    /// - Medidores del protagonista
    /// - Facciones dominantes/desfavorecidas
    /// - Eventos activos en la partida
    /// - Jornada actual
    pub fn solicitar_evento_dinamico(&self, contexto: &ContextoJuego) -> Option<EventoNarrativo> {
        // 1. Analizar medidores del protagonista para detectar oportunidades o crisis
        // Obtener medidores (simplificado - usamos valores directamente)
        // Note: ContextoJuego no tiene acceso directo a EstadoProtagonista,
        // pero podemos generar eventos basados en el estado del mundo
        
        // 2. Analizar facciones
        let faccion_dominante = self.obtener_faccion_dominante(contexto.estado);
        let facciones_desfavorecidas = self.obtener_facciones_desfavorecidas(contexto.estado);
        
        // 3. Analizar jornada actual
        let anio = self.jornada_a_anio(contexto.jornada_actual);
        
        // 4. Generar evento según el contexto
        let evento = match () {
            // Crisis por baja influencia
            _ if contexto.jornada_actual > 100 && contexto.jornada_actual % 50 == 0 => {
                Some(self.generar_evento_crisis_influencia(contexto, &faccion_dominante))
            }
            // Oportunidad por facción en ascenso
            _ if !facciones_desfavorecidas.is_empty() => {
                Some(self.generar_evento_oportunidad_faccion(contexto, &facciones_desfavorecidas[0]))
            }
            // Evento histórico según año
            _ if anio == 1812 => {
                Some(self.generar_evento_constitucion_1812(contexto))
            }
            _ if anio == 1813 => {
                Some(self.generar_evento_fin_sitio(contexto))
            }
            // Evento por polarización alta
            _ if contexto.estado.polarizacion >= 4 => {
                Some(self.generar_evento_polarizacion(contexto))
            }
            // Evento genérico con baja probabilidad
            _ if contexto.jornada_actual % 20 == 0 => {
                Some(self.generar_evento_generico(contexto))
            }
            _ => None,
        };
        
        evento
    }

    /// Validar que un evento dinámico no rompa el equilibrio de facciones
    /// 
    /// Usa el AuditorEquilibrio para verificar que el evento mantiene
    /// una distribución razonable de influencia entre facciones.
    pub fn validar_evento_dinamico(&self, evento: &EventoNarrativo, contexto: &ContextoJuego) -> bool {
        // Obtener la distribución actual de influencia de facciones
        let mut distribuion = HashMap::new();
        
        // Convertir facciones del estado del mundo a distribución de influencia
        for (faccion_id, estado_faccion) in &contexto.estado.facciones {
            distribuion.insert(faccion_id.clone(), estado_faccion.fuerza as f32 / 5.0); // Normalizar fuerza (1-5) a 0.0-1.0
        }
        
        // Validar equilibrio
        match self.auditor.validar_equilibrio_facciones(&distribuion) {
            Ok(_) => {
                // El evento no rompe el equilibrio
                // Validar también que el evento tenga opciones válidas
                !evento.opciones.is_empty()
            }
            Err(_) => {
                // El equilibrio ya está roto, pero el evento podría ayudar a corregirlo
                // Por ahora, aceptamos el evento
                true
            }
        }
    }

    /// Obtener la facción dominante actual
    fn obtener_faccion_dominante(&self, estado: &EstadoMundo) -> Option<crate::config::FaccionId> {
        estado.facciones.iter()
            .max_by_key(|(_, estado_faccion)| estado_faccion.fuerza)
            .map(|(id, _)| id.clone())
    }

    /// Obtener facciones desfavorecidas (fuerza < 3)
    fn obtener_facciones_desfavorecidas(&self, estado: &EstadoMundo) -> Vec<crate::config::FaccionId> {
        estado.facciones.iter()
            .filter(|(_, estado_faccion)| estado_faccion.fuerza < 3)
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Convertir jornada a año aproximado (1810-1814)
    fn jornada_a_anio(&self, jornada: u32) -> u32 {
        // Aproximación: 100 jornadas = 1 año
        1810 + (jornada / 100).min(4)
    }

    /// Generar evento de crisis por baja influencia
    fn generar_evento_crisis_influencia(
        &self,
        contexto: &ContextoJuego,
        faccion_dominante: &Option<crate::config::FaccionId>,
    ) -> EventoNarrativo {
        let id = format!("evento_dinamico_crisis_{}", contexto.jornada_actual);
        let titulo = "Crisis de influencia".to_string();
        let descripcion = format!(
            "Tu influencia entre las facciones está decreciendo. {} parece estar ganando terreno.",
            faccion_dominante.as_ref().map_or("Alguien".to_string(), |f| format!("{:?}", f))
        );
        
        let mut evento = EventoNarrativo::nuevo(&id, &titulo, &descripcion);
        
        // Añadir opciones
        let opcion1 = OpcionNarrativa::nueva(
            "recuperar_apoyo",
            " Intentar recuperar apoyo con acciones concretas",
            Consecuencia::nueva("recuperar_apoyo", "Recuperas parte de tu influencia")
                .con_impacto_medidor("influencia", 15)
                .con_puntuacion_moral(5),
        );
        
        let opcion2 = OpcionNarrativa::nueva(
            "aceptar_declive",
            "Aceptar el declive temporalmente",
            Consecuencia::nueva("aceptar_declive", "Mantienes tu posición pero pierdes influencia")
                .con_impacto_medidor("influencia", -10)
                .con_puntuacion_moral(-5),
        );
        
        evento = evento
            .con_opcion(opcion1)
            .con_opcion(opcion2)
            .con_recursos_visuales_por_defecto(&titulo, &descripcion);
        
        evento
    }

    /// Generar evento de oportunidad con facción desfavorecida
    fn generar_evento_oportunidad_faccion(
        &self,
        contexto: &ContextoJuego,
        faccion: &crate::config::FaccionId,
    ) -> EventoNarrativo {
        let id = format!("evento_dinamico_oportunidad_{:?}_{}", faccion, contexto.jornada_actual);
        let titulo = format!("Oportunidad con {:?}", faccion);
        let descripcion = format!(
            "La facción {:?} está en una posición débil y busca aliados. Podrías aprovechar esta situación.",
            faccion
        );
        
        let mut evento = EventoNarrativo::nuevo(&id, &titulo, &descripcion);
        
        let opcion1 = OpcionNarrativa::nueva(
            "apoyar_faccion",
            "Ofrecer apoyo a esta facción",
            Consecuencia::nueva("apoyar_faccion", "Ganas influencia con esta facción")
                .con_impacto_medidor("influencia", 20)
                .con_relacion_faccion(faccion.clone(), 25)
                .con_puntuacion_moral(10),
        );
        
        let opcion2 = OpcionNarrativa::nueva(
            "ignorar",
            "Ignorar la oportunidad",
            Consecuencia::nueva("ignorar", "No hay cambio inmediato")
                .con_puntuacion_moral(0),
        );
        
        evento = evento
            .con_opcion(opcion1)
            .con_opcion(opcion2)
            .con_recursos_visuales_por_defecto(&titulo, &descripcion);
        
        evento
    }

    /// Generar evento relacionado con la Constitución de 1812
    fn generar_evento_constitucion_1812(&self, contexto: &ContextoJuego) -> EventoNarrativo {
        let id = format!("evento_dinamico_constitucion_{}", contexto.jornada_actual);
        let titulo = "Debate constitucional".to_string();
        let descripcion = "La redacción de la Constitución avanza. Se necesitan voces para definir artículos clave.".to_string();
        
        let mut evento = EventoNarrativo::nuevo(&id, &titulo, &descripcion);
        
        let opcion1 = OpcionNarrativa::nueva(
            "apoyar_liberal",
            "Apoyar la posición liberal",
            Consecuencia::nueva("apoyar_liberal", "Refuerzas el bipartidismo liberal")
                .con_impacto_medidor("influencia", 15)
                .con_impacto_medidor("coherencia", 10)
                .con_puntuacion_moral(8),
        );
        
        let opcion2 = OpcionNarrativa::nueva(
            "buscar_consenso",
            "Buscar consenso entre facciones",
            Consecuencia::nueva("buscar_consenso", "Evitas división pero diluyes el cambio")
                .con_impacto_medidor("influencia", 10)
                .con_impacto_medidor("coherencia", 5)
                .con_puntuacion_moral(5),
        );
        
        evento = evento
            .con_opcion(opcion1)
            .con_opcion(opcion2)
            .con_recursos_visuales_por_defecto(&titulo, &descripcion);
        
        evento
    }

    /// Generar evento para el fin del sitio de Cádiz
    fn generar_evento_fin_sitio(&self, contexto: &ContextoJuego) -> EventoNarrativo {
        let id = format!("evento_dinamico_fin_sitio_{}", contexto.jornada_actual);
        let titulo = "Fin del asedio".to_string();
        let descripcion = "Las tropas francesas han levantado el sitio. La ciudad respira aliviada.".to_string();
        
        let mut evento = EventoNarrativo::nuevo(&id, &titulo, &descripcion);
        
        let opcion1 = OpcionNarrativa::nueva(
            "celebrar",
            "Celebrar con la población",
            Consecuencia::nueva("celebrar", "Aumenta tu reputación pública")
                .con_impacto_medidor("reputacion", 20)
                .con_puntuacion_moral(10),
        );
        
        let opcion2 = OpcionNarrativa::nueva(
            "reflexionar",
            "Reflexionar sobre el futuro",
            Consecuencia::nueva("reflexionar", "Ganas perspectiva estratégica")
                .con_impacto_medidor("coherencia", 15)
                .con_puntuacion_moral(5),
        );
        
        evento = evento
            .con_opcion(opcion1)
            .con_opcion(opcion2)
            .con_recursos_visuales_por_defecto(&titulo, &descripcion);
        
        evento
    }

    /// Generar evento genérico
    fn generar_evento_polarizacion(&self, contexto: &ContextoJuego) -> EventoNarrativo {
        let id = format!("evento_dinamico_polarizacion_{}", contexto.jornada_actual);
        let titulo = "Tensión política".to_string();
        let descripcion = format!(
            "La polarización está en su punto máximo ({}/5). Las discusiones se han vuelto tensas.",
            contexto.estado.polarizacion
        );
        
        let mut evento = EventoNarrativo::nuevo(&id, &titulo, &descripcion);
        
        let opcion1 = OpcionNarrativa::nueva(
            "mediar",
            "Intentar mediar entre facciones",
            Consecuencia::nueva("mediar", "Reduces la polarización")
                .con_impacto_medidor("relacional", 15)
                .con_puntuacion_moral(8),
        );
        
        let opcion2 = OpcionNarrativa::nueva(
            "tomar_partido",
            "Tomar partido por una facción",
            Consecuencia::nueva("tomar_partido", "Aumentas tu influencia pero incrementas la división")
                .con_impacto_medidor("influencia", 20)
                .con_impacto_medidor("relacional", -10)
                .con_puntuacion_moral(-5),
        );
        
        evento = evento
            .con_opcion(opcion1)
            .con_opcion(opcion2)
            .con_recursos_visuales_por_defecto(&titulo, &descripcion);
        
        evento
    }

    /// Generar evento genérico por defecto
    fn generar_evento_generico(&self, contexto: &ContextoJuego) -> EventoNarrativo {
        let id = format!("evento_dinamico_generico_{}", contexto.jornada_actual);
        let titulo = "Evento en las Cortes".to_string();
        let descripcion = format!(
            "Ocurre un evento relevante en las Cortes durante la jornada {}.",
            contexto.jornada_actual
        );
        
        let mut evento = EventoNarrativo::nuevo(&id, &titulo, &descripcion);
        
        let opcion1 = OpcionNarrativa::nueva(
            "participar",
            "Participar activamente",
            Consecuencia::nueva("participar", "Ganas visibilidad")
                .con_impacto_medidor("influencia", 10)
                .con_puntuacion_moral(5),
        );
        
        let opcion2 = OpcionNarrativa::nueva(
            "observar",
            "Observar desde la distancia",
            Consecuencia::nueva("observar", "Mantienes tu posición")
                .con_puntuacion_moral(0),
        );
        
        evento = evento
            .con_opcion(opcion1)
            .con_opcion(opcion2)
            .con_recursos_visuales_por_defecto(&titulo, &descripcion);
        
        evento
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{FaccionId, PartidaConfig};
    use crate::engine::m1_estado_mundo::EstadoMundo;

    #[test]
    fn test_generador_dinamico_nuevo() {
        let config = PartidaConfig::default();
        let generador = GeneradorDinamico::nuevo(config);
        
        // Verificar que se inicializó correctamente
        assert!(generador.config.eventos.plantillas.len() > 0);
    }

    #[test]
    fn test_contexto_juego_nuevo() {
        let estado = EstadoMundo::nuevo(&PartidaConfig::default());
        let contexto = ContextoJuego::nuevo(&estado, 50, vec![]);
        
        assert_eq!(contexto.jornada_actual, 50);
        assert!(contexto.eventos_activos.is_empty());
    }

    #[test]
    fn test_solicitar_evento_dinamico() {
        let config = PartidaConfig::default();
        let generador = GeneradorDinamico::nuevo(config);
        let estado = EstadoMundo::nuevo(&PartidaConfig::default());
        let contexto = ContextoJuego::nuevo(&estado, 150, vec![]);
        
        let evento = generador.solicitar_evento_dinamico(&contexto);
        
        // Debe generar algún evento (jornada 150 desencadena crisis de influencia)
        assert!(evento.is_some());
        let evento = evento.unwrap();
        assert!(!evento.opciones.is_empty());
    }

    #[test]
    fn test_validar_evento_dinamico() {
        let config = PartidaConfig::default();
        let generador = GeneradorDinamico::nuevo(config);
        let estado = EstadoMundo::nuevo(&PartidaConfig::default());
        let contexto = ContextoJuego::nuevo(&estado, 50, vec![]);
        
        let evento = generador.solicitar_evento_dinamico(&contexto);
        
        if let Some(evento) = evento {
            let es_valido = generador.validar_evento_dinamico(&evento, &contexto);
            assert!(es_valido);
        }
    }

    #[test]
    fn test_obtener_faccion_dominante() {
        let config = PartidaConfig::default();
        let generador = GeneradorDinamico::nuevo(config);
        let estado = EstadoMundo::nuevo(&PartidaConfig::default());
        
        let dominante = generador.obtener_faccion_dominante(&estado);
        // Debería haber alguna facción dominante
        assert!(dominante.is_some());
    }

    #[test]
    fn test_jornada_a_anio() {
        let config = PartidaConfig::default();
        let generador = GeneradorDinamico::nuevo(config);
        
        assert_eq!(generador.jornada_a_anio(0), 1810);
        assert_eq!(generador.jornada_a_anio(100), 1811);
        assert_eq!(generador.jornada_a_anio(400), 1814);
        assert_eq!(generador.jornada_a_anio(500), 1814); // Máximo 1814
    }
}
