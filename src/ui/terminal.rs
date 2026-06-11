use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Stdout, Write},
    time::Duration,
};
use crate::{
    config::PartidaConfig,
    engine::Motor,
    engine::dtos::EstadoJornadaDto,
};

/// App principal de la UI de terminal
pub struct App {
    motor: Motor,
    estado_actual: Option<EstadoJornadaDto>,
    evento_seleccionado: Option<usize>,
    opcion_seleccionada: Option<usize>,
    mensaje: Option<String>,
    running: bool,
}

impl App {
    pub fn nuevo(config: PartidaConfig) -> Self {
        Self {
            motor: Motor::nuevo(config),
            estado_actual: None,
            evento_seleccionado: None,
            opcion_seleccionada: None,
            mensaje: None,
            running: true,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = ratatui::backend::CrosstermBackend::new(stdout);
        let mut terminal = ratatui::Terminal::new(backend)?;

        self.estado_actual = Some(self.motor.api.estado_jornada());

        while self.running {
            terminal.draw(|f| self.draw(f))?;

            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn draw(&self, f: &mut Frame) {
        let size = f.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(size);

        self.draw_header(f, chunks[0]);
        self.draw_content(f, chunks[1]);
        self.draw_footer(f, chunks[2]);
    }

    fn draw_header(&self, f: &mut Frame, area: Rect) {
        if let Some(estado) = &self.estado_actual {
            let header = Block::default()
                .borders(Borders::BOTTOM)
                .title(" CÁDIZ 1812 ")
                .title_style(Style::new().bold());

            let info = Text::from(vec![
                Line::from(vec![
                    Span::raw("Jornada: ").light_cyan(),
                    Span::raw(estado.tiempo.jornada.to_string()).white().bold(),
                    Span::raw(" | Acto: ").light_cyan(),
                    Span::raw(estado.tiempo.acto.to_string()).white(),
                    Span::raw(" | ").dim(),
                    Span::raw("Tramo: ").light_cyan(),
                    Span::raw(&estado.tiempo.tramo_id).white(),
                ]),
                Line::from(vec![
                    Span::raw("Presupuesto: ").light_cyan(),
                    Span::raw(estado.presupuesto_temporal.to_string()).white(),
                    Span::raw("/").dim(),
                    Span::raw(estado.presupuesto_temporal.to_string()).white(),
                ]),
            ]);

            let paragraph = Paragraph::new(info)
                .block(header)
                .wrap(Wrap { trim: false });

            f.render_widget(paragraph, area);
        }
    }

    fn draw_content(&self, f: &mut Frame, area: Rect) {
        if let Some(estado) = &self.estado_actual {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(area);

            self.draw_medidores(f, chunks[0], estado);
            self.draw_eventos(f, chunks[1], estado);
        }
    }

    fn draw_medidores(&self, f: &mut Frame, area: Rect, estado: &EstadoJornadaDto) {
        let title = Block::default()
            .title(" MEDIDORES ")
            .borders(Borders::ALL);

        let rows: Vec<Row> = estado.protagonista.medidores.iter()
            .map(|m| {
                let valor_str = m.valor.to_string();
                let tendencia_str = match m.tendencia {
                    t if t > 0 => format!("+{}", t),
                    t => format!("{}", t),
                };
                let umbrales = format!("[{}-{}]", m.umbral_bajo, m.umbral_alto);

                Row::new(vec![
                    Cell::from(m.nombre.clone()).style(Style::new().light_cyan()),
                    Cell::from(valor_str).style(self.get_medidor_style(m.valor, m.umbral_bajo, m.umbral_alto)),
                    Cell::from(tendencia_str).style(self.get_tendencia_style(m.tendencia)),
                    Cell::from(umbrales).style(Style::new().dim()),
                ])
            })
            .collect();

        let table = Table::new(rows, [
            Constraint::Length(12),
            Constraint::Length(6),
            Constraint::Length(5),
            Constraint::Length(10),
        ])
        .header(Row::new(vec![
            Cell::from("Medidor").bold(),
            Cell::from("Valor").bold(),
            Cell::from("Tend").bold(),
            Cell::from("Umbrales").bold(),
        ]))
        .block(title)
        .column_spacing(1);

        f.render_widget(table, area);
    }

    fn draw_eventos(&self, f: &mut Frame, area: Rect, estado: &EstadoJornadaDto) {
        let title = Block::default()
            .title(" EVENTOS DISPONIBLES ")
            .borders(Borders::ALL);

        if estado.eventos_disponibles.is_empty() {
            let paragraph = Paragraph::new("No hay eventos disponibles")
                .block(title)
                .wrap(Wrap { trim: false });
            f.render_widget(paragraph, area);
            return;
        }

        let mut lines: Vec<Line> = Vec::new();

        if let Some(mensaje) = &self.mensaje {
            lines.push(Line::from(vec![Span::from(mensaje.clone()).fg(Color::Yellow)]));
            lines.push(Line::from(vec![Span::from("")]));
        }

        for (i, evento) in estado.eventos_disponibles.iter().enumerate() {
            let prefix = if Some(i) == self.evento_seleccionado {
                "> "
            } else {
                "  "
            };

            let estilo = if Some(i) == self.evento_seleccionado {
                Style::new().fg(Color::Green).bold()
            } else {
                Style::new()
            };

            lines.push(Line::from(vec![
                Span::from(prefix).fg(Color::LightGreen),
                Span::from(&evento.titulo).style(estilo),
                Span::from(" ").dim(),
                Span::from(format!("({} pts)", evento.coste_temporal)).dim(),
            ]));
        }

        let text = Text::from(lines);
        let paragraph = Paragraph::new(text)
            .block(title)
            .wrap(Wrap { trim: false });

        f.render_widget(paragraph, area);
    }

    fn draw_footer(&self, f: &mut Frame, area: Rect) {
        let help_text = vec![
            Span::from("Q").light_cyan(), Span::from(" = Salir | "),
            Span::from("↑↓").light_cyan(), Span::from(" = Mover | "),
            Span::from("Enter").light_cyan(), Span::from(" = Seleccionar | "),
            Span::from("1-9").light_cyan(), Span::from(" = Opción"),
        ];

        let paragraph = Paragraph::new(Line::from(help_text))
            .block(Block::default().borders(Borders::TOP))
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(paragraph, area);
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if let Some(estado) = &self.estado_actual {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                    self.running = false;
                }

                KeyCode::Up => {
                    if let Some(i) = self.evento_seleccionado {
                        if i > 0 {
                            self.evento_seleccionado = Some(i - 1);
                        }
                    } else if !estado.eventos_disponibles.is_empty() {
                        self.evento_seleccionado = Some(estado.eventos_disponibles.len() - 1);
                    }
                    self.opcion_seleccionada = None;
                }
                KeyCode::Down => {
                    if let Some(i) = self.evento_seleccionado {
                        if i + 1 < estado.eventos_disponibles.len() {
                            self.evento_seleccionado = Some(i + 1);
                        }
                    } else if !estado.eventos_disponibles.is_empty() {
                        self.evento_seleccionado = Some(0);
                    }
                    self.opcion_seleccionada = None;
                }

                KeyCode::Enter => {
                    if let Some(i) = self.evento_seleccionado {
                        let evento = &estado.eventos_disponibles[i];
                        self.mensaje = Some(format!("Evento seleccionado: {}", evento.titulo));
                    }
                }

                KeyCode::Char(c) if c >= '1' && c <= '9' => {
                    if let Some(i) = self.evento_seleccionado {
                        let opcion_index = (c.to_digit(10).unwrap() - 1) as usize;

                        let evento = &estado.eventos_disponibles[i];
                        if opcion_index < 10 {
                            let opcion_id = match opcion_index {
                                0 => "apoyar_liberal",
                                1 => "apoyar_moderado",
                                2 => "abstenerse",
                                3 => "escuchar_atentamente",
                                4 => "difundir_rumor",
                                _ => "apoyar_liberal",
                            };

                            let resultado = self.motor.api.resolver_evento(&evento.evento_id, opcion_id);
                            match resultado {
                                Ok(_) => {
                                    self.mensaje = Some(format!("Resuelto: {} con opcion {}", evento.titulo, opcion_id));
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

                _ => {}
            }
        }
    }

    fn get_medidor_style(&self, valor: u8, umbral_bajo: u8, umbral_alto: u8) -> Style {
        if valor < umbral_bajo {
            Style::new().fg(Color::Red).bold()
        } else if valor > umbral_alto {
            Style::new().fg(Color::LightGreen).bold()
        } else {
            Style::new().fg(Color::White)
        }
    }

    fn get_tendencia_style(&self, tendencia: i8) -> Style {
        match tendencia {
            t if t > 0 => Style::new().fg(Color::LightGreen),
            t if t < 0 => Style::new().fg(Color::Red),
            _ => Style::new().fg(Color::White),
        }
    }
}

pub fn ejecutar_juego(config: PartidaConfig) -> io::Result<()> {
    let mut app = App::nuevo(config);
    app.run()
}