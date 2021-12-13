use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use quake_core::entry::entry_paths::EntryPaths;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Corner, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Widget};

use quake_core::entry::EntryDefines;
use quake_core::quake::QuakeActionNode;
use quake_core::usecases::entry_usecases;
use quake_core::QuakeConfig;

pub struct App {
    pub mode: Mode,
    pub command: String,
    pub main_widget: MainWidget,
    pub state: AppState,
    pub config: QuakeConfig,
}

impl App {
    pub fn new(config: QuakeConfig) -> App {
        App {
            mode: Mode::Command,
            command: "".to_string(),
            main_widget: MainWidget::Home,
            state: Default::default(),
            config,
        }
    }

    pub fn running(&self) -> bool {
        self.state.running
    }

    pub fn shutdown(&mut self) {
        self.state.running = false;
    }

    pub fn save_entry(&mut self) {
        if let MainWidget::Editor(ref action, ref string) = self.main_widget {
            entry_usecases::create_entry(&self.config.workspace, &action.object, &action.text)
                .and_then(|(_, file)| {
                    let type_path = PathBuf::from(&self.config.workspace).join(&action.object);
                    let mut fields = HashMap::new();
                    fields.insert("content".to_string(), string.clone());
                    entry_usecases::update_entry_fields(type_path, &action.object, file.id, &fields)
                })
                .unwrap();
        }
    }
}

pub enum Mode {
    Command,
    Normal,
    Insert,
}

#[derive(Clone, Debug)]
pub enum MainWidget {
    Home,
    EntryTypes,
    Editor(QuakeActionNode, String),
}

impl Widget for MainWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            MainWidget::Home => {
                let help_messages = vec![
                    Spans::from(vec![
                        Span::raw("Press "),
                        Span::styled(":", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" into command mode, "),
                        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" back to normal mode."),
                    ]),
                    Spans::from(vec![
                        Span::raw("Command "),
                        Span::styled("listAll", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" list all workspace."),
                    ]),
                    Spans::from(vec![
                        Span::raw("Command "),
                        Span::styled("quit", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" quit quake app."),
                    ]),
                ];
                let p = Paragraph::new(help_messages)
                    .block(Block::default().title("Main").borders(Borders::ALL))
                    .alignment(Alignment::Center);
                p.render(area, buf);
            }
            MainWidget::EntryTypes => {
                let entry_types: Vec<ListItem> = list_entry_types().unwrap_or_default();
                let entry_types_list = List::new(entry_types)
                    .block(Block::default().borders(Borders::ALL).title("List"))
                    .start_corner(Corner::TopLeft);

                entry_types_list.render(area, buf);
            }
            MainWidget::Editor(_, string) => {
                let editor = Paragraph::new(string.as_ref())
                    .block(Block::default().borders(Borders::ALL).title("Editro"));
                editor.render(area, buf);
            }
        }
    }
}

impl MainWidget {
    pub fn get_input(&self) -> &str {
        match self {
            Self::Editor(_, string) => string,
            _ => "",
        }
    }

    pub fn collect_input(&mut self, c: char) {
        if let Self::Editor(_, ref mut string) = self {
            string.push(c);
        }
    }
}

fn list_entry_types() -> Result<Vec<ListItem<'static>>, Box<dyn Error>> {
    let config: QuakeConfig = serde_yaml::from_str(fs::read_to_string(".quake.yaml")?.as_str())?;
    let entry_defines_path = Path::new(&config.workspace).join(EntryPaths::entries_define());
    let entry_defines: EntryDefines =
        serde_yaml::from_str(&fs::read_to_string(entry_defines_path)?)?;

    Ok(entry_defines
        .entries
        .iter()
        .map(|define| {
            let entry_type = Spans::from(vec![Span::styled(
                define.entry_type.clone(),
                Style::default().fg(Color::Yellow),
            )]);

            ListItem::new(vec![entry_type])
        })
        .collect())
}

pub struct AppState {
    running: bool,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState { running: true }
    }
}
