use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode};
use quake_core::parser::quake::QuakeActionNode;
use quake_core::usecases::entry_usecases;
use quake_core::QuakeConfig;
use tui::backend::Backend;
use tui::Terminal;

use crate::ui::draw;
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

    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        while self.state.running {
            terminal.draw(|f| {
                draw(f, &mut self);
            })?;
            if let Event::Key(key) = event::read()? {
                self.handle_key_event(key.code)?;
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_code: KeyCode) -> Result<(), Box<dyn Error>> {
        match self.state.mode {
            Mode::Normal => match key_code {
                KeyCode::Char(':') => {
                    self.collect_command();
                    self.message_clear();
                    self.state.mode = Mode::Command;
                }
                KeyCode::Char('i') => {
                    self.state.mode = Mode::Insert;
                }
                _ => {}
            },
            Mode::Command => match key_code {
                KeyCode::Enter => {
                    self.execute_command()?;
                }
                KeyCode::Char(c) => {
                    self.input_push(c);
                }
                KeyCode::Backspace => {
                    self.input_pop();
                }
                KeyCode::Esc => {
                    self.collect_command();
                    self.message_clear();
                    self.state.mode = Mode::Normal;
                }
                _ => {}
            },
            Mode::Insert => match key_code {
                KeyCode::Esc => {
                    self.state.mode = Mode::Normal;
                }
                KeyCode::Char(c) => {
                    self.main_widget.collect_input(c);
                }
                _ => {}
            },
        }

        Ok(())
    }

    fn execute_command(&mut self) -> Result<(), String> {
        self.state.mode = Mode::Normal;
        let command: String = self.collect_command();
        match command.as_str() {
            "quit" => self.shutdown(),
            "listAll" => self.main_widget = MainWidget::EntryTypes,
            "save" => self.save_entry(),
            other => {
                if let Ok(action) = QuakeActionNode::action_from_text(other) {
                    self.main_widget = MainWidget::Editor(action, "".to_string());
                } else {
                    return Err(format!("Unknown command: {}", command));
                }
            }
        }

        Ok(())
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

#[derive(Debug, PartialEq)]
pub enum Mode {
    Command,
    Normal,
    Insert,
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;
    use quake_core::QuakeConfig;
    use rstest::{fixture, rstest};

    use super::{App, Mode};

    #[fixture]
    pub fn app() -> App {
        App::new(QuakeConfig::default())
    }

    #[rstest]
    #[case(vec!['g', 't'], "gt")]
    #[case(vec!['l', 's'], "ls")]
    #[case(vec!['q', 'u', 'i', 't'], "quit")]
    #[case(vec!['*', '/', '#', '@'], "*/#@")]
    fn test_command_collect(mut app: App, #[case] input_chars: Vec<char>, #[case] expect: &str) {
        app.state.mode = Mode::Command;

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
        app.state.mode = Mode::Command;

        app.input_push('g');
        app.input_push('t');
        assert_eq!(app.cmd_line.message, "gt".to_string());

        app.send_message(message);
        assert_eq!(app.cmd_line.message, message.to_string());
    }

    #[rstest]
    fn test_cancel_command(mut app: App) {
        assert_eq!(app.state.mode, Mode::Normal);
        app.handle_key_event(KeyCode::Char(':')).unwrap();
        assert_eq!(app.state.mode, Mode::Command);

        app.handle_key_event(KeyCode::Char('l')).unwrap();
        app.handle_key_event(KeyCode::Char('s')).unwrap();
        assert_eq!(app.state.input, "ls".to_string());
        assert_eq!(app.cmd_line.message, "ls".to_string());

        app.handle_key_event(KeyCode::Esc).unwrap();
        assert_eq!(app.state.mode, Mode::Normal);
        assert_eq!(app.state.input, "".to_string());
        assert_eq!(app.cmd_line.message, "".to_string());
    }

    #[rstest]
    #[case("todo.show")]
    #[case("todo.edit(1)")]
    #[case("todo.add: hello")]
    #[case("success")]
    fn test_into_command_mode(mut app: App, #[case] message: &str) {
        app.state.input = message.to_string();
        app.cmd_line.message = message.to_string();

        app.handle_key_event(KeyCode::Char(':')).unwrap();
        assert_eq!(app.state.input, "".to_string());
        assert_eq!(app.cmd_line.message, "".to_string());
    }

    #[rstest]
    fn test_into_insert_mode(mut app: App) {
        assert_eq!(app.state.mode, Mode::Normal);
        app.handle_key_event(KeyCode::Char('i')).unwrap();
        assert_eq!(app.state.mode, Mode::Insert);
    }

    #[rstest]
    fn test_command_quit(mut app: App) {
        app.state.input = "quit".to_string();
        assert!(app.state.running);
        app.execute_command().unwrap();
        assert!(!app.state.running);
    }

    #[rstest]
    fn test_unknown_command(mut app: App) {
        app.state.input = "nonexistent".to_string();
        let result = app.execute_command();
        assert_eq!(result, Err("Unknown command: nonexistent".to_string()));
    }

    #[rstest]
    fn test_back_to_normal_after_command_exec(mut app: App) {
        app.state.mode = Mode::Command;
        app.state.input = "nonexistent".to_string();
        let _ = app.execute_command();
        assert_eq!(app.state.mode, Mode::Normal);
    }
}
