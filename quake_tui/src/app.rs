use std::path::PathBuf;
use std::{fs, io};

pub struct App {
    pub entry_dirs: Vec<PathBuf>,
}

impl App {
    pub fn new() -> App {
        App::default()
    }

    pub fn on_tick(&mut self) {
        if let Ok(dirs) = list_entries_dirs() {
            self.entry_dirs = dirs;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App { entry_dirs: vec![] }
    }
}

fn list_entries_dirs() -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .filter(|path| path.as_ref().map(is_entries_dir).unwrap_or(false))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(entries)
}

fn is_entries_dir(path: &PathBuf) -> bool {
    path.is_dir() && path.join("entries.csv").exists()
}
