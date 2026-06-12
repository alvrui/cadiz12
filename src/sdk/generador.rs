use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use md5::compute;
use crate::config::{
    PartidaConfig, FaccionId, FaccionConfig, FaccionEstado, FaccionLider,
    EspacioId, EspacioConfig, ClimaEspacio,
    Origen, ClaseSocial, Oficio, Temperamento, AdscripcionPolitica,
    PerfilProtagonista, NpcConfig, HabilidadNarrativa,
    TipoEvento, EventoFijo, OpcionEvento, EventoPlantilla, EventosConfig,
    MedidoresConfig,
};
use anyhow::{Result, Context};
use rand::Rng;

/// Configuracion de inteligencia artificial para generacion de contenido
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfiguracionIA {
    /// Modelo de lenguaje a usar
    pub modelo: ModeloIA,
    /// Temperatura para generacion (0.0 a 2.0)
    pub temperatura: f32,
    /// Maximo de tokens a generar
    pub max_tokens: u32,
    /// Top-k sampling
    pub top_k: Option<u32>,
    /// Top-p sampling (nucleus)
    pub top_p: Option<f32>,
    /// Frecuencia penalty
    pub frequency_penalty: Option<f32>,
    /// Presencia penalty
    pub presence_penalty: Option<f32>,
    /// Numero de resultados a generar
    pub n: u8,
    /// Stop sequences
    pub stop_sequences: Vec<String>,
}

impl Default for ConfiguracionIA {
    fn default() -> Self {
        Self {
            modelo: ModeloIA::MistralSmall,
            temperatura: 0.7,
            max_tokens: 1024,
            top_k: None,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            n: 1,
            stop_sequences: vec!["\n\n".to_string(), "<|im_end|>".to_string()],
        }
    }
}

/// Modelo de IA disponible
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModeloIA {
    /// Mistral 7B
    Mistral7B,
    /// Mistral Small
    MistralSmall,
    /// Mistral Large
    MistralLarge,
    /// OpenAI GPT-3.5
    Gpt35Turbo,
    /// OpenAI GPT-4
    Gpt4,
    /// Local LLaMA
    Llama2,
    /// Local CodeLLaMA
    CodeLlama,
}

impl Default for ModeloIA {
    fn default() -> Self {
        Self::MistralSmall
    }
}

impl std::fmt::Display for ModeloIA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mistral7B => write!(f, "mistral-7b"),
            Self::MistralSmall => write!(f, "mistral-small"),
            Self::MistralLarge => write!(f, "mistral-large"),
            Self::Gpt35Turbo => write!(f, "gpt-3.5-turbo"),
            Self::Gpt4 => write!(f, "gpt-4"),
            Self::Llama2 => write!(f, "llama2"),
            Self::CodeLlama => write!(f, "codellama"),
        }
    }
}

/// Cache de generacion para evitar calls redundantes a la API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheGeneracion {
    /// Directorio de cache
    pub directorio: PathBuf,
    /// Tiempo de expiracion en segundos (default: 86400 = 24 horas)
    pub ttl_segundos: u64,
    /// Tamano maximo del cache en MB
    pub max_tamano_mb: usize,
    /// Habilitar cache
    pub habilitado: bool,
}

impl Default for CacheGeneracion {
    fn default() -> Self {
        Self {
            directorio: PathBuf::from(".cache/ia"),
            ttl_segundos: 86400, // 24 horas
            max_tamano_mb: 100,
            habilitado: true,
        }
    }
}

impl CacheGeneracion {
    pub fn nuevo(directorio: &str) -> Self {
        Self {
            directorio: PathBuf::from(directorio),
            ..Default::default()
        }
    }

    pub fn con_ttl(mut self, horas: u64) -> Self {
        self.ttl_segundos = horas * 3600;
        self
    }

    pub fn con_tamano_max(mut self, mb: usize) -> Self {
        self.max_tamano_mb = mb;
        self
    }

    /// Generar clave de cache para una peticion
    pub fn generar_clave(&self, prompt: &str, configuracion: &ConfiguracionIA) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let hash = compute(prompt.as_bytes());
        
        format!(
            "{}_{}_{}_{:x}",
            timestamp / self.ttl_segundos,
            configuracion.modelo,
            configuracion.temperatura,
            hash
        )
    }
}

/// Estado del cache de generacion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstadoCache {
    pub entradas: usize,
    pub tamano_bytes: usize,
    pub tiempo_ultima_limpieza: u64,
}

/// Generador de configuraciones usando patrones predefinidos
#[derive(Debug, Clone)]
pub struct GeneradorConfig {
    /// Configuracion de IA para generacion
    pub configuracion_ia: Option<ConfiguracionIA>,
    /// Cache de generacion
    pub cache: CacheGeneracion,
    #[allow(dead_code)]
    cliente_mistral: Option<ClienteMistral>,
}

impl Default for GeneradorConfig {
    fn default() -> Self {
        Self {
            configuracion_ia: None,
            cache: CacheGeneracion::default(),
            cliente_mistral: None,
        }
    }
}

impl GeneradorConfig {
    pub fn nuevo() -> Self {
        Self::default()
    }

    pub fn con_mistral(api_key: String) -> Self {
        Self {
            cliente_mistral: Some(ClienteMistral::nuevo(api_key)),
            ..Default::default()
        }
    }

    /// Configurar IA con configuracion personalizada
    pub fn con_ia(mut self, configuracion: ConfiguracionIA) -> Self {
        self.configuracion_ia = Some(configuracion);
        self
    }

    /// Configurar cache
    pub fn con_cache(mut self, cache: CacheGeneracion) -> Self {
        self.cache = cache;
        self
    }

