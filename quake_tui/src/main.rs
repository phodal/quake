use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut _terminal = Terminal::new(backend)?;
    Ok(())
}
