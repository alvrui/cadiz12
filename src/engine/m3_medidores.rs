use std::collections::HashMap;
use super::m2_estado_protagonista::{EstadoProtagonista, Medidor, Reputacion};

/// Sistema de medidores (M3)
#[derive(Debug, Clone)]
pub struct SistemaMedidores;

impl SistemaMedidores {
    /// Aplicar consecuencias de una decisión a los medidores y reputaciones
    pub fn aplicar_consecuencias(
        estado: &mut EstadoProtagonista,
        consecuencias: &HashMap<String, i16>,
    ) {
        for (clave, delta) in consecuencias {
            Self::aplicar_consecuencia_individual(estado, clave, *delta);
        }
    }

    fn aplicar_consecuencia_individual(
        estado: &mut EstadoProtagonista,
        clave: &str,
        delta: i16,
    ) {
        match clave {
            // Medidores principales
            "influencia" => estado.medidores.influencia.aplicar_delta(delta),
            "relacional" => estado.medidores.relacional.aplicar_delta(delta),
            "reputacion" => estado.medidores.reputacion.aplicar_delta(delta),
            "coherencia" => estado.medidores.coherencia.aplicar_delta(delta),
            "recursos" => estado.medidores.recursos.aplicar_delta(delta),
            "aguante" => estado.medidores.aguante.aplicar_delta(delta),

            // Reputaciones segmentadas (formato: reputacion_{grupo})
            clave if clave.starts_with("reputacion_") => {
                let grupo = clave.trim_start_matches("reputacion_");
                let rep = estado.reputaciones.entry(grupo.to_string())
                    .or_insert_with(|| Reputacion::nueva(50));
                let nuevo_valor = (rep.valor as i16) + delta;
                rep.valor = nuevo_valor.clamp(0, 100) as u8;
                // Actualizar tendencia
                if delta > 0 {
                    rep.tendencia = (rep.tendencia + 1).min(3);
                } else if delta < 0 {
                    rep.tendencia = (rep.tendencia - 1).max(-3);
                }
            },

            // Consecuencias a relaciones con NPCs (formato: relacion_{npc_id})
            clave if clave.starts_with("relacion_") => {
                let npc_id = clave.trim_start_matches("relacion_");
                if let Some(relacion) = estado.relaciones.get_mut(npc_id) {
                    let nuevo_valor = (relacion.confianza as i16) + delta;
                    relacion.confianza = nuevo_valor.clamp(0, 100) as u8;
                }
            },

            // Consecuencias a deuda con NPCs (formato: deuda_{npc_id})
            clave if clave.starts_with("deuda_") => {
                let npc_id = clave.trim_start_matches("deuda_");
                if let Some(relacion) = estado.relaciones.get_mut(npc_id) {
                    relacion.deuda += delta;
                }
            },

            // Consecuencias condicionales (requerimientos)
            clave if clave.starts_with("requerimiento_") => {
                // No aplicar directamente, solo registrar
                log::debug!("Requerimiento no cumplido: {}", clave);
            },

            // Ignorar claves desconocidas (pero loguear en debug)
            _ => {
                log::debug!("Consecuencia ignorada: {} = {}", clave, delta);
            }
        }
    }

    /// Verificar si algún medidor ha cruzado umbrales
    pub fn verificar_umbrales(&self, estado: &EstadoProtagonista) -> Vec<(String, String)> {
        let mut eventos_umbral = Vec::new();

        if estado.medidores.influencia.valor < estado.medidores.influencia.umbral_bajo {
            eventos_umbral.push(("bloqueo_acceso_parlamentario".to_string(), "influencia_bajo".to_string()));
        }
        if estado.medidores.influencia.valor > estado.medidores.influencia.umbral_alto {
            eventos_umbral.push(("confidencias_facciones".to_string(), "influencia_alto".to_string()));
        }

        if estado.medidores.coherencia.valor < estado.medidores.coherencia.umbral_bajo {
            eventos_umbral.push(("crisis_credibilidad".to_string(), "coherencia_baja".to_string()));
        }
        if estado.medidores.coherencia.valor > estado.medidores.coherencia.umbral_alto {
            eventos_umbral.push(("legado_reformista".to_string(), "coherencia_alta".to_string()));
        }

        if estado.medidores.recursos.valor < estado.medidores.recursos.umbral_bajo {
            eventos_umbral.push(("presion_consulado".to_string(), "recursos_bajos".to_string()));
        }

        if estado.medidores.aguante.valor < estado.medidores.aguante.umbral_bajo {
            eventos_umbral.push(("momento_personal_obligatorio".to_string(), "aguante_bajo".to_string()));
        }

        eventos_umbral
    }

    /// Aplicar decaimiento a medidores (si está configurado)
    pub fn aplicar_decaimiento(
        estado: &mut EstadoProtagonista,
        config: &crate::config::MedidoresConfig,
        jornada: u32,
    ) {
        if jornada >= config.influencia.decaimiento_inicio as u32 && config.influencia.decaimiento_tasa > 0.0 {
            let delta = (config.influencia.decaimiento_tasa * -1.0) as i16;
            estado.medidores.influencia.aplicar_delta(delta);
        }
        if jornada >= config.relacional.decaimiento_inicio as u32 && config.relacional.decaimiento_tasa > 0.0 {
            let delta = (config.relacional.decaimiento_tasa * -1.0) as i16;
            estado.medidores.relacional.aplicar_delta(delta);
        }
        if jornada >= config.reputacion.decaimiento_inicio as u32 && config.reputacion.decaimiento_tasa > 0.0 {
            let delta = (config.reputacion.decaimiento_tasa * -1.0) as i16;
            estado.medidores.reputacion.aplicar_delta(delta);
        }
        if jornada >= config.coherencia.decaimiento_inicio as u32 && config.coherencia.decaimiento_tasa > 0.0 {
            let delta = (config.coherencia.decaimiento_tasa * -1.0) as i16;
            estado.medidores.coherencia.aplicar_delta(delta);
        }
        if jornada >= config.recursos.decaimiento_inicio as u32 && config.recursos.decaimiento_tasa > 0.0 {
            let delta = (config.recursos.decaimiento_tasa * -1.0) as i16;
            estado.medidores.recursos.aplicar_delta(delta);
        }
        if jornada >= config.aguante.decaimiento_inicio as u32 && config.aguante.decaimiento_tasa > 0.0 {
            let delta = (config.aguante.decaimiento_tasa * -1.0) as i16;
            estado.medidores.aguante.aplicar_delta(delta);
        }
    }
}