    /// Configurar con IA y cache
    pub fn con_ia_y_cache(api_key: String, configuracion: ConfiguracionIA, cache: CacheGeneracion) -> Self {
        Self {
            cliente_mistral: Some(ClienteMistral::nuevo(api_key)),
            configuracion_ia: Some(configuracion),
            cache,
        }
    }

    /// Obtener configuracion de IA o default
    pub fn obtener_configuracion_ia(&self) -> ConfiguracionIA {
        self.configuracion_ia.clone().unwrap_or_default()
    }

    /// Validar configuracion
    pub fn validar(&self) -> Result<(), String> {
        if let Some(ref ia) = self.configuracion_ia {
            if ia.temperatura < 0.0 || ia.temperatura > 2.0 {
                return Err("Temperatura debe estar entre 0.0 y 2.0".to_string());
            }
            if ia.max_tokens == 0 {
                return Err("max_tokens debe ser mayor que 0".to_string());
            }
            if ia.n == 0 {
                return Err("n debe ser mayor que 0".to_string());
            }
        }
        if self.cache.habilitado && self.cache.directorio.to_string_lossy().is_empty() {
            return Err("El directorio de cache no puede estar vacio".to_string());
        }
        Ok(())
    }

    /// Generar una configuración de partida completa
    pub fn generar_partida(
        &self,
        nombre: &str,
        descripcion: &str,
        perfil: PerfilProtagonista,
    ) -> Result<PartidaConfig> {
        let mut config = PartidaConfig {
            nombre: nombre.to_string(),
            descripcion: descripcion.to_string(),
            periodo_inicio: "1810-09-24".to_string(),
            periodo_fin: "1814-05-01".to_string(),
            jornadas_totales: 1000,
            medidores: MedidoresConfig::default(),
            facciones: self.generar_facciones()?,
            espacios: self.generar_espacios()?,
            personajes: self.generar_personajes(&perfil)?,
            eventos: self.generar_eventos()?,
            perfil_inicial: perfil.clone(),
            espacio_inicial: EspacioId::CafeApolo,
            presupuesto_temporal: 8,
            preferencias: crate::config::PreferenciasJuego::default(),
            progresion: crate::config::ConfiguracionProgresion::default(),
            contexto_inicial: crate::config::ContextoHistorico::default(),
        };

        // Ajustar medidores según perfil
        self.ajustar_medidores_iniciales(&mut config.medidores, &perfil);

        Ok(config)
    }

    /// Generar facciones con valores aleatorios pero coherentes
    pub fn generar_facciones(&self) -> Result<crate::config::facciones::FaccionesConfig> {
        let mut facciones = HashMap::new();
        let mut rng = rand::thread_rng();

        // Liberal Progresista
        facciones.insert(FaccionId::LiberalProgresista, FaccionConfig {
            id: FaccionId::LiberalProgresista,
            nombre: "Liberales Progresistas".to_string(),
            descripcion: "Partidarios de reformas profundas y rápidas. Defienden la soberanía nacional y las libertades individuales.".to_string(),
            lideres: vec![
                FaccionLider { id: "arguelles".to_string(), nombre: "Agustín Argüelles".to_string(), descripcion: "Líder intelectual".to_string(), influencia: rng.gen_range(90..100) },
                FaccionLider { id: "munoz_torrero".to_string(), nombre: "Diego Muñoz Torrero".to_string(), descripcion: "Estratega".to_string(), influencia: rng.gen_range(85..95) },
            ],
            estado_inicial: FaccionEstado {
                fuerza: rng.gen_range(3..5),
                necesidad_del_jugador: rng.gen_range(1..3),
                cohesion_interna: rng.gen_range(2..4),
                linea_roja_activa: false,
                vigilancia_sobre_jugador: rng.gen_range(1..3),
            },
            temas_sensibles: vec!["soberania_nacional".to_string(), "libertades_individuales".to_string()],
            alianzas: vec![FaccionId::LiberalModerado, FaccionId::Americanista],
            conflictos: vec![FaccionId::AbsolutistaServil],
        });

        // Liberal Moderado
        facciones.insert(FaccionId::LiberalModerado, FaccionConfig {
            id: FaccionId::LiberalModerado,
            nombre: "Liberales Moderados".to_string(),
            descripcion: "Reformistas graduales. Buscan equilibrio entre tradición y progreso.".to_string(),
            lideres: vec![
                FaccionLider { id: "martinez_rosa".to_string(), nombre: "Francisco de Paula Martínez de la Rosa".to_string(), descripcion: "Mediador".to_string(), influencia: rng.gen_range(85..95) },
            ],
            estado_inicial: FaccionEstado {
                fuerza: rng.gen_range(2..4),
                necesidad_del_jugador: rng.gen_range(2..4),
                cohesion_interna: rng.gen_range(1..3),
                linea_roja_activa: false,
                vigilancia_sobre_jugador: rng.gen_range(1..3),
            },
            temas_sensibles: vec!["equilibrio_poder".to_string(), "estabilidad".to_string()],
            alianzas: vec![FaccionId::LiberalProgresista, FaccionId::AbsolutistaServil],
            conflictos: vec![FaccionId::IndependienteOportunista],
        });

        // Absolutista/Servil
        facciones.insert(FaccionId::AbsolutistaServil, FaccionConfig {
            id: FaccionId::AbsolutistaServil,
            nombre: "Absolutistas/Serviles".to_string(),
            descripcion: "Defensores del Antiguo Régimen y la autoridad real.".to_string(),
            lideres: vec![
                FaccionLider { id: "ostolaza".to_string(), nombre: "Fernando Ostolaza".to_string(), descripcion: "Conservador intransigente".to_string(), influencia: rng.gen_range(80..90) },
            ],
            estado_inicial: FaccionEstado {
                fuerza: rng.gen_range(1..3),
                necesidad_del_jugador: rng.gen_range(0..2),
                cohesion_interna: rng.gen_range(2..4),
                linea_roja_activa: true,
                vigilancia_sobre_jugador: rng.gen_range(2..4),
            },
            temas_sensibles: vec!["tradicion".to_string(), "autoridad_real".to_string()],
            alianzas: vec![FaccionId::LiberalModerado],
            conflictos: vec![FaccionId::LiberalProgresista, FaccionId::Americanista],
        });

        // Independiente/Oportunista
        facciones.insert(FaccionId::IndependienteOportunista, FaccionConfig {
            id: FaccionId::IndependienteOportunista,
            nombre: "Independientes/Oportunistas".to_string(),
            descripcion: "Políticos sin adhesión fija. Actúan según conveniencia.".to_string(),
            lideres: vec![],
            estado_inicial: FaccionEstado {
                fuerza: rng.gen_range(1..3),
                necesidad_del_jugador: rng.gen_range(1..3),
                cohesion_interna: 1,
                linea_roja_activa: false,
                vigilancia_sobre_jugador: rng.gen_range(0..2),
            },
            temas_sensibles: vec!["beneficio_personal".to_string()],
            alianzas: vec![],
            conflictos: vec![FaccionId::LiberalModerado],
        });

        // Americanista
        facciones.insert(FaccionId::Americanista, FaccionConfig {
            id: FaccionId::Americanista,
            nombre: "Americanistas".to_string(),
            descripcion: "Diputados de América. Divididos entre reformar el Imperio o independizarse.".to_string(),
            lideres: vec![
                FaccionLider { id: "power".to_string(), nombre: "José Miguel Power y Morante".to_string(), descripcion: "Representante de Buenos Aires".to_string(), influencia: rng.gen_range(80..90) },
                FaccionLider { id: "ramos_arizpe".to_string(), nombre: "Miguel Ramos Arizpe".to_string(), descripcion: "Representante de Nueva España".to_string(), influencia: rng.gen_range(85..95) },
            ],
            estado_inicial: FaccionEstado {
                fuerza: rng.gen_range(2..4),
                necesidad_del_jugador: rng.gen_range(2..4),
                cohesion_interna: rng.gen_range(1..3),
                linea_roja_activa: false,
                vigilancia_sobre_jugador: rng.gen_range(1..3),
            },
            temas_sensibles: vec!["equacion_imperial".to_string(), "reformar_o_independizar".to_string()],
            alianzas: vec![FaccionId::LiberalProgresista],
            conflictos: vec![FaccionId::AbsolutistaServil],
        });

        Ok(crate::config::facciones::FaccionesConfig { facciones })
    }

