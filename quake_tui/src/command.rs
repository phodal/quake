use crate::app::{App, MainWidget};

pub fn execute_command(command: &str, app: &mut App) -> Result<(), String> {
    match command {
        "quit" => app.shutdown(),
        "listAll" => app.main_widget = MainWidget::EntryTypes,
        _ => return Err(format!("Unknown command: {}", command)),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::execute_command;
    use crate::app::App;

    #[test]
    fn test_command_quit() {
        let mut app = App::new();

        assert!(app.running());
        execute_command("quit", &mut app).unwrap();
        assert!(!app.running());
    }

    #[test]
    fn test_unknown_command() {
        let mut app = App::new();

        let result = execute_command("nonexistent", &mut app);
        assert_eq!(result, Err("Unknown command: nonexistent".to_string()));
    }
}
