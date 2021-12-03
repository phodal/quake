use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut _terminal = Terminal::new(backend)?;
    Ok(())
}
