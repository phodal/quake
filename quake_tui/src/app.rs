use std::collections::HashMap;
use std::path::PathBuf;

use quake_core::usecases::entry_usecases;
use quake_core::QuakeConfig;

use crate::widgets::{CmdLine, MainWidget};

pub struct App {
    pub mode: Mode,
    pub state: AppState,
    pub config: QuakeConfig,
    pub main_widget: MainWidget,
    pub cmd_line: CmdLine,
}

impl App {
    pub fn new(config: QuakeConfig) -> App {
        App {
            mode: Mode::Normal,
            main_widget: MainWidget::Home,
            cmd_line: CmdLine::default(),
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
                    entry_usecases::update_entry_properties(
                        type_path,
                        &action.object,
                        file.id,
                        &fields,
                    )
                })
                .unwrap();
        }
    }

    pub fn message_push(&mut self, c: char) {
        self.cmd_line.message.push(c);
    }

    pub fn message_pop(&mut self) {
        self.cmd_line.message.pop();
    }

    pub fn message_clear(&mut self) {
        self.cmd_line.message.clear();
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

#[cfg(test)]
mod tests {
    use quake_core::QuakeConfig;

    use super::App;

    #[test]
    fn test_message_collect() {
        let mut app = App::new(QuakeConfig::default());
        app.message_push('g');
        app.message_push('t');
        assert_eq!(app.cmd_line.message, "gt".to_string());

        app.message_pop();
        app.message_pop();
        assert_eq!(app.cmd_line.message, "".to_string());
    }
}
