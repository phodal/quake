mod app;
mod ui;

use crate::app::{App, MainWidget, Mode};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::error::Error;
use std::io;
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use ui::draw;

pub fn tui_main_loop() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    // TODO: refactor
    while app.running() {
        terminal.draw(|f| {
            draw(f, &mut app);
        })?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::Normal => {
                    if let KeyCode::Char(':') = key.code {
                        app.mode = Mode::Command;
                    }
                }
                Mode::Command => match key.code {
                    KeyCode::Enter => {
                        let command: String = app.command.drain(..).collect();
                        match command.as_str() {
                            "quit" => app.shutdown(),
                            "listAll" => app.main_widget = MainWidget::EntryTypes,
                            _ => {}
                        }
                    }
                    KeyCode::Char(c) => {
                        app.command.push(c);
                    }
                    KeyCode::Backspace => {
                        app.command.pop();
                    }
                    KeyCode::Esc => {
                        app.mode = Mode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }

    Ok(())
}
