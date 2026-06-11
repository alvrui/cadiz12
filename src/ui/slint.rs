//! Modulo UI Slint para Cadiz 1812
//!
//! Este modulo implementa la interfaz grafica usando Slint 1.3.

slint::slint! {
    export struct Medidor {
        nombre: string,
        valor: int,
        tendencia: string,
        umbrales: string,
    }

    export struct Evento {
        titulo: string,
        coste: int,
    }

    export component MainWindow inherits Window {
        in-out property <int> jornada;
        in-out property <int> acto;
        in-out property <string> tramo;
        in-out property <string> presupuesto;
        in-out property <string> mensaje;
        in-out property <[Medidor]> medidores;
        in-out property <[Evento]> eventos;
        
        preferred-width: 1024px;
        preferred-height: 768px;
        title: "Cádiz 1812";
        
        Text { text: "CÁDIZ 1812 - Slint UI"; }
        Text { text: "Jornada: " + jornada; }
        Text { text: "Acto: " + acto; }
        Text { text: "Tramo: " + tramo; }
        Text { text: "Presupuesto: " + presupuesto; }
        Text { text: mensaje; }
    }
}

use slint::SharedString;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

use crate::{
    config::PartidaConfig,
    engine::Motor,
    engine::dtos::EstadoJornadaDto,
};

pub fn ejecutar_juego(config: PartidaConfig) -> anyhow::Result<()> {
    let motor = Arc::new(Mutex::new(Motor::nuevo(config)));
    
    let ui = MainWindow::new()?;
    
    // Inicializar datos
    let estado = motor.lock().unwrap().api.estado_jornada();
    actualizar_ui(&ui, &estado);
    
    ui.run()?;
    Ok(())
}

fn actualizar_ui(ui: &MainWindow, estado: &EstadoJornadaDto) {
    ui.set_jornada(estado.tiempo.jornada as i32);
    ui.set_acto(estado.tiempo.acto as i32);
    ui.set_tramo(estado.tiempo.tramo_id.clone().into());
    ui.set_presupuesto(format!("{}/{}", estado.presupuesto_temporal, estado.presupuesto_temporal).into());
    
    let medidores = slint::VecModel::from(
        estado.protagonista.medidores.iter().map(|m| {
            Medidor {
                nombre: SharedString::from(m.nombre.clone()),
                valor: m.valor as i32,
                tendencia: if m.tendencia > 0 {
                    SharedString::from(format!("+{}", m.tendencia))
                } else {
                    SharedString::from(m.tendencia.to_string())
                },
                umbrales: SharedString::from(format!("[{}-{}]", m.umbral_bajo, m.umbral_alto)),
            }
        }).collect::<Vec<_>>()
    );
    
    ui.set_medidores(Rc::new(medidores).into());
    
    let eventos = slint::VecModel::from(
        estado.eventos_disponibles.iter().map(|e| {
            Evento {
                titulo: SharedString::from(e.titulo.clone()),
                coste: e.coste_temporal as i32,
            }
        }).collect::<Vec<_>>()
    );
    
    ui.set_eventos(Rc::new(eventos).into());
}
