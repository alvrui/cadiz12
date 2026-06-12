use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use crate::config::{FaccionId, EspacioId, EstiloNarrativo};
use super::generador::ConfiguracionIA;

/// Tipo de plantilla de guion narrativa
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TipoPlantillaGuion {
    /// Estructura clásica de 3 actos (setup, confrontación, resolución)
    HollywoodAnimal,
    /// Viaje del héroe con etapas definidas
    HeroesJourney,
    /// Situación que requiere elección moral
    DilemaMoral,
    /// Proceso judicial con acusación, defensa, veredicto
    ProcedimientoJudicial,
}

impl Default for TipoPlantillaGuion {
    fn default() -> Self {
        Self::HollywoodAnimal
    }
}

impl fmt::Display for TipoPlantillaGuion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HollywoodAnimal => write!(f, "HollywoodAnimal"),
            Self::HeroesJourney => write!(f, "HeroesJourney"),
            Self::DilemaMoral => write!(f, "DilemaMoral"),
            Self::ProcedimientoJudicial => write!(f, "ProcedimientoJudicial"),
        }
    }
}

/// Metadatos de una narrativa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativaMetadatos {
    /// Identificador único
    pub id: String,
    /// Título de la narrativa
    pub titulo: String,
    /// Descripción breve
    pub descripcion: String,
    /// Tipo de plantilla usado
    pub tipo_plantilla: TipoPlantillaGuion,
    /// Dificultad (1-10)
    pub dificultad: u8,
    /// Tags para clasificación
    pub tags: Vec<String>,
    /// Facciones involucradas
    pub facciones_involucradas: Vec<FaccionId>,
    /// Espacios relacionados
    pub espacios_relacionados: Vec<EspacioId>,
    /// Estilo narrativo recomendado
    pub estilo_recomendado: EstiloNarrativo,
    /// Autor o generador
    pub autor: String,
    /// Fecha de creación (timestamp)
    pub fecha_creacion: u64,
}

impl NarrativaMetadatos {
    pub fn nuevo(id: &str, titulo: &str, tipo_plantilla: TipoPlantillaGuion) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Self {
            id: id.to_string(),
            titulo: titulo.to_string(),
            descripcion: String::new(),
            tipo_plantilla,
            dificultad: 5,
            tags: vec![],
            facciones_involucradas: vec![],
            espacios_relacionados: vec![],
            estilo_recomendado: EstiloNarrativo::Equilibrado,
            autor: "Sistema".to_string(),
            fecha_creacion: timestamp,
        }
    }

    pub fn con_descripcion(mut self, descripcion: &str) -> Self {
        self.descripcion = descripcion.to_string();
        self
    }

    pub fn con_dificultad(mut self, dificultad: u8) -> Self {
        self.dificultad = dificultad.min(10).max(1);
        self
    }

    pub fn con_tags(mut self, tags: Vec<&str>) -> Self {
        self.tags = tags.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn con_facciones(mut self, facciones: Vec<FaccionId>) -> Self {
        self.facciones_involucradas = facciones;
        self
    }

    pub fn con_espacios(mut self, espacios: Vec<EspacioId>) -> Self {
        self.espacios_relacionados = espacios;
        self
    }

    pub fn con_estilo(mut self, estilo: EstiloNarrativo) -> Self {
        self.estilo_recomendado = estilo;
        self
    }
}

/// Consecuencia de una opción narrativa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consecuencia {
    /// Identificador único
    pub id: String,
    /// Descripción de lo que ocurre
    pub descripcion: String,
    /// Impacto en medidores (clave: nombre_medidor, valor: cambio)
    pub impacto_medidores: HashMap<String, i16>,
    /// Eventos que se desbloquean
    pub eventos_desbloqueados: Vec<String>,
    /// Eventos que se bloquean
    pub eventos_bloqueados: Vec<String>,
    /// Cambio en relaciones con facciones
    pub cambio_relaciones: HashMap<FaccionId, i16>,
    /// Reputación ganada/perdida
    pub reputacion: HashMap<String, i16>,
    /// Recursos consumidos o ganados
    pub recursos: HashMap<String, i16>,
    /// Puntuación de moral (positivo/negativo)
    pub puntuacion_moral: i16,
    /// Indica si es el final de la narrativa
    pub es_final: bool,
}

