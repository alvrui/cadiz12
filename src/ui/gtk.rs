use gtk4::{
    prelude::*,
    Application,
    ApplicationWindow,
    Box,
    Label,
    ListBox,
    ListBoxRow,
    ScrolledWindow,
    Separator,
    TreeView,
    TreeViewColumn,
    CellRendererText,
    ListStore,
    EventControllerKey,
    Align,
    Orientation,
    SelectionMode,
};
use glib::{clone, ToValue, Value};
use std::sync::{Arc, Mutex};

use crate::{
    config::PartidaConfig,
    engine::Motor,
    engine::dtos::EstadoJornadaDto,
};

/// Estructura principal de la UI GTK
pub struct GtkApp {
    motor: Arc<Mutex<Motor>>,
    estado_actual: Arc<Mutex<Option<EstadoJornadaDto>>>,
    evento_seleccionado: Arc<Mutex<Option<usize>>>,
    window: ApplicationWindow,
    medidores_list_store: ListStore,
    eventos_list_box: ListBox,
    header_labels: Vec<Label>,
}

impl GtkApp {
    pub fn nuevo(config: PartidaConfig) -> Self {
        let motor = Motor::nuevo(config);
        let motor_arc = Arc::new(Mutex::new(motor));
        let estado = motor_arc.lock().unwrap().api.estado_jornada();
        let estado_actual = Arc::new(Mutex::new(Some(estado)));
        
        let window = ApplicationWindow::new();
        window.set_title("CADIZ 1812");
        window.set_default_size(1024, 768);
        
        let medidores_list_store = ListStore::new(vec![
            String::static_type(),
            u8::static_type(),
            String::static_type(),
            String::static_type(),
        ]);
        
        let eventos_list_box = ListBox::new();
        eventos_list_box.set_selection_mode(SelectionMode::Single);
        
        Self {
            motor: motor_arc,
            estado_actual,
            evento_seleccionado: Arc::new(Mutex::new(None)),
            window,
            medidores_list_store,
            eventos_list_box,
            header_labels: Vec::new(),
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let app = Application::builder()
            .application_id("com.cadiz12.game")
            .build();
        
        app.connect_activate(clone!(@weak self.window => move |app| {
            app.add_window(&self.window);
        }));
        
        self.setup_ui();
        self.update_ui();
        self.window.present();
        
        app.run();
        Ok(())
    }

    fn setup_ui(&mut self) {
        let main_box = Box::new(Orientation::Vertical, 5);
        self.window.set_child(Some(&main_box));
        
        let header = self.create_header();
        main_box.append(&header);
        main_box.append(&Separator::new(Orientation::Horizontal));
        
        let content_box = Box::new(Orientation::Horizontal, 10);
        main_box.append(&content_box);
        
        let medidores_panel = self.create_medidores_panel();
        content_box.append(&medidores_panel);
        
        let eventos_panel = self.create_eventos_panel();
        content_box.append(&eventos_panel);
        
        let footer = self.create_footer();
        main_box.append(&Separator::new(Orientation::Horizontal));
        main_box.append(&footer);
        
        self.setup_keyboard_controller();
    }

    fn create_header(&mut self) -> Box {
        let header_box = Box::new(Orientation::Vertical, 2);
        
        let title = Label::new(Some("CADIZ 1812"));
        title.set_halign(Align::Center);
        header_box.append(&title);
        
        let info_box = Box::new(Orientation::Horizontal, 10);
        info_box.set_halign(Align::Center);
        
        let jornada_label = Label::new(Some("Jornada: -"));
        let acto_label = Label::new(Some("Acto: -"));
        let tramo_label = Label::new(Some("Tramo: -"));
        let presupuesto_label = Label::new(Some("Presupuesto: -/-"));
        
        self.header_labels = vec![
            jornada_label.clone(),
            acto_label.clone(),
            tramo_label.clone(),
            presupuesto_label.clone(),
        ];
        
        info_box.append(&jornada_label);
        info_box.append(&Separator::new(Orientation::Vertical));
        info_box.append(&acto_label);
        info_box.append(&Separator::new(Orientation::Vertical));
        info_box.append(&tramo_label);
        info_box.append(&Separator::new(Orientation::Vertical));
        info_box.append(&presupuesto_label);
        
        header_box.append(&info_box);
        header_box
    }

    fn create_medidores_panel(&self) -> ScrolledWindow {
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        
        let tree_view = TreeView::new();
        tree_view.set_model(Some(&self.medidores_list_store));
        
        let col_nombre = TreeViewColumn::new();
        let renderer_nombre = CellRendererText::new();
        col_nombre.pack_start(&renderer_nombre, true);
        col_nombre.add_attribute(&renderer_nombre, "text", 0);
        col_nombre.set_title("Medidor");
        tree_view.append_column(&col_nombre);
        
        let col_valor = TreeViewColumn::new();
        let renderer_valor = CellRendererText::new();
        col_valor.pack_start(&renderer_valor, true);
        col_valor.add_attribute(&renderer_valor, "text", 1);
        col_valor.set_title("Valor");
        tree_view.append_column(&col_valor);
        
        let col_tendencia = TreeViewColumn::new();
        let renderer_tendencia = CellRendererText::new();
        col_tendencia.pack_start(&renderer_tendencia, true);
        col_tendencia.add_attribute(&renderer_tendencia, "text", 2);
        col_tendencia.set_title("Tendencia");
        tree_view.append_column(&col_tendencia);
        
        let col_umbrales = TreeViewColumn::new();
        let renderer_umbrales = CellRendererText::new();
        col_umbrales.pack_start(&renderer_umbrales, true);
        col_umbrales.add_attribute(&renderer_umbrales, "text", 3);
        col_umbrales.set_title("Umbrales");
        tree_view.append_column(&col_umbrales);
        
        scrolled.set_child(Some(&tree_view));
        scrolled
    }

    fn create_eventos_panel(&mut self) -> ScrolledWindow {
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        
        self.eventos_list_box.set_selection_mode(SelectionMode::Single);
        
        self.eventos_list_box.connect_row_activated(clone!(@weak self.eventos_list_box => move |_, row| {
            if let Some(row) = row {
                if let Some(index) = row.index() {
                    eprintln!("Evento activado: {}", index);
                }
            }
        }));
        
        scrolled.set_child(Some(&self.eventos_list_box));
        scrolled
    }

    fn create_footer(&self) -> Box {
        let footer_box = Box::new(Orientation::Horizontal, 10);
        footer_box.set_halign(Align::Center);
        let help_label = Label::new(Some("Q/Esc = Salir | Flechas = Navegar | 1-9 = Seleccionar opcion"));
        footer_box.append(&help_label);
        footer_box
    }

    fn setup_keyboard_controller(&self) {
        let controller = EventControllerKey::new();
        let eventos_list_box = self.eventos_list_box.clone();
        let window = self.window.clone();
        let motor = self.motor.clone();
        let estado = self.estado_actual.clone();
        let evento_seleccionado = self.evento_seleccionado.clone();
        
        controller.connect_key_pressed(move |_, key, _, _| {
            let ev = eventos_list_box.clone();
            let win = window.clone();
            let mot = motor.clone();
            let est = estado.clone();
            let ev_sel = evento_seleccionado.clone();
            
            match key {
                gtk4::gdk::Key::q | gtk4::gdk::Key::Escape => {
                    win.close();
                    glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::Up => {
                    navigate_eventos(&ev, false);
                    glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::Down => {
                    navigate_eventos(&ev, true);
                    glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
                    if let Some(row) = ev.selected_row() {
                        if let Some(index) = row.index() {
                            *ev_sel.lock().unwrap() = Some(index);
                        }
                    }
                    glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::n1 => handle_option(0, mot, est, ev_sel),
                gtk4::gdk::Key::n2 => handle_option(1, mot, est, ev_sel),
                gtk4::gdk::Key::n3 => handle_option(2, mot, est, ev_sel),
                gtk4::gdk::Key::n4 => handle_option(3, mot, est, ev_sel),
                gtk4::gdk::Key::n5 => handle_option(4, mot, est, ev_sel),
                gtk4::gdk::Key::n6 => handle_option(5, mot, est, ev_sel),
                gtk4::gdk::Key::n7 => handle_option(6, mot, est, ev_sel),
                gtk4::gdk::Key::n8 => handle_option(7, mot, est, ev_sel),
                gtk4::gdk::Key::n9 => handle_option(8, mot, est, ev_sel),
                _ => glib::signal::Inhibit(false),
            }
        });
        
        self.window.add_controller(&controller);
    }

    fn populate_medidores(&self) {
        self.medidores_list_store.remove_all();
        
        if let Some(estado) = &*self.estado_actual.lock().unwrap() {
            for medidor in &estado.protagonista.medidores {
                let row = self.medidores_list_store.append();
                self.medidores_list_store.set(&row, 0, &Value::from(&medidor.nombre));
                self.medidores_list_store.set(&row, 1, &medidor.valor.to_value());
                
                let tendencia_str = if medidor.tendencia > 0 {
                    format!("+{}", medidor.tendencia)
                } else {
                    medidor.tendencia.to_string()
                };
                self.medidores_list_store.set(&row, 2, &Value::from(&tendencia_str));
                
                let umbrales = format!("[{}-{}]", medidor.umbral_bajo, medidor.umbral_alto);
                self.medidores_list_store.set(&row, 3, &Value::from(&umbrales));
            }
        }
    }

    fn populate_eventos(&self) {
        while let Some(child) = self.eventos_list_box.first_child() {
            self.eventos_list_box.remove(&child);
        }
        
        if let Some(estado) = &*self.estado_actual.lock().unwrap() {
            for evento in &estado.eventos_disponibles {
                let row_box = Box::new(Orientation::Horizontal, 5);
                
                let title_label = Label::new(Some(&evento.titulo));
                title_label.set_xalign(0.0);
                
                let cost_label = Label::new(Some(&format!("({} pts)", evento.coste_temporal)));
                cost_label.set_xalign(1.0);
                
                row_box.append(&title_label);
                row_box.append(&cost_label);
                
                let list_row = ListBoxRow::new();
                list_row.set_child(Some(&row_box));
                
                self.eventos_list_box.append(&list_row);
            }
        }
    }

    fn update_ui(&self) {
        if let Some(estado) = &*self.estado_actual.lock().unwrap() {
            if self.header_labels.len() >= 4 {
                self.header_labels[0].set_text(&format!("Jornada: {}", estado.tiempo.jornada));
                self.header_labels[1].set_text(&format!("Acto: {}", estado.tiempo.acto));
                self.header_labels[2].set_text(&format!("Tramo: {}", estado.tiempo.tramo_id));
                self.header_labels[3].set_text(&format!("Presupuesto: {}/{}", 
                    estado.presupuesto_temporal, estado.presupuesto_temporal));
            }
        }
        
        self.populate_medidores();
        self.populate_eventos();
    }
}

fn navigate_eventos(list_box: &ListBox, down: bool) {
    if down {
        if let Some(current_row) = list_box.selected_row() {
            if let Some(next_row) = current_row.next_sibling() {
                list_box.select_row(Some(&next_row));
                return;
            }
        }
        if let Some(first_row) = list_box.first_child() {
            list_box.select_row(Some(&first_row));
        }
    } else {
        if let Some(current_row) = list_box.selected_row() {
            if let Some(prev_row) = current_row.prev_sibling() {
                list_box.select_row(Some(&prev_row));
                return;
            }
        }
        if let Some(last_row) = list_box.last_child() {
            list_box.select_row(Some(&last_row));
        }
    }
}

fn handle_option(option_index: u32, 
                 motor: Arc<Mutex<Motor>>, 
                 estado: Arc<Mutex<Option<EstadoJornadaDto>>>,
                 evento_seleccionado: Arc<Mutex<Option<usize>>>) 
-> glib::signal::Inhibit {
    if let Some(event_idx) = *evento_seleccionado.lock().unwrap() {
        if let Some(estado_guard) = &*estado.lock().unwrap() {
            if event_idx < estado_guard.eventos_disponibles.len() {
                let evento = &estado_guard.eventos_disponibles[event_idx];
                
                let opcion_id = match option_index {
                    0 => "apoyar_liberal",
                    1 => "apoyar_moderado",
                    2 => "abstenerse",
                    3 => "escuchar_atentamente",
                    4 => "difundir_rumor",
                    _ => "apoyar_liberal",
                };
                
                let motor_guard = motor.lock().unwrap();
                let resultado = motor_guard.api.resolver_evento(&evento.evento_id, opcion_id);
                
                match resultado {
                    Ok(_) => {
                        let new_estado = motor_guard.api.estado_jornada();
                        *estado.lock().unwrap() = Some(new_estado);
                        *evento_seleccionado.lock().unwrap() = None;
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    }
    glib::signal::Inhibit(false)
}

/// Funcion principal para ejecutar el juego con GTK
pub fn ejecutar_juego(config: PartidaConfig) -> anyhow::Result<()> {
    let mut app = GtkApp::nuevo(config);
    app.run()
}
