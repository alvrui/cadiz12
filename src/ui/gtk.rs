use gtk4::{
    prelude::*,
    Application,
    ApplicationWindow,
    Box,
    Button,
    Label,
    ListBox,
    ScrolledWindow,
    Separator,
    TreeView,
    TreeViewColumn,
    CellRendererText,
    GestureClick,
    EventControllerKey,
    Align,
    Orientation,
    PackType,
};
use glib::{clone, MainContext};
use std::sync::{Arc, Mutex};

use crate::{
    config::PartidaConfig,
    engine::Motor,
    engine::dtos::EstadoJornadaDto,
};

/// Estructura principal de la UI GTK
pub struct GtkApp {
    motor: Motor,
    estado_actual: Option<EstadoJornadaDto>,
    evento_seleccionado: Option<usize>,
    opcion_seleccionada: Option<usize>,
    mensaje: Option<String>,
    window: ApplicationWindow,
}

impl GtkApp {
    pub fn nuevo(config: PartidaConfig) -> Self {
        // Crear motor
        let motor = Motor::nuevo(config);
        
        // Obtener estado inicial
        let estado_actual = Some(motor.api.estado_jornada());
        
        // Crear ventana
        let window = ApplicationWindow::new();
        window.set_title("CÁDIZ 1812");
        window.set_default_size(1024, 768);
        
        Self {
            motor,
            estado_actual,
            evento_seleccionado: None,
            opcion_seleccionada: None,
            mensaje: None,
            window,
        }
    }

    pub fn run(&self) -> glib::ExitCode {
        // Configurar la aplicación GTK
        let app = Application::builder()
            .application_id("com.cadiz12.game")
            .build();
        
        app.connect_activate(clone!(@weak self.window => move |app| {
            app.add_window(&self.window);
            self.setup_ui(app);
        }));
        
        app.run()
    }

    fn setup_ui(&self, app: &Application) {
        // Contenedor principal vertical
        let main_box = Box::new(Orientation::Vertical, 5);
        self.window.set_child(Some(&main_box));
        
        // Header: Información de la jornada
        let header = self.create_header();
        main_box.append(&header);
        main_box.append(&Separator::new(Orientation::Horizontal));
        
        // Content: Medidores (izquierda) y Eventos (derecha)
        let content_box = Box::new(Orientation::Horizontal, 10);
        main_box.append(&content_box);
        
        // Panel izquierdo: Tabla de medidores
        let medidores_panel = self.create_medidores_panel();
        content_box.append(&medidores_panel);
        
        // Panel derecho: Lista de eventos disponibles
        let eventos_panel = self.create_eventos_panel();
        content_box.append(&eventos_panel);
        
        // Footer: Controles
        let footer = self.create_footer();
        main_box.append(&Separator::new(Orientation::Horizontal));
        main_box.append(&footer);
        
        // Configurar controlador de teclado
        self.setup_keyboard_controller(&self.window);
        
        // Mostrar ventana
        self.window.present();
    }

    fn create_header(&self) -> Box {
        let header_box = Box::new(Orientation::Vertical, 2);
        
        let title = Label::new(Some("CÁDIZ 1812"));
        title.set_halign(Align::Center);
        title.add_css_class("title");
        header_box.append(&title);
        
        if let Some(estado) = &self.estado_actual {
            let info_box = Box::new(Orientation::Horizontal, 10);
            info_box.set_halign(Align::Center);
            
            let jornada_label = Label::new(Some(&format!("Jornada: {}", estado.tiempo.jornada)));
            let acto_label = Label::new(Some(&format!("Acto: {}", estado.tiempo.acto)));
            let tramo_label = Label::new(Some(&format!("Tramo: {}", estado.tiempo.tramo_id)));
            let presupuesto_label = Label::new(Some(&format!("Presupuesto: {}/{}", 
                estado.presupuesto_temporal, estado.presupuesto_temporal)));
            
            info_box.append(&jornada_label);
            info_box.append(&Separator::new(Orientation::Vertical));
            info_box.append(&acto_label);
            info_box.append(&Separator::new(Orientation::Vertical));
            info_box.append(&tramo_label);
            info_box.append(&Separator::new(Orientation::Vertical));
            info_box.append(&presupuesto_label);
            
            header_box.append(&info_box);
        }
        
        header_box
    }

