use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};

use crossterm::event::{self, Event, KeyCode};
use quake_core::entry::entry_defines;
use quake_core::entry::entry_paths::EntryPaths;
use quake_core::usecases::entry_usecases;
use quake_core::QuakeConfig;
use tui::backend::Backend;
use tui::widgets::ListState;
use tui::Terminal;

use crate::entry_action::action_result_to_main_widget;
use crate::ui::draw;
use crate::widgets::MainWidget;

pub struct App {
    pub state: AppState,
    pub config: QuakeConfig,
    pub main_widget: MainWidget,
}

impl App {
    pub fn new(config: QuakeConfig) -> App {
        App {
            main_widget: MainWidget::Home,
            state: Default::default(),
            config,
        }
    }

    pub fn shutdown(&mut self) {
        self.state.running = false;
    }

    pub fn save_entry(&mut self) {
        if let MainWidget::Editor {
            ref entry_type,
            ref id,
            ref content,
        } = self.main_widget
        {
            let mut fields = HashMap::new();
            fields.insert("content".to_string(), content.clone());
            let type_path = PathBuf::from(&self.config.workspace).join(entry_type);
            match entry_usecases::update_entry_properties(type_path, entry_type, *id, &fields) {
                Ok(_) => {
                    entry_usecases::sync_in_path(&EntryPaths::init(
                        &self.config.workspace,
                        entry_type,
                    ))
                    .unwrap();
                    self.send_message("saved!")
                }
                Err(_) => self.send_message("save failed!"),
            }
        }
    }

    pub fn message_clear(&mut self) {
        self.state.message.clear();
    }

    pub fn send_message(&mut self, message: &str) {
        self.message_clear();
        self.state.message.push_str(message);
    }

    pub fn input_push(&mut self, ch: char) {
        self.state.input.push(ch);
        self.state.message.push(ch);
    }

    pub fn input_pop(&mut self) {
        self.state.input.pop();
        self.state.message.pop();
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
                    self.jump_to_command_mode();
                }
                KeyCode::Char('i') => {
                    self.state.mode = Mode::Insert;
                }
                KeyCode::Char('j') => self.entry_next(),
                KeyCode::Char('k') => self.entry_prev(),
                KeyCode::Char('a') => self.action_auto_complete(),
                _ => {}
            },
            Mode::Command => match key_code {
                KeyCode::Enter => {
                    if let Err(err_msg) = self.execute_command() {
                        self.send_message(&err_msg);
                    }
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
                    self.main_widget.input_push(c);
                }
                KeyCode::Backspace => {
                    self.main_widget.input_pop();
                }
                KeyCode::Enter => {
                    self.main_widget.input_push('\n');
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
            "listAll" => {
                let entry_defines_path =
                    Path::new(&self.config.workspace).join(EntryPaths::entries_define());
                let defines = entry_defines::from_path(&entry_defines_path);
                if !defines.is_empty() {
                    self.state.entry_list_state.select(Some(0));
                }
                self.main_widget = MainWidget::EntryTypes(defines);
            }
            "save" => self.save_entry(),
            other => {
                return action_result_to_main_widget(other, &self.config)
                    .map(|widget| self.main_widget = widget);
            }
        }

        Ok(())
    }

    pub fn entry_next(&mut self) {
        if let MainWidget::EntryTypes(defines) = &self.main_widget {
            let len = defines.len();
            let i = match self.state.entry_list_state.selected() {
                Some(i) => {
                    if i >= len - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.state.entry_list_state.select(Some(i));
        }
    }

    pub fn entry_prev(&mut self) {
        if let MainWidget::EntryTypes(defines) = &self.main_widget {
            let len = defines.len();
            let i = match self.state.entry_list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        len - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.state.entry_list_state.select(Some(i));
        }
    }

    fn action_auto_complete(&mut self) {
        if let MainWidget::EntryTypes(defines) = &self.main_widget {
            if let Some(idx) = self.state.entry_list_state.selected() {
                let cmd = format!("{}.add: ", defines[idx].entry_type);
                self.jump_to_command_mode();
                self.state.input = cmd.clone();
                self.state.message = cmd;
            }
        }
    }

    fn jump_to_command_mode(&mut self) {
        self.collect_command();
        self.message_clear();
        self.state.mode = Mode::Command;
    }
}

pub struct AppState {
    running: bool,
    pub mode: Mode,
    input: String,
    pub entry_list_state: ListState,
    pub message: String,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState {
            running: true,
            mode: Mode::Normal,
            input: "".to_string(),
            entry_list_state: ListState::default(),
            message: "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
        assert_eq!(app.state.message, expect.to_string());
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
        assert_eq!(app.state.message, "gt".to_string());

        app.send_message(message);
        assert_eq!(app.state.message, message.to_string());
    }

    #[rstest]
    fn test_cancel_command(mut app: App) {
        assert_eq!(app.state.mode, Mode::Normal);
        app.handle_key_event(KeyCode::Char(':')).unwrap();
        assert_eq!(app.state.mode, Mode::Command);

        app.handle_key_event(KeyCode::Char('l')).unwrap();
        app.handle_key_event(KeyCode::Char('s')).unwrap();
        assert_eq!(app.state.input, "ls".to_string());
        assert_eq!(app.state.message, "ls".to_string());

        app.handle_key_event(KeyCode::Esc).unwrap();
        assert_eq!(app.state.mode, Mode::Normal);
        assert_eq!(app.state.input, "".to_string());
        assert_eq!(app.state.message, "".to_string());
    }

    #[rstest]
    #[case("todo.show")]
    #[case("todo.edit(1)")]
    #[case("todo.add: hello")]
    #[case("success")]
    fn test_into_command_mode(mut app: App, #[case] message: &str) {
        app.state.input = message.to_string();
        app.state.message = message.to_string();

        app.handle_key_event(KeyCode::Char(':')).unwrap();
        assert_eq!(app.state.input, "".to_string());
        assert_eq!(app.state.message, "".to_string());
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
    #[case(vec!['u', 'k', 'n', 'o', 'w', 'n'])]
    #[case(vec!['e', 'r', 'r', 'o', 'r'])]
    #[case(vec!['t', 'o', 'd', 'o', ' ', 'a', 'd', 'd', 'd'])]
    fn test_unknown_command_error_message(mut app: App, #[case] inputs: Vec<char>) {
        app.handle_key_event(KeyCode::Char(':')).unwrap();
        inputs
            .iter()
            .for_each(|ch| app.handle_key_event(KeyCode::Char(ch.to_owned())).unwrap());

        app.handle_key_event(KeyCode::Enter).unwrap();

        assert_eq!(app.state.mode, Mode::Normal);

        let error_message = format!("Unknown command: {}", inputs.iter().collect::<String>());
        assert_eq!(app.state.message, error_message);
    }

    #[rstest]
    fn test_back_to_normal_after_command_exec(mut app: App) {
        app.state.mode = Mode::Command;
        app.state.input = "nonexistent".to_string();
        let _ = app.execute_command();
        assert_eq!(app.state.mode, Mode::Normal);
    }
}
