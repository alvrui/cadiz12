use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Estado completo de una jornada (respuesta a GET /estado_jornada)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstadoJornadaDto {
    pub tiempo: TiempoDto,
    pub protagonista: ProtagonistaDto,
    pub crisis_activa: Option<CrisisActivaDto>,
    pub eventos_disponibles: Vec<EventoDisponibleDto>,
    pub presupuesto_temporal: u8,
}

/// Posicion temporal en el juego
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiempoDto {
    pub tramo_id: String,
    pub acto: u32,
    pub jornada: u32,
}

/// Resumen del protagonista para la interfaz
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtagonistaDto {
    pub posicion_formal_id: String,
    pub visibilidad: String,
    pub medidores: Vec<MedidorResumenDto>,
}

/// Resumen de un medidor (siempre 6, orden fijo)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedidorResumenDto {
    pub nombre: String,
    pub valor: u8,
    pub tendencia: i8,
    pub umbral_bajo: u8,
    pub umbral_alto: u8,
}

/// Crisis activa en el tablero
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrisisActivaDto {
    pub tipo_id: String,
    pub fase: String,
}

/// Evento disponible para el jugador
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventoDisponibleDto {
    pub evento_id: String,
    pub titulo: String,
    pub tipo: String,
    pub coste_temporal: u8,
    pub prioridad: u8,
    pub modificador_perfil: f32,
}

/// DTO completo del estado del protagonista
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstadoProtagonistaDto {
    pub perfil: PerfilDto,
    pub posicion: PosicionDto,
    pub medidores: Vec<MedidorDto>,
    pub reputaciones: HashMap<String, ReputacionDto>,
    pub relaciones: Vec<RelacionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfilDto {
    pub origen: String,
    pub clase_social: String,
    pub oficio: String,
    pub adscripcion: String,
    pub temperamento: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosicionDto {
    pub formal_id: String,
    pub visibilidad: String,
    pub trayectoria_moral: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedidorDto {
    pub nombre: String,
    pub valor: u8,
    pub tendencia: i8,
    pub umbral_bajo: u8,
    pub umbral_alto: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputacionDto {
    pub valor: u8,
    pub tendencia: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelacionDto {
    pub npc_id: String,
    pub estado: String,
    pub confianza: u8,
    pub deuda: i16,
    pub cooldown: u8,
}
