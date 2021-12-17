use std::collections::HashMap;
use std::path::PathBuf;

use quake_core::usecases::entry_usecases;
use quake_core::QuakeConfig;

use crate::widgets::{CmdLine, MainWidget};

pub struct App {
    pub state: AppState,
    pub config: QuakeConfig,
    pub main_widget: MainWidget,
    pub cmd_line: CmdLine,
}

impl App {
    pub fn new(config: QuakeConfig) -> App {
        App {
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
            let result =
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
                    });
            match result {
                Ok(_) => self.send_message("saved!"),
                Err(_) => self.send_message("save failed!"),
            }
        }
    }

    pub fn message_clear(&mut self) {
        self.cmd_line.message.clear();
    }

    pub fn send_message(&mut self, message: &str) {
        self.message_clear();
        self.cmd_line.message.push_str(message);
    }

    pub fn back_to_normal(&mut self) {
        self.state.mode = Mode::Normal;
        self.state.input.clear();
        self.message_clear();
    }

    pub fn input_push(&mut self, ch: char) {
        self.state.input.push(ch);
        self.cmd_line.message.push(ch);
    }

    pub fn input_pop(&mut self) {
        self.state.input.pop();
        self.cmd_line.message.pop();
    }

    pub fn collect_command(&mut self) -> String {
        self.state.input.drain(..).collect()
    }
}

pub struct AppState {
    running: bool,
    pub mode: Mode,
    input: String,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState {
            running: true,
            mode: Mode::Normal,
            input: "".to_string(),
        }
    }
}

pub enum Mode {
    Command,
    Normal,
    Insert,
}

#[cfg(test)]
mod tests {
    use quake_core::QuakeConfig;
    use rstest::{fixture, rstest};

    use super::{App, Mode};

    #[fixture]
    pub fn app() -> App {
        let mut app = App::new(QuakeConfig::default());
        app.state.mode = Mode::Command;
        app
    }

    #[rstest]
    #[case(vec!['g', 't'], "gt")]
    #[case(vec!['l', 's'], "ls")]
    #[case(vec!['q', 'u', 'i', 't'], "quit")]
    #[case(vec!['*', '/', '#', '@'], "*/#@")]
    fn test_command_collect(mut app: App, #[case] input_chars: Vec<char>, #[case] expect: &str) {
        for ch in input_chars {
            app.input_push(ch);
        }
        assert_eq!(app.state.input, expect.to_string());

        let command = app.collect_command();
        assert_eq!(command, expect.to_string());
        assert_eq!(app.state.input, "".to_string());
        assert_eq!(app.cmd_line.message, expect.to_string());
    }

    #[rstest]
    #[case("todo.show")]
    #[case("todo.edit(1)")]
    #[case("todo.add: hello")]
    #[case("success")]
    fn test_send_message(mut app: App, #[case] message: &str) {
        app.input_push('g');
        app.input_push('t');
        assert_eq!(app.cmd_line.message, "gt".to_string());

        app.send_message(message);
        assert_eq!(app.cmd_line.message, message.to_string());
    }

    #[rstest]
    fn test_clear_state_after_back_to_normal(mut app: App) {
        app.input_push('l');
        app.input_push('s');
        assert_eq!(app.cmd_line.message, "ls".to_string());
        assert_eq!(app.state.input, "ls".to_string());

        app.back_to_normal();
        assert_eq!(app.cmd_line.message, "".to_string());
        assert_eq!(app.state.input, "".to_string());
    }
}
