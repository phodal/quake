use std::collections::HashMap;
use std::path::PathBuf;

use quake_core::usecases::entry_usecases;
use quake_core::QuakeConfig;

use crate::widgets::MainWidget;

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

pub struct AppState {
    running: bool,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState { running: true }
    }
}
