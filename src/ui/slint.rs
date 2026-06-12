use slint::SharedString;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use crate::{config::PartidaConfig, engine::Motor, engine::dtos::EstadoJornadaDto};

// Global state for motor and UI handle
thread_local! {
    static UI_STATE: RefCell<Option<GameWindowState>> = RefCell::new(None);
}

#[derive(Clone)]
pub struct GameWindowState {
    pub motor: Arc<Mutex<Motor>>,
    pub handle: slint::Weak<GameWindow>,
}

slint::slint! {
    import { Button, VerticalBox, HorizontalBox, ScrollView, LineEdit } from "std-widgets.slint";

    export component GameWindow inherits Window {
        width: 1024px;
        height: 768px;
        title: "Cádiz 1812";

        in-out property <string> jornada-text;
        in-out property <string> acto-text;
        in-out property <string> tramo-text;
        in-out property <string> presupuesto-text;
        in-out property <string> mensaje-text;

        // Propiedades para medidores (maximo 5)
        in-out property <string> medidor-0-nombre;
        in-out property <string> medidor-0-valor;
        in-out property <string> medidor-0-tendencia;
        in-out property <string> medidor-0-umbrales;
        in-out property <string> medidor-1-nombre;
        in-out property <string> medidor-1-valor;
        in-out property <string> medidor-1-tendencia;
        in-out property <string> medidor-1-umbrales;
        in-out property <string> medidor-2-nombre;
        in-out property <string> medidor-2-valor;
        in-out property <string> medidor-2-tendencia;
        in-out property <string> medidor-2-umbrales;
        in-out property <string> medidor-3-nombre;
        in-out property <string> medidor-3-valor;
        in-out property <string> medidor-3-tendencia;
        in-out property <string> medidor-3-umbrales;
        in-out property <string> medidor-4-nombre;
        in-out property <string> medidor-4-valor;
        in-out property <string> medidor-4-tendencia;
        in-out property <string> medidor-4-umbrales;

        // Propiedades para eventos (maximo 10)
        in-out property <string> evento-0-titulo;
        in-out property <string> evento-1-titulo;
        in-out property <string> evento-2-titulo;
        in-out property <string> evento-3-titulo;
        in-out property <string> evento-4-titulo;
        in-out property <string> evento-5-titulo;
        in-out property <string> evento-6-titulo;
        in-out property <string> evento-7-titulo;
        in-out property <string> evento-8-titulo;
        in-out property <string> evento-9-titulo;

        // Propiedades para seleccion
        in-out property <int> selected-event;
        in-out property <int> selected-option;
        
        // Callback para resolver eventos
        callback resolve-event-callback(int, int);

        VerticalBox {
            width: 100%;
            height: 100%;
            spacing: 0px;

            // Header
            Rectangle {
                width: 100%;
                height: 80px;
                background: #1a1a20;

                HorizontalBox {
                    width: 100%;
                    height: 100%;
                    padding-left: 20px;
                    padding-right: 20px;

                    Text {
                        text: "CADIZ 1812";
                        font-size: 28px;
                        font-weight: 700;
                    }
                }
            }

            // Info bar
            Rectangle {
                width: 100%;
                height: 40px;
                background: #162130;

                HorizontalBox {
                    width: 100%;
                    height: 100%;
                    padding-left: 20px;
                    padding-right: 20px;
                    spacing: 15px;

                    Text { text: "Jornada: "; }
                    Text { text: jornada-text; font-weight: 600; }
                    Text { text: "|"; }
                    Text { text: " Acto: "; }
                    Text { text: acto-text; font-weight: 600; }
                    Text { text: "|"; }
                    Text { text: " Tramo: "; }
                    Text { text: tramo-text; font-weight: 600; }
                    Text { text: "|"; }
                    Text { text: " Presupuesto: "; }
                    Text { text: presupuesto-text; font-weight: 600; }
                }
            }

            // Separator
            Rectangle {
                height: 2px;
                width: 100%;
                background: #0f3460;
            }

            // Content
            HorizontalBox {
                width: 100%;
                height: 100%;
                spacing: 0px;

                // Medidores panel (30%)
                Rectangle {
                    width: 30%;
                    height: 100%;
                    background: #162130;

                    VerticalBox {
                        width: 100%;
                        height: 100%;
                        padding-left: 10px;
                        padding-right: 10px;
                        padding-top: 10px;
                        spacing: 5px;

                        Text {
                            text: "MEDIDORES";
                            font-size: 16px;
                            font-weight: 700;
                            horizontal-alignment: center;
                        }
                        
                        Rectangle {
                            width: 100%;
                            height: 1px;
                            background: #0f3460;
                        }
                        
                        ScrollView {
                            width: 100%;
                            height: 100%;
                            VerticalBox {
                                spacing: 5px;
                                padding-top: 5px;

                                HorizontalBox {
                                    Text { text: medidor-0-nombre; width: 120px; }
                                    Text { text: medidor-0-valor; width: 50px; horizontal-alignment: end; }
                                    Text { text: medidor-0-tendencia; width: 50px; horizontal-alignment: end; }
                                    Text { text: medidor-0-umbrales; width: 80px; horizontal-alignment: end; }
                                }
                                HorizontalBox {
                                    Text { text: medidor-1-nombre; width: 120px; }
                                    Text { text: medidor-1-valor; width: 50px; horizontal-alignment: end; }
                                    Text { text: medidor-1-tendencia; width: 50px; horizontal-alignment: end; }
                                    Text { text: medidor-1-umbrales; width: 80px; horizontal-alignment: end; }
                                }
                                HorizontalBox {
                                    Text { text: medidor-2-nombre; width: 120px; }
                                    Text { text: medidor-2-valor; width: 50px; horizontal-alignment: end; }
                                    Text { text: medidor-2-tendencia; width: 50px; horizontal-alignment: end; }
                                    Text { text: medidor-2-umbrales; width: 80px; horizontal-alignment: end; }
                                }
                                HorizontalBox {
                                    Text { text: medidor-3-nombre; width: 120px; }
                                    Text { text: medidor-3-valor; width: 50px; horizontal-alignment: end; }
                                    Text { text: medidor-3-tendencia; width: 50px; horizontal-alignment: end; }
                                    Text { text: medidor-3-umbrales; width: 80px; horizontal-alignment: end; }
                                }
                                HorizontalBox {
                                    Text { text: medidor-4-nombre; width: 120px; }
                                    Text { text: medidor-4-valor; width: 50px; horizontal-alignment: end; }
                                    Text { text: medidor-4-tendencia; width: 50px; horizontal-alignment: end; }
                                    Text { text: medidor-4-umbrales; width: 80px; horizontal-alignment: end; }
                                }
                            }
                        }
                    }
                }

                // Eventos panel (70%)
                Rectangle {
                    width: 70%;
                    height: 100%;
                    background: #1a1a20;

                    VerticalBox {
                        width: 100%;
                        height: 100%;
                        padding-left: 10px;
                        padding-right: 10px;
                        padding-top: 10px;
                        spacing: 10px;

                        Text {
                            text: "EVENTOS DISPONIBLES";
                            font-size: 16px;
                            font-weight: 700;
                            horizontal-alignment: center;
                        }
                        
                        Rectangle {
                            width: 100%;
                            height: 1px;
                            background: #0f3460;
                        }
                        
                        ScrollView {
                            width: 100%;
                            height: 100%;
                            VerticalBox {
                                spacing: 8px;
                                padding-top: 5px;

                                Button {
                                    text: evento-0-titulo;
                                    width: 100%;
                                    clicked => { 
                                        root.selected-event = 0;
                                        root.selected-option = -1;
                                        root.mensaje-text = "Evento 0 seleccionado. Elija opcion (1-9)";
                                    }
                                }
                                Button {
                                    text: evento-1-titulo;
                                    width: 100%;
                                    clicked => { 
                                        root.selected-event = 1;
                                        root.selected-option = -1;
                                        root.mensaje-text = "Evento 1 seleccionado. Elija opcion (1-9)";
                                    }
                                }
                                Button {
                                    text: evento-2-titulo;
                                    width: 100%;
                                    clicked => { 
                                        root.selected-event = 2;
                                        root.selected-option = -1;
                                        root.mensaje-text = "Evento 2 seleccionado. Elija opcion (1-9)";
                                    }
                                }
                                Button {
                                    text: evento-3-titulo;
                                    width: 100%;
                                    clicked => { 
                                        root.selected-event = 3;
                                        root.selected-option = -1;
                                        root.mensaje-text = "Evento 3 seleccionado. Elija opcion (1-9)";
                                    }
                                }
                                Button {
                                    text: evento-4-titulo;
                                    width: 100%;
                                    clicked => { 
                                        root.selected-event = 4;
                                        root.selected-option = -1;
                                        root.mensaje-text = "Evento 4 seleccionado. Elija opcion (1-9)";
                                    }
                                }
                                Button {
                                    text: evento-5-titulo;
                                    width: 100%;
                                    clicked => { 
                                        root.selected-event = 5;
                                        root.selected-option = -1;
                                        root.mensaje-text = "Evento 5 seleccionado. Elija opcion (1-9)";
                                    }
                                }
                                Button {
                                    text: evento-6-titulo;
                                    width: 100%;
                                    clicked => { 
                                        root.selected-event = 6;
                                        root.selected-option = -1;
                                        root.mensaje-text = "Evento 6 seleccionado. Elija opcion (1-9)";
                                    }
                                }
                                Button {
                                    text: evento-7-titulo;
                                    width: 100%;
                                    clicked => { 
                                        root.selected-event = 7;
                                        root.selected-option = -1;
                                        root.mensaje-text = "Evento 7 seleccionado. Elija opcion (1-9)";
                                    }
                                }
                                Button {
                                    text: evento-8-titulo;
                                    width: 100%;
                                    clicked => { 
                                        root.selected-event = 8;
                                        root.selected-option = -1;
                                        root.mensaje-text = "Evento 8 seleccionado. Elija opcion (1-9)";
                                    }
                                }
                                Button {
                                    text: evento-9-titulo;
                                    width: 100%;
                                    clicked => { 
                                        root.selected-event = 9;
                                        root.selected-option = -1;
                                        root.mensaje-text = "Evento 9 seleccionado. Elija opcion (1-9)";
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Separator
            Rectangle {
                height: 2px;
                width: 100%;
                background: #0f3460;
            }

            // Footer con botones de opcion 1-9
            Rectangle {
                width: 100%;
                height: 80px;
                background: #162130;
                
                VerticalBox {
                    width: 100%;
                    height: 100%;
                    padding-left: 20px;
                    padding-right: 20px;
                    padding-top: 5px;
                    padding-bottom: 5px;
                    spacing: 5px;

                    Text {
                        text: mensaje-text;
                        font-size: 14px;
                    }
                    
                    HorizontalBox {
                        width: 100%;
                        spacing: 5px;

                        Button {
                            text: "1";
                            width: 40px;
                            clicked => { 
                                if (root.selected-event >= 0) {
                                    root.selected-option = 1;
                                    resolve-event-callback(root.selected-event, 1);
                                }
                            }
                        }
                        Button {
                            text: "2";
                            width: 40px;
                            clicked => { 
                                if (root.selected-event >= 0) {
                                    root.selected-option = 2;
                                    resolve-event-callback(root.selected-event, 2);
                                }
                            }
                        }
                        Button {
                            text: "3";
                            width: 40px;
                            clicked => { 
                                if (root.selected-event >= 0) {
                                    root.selected-option = 3;
                                    resolve-event-callback(root.selected-event, 3);
                                }
                            }
                        }
                        Button {
                            text: "4";
                            width: 40px;
                            clicked => { 
                                if (root.selected-event >= 0) {
                                    root.selected-option = 4;
                                    resolve-event-callback(root.selected-event, 4);
                                }
                            }
                        }
                        Button {
                            text: "5";
                            width: 40px;
                            clicked => { 
                                if (root.selected-event >= 0) {
                                    root.selected-option = 5;
                                    resolve-event-callback(root.selected-event, 5);
                                }
                            }
                        }
                        Button {
                            text: "6";
                            width: 40px;
                            clicked => { 
                                if (root.selected-event >= 0) {
                                    root.selected-option = 6;
                                    resolve-event-callback(root.selected-event, 6);
                                }
                            }
                        }
                        Button {
                            text: "7";
                            width: 40px;
                            clicked => { 
                                if (root.selected-event >= 0) {
                                    root.selected-option = 7;
                                    resolve-event-callback(root.selected-event, 7);
                                }
                            }
                        }
                        Button {
                            text: "8";
                            width: 40px;
                            clicked => { 
                                if (root.selected-event >= 0) {
                                    root.selected-option = 8;
                                    resolve-event-callback(root.selected-event, 8);
                                }
                            }
                        }
                        Button {
                            text: "9";
                            width: 40px;
                            clicked => { 
                                if (root.selected-event >= 0) {
                                    root.selected-option = 9;
                                    resolve-event-callback(root.selected-event, 9);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn ejecutar_juego(config: PartidaConfig) -> anyhow::Result<()> {
    let motor = Arc::new(Mutex::new(Motor::nuevo(config)));
    let estado = {
        let mut m = motor.lock().unwrap();
        m.api.estado_jornada()
    };

    let ui = GameWindow::new()?;
    
    // Store motor and UI handle in thread-local state
    let ui_state = GameWindowState {
        motor: Arc::clone(&motor),
        handle: ui.as_weak(),
    };
    
    UI_STATE.with(|state| {
        *state.borrow_mut() = Some(ui_state);
    });
    
    // Register the callback for resolving events
    let ui_handle = ui.as_weak();
    ui.on_resolve_event_callback(move |event_index, option_index| {
        if let Some(handle) = ui_handle.upgrade() {
            resolve_event_callback(event_index, option_index, handle);
        }
    });
    
    actualizar_ui(&ui, &estado);
    ui.run()?;
    Ok(())
}

fn actualizar_ui(ui: &GameWindow, estado: &EstadoJornadaDto) {
    ui.set_jornada_text(SharedString::from(estado.tiempo.jornada.to_string()));
    ui.set_acto_text(SharedString::from(estado.tiempo.acto.to_string()));
    ui.set_tramo_text(SharedString::from(estado.tiempo.tramo_id.clone()));
    ui.set_presupuesto_text(SharedString::from(format!("{}/{}", estado.presupuesto_temporal, estado.presupuesto_temporal)));
    ui.set_mensaje_text(SharedString::from("Seleccione un evento y luego una opcion (1-9)".to_string()));
    ui.set_selected_event(-1);
    ui.set_selected_option(-1);

    // Actualizar medidores (hasta 5)
    for (i, m) in estado.protagonista.medidores.iter().take(5).enumerate() {
        match i {
            0 => {
                ui.set_medidor_0_nombre(SharedString::from(m.nombre.clone()));
                ui.set_medidor_0_valor(SharedString::from(m.valor.to_string()));
                ui.set_medidor_0_tendencia(SharedString::from(m.tendencia.to_string()));
                ui.set_medidor_0_umbrales(SharedString::from(format!("[{}-{}]", m.umbral_bajo, m.umbral_alto)));
            }
            1 => {
                ui.set_medidor_1_nombre(SharedString::from(m.nombre.clone()));
                ui.set_medidor_1_valor(SharedString::from(m.valor.to_string()));
                ui.set_medidor_1_tendencia(SharedString::from(m.tendencia.to_string()));
                ui.set_medidor_1_umbrales(SharedString::from(format!("[{}-{}]", m.umbral_bajo, m.umbral_alto)));
            }
            2 => {
                ui.set_medidor_2_nombre(SharedString::from(m.nombre.clone()));
                ui.set_medidor_2_valor(SharedString::from(m.valor.to_string()));
                ui.set_medidor_2_tendencia(SharedString::from(m.tendencia.to_string()));
                ui.set_medidor_2_umbrales(SharedString::from(format!("[{}-{}]", m.umbral_bajo, m.umbral_alto)));
            }
            3 => {
                ui.set_medidor_3_nombre(SharedString::from(m.nombre.clone()));
                ui.set_medidor_3_valor(SharedString::from(m.valor.to_string()));
                ui.set_medidor_3_tendencia(SharedString::from(m.tendencia.to_string()));
                ui.set_medidor_3_umbrales(SharedString::from(format!("[{}-{}]", m.umbral_bajo, m.umbral_alto)));
            }
            4 => {
                ui.set_medidor_4_nombre(SharedString::from(m.nombre.clone()));
                ui.set_medidor_4_valor(SharedString::from(m.valor.to_string()));
                ui.set_medidor_4_tendencia(SharedString::from(m.tendencia.to_string()));
                ui.set_medidor_4_umbrales(SharedString::from(format!("[{}-{}]", m.umbral_bajo, m.umbral_alto)));
            }
            _ => {}
        }
    }

    // Limpiar medidores restantes
    if estado.protagonista.medidores.len() < 5 {
        for i in estado.protagonista.medidores.len()..5 {
            match i {
                0 => { ui.set_medidor_0_nombre(SharedString::from("")); }
                1 => { ui.set_medidor_1_nombre(SharedString::from("")); }
                2 => { ui.set_medidor_2_nombre(SharedString::from("")); }
                3 => { ui.set_medidor_3_nombre(SharedString::from("")); }
                4 => { ui.set_medidor_4_nombre(SharedString::from("")); }
                _ => {}
            }
        }
    }

    // Actualizar eventos (hasta 10)
    for (i, e) in estado.eventos_disponibles.iter().take(10).enumerate() {
        match i {
            0 => ui.set_evento_0_titulo(SharedString::from(format!("{} ({} pts)", e.titulo, e.coste_temporal))),
            1 => ui.set_evento_1_titulo(SharedString::from(format!("{} ({} pts)", e.titulo, e.coste_temporal))),
            2 => ui.set_evento_2_titulo(SharedString::from(format!("{} ({} pts)", e.titulo, e.coste_temporal))),
            3 => ui.set_evento_3_titulo(SharedString::from(format!("{} ({} pts)", e.titulo, e.coste_temporal))),
            4 => ui.set_evento_4_titulo(SharedString::from(format!("{} ({} pts)", e.titulo, e.coste_temporal))),
            5 => ui.set_evento_5_titulo(SharedString::from(format!("{} ({} pts)", e.titulo, e.coste_temporal))),
            6 => ui.set_evento_6_titulo(SharedString::from(format!("{} ({} pts)", e.titulo, e.coste_temporal))),
            7 => ui.set_evento_7_titulo(SharedString::from(format!("{} ({} pts)", e.titulo, e.coste_temporal))),
            8 => ui.set_evento_8_titulo(SharedString::from(format!("{} ({} pts)", e.titulo, e.coste_temporal))),
            9 => ui.set_evento_9_titulo(SharedString::from(format!("{} ({} pts)", e.titulo, e.coste_temporal))),
            _ => {}
        }
    }

    // Limpiar eventos restantes
    if estado.eventos_disponibles.len() < 10 {
        for i in estado.eventos_disponibles.len()..10 {
            match i {
                0 => ui.set_evento_0_titulo(SharedString::from("")),
                1 => ui.set_evento_1_titulo(SharedString::from("")),
                2 => ui.set_evento_2_titulo(SharedString::from("")),
                3 => ui.set_evento_3_titulo(SharedString::from("")),
                4 => ui.set_evento_4_titulo(SharedString::from("")),
                5 => ui.set_evento_5_titulo(SharedString::from("")),
                6 => ui.set_evento_6_titulo(SharedString::from("")),
                7 => ui.set_evento_7_titulo(SharedString::from("")),
                8 => ui.set_evento_8_titulo(SharedString::from("")),
                9 => ui.set_evento_9_titulo(SharedString::from("")),
                _ => {}
            }
        }
    }
}

/// Callback para resolver un evento, invocado desde Slint
fn resolve_event_callback(event_index: i32, option_index: i32, ui: GameWindow) {
    // Obtener estado global
    let state = UI_STATE.with(|s| s.borrow().clone());
    
    if let Some(state) = state {
        // Validar indices
        if event_index < 0 || event_index > 9 || option_index < 1 || option_index > 9 {
            ui.set_mensaje_text(SharedString::from("Error: Seleccion invalida".to_string()));
            return;
        }

        // Obtener estado actual para obtener el evento_id
        let evento_id = {
            let mut motor = state.motor.lock().unwrap();
            let estado = motor.api.estado_jornada();
            
            // Obtener evento_id del evento seleccionado
            if event_index as usize >= estado.eventos_disponibles.len() {
                ui.set_mensaje_text(SharedString::from("Error: Indice de evento invalido".to_string()));
                return;
            }
            estado.eventos_disponibles[event_index as usize].evento_id.clone()
        };

        let option_id = format!("opcion_{}", option_index);

        // Llamar a resolver_evento
        let result = {
            let mut motor = state.motor.lock().unwrap();
            motor.api.resolver_evento(&evento_id, &option_id)
        };

        // Actualizar UI
        match result {
            Ok(_) => {
                // Obtener nuevo estado y actualizar UI
                let nuevo_estado = {
                    let mut motor = state.motor.lock().unwrap();
                    motor.api.estado_jornada()
                };
                actualizar_ui(&ui, &nuevo_estado);
                ui.set_selected_event(-1);
                ui.set_selected_option(-1);
                ui.set_mensaje_text(SharedString::from("Evento resuelto exitosamente".to_string()));
            }
            Err(e) => {
                let error_msg = format!("Error al resolver evento: {}", e);
                ui.set_mensaje_text(SharedString::from(error_msg));
            }
        }
    }
}