impl Consecuencia {
    pub fn nueva(id: &str, descripcion: &str) -> Self {
        Self {
            id: id.to_string(),
            descripcion: descripcion.to_string(),
            impacto_medidores: HashMap::new(),
            eventos_desbloqueados: vec![],
            eventos_bloqueados: vec![],
            cambio_relaciones: HashMap::new(),
            reputacion: HashMap::new(),
            recursos: HashMap::new(),
            puntuacion_moral: 0,
            es_final: false,
        }
    }

    pub fn con_impacto_medidor(mut self, medidor: &str, valor: i16) -> Self {
        self.impacto_medidores.insert(medidor.to_string(), valor);
        self
    }

    pub fn con_evento_desbloqueado(mut self, evento_id: &str) -> Self {
        self.eventos_desbloqueados.push(evento_id.to_string());
        self
    }

    pub fn con_evento_bloqueado(mut self, evento_id: &str) -> Self {
        self.eventos_bloqueados.push(evento_id.to_string());
        self
    }

    pub fn con_relacion_faccion(mut self, faccion: FaccionId, cambio: i16) -> Self {
        self.cambio_relaciones.insert(faccion, cambio);
        self
    }

    pub fn con_reputacion(mut self, clave: &str, valor: i16) -> Self {
        self.reputacion.insert(clave.to_string(), valor);
        self
    }

    pub fn con_recurso(mut self, clave: &str, valor: i16) -> Self {
        self.recursos.insert(clave.to_string(), valor);
        self
    }

    pub fn con_puntuacion_moral(mut self, puntuacion: i16) -> Self {
        self.puntuacion_moral = puntuacion;
        self
    }

    pub fn como_final(mut self) -> Self {
        self.es_final = true;
        self
    }
}

impl Default for Consecuencia {
    fn default() -> Self {
        Self::nueva("consecuencia_vacia", "Sin consecuencias")
    }
}

/// Opción dentro de un evento narrativo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpcionNarrativa {
    /// Identificador único (ej: "opcion_1", "opcion_2")
    pub id: String,
    /// Texto de la opción
    pub texto: String,
    /// Consecuencia de seleccionar esta opción
    pub consecuencia: Consecuencia,
    /// Peso para selección aleatoria (0-100)
    pub peso: u8,
    /// Requisitos para que esta opción esté disponible
    pub requisitos: Vec<String>,
    /// Indica si es la opción "recomendada"
    pub recomendada: bool,
}

impl OpcionNarrativa {
    pub fn nueva(id: &str, texto: &str, consecuencia: Consecuencia) -> Self {
        Self {
            id: id.to_string(),
            texto: texto.to_string(),
            consecuencia,
            peso: 50,
            requisitos: vec![],
            recomendada: false,
        }
    }

    pub fn con_peso(mut self, peso: u8) -> Self {
        self.peso = peso;
        self
    }

    pub fn con_requisito(mut self, requisito: &str) -> Self {
        self.requisitos.push(requisito.to_string());
        self
    }

    pub fn como_recomendada(mut self) -> Self {
        self.recomendada = true;
        self
    }
}

/// Evento dentro de una cadena narrativa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventoNarrativo {
    /// Identificador único
    pub id: String,
    /// Título del evento
    pub titulo: String,
    /// Descripción completa
    pub descripcion: String,
    /// Opciones disponibles
    pub opciones: Vec<OpcionNarrativa>,
    /// Evento anterior en la cadena (None si es el primero)
    pub evento_anterior: Option<String>,
    /// Eventos siguientes posibles
    pub eventos_siguientes: Vec<String>,
    /// Metadatos específicos del evento
    pub metadatos: HashMap<String, String>,
    /// Indica si es un punto de decisión crítica
    pub es_decision_critica: bool,
}

impl EventoNarrativo {
    pub fn nuevo(id: &str, titulo: &str, descripcion: &str) -> Self {
        Self {
            id: id.to_string(),
            titulo: titulo.to_string(),
            descripcion: descripcion.to_string(),
            opciones: vec![],
            evento_anterior: None,
            eventos_siguientes: vec![],
            metadatos: HashMap::new(),
            es_decision_critica: false,
        }
    }

    pub fn con_opcion(mut self, opcion: OpcionNarrativa) -> Self {
        self.opciones.push(opcion);
        self
    }

    pub fn con_opciones(mut self, opciones: Vec<OpcionNarrativa>) -> Self {
        self.opciones = opciones;
        self
    }

