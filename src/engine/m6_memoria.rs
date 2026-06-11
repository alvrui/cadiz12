use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

/// Memoria del sistema (M6)
#[derive(Debug, Clone)]
pub struct MemoriaSistema {
    /// Eventos resueltos en las ultimas N jornadas
    historial_eventos: Vec<EventoResuelto>,
    /// Cooldowns activos por tipo de evento
    cooldowns: HashMap<String, u32>,
    /// Ultima vez que se mostro cada evento
    ultima_vez: HashMap<String, u32>,
    /// Eventos bloqueados temporalmente
    bloqueados: HashSet<String>,
}

#[derive(Debug, Clone)]
struct EventoResuelto {
    pub evento_id: String,
    pub opcion_id: String,
    pub jornada: u32,
    pub timestamp: u64,
}

impl MemoriaSistema {
    pub fn nueva() -> Self {
        Self {
            historial_eventos: Vec::new(),
            cooldowns: HashMap::new(),
            ultima_vez: HashMap::new(),
            bloqueados: HashSet::new(),
        }
    }

    /// Registrar un evento resuelto
    pub fn registrar_evento(&mut self, evento_id: &str, opcion_id: &str, jornada: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.historial_eventos.push(EventoResuelto {
            evento_id: evento_id.to_string(),
            opcion_id: opcion_id.to_string(),
            jornada,
            timestamp,
        });

        self.ultima_vez.insert(evento_id.to_string(), jornada);

        if self.historial_eventos.len() > 100 {
            self.historial_eventos.remove(0);
        }
    }

    /// Verificar si un evento puede mostrarse
    pub fn puede_mostrar(&self, evento_id: &str, jornada: u32, cooldown: u32) -> bool {
        if self.bloqueados.contains(evento_id) {
            return false;
        }

        if let Some(ultima_vez) = self.ultima_vez.get(evento_id) {
            if jornada - ultima_vez < cooldown {
                return false;
            }
        }

        if let Some(ultimo) = self.historial_eventos.last() {
            if ultimo.jornada == jornada && ultimo.evento_id == evento_id {
                return false;
            }
        }

        true
    }

    /// Bloquear un evento temporalmente
    pub fn bloquear_evento(&mut self, evento_id: &str) {
        self.bloqueados.insert(evento_id.to_string());
    }

    /// Desbloquear un evento
    pub fn desbloquear_evento(&mut self, evento_id: &str) {
        self.bloqueados.remove(evento_id);
    }

    /// Limpiar cooldowns
    pub fn limpiar_cooldowns(&mut self, jornada: u32) {
        self.cooldowns.retain(|_, &mut cooldown| cooldown > jornada);
    }
}