    /// Generar espacios
    pub fn generar_espacios(&self) -> Result<crate::config::espacios::EspaciosConfig> {
        let mut espacios = HashMap::new();
        let mut rng = rand::thread_rng();

        // Oratorio de San Felipe Neri
        espacios.insert(EspacioId::OratorioSanFelipeNeri, EspacioConfig {
            id: EspacioId::OratorioSanFelipeNeri,
            nombre: "Oratorio de San Felipe Neri".to_string(),
            descripcion: "Sede de las Cortes desde febrero de 1811.".to_string(),
            funcion: "Debate formal, votación, enmienda, procedimiento parlamentario".to_string(),
            tipo_interaccion: "Institucional".to_string(),
            coste_temporal: 4,
            nivel_riesgo: rng.gen_range(2..4),
            clima_inicial: ClimaEspacio::Nervioso,
            npcs_iniciales: vec!["arguelles".to_string(), "munoz_torrero".to_string()],
            requerimientos: vec!["ser_diputado".to_string()],
        });

        // Café de Apolo
        espacios.insert(EspacioId::CafeApolo, EspacioConfig {
            id: EspacioId::CafeApolo,
            nombre: "Café de Apolo".to_string(),
            descripcion: "El filtro popular de la información impresa.".to_string(),
            funcion: "Rumor, opinión, reclutamiento de apoyos, provocación".to_string(),
            tipo_interaccion: "Social".to_string(),
            coste_temporal: 2,
            nivel_riesgo: rng.gen_range(1..3),
            clima_inicial: ClimaEspacio::Efervescente,
            npcs_iniciales: vec!["periodista_1".to_string()],
            requerimientos: vec![],
        });

        // Muelles y Aduana
        espacios.insert(EspacioId::MuellesAduana, EspacioConfig {
            id: EspacioId::MuellesAduana,
            nombre: "Muelles y Aduana".to_string(),
            descripcion: "Nodo del comercio atlántico.".to_string(),
            funcion: "Inteligencia comercial, favores financieros, espionaje".to_string(),
            tipo_interaccion: "Económico".to_string(),
            coste_temporal: 3,
            nivel_riesgo: rng.gen_range(2..4),
            clima_inicial: ClimaEspacio::Saturado,
            npcs_iniciales: vec!["comerciante_1".to_string()],
            requerimientos: vec![],
        });

        // Calle Nueva y Consulado
        espacios.insert(EspacioId::CalleNuevaConsulado, EspacioConfig {
            id: EspacioId::CalleNuevaConsulado,
            nombre: "Calle Nueva y el Consulado".to_string(),
            descripcion: "Centro financiero de Cádiz.".to_string(),
            funcion: "Crédito, lobby, presión económica".to_string(),
            tipo_interaccion: "Económico".to_string(),
            coste_temporal: 3,
            nivel_riesgo: rng.gen_range(1..3),
            clima_inicial: ClimaEspacio::Vigilado,
            npcs_iniciales: vec![],
            requerimientos: vec![],
        });

        // Catedral
        espacios.insert(EspacioId::CatedralRedEclesiastica, EspacioConfig {
            id: EspacioId::CatedralRedEclesiastica,
            nombre: "Catedral y red eclesiástica".to_string(),
            descripcion: "Centro religioso de la ciudad.".to_string(),
            funcion: "Legitimidad moral, presión doctrinal, mediación conservadora".to_string(),
            tipo_interaccion: "Religioso".to_string(),
            coste_temporal: 3,
            nivel_riesgo: rng.gen_range(1..3),
            clima_inicial: ClimaEspacio::Tranquilo,
            npcs_iniciales: vec!["obispo_1".to_string()],
            requerimientos: vec![],
        });

        Ok(crate::config::espacios::EspaciosConfig { espacios })
    }