    pub fn con_evento_anterior(mut self, evento_id: &str) -> Self {
        self.evento_anterior = Some(evento_id.to_string());
        self
    }

    pub fn con_evento_siguiente(mut self, evento_id: &str) -> Self {
        self.eventos_siguientes.push(evento_id.to_string());
        self
    }

    pub fn con_metadato(mut self, clave: &str, valor: &str) -> Self {
        self.metadatos.insert(clave.to_string(), valor.to_string());
        self
    }

    pub fn como_decision_critica(mut self) -> Self {
        self.es_decision_critica = true;
        self
    }

    /// Validar que el evento tiene al menos una opción
    pub fn validar(&self) -> Result<(), String> {
        if self.opciones.is_empty() {
            return Err(format!("Evento '{}' no tiene opciones", self.id));
        }
        
        // Validar que todas las opciones tienen consecuencias
        for opcion in &self.opciones {
            if opcion.consecuencia.impacto_medidores.is_empty() &&
               opcion.consecuencia.eventos_desbloqueados.is_empty() &&
               opcion.consecuencia.eventos_bloqueados.is_empty() &&
               opcion.consecuencia.cambio_relaciones.is_empty() &&
               opcion.consecuencia.reputacion.is_empty() &&
               opcion.consecuencia.recursos.is_empty() &&
               opcion.consecuencia.puntuacion_moral == 0 &&
               !opcion.consecuencia.es_final {
                return Err(format!("Opcion '{}' en evento '{}' no tiene consecuencias", opcion.id, self.id));
            }
        }
        
        Ok(())
    }
}

/// Cadena de eventos que forma una narrativa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadenaEventos {
    /// Identificador único de la cadena
    pub id: String,
    /// Eventos en orden
    pub eventos: Vec<EventoNarrativo>,
    /// Evento inicial
    pub evento_inicial: String,
    /// Eventos finales posibles
    pub eventos_finales: Vec<String>,
    /// Longitud máxima de la cadena
    pub longitud_maxima: Option<usize>,
}

impl CadenaEventos {
    pub fn nueva(id: &str, evento_inicial: &str) -> Self {
        Self {
            id: id.to_string(),
            eventos: vec![],
            evento_inicial: evento_inicial.to_string(),
            eventos_finales: vec![],
            longitud_maxima: None,
        }
    }

    pub fn con_evento(mut self, evento: EventoNarrativo) -> Self {
        self.eventos.push(evento);
        self
    }

    pub fn con_eventos(mut self, eventos: Vec<EventoNarrativo>) -> Self {
        self.eventos = eventos;
        self
    }

    pub fn con_evento_final(mut self, evento_id: &str) -> Self {
        self.eventos_finales.push(evento_id.to_string());
        self
    }

    pub fn con_longitud_maxima(mut self, max: usize) -> Self {
        self.longitud_maxima = Some(max);
        self
    }

    /// Validar que no hay bucles en la cadena
    pub fn validar_sin_bucles(&self) -> Result<(), String> {
        let mut visitados = HashSet::new();
        let mut en_pila = HashSet::new();
        
        fn tiene_bucle(
            evento_id: &str,
            eventos: &HashMap<String, &EventoNarrativo>,
            visitados: &mut HashSet<String>,
            en_pila: &mut HashSet<String>,
        ) -> Result<(), String> {
            if en_pila.contains(evento_id) {
                return Err(format!("Bucle detectado en evento: {}", evento_id));
            }
            if visitados.contains(evento_id) {
                return Ok(());
            }
            
            visitados.insert(evento_id.to_string());
            en_pila.insert(evento_id.to_string());
            
            if let Some(evento) = eventos.get(evento_id) {
                for siguiente in &evento.eventos_siguientes {
                    tiene_bucle(siguiente, eventos, visitados, en_pila)?;
                }
            }
            
            en_pila.remove(evento_id);
            Ok(())
        }
        
        let eventos_map: HashMap<String, &EventoNarrativo> = self.eventos
            .iter()
            .map(|e| (e.id.clone(), e))
            .collect();
        
        tiene_bucle(&self.evento_inicial, &eventos_map, &mut visitados, &mut en_pila)?;
        
        Ok(())
    }

    /// Obtener el evento por ID
    pub fn obtener_evento(&self, id: &str) -> Option<&EventoNarrativo> {
        self.eventos.iter().find(|e| e.id == id)
    }
}

