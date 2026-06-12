use slint::SharedString;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use crate::{config::PartidaConfig, engine::Motor, engine::dtos::EstadoJornadaDto};

slint::slint! {
    import { Button, VerticalBox, HorizontalBox, ScrollView, LineEdit } from "std-widgets.slint";

    export struct MedidorData {
        nombre: string,
        valor: int,
        tendencia: int,
        umbral-bajo: int,
        umbral-alto: int,
    }

    export struct EventoData {
        titulo: string,
        coste: int,
    }

    export component GameWindow inherits Window {
        width: 1024px;
        height: 768px;
        title: "Cádiz 1812";

        in-out property <string> jornada-text;
        in-out property <string> acto-text;
        in-out property <string> tramo-text;
        in-out property <string> presupuesto-text;
        in-out property <string> mensaje-text;
        in-out property <[MedidorData]> medidores-model;
        in-out property <[EventoData]> eventos-model;

        VerticalBox {
            width: 100%;
            height: 100%;
            spacing: 0px;

            // Header con fondo oscuro y estilo mejorado
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
                        text: "CÁDIZ 1812";
                        font-size: 28px;
                        font-weight: 700;
                    }
                }
            }

            // Info bar con fondo
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

            // Separator con color
            Rectangle {
                height: 2px;
                width: 100%;
                background: #0f3460;
            }

            // Content - split horizontal
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
                        spacing: 10px;

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

                        HorizontalBox {
                            width: 100%;
                            Text {
                                text: "EVENTOS DISPONIBLES";
                                font-size: 16px;
                                font-weight: 700;
                                horizontal-alignment: center;
                            }
                        }
                        
                        Rectangle {
                            width: 100%;
                            height: 1px;
                            background: #0f3460;
                        }
                        
                        ScrollView {
                            width: 100%;
                            height: 100%;
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

            // Footer con fondo y boton de prueba
            Rectangle {
                width: 100%;
                height: 50px;
                background: #162130;
                
                HorizontalBox {
                    width: 100%;
                    height: 100%;
                    padding-left: 20px;
                    padding-right: 20px;
                    spacing: 15px;
                    
                    Text {
                        text: mensaje-text;
                        font-size: 14px;
                    }
                    
                    Button {
                        text: "Siguiente";
                        width: 100px;
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
    actualizar_ui(&ui, &estado);
    ui.run()?;
    Ok(())
}

fn actualizar_ui(ui: &GameWindow, estado: &EstadoJornadaDto) {
    ui.set_jornada_text(SharedString::from(estado.tiempo.jornada.to_string()));
    ui.set_acto_text(SharedString::from(estado.tiempo.acto.to_string()));
    ui.set_tramo_text(SharedString::from(estado.tiempo.tramo_id.clone()));
    ui.set_presupuesto_text(SharedString::from(format!("{}/{}", estado.presupuesto_temporal, estado.presupuesto_temporal)));
    ui.set_mensaje_text(SharedString::from("".to_string()));

    let medidores: Vec<MedidorData> = estado.protagonista.medidores.iter().map(|m| {
        MedidorData {
            nombre: SharedString::from(m.nombre.clone()),
            valor: m.valor as i32,
            tendencia: m.tendencia as i32,
            umbral_bajo: m.umbral_bajo as i32,
            umbral_alto: m.umbral_alto as i32,
        }
    }).collect();
    ui.set_medidores_model(Rc::new(slint::VecModel::from(medidores)).into());

    let eventos: Vec<EventoData> = estado.eventos_disponibles.iter().map(|e| {
        EventoData {
            titulo: SharedString::from(e.titulo.clone()),
            coste: e.coste_temporal as i32,
        }
    }).collect();
    ui.set_eventos_model(Rc::new(slint::VecModel::from(eventos)).into());
}