    /// Generar personajes/NPCs
    pub fn generar_personajes(
        &self,
        _perfil_jugador: &PerfilProtagonista,
    ) -> Result<crate::config::personajes::PersonajesConfig> {
        let mut npcs = HashMap::new();
        let mut rng = rand::thread_rng();

        // Generar líderes de facciones
        let lideres = vec![
            ("arguelles", FaccionId::LiberalProgresista, Origen::Peninsular, ClaseSocial::HidalguiaProfesionLetrada, Oficio::JuristaAbogado, AdscripcionPolitica::LiberalProgresista, Temperamento::Retorico),
            ("munoz_torrero", FaccionId::LiberalProgresista, Origen::Peninsular, ClaseSocial::HidalguiaProfesionLetrada, Oficio::JuristaAbogado, AdscripcionPolitica::LiberalProgresista, Temperamento::Prudente),
            ("martinez_rosa", FaccionId::LiberalModerado, Origen::Peninsular, ClaseSocial::HidalguiaProfesionLetrada, Oficio::JuristaAbogado, AdscripcionPolitica::LiberalModerado, Temperamento::Pragmatico),
            ("ostolaza", FaccionId::AbsolutistaServil, Origen::Peninsular, ClaseSocial::HidalguiaProfesionLetrada, Oficio::JuristaAbogado, AdscripcionPolitica::AbsolutistaServil, Temperamento::Esceptico),
            ("power", FaccionId::Americanista, Origen::AmericanoVirreinal, ClaseSocial::EliteCriollaAmericana, Oficio::JuristaAbogado, AdscripcionPolitica::Americanista, Temperamento::Idealista),
        ];

        for (id, faccion, origen, clase, oficio, adscripcion, temperamento) in lideres {
            let habilidades = match oficio {
                Oficio::JuristaAbogado => vec![
                    HabilidadNarrativa { id: "lectura_tecnica".to_string(), nombre: "Lectura técnica".to_string(), descripcion: "Analiza textos legales".to_string(), escenas_determinantes: vec!["comisiones".to_string()] },
                ],
                Oficio::PeriodistaPublicista => vec![
                    HabilidadNarrativa { id: "encargo_velado".to_string(), nombre: "Encargo velado".to_string(), descripcion: "Publica sin firma".to_string(), escenas_determinantes: vec!["imprentas".to_string()] },
                ],
                _ => vec![],
            };

            let espacios = match oficio {
                Oficio::JuristaAbogado => vec!["oratorio_san_felipe_neri".to_string(), "comisiones_parlamentarias".to_string()],
                Oficio::PeriodistaPublicista => vec!["cafe_apolo".to_string(), "imprentas_publicaciones".to_string()],
                Oficio::ComercianteAgenteMercantil => vec!["muelles_aduana".to_string(), "calle_nueva_consulado".to_string()],
                _ => vec!["cafe_apolo".to_string()],
            };

            npcs.insert(id.to_string(), NpcConfig {
                id: id.to_string(),
                nombre: self.generar_nombre(id, &faccion)?,
                descripcion: self.generar_descripcion(id, &faccion, &clase)?,
                faccion: faccion.clone(),
                origen: origen.clone(),
                clase_social: clase.clone(),
                oficio: oficio.clone(),
                adscripcion: adscripcion.clone(),
                temperamento: temperamento.clone(),
                relacion_inicial: rng.gen_range(-10..11),
                confianza_inicial: rng.gen_range(20..51),
                deuda_inicial: 0,
                habilidades,
                espacios,
            });
        }

        // Generar NPCs genéricos
        let npcs_genericos = vec![
            ("periodista_1", FaccionId::LiberalProgresista, Origen::Peninsular, ClaseSocial::HidalguiaProfesionLetrada, Oficio::PeriodistaPublicista),
            ("comerciante_1", FaccionId::LiberalModerado, Origen::Gaditano, ClaseSocial::EliteMercantilGaditana, Oficio::ComercianteAgenteMercantil),
            ("obispo_1", FaccionId::AbsolutistaServil, Origen::Peninsular, ClaseSocial::CleroIlustrado, Oficio::ClerigoIlustrado),
            ("oficial_1", FaccionId::LiberalProgresista, Origen::Peninsular, ClaseSocial::CarreraMilitar, Oficio::OficialEjercito),
        ];

        for (id, faccion, origen, clase, oficio) in npcs_genericos {
            let adscripcion = match faccion {
                FaccionId::LiberalProgresista => AdscripcionPolitica::LiberalProgresista,
                FaccionId::LiberalModerado => AdscripcionPolitica::LiberalModerado,
                FaccionId::AbsolutistaServil => AdscripcionPolitica::AbsolutistaServil,
                _ => AdscripcionPolitica::IndependienteOportunista,
            };

            let temperamento = match rng.gen_range(0..6) {
                0 => Temperamento::Retorico,
                1 => Temperamento::Prudente,
                2 => Temperamento::Ambicioso,
                3 => Temperamento::Esceptico,
                4 => Temperamento::Idealista,
                _ => Temperamento::Pragmatico,
            };

            let habilidades = match oficio {
                Oficio::PeriodistaPublicista => vec![
                    HabilidadNarrativa { id: "encargo_velado".to_string(), nombre: "Encargo velado".to_string(), descripcion: "Publica sin firma".to_string(), escenas_determinantes: vec!["imprentas".to_string()] },
                    HabilidadNarrativa { id: "lectura_clima".to_string(), nombre: "Lectura de clima".to_string(), descripcion: "Evalúa impacto público".to_string(), escenas_determinantes: vec!["cafes".to_string()] },
                ],
                Oficio::ComercianteAgenteMercantil => vec![
                    HabilidadNarrativa { id: "tasacion_favor".to_string(), nombre: "Tasación de favor".to_string(), descripcion: "Evalúa valor de compromisos".to_string(), escenas_determinantes: vec!["negociaciones".to_string()] },
                    HabilidadNarrativa { id: "contacto_muelle".to_string(), nombre: "Contacto de muelle".to_string(), descripcion: "Información antes que la prensa".to_string(), escenas_determinantes: vec!["muelles".to_string()] },
                ],
                Oficio::ClerigoIlustrado => vec![
                    HabilidadNarrativa { id: "autoridad_moral".to_string(), nombre: "Autoridad moral".to_string(), descripcion: "Posiciones difíciles de atacar".to_string(), escenas_determinantes: vec!["debates".to_string()] },
                    HabilidadNarrativa { id: "red_confesional".to_string(), nombre: "Red confesional".to_string(), descripcion: "Información privada".to_string(), escenas_determinantes: vec!["investigaciones".to_string()] },
                ],
                Oficio::OficialEjercito => vec![
                    HabilidadNarrativa { id: "analisis_tactico".to_string(), nombre: "Análisis táctico".to_string(), descripcion: "Evalúa partes militares".to_string(), escenas_determinantes: vec!["crisis_militares".to_string()] },
                ],
                _ => vec![],
            };

            let espacios = match oficio {
                Oficio::PeriodistaPublicista => vec!["cafe_apolo".to_string(), "imprentas_publicaciones".to_string()],
                Oficio::ComercianteAgenteMercantil => vec!["muelles_aduana".to_string(), "calle_nueva_consulado".to_string()],
                Oficio::ClerigoIlustrado => vec!["catedral_red_eclesiastica".to_string()],
                Oficio::OficialEjercito => vec!["barrio_san_carlos_capitania".to_string()],
                _ => vec!["cafe_apolo".to_string()],
            };

            npcs.insert(id.to_string(), NpcConfig {
                id: id.to_string(),
                nombre: self.generar_nombre(id, &faccion)?,
                descripcion: self.generar_descripcion(id, &faccion, &clase)?,
                faccion: faccion.clone(),
                origen: origen.clone(),
                clase_social: clase.clone(),
                oficio: oficio.clone(),
                adscripcion,
                temperamento,
                relacion_inicial: rng.gen_range(-5..6),
                confianza_inicial: rng.gen_range(15..41),
                deuda_inicial: 0,
                habilidades,
                espacios,
            });
        }

        Ok(crate::config::personajes::PersonajesConfig { npcs })
    }