/// Narrativa completa con metadatos y cadena de eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Narrativa {
    /// Metadatos de la narrativa
    pub metadatos: NarrativaMetadatos,
    /// Cadena de eventos principal
    pub cadena_principal: CadenaEventos,
    /// Cadenas alternativas (ramificaciones)
    pub cadenas_alternativas: Vec<CadenaEventos>,
    /// Personajes involucrados
    pub personajes: Vec<String>,
    /// Objetos importantes
    pub objetos: Vec<String>,
}

impl Narrativa {
    pub fn nueva(metadatos: NarrativaMetadatos, cadena_principal: CadenaEventos) -> Self {
        Self {
            metadatos,
            cadena_principal,
            cadenas_alternativas: vec![],
            personajes: vec![],
            objetos: vec![],
        }
    }

    pub fn con_cadena_alternativa(mut self, cadena: CadenaEventos) -> Self {
        self.cadenas_alternativas.push(cadena);
        self
    }

    pub fn con_personaje(mut self, personaje: &str) -> Self {
        self.personajes.push(personaje.to_string());
        self
    }

    pub fn con_objeto(mut self, objeto: &str) -> Self {
        self.objetos.push(objeto.to_string());
        self
    }

    /// Validar coherencia completa de la narrativa
    pub fn validar(&self) -> Result<(), Vec<String>> {
        let mut errores = Vec::new();
        
        // Validar cadena principal
        if let Err(e) = self.cadena_principal.validar_sin_bucles() {
            errores.push(e);
        }
        
        // Validar cada evento en cadena principal
        for evento in &self.cadena_principal.eventos {
            if let Err(e) = evento.validar() {
                errores.push(e);
            }
        }
        
        // Validar cadenas alternativas
        for cadena in &self.cadenas_alternativas {
            if let Err(e) = cadena.validar_sin_bucles() {
                errores.push(e);
            }
            for evento in &cadena.eventos {
                if let Err(e) = evento.validar() {
                    errores.push(e);
                }
            }
        }
        
        // Validar referencias entre eventos
        let todas_cadenas: Vec<&CadenaEventos> = 
            std::iter::once(&self.cadena_principal)
                .chain(self.cadenas_alternativas.iter())
                .collect();
        
        for cadena in &todas_cadenas {
            for evento in &cadena.eventos {
                // Validar que evento_anterior existe (si lo tiene)
                if let Some(ref anterior) = &evento.evento_anterior {
                    let existe = todas_cadenas.iter().any(|c| c.obtener_evento(anterior).is_some());
                    if !existe {
                        errores.push(format!("Evento '{}' referencia a evento anterior '{}' que no existe", evento.id, anterior));
                    }
                }
                
                // Validar que eventos_siguientes existen
                for siguiente in &evento.eventos_siguientes {
                    let existe = todas_cadenas.iter().any(|c| c.obtener_evento(siguiente).is_some());
                    if !existe {
                        errores.push(format!("Evento '{}' referencia a evento siguiente '{}' que no existe", evento.id, siguiente));
                    }
                }
            }
        }
        
        if errores.is_empty() {
            Ok(())
        } else {
            Err(errores)
        }
    }
}

/// Plantilla de guion para generación de narrativas
#[derive(Debug, Clone)]
pub struct PlantillaGuion {
    /// Tipo de plantilla
    pub tipo: TipoPlantillaGuion,
    /// Nombre de la plantilla
    pub nombre: String,
    /// Descripción
    pub descripcion: String,
    /// Pasos estructurados
    pub pasos: Vec<PasoPlantilla>,
}

impl PlantillaGuion {
    pub fn nueva(tipo: TipoPlantillaGuion, nombre: &str, descripcion: &str) -> Self {
        Self {
            tipo,
            nombre: nombre.to_string(),
            descripcion: descripcion.to_string(),
            pasos: vec![],
        }
    }

    pub fn con_paso(mut self, paso: PasoPlantilla) -> Self {
        self.pasos.push(paso);
        self
    }

    /// Generar una narrativa usando esta plantilla
    pub fn generar_narrativa(&self, configuracion: &ConfiguracionIA) -> Narrativa {
        let metadatos = NarrativaMetadatos::nuevo(
            &format!("narrativa_{}", self.tipo.to_string().to_lowercase()),
            &self.nombre,
            self.tipo.clone(),
        );
        
        let cadena = self.generar_cadena_eventos(configuracion);
        
        Narrativa::nueva(metadatos, cadena)
    }

