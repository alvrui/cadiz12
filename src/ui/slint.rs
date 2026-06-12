use slint::SharedString;
use std::sync::{Arc, Mutex};
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

        VerticalBox {
            width: 100%;
            height: 100%;

            // Header
            HorizontalBox {
                width: 100%;
                height: 60px;

                Text {
                    text: "CÁDIZ 1812";
                    font-size: 24px;
                    font-weight: 700;
                }
            }

            HorizontalBox {
                width: 100%;
                height: 30px;

                Text { text: "Jornada: "; }
                Text { text: jornada-text; }
                Text { text: " | Acto: "; }
                Text { text: acto-text; }
                Text { text: " | Tramo: "; }
                Text { text: tramo-text; }
                Text { text: " | Presupuesto: "; }
                Text { text: presupuesto-text; }
            }

            // Separator
            Rectangle {
                height: 1px;
                width: 100%;
                background: #cccccc;
            }

            // Content - split horizontal
            HorizontalBox {
                width: 100%;
                height: 100%;
                spacing: 10px;

                // Medidores panel (30%)
                VerticalBox {
                    width: 30%;
                    height: 100%;

                    Text { text: "MEDIDORES"; }
                    ScrollView {
                        width: 100%;
                        height: 100%;
                    }
                }

                // Eventos panel (70%)
                VerticalBox {
                    width: 70%;
                    height: 100%;

                    Text { text: "EVENTOS DISPONIBLES"; }
                    ScrollView {
                        width: 100%;
                        height: 100%;
                    }
                }
            }

            // Separator
            Rectangle {
                height: 1px;
                width: 100%;
                background: #cccccc;
            }

            // Footer
            HorizontalBox {
                width: 100%;
                height: 30px;
                Text { text: mensaje-text; }
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

    // Inicializar datos
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
}
