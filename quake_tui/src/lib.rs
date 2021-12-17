mod app;
mod command;
mod ui;
mod widgets;

use crate::app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use quake_core::QuakeConfig;
use std::error::Error;
use std::fs;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub fn tui_main_loop() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config: QuakeConfig = serde_yaml::from_str(fs::read_to_string(".quake.yaml")?.as_str())?;
    let app = App::new(config);
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