    fn generar_nombre(&self, id: &str, _faccion: &FaccionId) -> Result<String> {
        let nombres: HashMap<&str, &str> = HashMap::from([
            ("arguelles", "Agustín Argüelles"),
            ("munoz_torrero", "Diego Muñoz Torrero"),
            ("martinez_rosa", "Francisco de Paula Martínez de la Rosa"),
            ("ostolaza", "Fernando Ostolaza"),
            ("power", "José Miguel Power y Morante"),
            ("ramos_arizpe", "Miguel Ramos Arizpe"),
            ("periodista_1", "Manuel José Quintano"),
            ("comerciante_1", "Juan Pérez de la Riva"),
            ("obispo_1", "Fray Francisco de Paula Sancha"),
            ("oficial_1", "Coronel Francisco Copons"),
        ]);

        Ok(nombres.get(id).map(|&s| s.to_string()).unwrap_or_else(|| format!("Personaje {}", id)))
    }

    fn generar_descripcion(&self, id: &str, faccion: &FaccionId, clase: &ClaseSocial) -> Result<String> {
        let descripciones: HashMap<&str, &str> = HashMap::from([
            ("arguelles", "Líder intelectual del grupo liberal progresista. Jurista brillante y orador elocuente."),
            ("munoz_torrero", "Estratega liberal. Menos visible que Argüelles pero igualmente influyente."),
            ("martinez_rosa", "El gran mediador entre facciones. Todos le necesitan, nadie le respeta del todo."),
            ("ostolaza", "Conservador intransigente. Defensor a ultranza del Antiguo Régimen."),
            ("power", "Representante de Buenos Aires. Defensor de los intereses americanos."),
            ("periodista_1", "Periodista liberal. Escribe para el Semanario Patriótico."),
            ("comerciante_1", "Comerciante del Consulado. Tiene contactos en el comercio atlántico."),
            ("obispo_1", "Obispo de Cádiz. Representa la voz conservadora de la Iglesia."),
            ("oficial_1", "Oficial del Ejército. Defensor de Cádiz durante el asedio."),
        ]);

        Ok(descripciones.get(id).map(|&s| s.to_string()).unwrap_or_else(|| {
            match (faccion, clase) {
                (FaccionId::LiberalProgresista, ClaseSocial::HidalguiaProfesionLetrada) => String::from("Jurista liberal progresista"),
                (FaccionId::LiberalModerado, ClaseSocial::HidalguiaProfesionLetrada) => String::from("Jurista liberal moderado"),
                (FaccionId::AbsolutistaServil, ClaseSocial::HidalguiaProfesionLetrada) => String::from("Jurista conservador"),
                (FaccionId::Americanista, ClaseSocial::EliteCriollaAmericana) => String::from("Representante americano"),
                _ => format!("Miembro de {:?} de clase {:?}", faccion, clase),
            }
        }))
    }