    fn create_medidores_panel(&self) -> ScrolledWindow {
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        
        let tree_view = TreeView::new();
        
        // Columna: Nombre
        let col_nombre = TreeViewColumn::new();
        let renderer_nombre = CellRendererText::new();
        col_nombre.pack_start(&renderer_nombre, true);
        col_nombre.add_attribute(&renderer_nombre, "text", 0);
        col_nombre.set_title("Medidor");
        tree_view.append_column(&col_nombre);
        
        // Columna: Valor
        let col_valor = TreeViewColumn::new();
        let renderer_valor = CellRendererText::new();
        col_valor.pack_start(&renderer_valor, true);
        col_valor.add_attribute(&renderer_valor, "text", 1);
        col_valor.set_title("Valor");
        tree_view.append_column(&col_valor);
        
        // Columna: Tendencia
        let col_tendencia = TreeViewColumn::new();
        let renderer_tendencia = CellRendererText::new();
        col_tendencia.pack_start(&renderer_tendencia, true);
        col_tendencia.add_attribute(&renderer_tendencia, "text", 2);
        col_tendencia.set_title("Tendencia");
        tree_view.append_column(&col_tendencia);
        
        // Columna: Umbrales
        let col_umbrales = TreeViewColumn::new();
        let renderer_umbrales = CellRendererText::new();
        col_umbrales.pack_start(&renderer_umbrales, true);
        col_umbrales.add_attribute(&renderer_umbrales, "text", 3);
        col_umbrales.set_title("Umbrales");
        tree_view.append_column(&col_umbrales);
        
        // TODO: Cargar datos de medidores desde estado_actual
        // Esto requiere acceso al estado compartido
        
        scrolled.set_child(Some(&tree_view));
        scrolled
    }

    fn create_eventos_panel(&self) -> ScrolledWindow {
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        
        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk4::SelectionMode::Single);
        
        // TODO: Cargar eventos desde estado_actual
        // Por ahora, crear un evento de ejemplo
        let evento_label = Label::new(Some("Evento de ejemplo"));
        evento_label.set_xalign(0.0);
        list_box.append(&evento_label);
        
        scrolled.set_child(Some(&list_box));
        scrolled
    }

    fn create_footer(&self) -> Box {
        let footer_box = Box::new(Orientation::Horizontal, 10);
        footer_box.set_halign(Align::Center);
        
        let help_label = Label::new(Some("Q/Esc = Salir | Flechas = Navegar | 1-9 = Seleccionar opción"));
        footer_box.append(&help_label);
        
        footer_box
    }

    fn setup_keyboard_controller(&self, window: &ApplicationWindow) {
        let controller = EventControllerKey::new();
        
        controller.connect_key_pressed(clone!(@weak window => move |_, key, _, _| {
            match key {
                gtk4::gdk::Key::q | gtk4::gdk::Key::Escape => {
                    window.close();
                    gtk4::glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::Up => {
                    // Navegación hacia arriba
                    gtk4::glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::Down => {
                    // Navegación hacia abajo
                    gtk4::glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
                    // Seleccionar evento
                    gtk4::glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::n1 => {
                    // Opción 1
                    gtk4::glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::n2 => {
                    // Opción 2
                    gtk4::glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::n3 => {
                    // Opción 3
                    gtk4::glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::n4 => {
                    // Opción 4
                    gtk4::glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::n5 => {
                    // Opción 5
                    gtk4::glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::n6 => {
                    // Opción 6
                    gtk4::glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::n7 => {
                    // Opción 7
                    gtk4::glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::n8 => {
                    // Opción 8
                    gtk4::glib::signal::Inhibit(false)
                }
                gtk4::gdk::Key::n9 => {
                    // Opción 9
                    gtk4::glib::signal::Inhibit(false)
                }
                _ => gtk4::glib::signal::Inhibit(false),
            }
        }));
        
        window.add_controller(&controller);
    }

    fn update_ui(&self) {
        // Actualizar la UI con el estado actual
        // Esto se llamaría después de cualquier acción que cambie el estado
    }

    fn handle_event_selection(&mut self, event_index: usize) {
        if let Some(estado) = &self.estado_actual {
            if event_index < estado.eventos_disponibles.len() {
                self.evento_seleccionado = Some(event_index);
                // TODO: Mostrar opciones del evento
            }
        }
    }

    fn handle_option_selection(&mut self, option_index: usize) {
        if let Some(i) = self.evento_seleccionado {
            if let Some(estado) = &self.estado_actual {
                if i < estado.eventos_disponibles.len() {
                    let evento = &estado.eventos_disponibles[i];
                    
                    // Mapear opción numérica a option_id
                    let opcion_id = match option_index {
                        0 => "apoyar_liberal",
                        1 => "apoyar_moderado",
                        2 => "abstenerse",
                        3 => "escuchar_atentamente",
                        4 => "difundir_rumor",
                        _ => "apoyar_liberal",
                    };
                    
                    // Resolver evento usando MotorApi
                    let resultado = self.motor.api.resolver_evento(&evento.evento_id, opcion_id);
                    match resultado {
                        Ok(_) => {
                            self.mensaje = Some(format!("Resuelto: {} con opción {}", evento.titulo, opcion_id));
                            // Avanzar a la siguiente jornada
                            self.estado_actual = Some(self.motor.api.estado_jornada());
                            self.evento_seleccionado = None;
                            self.opcion_seleccionada = None;
                        }
                        Err(e) => {
                            self.mensaje = Some(format!("Error: {}", e));
                        }
                    }
                }
            }
        }
    }
}

/// Función principal para ejecutar el juego con GTK
pub fn ejecutar_juego(config: PartidaConfig) -> glib::ExitCode {
    let app = GtkApp::nuevo(config);
    app.run()
}