    fn generar_cadena_eventos(&self, _configuracion: &ConfiguracionIA) -> CadenaEventos {
        // Implementación base - cada plantilla puede especializar esto
        let mut cadena = CadenaEventos::nueva("cadena_principal", "inicio");
        
        for (i, paso) in self.pasos.iter().enumerate() {
            let evento_id = if i == 0 {
                "inicio".to_string()
            } else {
                format!("evento_{}", i)
            };
            
            let evento = EventoNarrativo::nuevo(
                &evento_id,
                &paso.nombre,
                &paso.descripcion,
            );
            
            // Generar opciones basadas en el tipo de paso
            let opciones = self.generar_opciones_paso(paso, &evento_id);
            
            let evento = evento.con_opciones(opciones);
            cadena = cadena.con_evento(evento);
        }
        
        cadena
    }

    fn generar_opciones_paso(&self, paso: &PasoPlantilla, _evento_id: &str) -> Vec<OpcionNarrativa> {
        match paso.tipo_paso {
            TipoPaso::Inicio => {
                vec![
                    OpcionNarrativa::nueva(
                        "opcion_1",
                        "Aceptar el desafío",
                        Consecuencia::nueva("consecuencia_1", "El personaje acepta el desafío")
                            .con_impacto_medidor("influencia", 5)
                            .con_impacto_medidor("reputacion", 10)
                            .con_puntuacion_moral(5),
                    ).como_recomendada(),
                    OpcionNarrativa::nueva(
                        "opcion_2",
                        "Rechazar el desafío",
                        Consecuencia::nueva("consecuencia_2", "El personaje rechaza el desafío")
                            .con_impacto_medidor("influencia", -3)
                            .con_impacto_medidor("reputacion", -5)
                            .con_puntuacion_moral(-5),
                    ),
                ]
            }
            TipoPaso::Conflicto => {
                vec![
                    OpcionNarrativa::nueva(
                        "opcion_1",
                        "Enfrentar directamente",
                        Consecuencia::nueva("consecuencia_1", "Resolución por confrontación")
                            .con_impacto_medidor("coherencia", 10)
                            .con_impacto_medidor("relacional", -8)
                            .con_puntuacion_moral(-15),
                    ),
                    OpcionNarrativa::nueva(
                        "opcion_2",
                        "Buscar mediación",
                        Consecuencia::nueva("consecuencia_2", "Resolución por diálogo")
                            .con_impacto_medidor("coherencia", 5)
                            .con_impacto_medidor("relacional", 10)
                            .con_puntuacion_moral(10),
                    ).como_recomendada(),
                ]
            }
            TipoPaso::Climax => {
                vec![
                    OpcionNarrativa::nueva(
                        "opcion_1",
                        "Tomar la decisión difícil",
                        Consecuencia::nueva("consecuencia_1", "Decisión con alto impacto moral")
                            .con_impacto_medidor("coherencia", -5)
                            .con_impacto_medidor("influencia", 20)
                            .con_puntuacion_moral(25)
                            .como_final(),
                    ),
                ]
            }
            TipoPaso::Resolucion => {
                vec![
                    OpcionNarrativa::nueva(
                        "opcion_1",
                        "Continuar",
                        Consecuencia::nueva("consecuencia_final", "Narrativa concluida satisfactoriamente")
                            .con_impacto_medidor("recursos", 5)
                            .como_final(),
                    ).como_recomendada(),
                ]
            }
        }
    }
}

/// Tipo de paso en una plantilla
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TipoPaso {
    Inicio,
    Conflicto,
    Climax,
    Resolucion,
}

/// Paso en una plantilla de guion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasoPlantilla {
    /// Nombre del paso
    pub nombre: String,
    /// Descripción del paso
    pub descripcion: String,
    /// Tipo de paso
    pub tipo_paso: TipoPaso,
    /// Duración estimada (en eventos)
    pub duracion: u8,
}

impl PasoPlantilla {
    pub fn nuevo(nombre: &str, descripcion: &str, tipo_paso: TipoPaso) -> Self {
        Self {
            nombre: nombre.to_string(),
            descripcion: descripcion.to_string(),
            tipo_paso,
            duracion: 1,
        }
    }