    /// Generar eventos
    pub fn generar_eventos(&self) -> Result<EventosConfig> {
        let _rng = rand::thread_rng();

        let eventos_fijos = vec![
            EventoFijo { id: "trafalgar".to_string(), nombre: "Batalla de Trafalgar".to_string(), fecha: "1805-10-21".to_string(), descripcion: "Derrota de la flota franco-española frente a Nelson.".to_string(), impacto: "Aumento del aislamiento de Cádiz.".to_string(), jornada: 0, recursos_visuales: None },
            EventoFijo { id: "inauguracion_cortes".to_string(), nombre: "Inauguración de las Cortes".to_string(), fecha: "1810-09-24".to_string(), descripcion: "Primera sesión de las Cortes de Cádiz.".to_string(), impacto: "Inicio del proceso constituyente.".to_string(), jornada: 100, recursos_visuales: None },
            EventoFijo { id: "decreto_libertad_imprenta".to_string(), nombre: "Decreto de libertad de imprenta".to_string(), fecha: "1810-11-10".to_string(), descripcion: "Las Cortes aprueban la libertad de imprenta.".to_string(), impacto: "Explosión de periódicos y panfletos.".to_string(), jornada: 120, recursos_visuales: None },
            EventoFijo { id: "promulgacion_la_pepa".to_string(), nombre: "Promulgación de La Pepa".to_string(), fecha: "1812-03-19".to_string(), descripcion: "Se promulga la Constitución de 1812.".to_string(), impacto: "Culmen del proceso constituyente.".to_string(), jornada: 300, recursos_visuales: None },
        ];

        let mut plantillas = HashMap::new();

        // Plantilla: Debate constitucional
        plantillas.insert("debate_articulo_constitucional".to_string(), EventoPlantilla {
            id: "debate_articulo_constitucional".to_string(),
            titulo: "Debate sobre artículo constitucional".to_string(),
            tipo: TipoEvento::SesionInstitucional,
            descripcion: "Las Cortes debaten un artículo clave de la nueva Constitución.".to_string(),
            coste_temporal: 4,
            prioridad_base: 85,
            familias: vec!["sesion_institucional".to_string(), "constitucion".to_string()],
            requerimientos: HashMap::from([("ser_diputado".to_string(), "true".to_string())]),
            consecuencias: HashMap::from([("influencia".to_string(), 5), ("coherencia".to_string(), 3)]),
            opciones: vec![
                OpcionEvento {
                    id: "apoyar_liberal".to_string(),
                    texto: "Apoyar la posición liberal".to_string(),
                    modificador_perfil: 1.5,
                    coste_adicional: 0,
                    consecuencias: HashMap::from([
                        ("reputacion_liberal_progresista".to_string(), 15),
                        ("reputacion_absolutista_servil".to_string(), -10),
                        ("coherencia".to_string(), 5),
                    ]),
                    requerimientos: vec![],
                },
                OpcionEvento {
                    id: "apoyar_moderado".to_string(),
                    texto: "Apoyar la posición moderada".to_string(),
                    modificador_perfil: 1.2,
                    coste_adicional: 0,
                    consecuencias: HashMap::from([
                        ("reputacion_liberal_moderado".to_string(), 12),
                        ("reputacion_liberal_progresista".to_string(), -5),
                        ("coherencia".to_string(), 3),
                    ]),
                    requerimientos: vec![],
                },
            ],
            recursos_visuales: None,
        });

        // Plantilla: Rumor en café
        plantillas.insert("rumor_en_cafe".to_string(), EventoPlantilla {
            id: "rumor_en_cafe".to_string(),
            titulo: "Rumor en el café".to_string(),
            tipo: TipoEvento::EncuentroUrbano,
            descripcion: "En el café circula un rumor sobre un evento importante.".to_string(),
            coste_temporal: 2,
            prioridad_base: 60,
            familias: vec!["social".to_string(), "informacion".to_string()],
            requerimientos: HashMap::new(),
            consecuencias: HashMap::from([("relacional".to_string(), 2)]),
            opciones: vec![
                OpcionEvento {
                    id: "escuchar_atentamente".to_string(),
                    texto: "Escuchar atentamente".to_string(),
                    modificador_perfil: 1.0,
                    coste_adicional: 0,
                    consecuencias: HashMap::from([("relacional".to_string(), 3)]),
                    requerimientos: vec![],
                },
                OpcionEvento {
                    id: "difundir_rumor".to_string(),
                    texto: "Difundir el rumor".to_string(),
                    modificador_perfil: 1.3,
                    coste_adicional: 0,
                    consecuencias: HashMap::from([
                        ("relacional".to_string(), 5),
                        ("reputacion".to_string(), -2),
                    ]),
                    requerimientos: vec![],
                },
            ],
            recursos_visuales: None,
        });

        let familias = HashMap::from([
            ("sesion_institucional".to_string(), vec!["debate_articulo_constitucional".to_string()]),
            ("social".to_string(), vec!["rumor_en_cafe".to_string()]),
        ]);

        Ok(EventosConfig {
            eventos_fijos,
            plantillas,
            familias,
        })
    }

