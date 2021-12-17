use crate::app::{App, Mode};
use crate::widgets::MainWidget;
use quake_core::parser::quake::QuakeActionNode;

pub fn execute_command(command: &str, app: &mut App) -> Result<(), String> {
    match command {
        "quit" => app.shutdown(),
        "listAll" => app.main_widget = MainWidget::EntryTypes,
        "save" => app.save_entry(),
        other => execute_action_command(other, app)?,
    }
    Ok(())
}

pub fn execute_action_command(command: &str, app: &mut App) -> Result<(), String> {
    if let Ok(action) = QuakeActionNode::action_from_text(command) {
        app.state.mode = Mode::Insert;
        app.main_widget = MainWidget::Editor(action, "".to_string());
        Ok(())
    } else {
        Err(format!("Unknown command: {}", command))
    }
}

#[cfg(test)]
mod tests {
    use super::execute_command;
    use crate::app::App;
    use quake_core::QuakeConfig;

    #[test]
    fn test_unknown_command() {
        let mut app = App::new(QuakeConfig::default());

        let result = execute_command("nonexistent", &mut app);
        assert_eq!(result, Err("Unknown command: nonexistent".to_string()));
    }
}