    pub fn con_duracion(mut self, duracion: u8) -> Self {
        self.duracion = duracion;
        self
    }
}

/// Generador de narrativas usando IA
pub struct GeneradorNarrativas {
    /// Configuración de IA
    configuracion_ia: ConfiguracionIA,
    /// Plantillas disponibles
    plantillas: HashMap<TipoPlantillaGuion, PlantillaGuion>,
    /// Contador para IDs
    contador: u64,
}

impl GeneradorNarrativas {
    pub fn nuevo(configuracion_ia: ConfiguracionIA) -> Self {
        Self {
            configuracion_ia,
            plantillas: Self::cargar_plantillas_por_defecto(),
            contador: 0,
        }
    }

    fn cargar_plantillas_por_defecto() -> HashMap<TipoPlantillaGuion, PlantillaGuion> {
        let mut plantillas = HashMap::new();
        
        // Plantilla Hollywood Animal
        let hollywood = PlantillaGuion::nueva(
            TipoPlantillaGuion::HollywoodAnimal,
            "Estructura Clásica",
            "Estructura de 3 actos: Setup, Confrontación, Resolución",
        )
        .con_paso(PasoPlantilla::nuevo(
            "Setup",
            "Presentación de personajes y situación inicial",
            TipoPaso::Inicio,
        ).con_duracion(1))
        .con_paso(PasoPlantilla::nuevo(
            "Confrontación",
            "El conflicto principal se revela",
            TipoPaso::Conflicto,
        ).con_duracion(2))
        .con_paso(PasoPlantilla::nuevo(
            "Resolución",
            "El conflicto se resuelve",
            TipoPaso::Resolucion,
        ).con_duracion(1));
        
        plantillas.insert(TipoPlantillaGuion::HollywoodAnimal, hollywood);
        
        // Plantilla Hero's Journey
        let heros_journey = PlantillaGuion::nueva(
            TipoPlantillaGuion::HeroesJourney,
            "Viaje del Héroe",
            "Estructura clásica del viaje del héroe",
        )
        .con_paso(PasoPlantilla::nuevo(
            "Llamado a la Aventura",
            "El personaje recibe el llamado",
            TipoPaso::Inicio,
        ))
        .con_paso(PasoPlantilla::nuevo(
            "Pruebas y Aliados",
            "El personaje enfrenta pruebas y encuentra aliados",
            TipoPaso::Conflicto,
        ))
        .con_paso(PasoPlantilla::nuevo(
            "Enfrentamiento Final",
            "El personaje se enfrenta al villano o desafío final",
            TipoPaso::Climax,
        ))
        .con_paso(PasoPlantilla::nuevo(
            "Retorno",
            "El personaje regresa transformado",
            TipoPaso::Resolucion,
        ));
        
        plantillas.insert(TipoPlantillaGuion::HeroesJourney, heros_journey);
        
        // Plantilla Dilema Moral
        let dilema = PlantillaGuion::nueva(
            TipoPlantillaGuion::DilemaMoral,
            "Dilema Moral",
            "Situación que requiere elección ética",
        )
        .con_paso(PasoPlantilla::nuevo(
            "Presentación del Dilema",
            "Se plantea la situación moral",
            TipoPaso::Inicio,
        ))
        .con_paso(PasoPlantilla::nuevo(
            "Análisis de Opciones",
            "Se exploran las consecuencias de cada opción",
            TipoPaso::Conflicto,
        ))
        .con_paso(PasoPlantilla::nuevo(
            "Decisión",
            "El personaje elige una opción",
            TipoPaso::Climax,
        ))
        .con_paso(PasoPlantilla::nuevo(
            "Consecuencias",
            "Se revelan las consecuencias de la decisión",
            TipoPaso::Resolucion,
        ));
        
        plantillas.insert(TipoPlantillaGuion::DilemaMoral, dilema);
        
        // Plantilla Procedimiento Judicial
        let judicial = PlantillaGuion::nueva(
            TipoPlantillaGuion::ProcedimientoJudicial,
            "Procedimiento Judicial",
            "Estructura de proceso judicial histórico",
        )
        .con_paso(PasoPlantilla::nuevo(
            "Acusación",
            "Se presenta la acusación",
            TipoPaso::Inicio,
        ))
        .con_paso(PasoPlantilla::nuevo(
            "Defensa",
            "La defensa presenta sus argumentos",
            TipoPaso::Conflicto,
        ))
        .con_paso(PasoPlantilla::nuevo(
            "Deliberación",
            "El tribunal delibera",
            TipoPaso::Conflicto,
        ))
        .con_paso(PasoPlantilla::nuevo(
            "Veredicto",
            "Se emite el veredicto final",
            TipoPaso::Resolucion,
        ));
        
        plantillas.insert(TipoPlantillaGuion::ProcedimientoJudicial, judicial);
        
        plantillas
    }