    /// Ajustar medidores iniciales según perfil
    fn ajustar_medidores_iniciales(
        &self,
        medidores: &mut MedidoresConfig,
        perfil: &PerfilProtagonista,
    ) {
        let _rng = rand::thread_rng();

        // Ajustar según origen
        match perfil.origen {
            Origen::Gaditano => {
                medidores.relacional.umbral_bajo = 20;
                medidores.relacional.umbral_alto = 80;
            }
            Origen::Peninsular => {
                medidores.influencia.umbral_bajo = 25;
                medidores.influencia.umbral_alto = 85;
            }
            Origen::AmericanoVirreinal => {
                medidores.coherencia.umbral_bajo = 45;
                medidores.coherencia.umbral_alto = 95;
            }
        }

        // Ajustar según clase social
        match perfil.clase_social {
            ClaseSocial::EliteMercantilGaditana => {
                medidores.recursos.umbral_bajo = 30;
                medidores.recursos.umbral_alto = 85;
            }
            ClaseSocial::HidalguiaProfesionLetrada => {
                medidores.influencia.umbral_bajo = 35;
                medidores.influencia.umbral_alto = 90;
            }
            _ => {}
        }

        // Ajustar según oficio
        match perfil.oficio {
            Oficio::JuristaAbogado => {
                medidores.influencia.umbral_bajo = 35;
                medidores.influencia.umbral_alto = 90;
            }
            Oficio::PeriodistaPublicista => {
                medidores.relacional.umbral_bajo = 25;
                medidores.relacional.umbral_alto = 80;
            }
            _ => {}
        }
    }
}

/// Cliente para interactuar con Mistral API
#[derive(Debug, Clone)]
pub struct ClienteMistral {
    api_key: String,
    client: reqwest::Client,
}