    /// Generar narrativa usando una plantilla específica
    pub fn generar_con_plantilla(&mut self, tipo: TipoPlantillaGuion) -> Option<Narrativa> {
        self.plantillas.get(&tipo).map(|plantilla| {
            self.contador += 1;
            let mut narrativa = plantilla.generar_narrativa(&self.configuracion_ia);
            narrativa.metadatos.id = format!("narrativa_{}", self.contador);
            narrativa
        })
    }

    /// Generar texto contextual usando IA (placeholder)
    pub fn generar_texto_contextual(
        &self,
        prompt: &str,
        estilo: &EstiloNarrativo,
        contexto: &str,
    ) -> String {
        // Implementación placeholder - en producción usaría la API de Mistral
        // con self.configuracion_ia
        
        let prompt_completo = match estilo {
            EstiloNarrativo::Detallado => {
                format!(
                    "Genera una descripción detallada y descriptiva para el siguiente contexto: {}. \
                     Usa un estilo literario rico y evocador. Contexto: {}",
                    prompt, contexto
                )
            }
            EstiloNarrativo::Equilibrado => {
                format!(
                    "Genera una descripción equilibrada para: {}. \
                     Mantén un balance entre detalle y concisión. Contexto: {}",
                    prompt, contexto
                )
            }
            EstiloNarrativo::Conciso => {
                format!(
                    "Genera una descripción breve y directa para: {}. \
                     Sé conciso y ve al punto. Contexto: {}",
                    prompt, contexto
                )
            }
        };
        
        // Simular respuesta de IA (en producción real se llamaría a la API)
        format!("[IA] Respuesta generada para: {}", prompt_completo)
    }

    /// Obtener prompt específico para un estilo narrativo
    pub fn obtener_prompt_estilo(&self, estilo: &EstiloNarrativo, tipo_evento: &str) -> String {
        match estilo {
            EstiloNarrativo::Detallado => {
                format!(
                    "Escribe una narrativa detallada y atmosférica para un evento de tipo '{}'. \
                     Incluye descripciones sensoriales y contexto histórico. \
                     Usa un lenguaje rico y evocador.",
                    tipo_evento
                )
            }
            EstiloNarrativo::Equilibrado => {
                format!(
                    "Escribe una narrativa equilibrada para un evento de tipo '{}'. \
                     Mantén un buen ritmo con descripciones adecuadas. \
                     Incluye contexto relevante sin exceso.",
                    tipo_evento
                )
            }
            EstiloNarrativo::Conciso => {
                format!(
                    "Escribe una narrativa breve para un evento de tipo '{}'. \
                     Sé directo y enfócate en lo esencial. \
                     Evita descripciones innecesarias.",
                    tipo_evento
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_narrativa_metadatos_creacion() {
        let meta = NarrativaMetadatos::nuevo("test1", "Test Narrativa", TipoPlantillaGuion::DilemaMoral);
        assert_eq!(meta.id, "test1");
        assert_eq!(meta.titulo, "Test Narrativa");
        assert_eq!(meta.tipo_plantilla, TipoPlantillaGuion::DilemaMoral);
        assert_eq!(meta.dificultad, 5);
    }

    #[test]
    fn test_consecuencia_creacion() {
        let consec = Consecuencia::nueva("cons1", "Consecuencia de prueba")
            .con_impacto_medidor("influencia", 10)
            .con_puntuacion_moral(5);
        
        assert_eq!(consec.id, "cons1");
        assert_eq!(consec.impacto_medidores.get("influencia"), Some(&10));
        assert_eq!(consec.puntuacion_moral, 5);
    }

    #[test]
    fn test_opcion_narrativa_creacion() {
        let consec = Consecuencia::nueva("cons1", "Test");
        let opcion = OpcionNarrativa::nueva("opt1", "Opcion 1", consec)
            .con_peso(75)
            .como_recomendada();
        
        assert_eq!(opcion.id, "opt1");
        assert_eq!(opcion.peso, 75);
        assert!(opcion.recomendada);
    }

    #[test]
    fn test_evento_narrativo_validacion() {
        let consec = Consecuencia::nueva("cons1", "Test").con_impacto_medidor("test", 1);
        let opcion = OpcionNarrativa::nueva("opt1", "Opcion 1", consec);
        let evento = EventoNarrativo::nuevo("evt1", "Evento 1", "Descripcion")
            .con_opcion(opcion);
        
        assert!(evento.validar().is_ok());
    }

    #[test]
    fn test_evento_narrativo_sin_opciones() {
        let evento = EventoNarrativo::nuevo("evt1", "Evento 1", "Descripcion");
        assert!(evento.validar().is_err());
    }

    #[test]
    fn test_cadena_eventos_sin_bucles() {
        let consec = Consecuencia::nueva("cons1", "Test").como_final();
        let opcion = OpcionNarrativa::nueva("opt1", "Opcion 1", consec);
        
        let evento1 = EventoNarrativo::nuevo("evt1", "Evento 1", "Desc 1")
            .con_opcion(opcion)
            .con_evento_siguiente("evt2");
        
        let consec2 = Consecuencia::nueva("cons2", "Test 2").como_final();
        let opcion2 = OpcionNarrativa::nueva("opt2", "Opcion 2", consec2);
        let evento2 = EventoNarrativo::nuevo("evt2", "Evento 2", "Desc 2")
            .con_opcion(opcion2);
        
        let cadena = CadenaEventos::nueva("cadena1", "evt1")
            .con_eventos(vec![evento1, evento2])
            .con_evento_final("evt2");
        
        assert!(cadena.validar_sin_bucles().is_ok());
    }

    #[test]
    fn test_narrativa_validacion_completa() {
        let consec = Consecuencia::nueva("cons1", "Test").con_impacto_medidor("test", 1);
        let opcion = OpcionNarrativa::nueva("opt1", "Opcion 1", consec);
        let evento = EventoNarrativo::nuevo("evt1", "Evento 1", "Desc")
            .con_opcion(opcion);
        
        let cadena = CadenaEventos::nueva("cadena1", "evt1")
            .con_evento(evento);
        
        let meta = NarrativaMetadatos::nuevo("narr1", "Narrativa 1", TipoPlantillaGuion::DilemaMoral);
        let narrativa = Narrativa::nueva(meta, cadena);
        
        assert!(narrativa.validar().is_ok());
    }

    #[test]
    fn test_generador_narrativas_plantillas() {
        let config_ia = ConfiguracionIA::default();
        let generador = GeneradorNarrativas::nuevo(config_ia);
        
        assert!(generador.plantillas.contains_key(&TipoPlantillaGuion::HollywoodAnimal));
        assert!(generador.plantillas.contains_key(&TipoPlantillaGuion::HeroesJourney));
        assert!(generador.plantillas.contains_key(&TipoPlantillaGuion::DilemaMoral));
        assert!(generador.plantillas.contains_key(&TipoPlantillaGuion::ProcedimientoJudicial));
    }

    #[test]
    fn test_generar_narrativa_con_plantilla() {
        let config_ia = ConfiguracionIA::default();
        let mut generador = GeneradorNarrativas::nuevo(config_ia);
        
        let narrativa = generador.generar_con_plantilla(TipoPlantillaGuion::DilemaMoral);
        assert!(narrativa.is_some());
        
        let narrativa = narrativa.unwrap();
        assert_eq!(narrativa.metadatos.tipo_plantilla, TipoPlantillaGuion::DilemaMoral);
        assert!(!narrativa.cadena_principal.eventos.is_empty());
    }

    #[test]
    fn test_prompt_estilo() {
        let config_ia = ConfiguracionIA::default();
        let generador = GeneradorNarrativas::nuevo(config_ia);
        
        let prompt_detallado = generador.obtener_prompt_estilo(&EstiloNarrativo::Detallado, "dilema");
        assert!(prompt_detallado.contains("detallada"));
        
        let prompt_conciso = generador.obtener_prompt_estilo(&EstiloNarrativo::Conciso, "dilema");
        assert!(prompt_conciso.contains("breve"));
    }
}