impl ClienteMistral {
    pub fn nuevo(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    /// Generar un personaje usando Mistral
    pub async fn generar_personaje(
        &self,
        prompt: &str,
        faccion: &FaccionId,
        oficio: &Oficio,
    ) -> Result<NpcConfig> {
        let system_prompt = format!(
            "Eres un experto en historia de las Cortes de Cádiz (1810-1814). \
            Genera un personaje histórico ficticio pero verosímil para el juego. \
            El personaje debe encajar en el contexto de Cádiz sitiada por los franceses. \
            Responde SOLO con JSON válido, sin markdown, sin explicaciones. \
            El JSON debe tener exactamente estos campos: id, nombre, descripcion, faccion, origen, clase_social, oficio, adscripcion, temperamento."
        );

        let user_prompt = format!(
            "Genera un personaje para la facción {:?} con oficio {:?}. \
            El personaje debe ser realista para el período 1810-1814 en Cádiz. \
            Usa nombres españoles de la época. \
            {}\n
            Formato esperado:\n            {{\n                \"id\": \"string\",\n                \"nombre\": \"string\",\n                \"descripcion\": \"string\",\n                \"faccion\": \"liberal_progresista|liberal_moderado|absolutista_servil|independiente_oportunista|americanista\",\n                \"origen\": \"gaditano|peninsular|americano_virreinal\",\n                \"clase_social\": \"elite_mercantil_gaditana|hidalguia_profesion_letrada|clero_ilustrado|carrera_militar|elite_criolla_americana|funcionariado_ilustrado\",\n                \"oficio\": \"jurista_abogado|periodista_publicista|comerciante_agente_mercantil|oficial_ejercito|clerigo_ilustrado|medico_ilustrado\",\n                \"adscripcion\": \"liberal_progresista|liberal_moderado|absolutista_servil|independiente_oportunista|americanista\",\n                \"temperamento\": \"retorico|prudente|ambicioso|esceptico|idealista|pragmatico\"\n            }}"
        , faccion, oficio, prompt);

        let request = serde_json::json!({
            "model": "mistral-medium",
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_prompt}
            ],
            "temperature": 0.7,
            "max_tokens": 512,
        });

        let response = self.client
            .post("https://api.mistral.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Error al conectar con Mistral API")?;

        if !response.status().is_success() {
            anyhow::bail!("Error de Mistral API: {}", response.status());
        }

        let json: serde_json::Value = response.json().await?;
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .context("No se encontró content en la respuesta")?
            .trim()
            .to_string();

        // Parsear el JSON generado
        let npc_data: serde_json::Value = serde_json::from_str(&content)
            .context("La respuesta de Mistral no es JSON válido")?;

        // Convertir a NpcConfig
        let id = npc_data["id"].as_str().context("Falta campo id")?.to_string();
        let nombre = npc_data["nombre"].as_str().context("Falta campo nombre")?.to_string();
        let descripcion = npc_data["descripcion"].as_str().context("Falta campo descripcion")?.to_string();

        let faccion_str = npc_data["faccion"].as_str().context("Falta campo faccion")?;
        let faccion = match faccion_str {
            "liberal_progresista" => FaccionId::LiberalProgresista,
            "liberal_moderado" => FaccionId::LiberalModerado,
            "absolutista_servil" => FaccionId::AbsolutistaServil,
            "independiente_oportunista" => FaccionId::IndependienteOportunista,
            "americanista" => FaccionId::Americanista,
            _ => anyhow::bail!("Facción desconocida: {}", faccion_str),
        };

        let origen_str = npc_data["origen"].as_str().context("Falta campo origen")?;
        let origen = match origen_str {
            "gaditano" => Origen::Gaditano,
            "peninsular" => Origen::Peninsular,
            "americano_virreinal" => Origen::AmericanoVirreinal,
            _ => anyhow::bail!("Origen desconocido: {}", origen_str),
        };

        let clase_str = npc_data["clase_social"].as_str().context("Falta campo clase_social")?;
        let clase_social = match clase_str {
            "elite_mercantil_gaditana" => ClaseSocial::EliteMercantilGaditana,
            "hidalguia_profesion_letrada" => ClaseSocial::HidalguiaProfesionLetrada,
            "clero_ilustrado" => ClaseSocial::CleroIlustrado,
            "carrera_militar" => ClaseSocial::CarreraMilitar,
            "elite_criolla_americana" => ClaseSocial::EliteCriollaAmericana,
            "funcionariado_ilustrado" => ClaseSocial::FuncionariadoIlustrado,
            _ => anyhow::bail!("Clase social desconocida: {}", clase_str),
        };

        let oficio_str = npc_data["oficio"].as_str().context("Falta campo oficio")?;
        let oficio = match oficio_str {
            "jurista_abogado" => Oficio::JuristaAbogado,
            "periodista_publicista" => Oficio::PeriodistaPublicista,
            "comerciante_agente_mercantil" => Oficio::ComercianteAgenteMercantil,
            "oficial_ejercito" => Oficio::OficialEjercito,
            "clerigo_ilustrado" => Oficio::ClerigoIlustrado,
            "medico_ilustrado" => Oficio::MedicoIlustrado,
            _ => anyhow::bail!("Oficio desconocido: {}", oficio_str),
        };

        let adscripcion_str = npc_data["adscripcion"].as_str().context("Falta campo adscripcion")?;
        let adscripcion = match adscripcion_str {
            "liberal_progresista" => AdscripcionPolitica::LiberalProgresista,
            "liberal_moderado" => AdscripcionPolitica::LiberalModerado,
            "absolutista_servil" => AdscripcionPolitica::AbsolutistaServil,
            "independiente_oportunista" => AdscripcionPolitica::IndependienteOportunista,
            "americanista" => AdscripcionPolitica::Americanista,
            _ => anyhow::bail!("Adscripción desconocida: {}", adscripcion_str),
        };

        let temperamento_str = npc_data["temperamento"].as_str().context("Falta campo temperamento")?;
        let temperamento = match temperamento_str {
            "retorico" => Temperamento::Retorico,
            "prudente" => Temperamento::Prudente,
            "ambicioso" => Temperamento::Ambicioso,
            "esceptico" => Temperamento::Esceptico,
            "idealista" => Temperamento::Idealista,
            "pragmatico" => Temperamento::Pragmatico,
            _ => anyhow::bail!("Temperamento desconocido: {}", temperamento_str),
        };

        // Generar datos adicionales
        let mut rng = rand::thread_rng();
        let habilidades = self.generar_habilidades(&oficio);
        let espacios = self.generar_espacios_npc(&oficio, &faccion);

        Ok(NpcConfig {
            id: id.clone(),
            nombre,
            descripcion,
            faccion,
            origen,
            clase_social,
            oficio,
            adscripcion,
            temperamento,
            relacion_inicial: rng.gen_range(-10..11),
            confianza_inicial: rng.gen_range(20..51),
            deuda_inicial: 0,
            habilidades,
            espacios,
        })
    }

    fn generar_habilidades(&self, oficio: &Oficio) -> Vec<HabilidadNarrativa> {
        match oficio {
            Oficio::JuristaAbogado => vec![
                HabilidadNarrativa { id: "lectura_tecnica".to_string(), nombre: "Lectura técnica".to_string(), descripcion: "Analiza textos legales".to_string(), escenas_determinantes: vec!["comisiones".to_string()] },
            ],
            Oficio::PeriodistaPublicista => vec![
                HabilidadNarrativa { id: "encargo_velado".to_string(), nombre: "Encargo velado".to_string(), descripcion: "Publica sin firma".to_string(), escenas_determinantes: vec!["imprentas".to_string()] },
                HabilidadNarrativa { id: "lectura_clima".to_string(), nombre: "Lectura de clima".to_string(), descripcion: "Evalúa impacto público".to_string(), escenas_determinantes: vec!["cafes".to_string()] },
            ],
            Oficio::ComercianteAgenteMercantil => vec![
                HabilidadNarrativa { id: "tasacion_favor".to_string(), nombre: "Tasación de favor".to_string(), descripcion: "Evalúa valor de compromisos".to_string(), escenas_determinantes: vec!["negociaciones".to_string()] },
            ],
            Oficio::OficialEjercito => vec![
                HabilidadNarrativa { id: "analisis_tactico".to_string(), nombre: "Análisis táctico".to_string(), descripcion: "Evalúa partes militares".to_string(), escenas_determinantes: vec!["crisis_militares".to_string()] },
            ],
            Oficio::ClerigoIlustrado => vec![
                HabilidadNarrativa { id: "autoridad_moral".to_string(), nombre: "Autoridad moral".to_string(), descripcion: "Posiciones difíciles de atacar".to_string(), escenas_determinantes: vec!["debates".to_string()] },
            ],
            Oficio::MedicoIlustrado => vec![
                HabilidadNarrativa { id: "diagnostico_crisis".to_string(), nombre: "Diagnóstico de crisis".to_string(), descripcion: "Evalúa gravedad de epidemias".to_string(), escenas_determinantes: vec!["epidemias".to_string()] },
            ],
        }
    }

    fn generar_espacios_npc(&self, oficio: &Oficio, _faccion: &FaccionId) -> Vec<String> {
        match oficio {
            Oficio::JuristaAbogado => vec!["oratorio_san_felipe_neri".to_string(), "comisiones_parlamentarias".to_string()],
            Oficio::PeriodistaPublicista => vec!["cafe_apolo".to_string(), "imprentas_publicaciones".to_string()],
            Oficio::ComercianteAgenteMercantil => vec!["muelles_aduana".to_string(), "calle_nueva_consulado".to_string()],
            Oficio::OficialEjercito => vec!["barrio_san_carlos_capitania".to_string()],
            Oficio::ClerigoIlustrado => vec!["catedral_red_eclesiastica".to_string()],
            Oficio::MedicoIlustrado => vec!["barrio_la_vina".to_string()],
        }
    }
}
