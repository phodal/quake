use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::entry::entry_paths::EntryPaths;
use crate::quake::SimpleLayout;

pub fn dump_dashboard_layout(path: PathBuf) -> Result<SimpleLayout, Box<dyn Error>> {
    let path = path
        .join(EntryPaths::quake())
        .join(EntryPaths::dashboard_layout());

    let content = fs::read_to_string(path)?;
    SimpleLayout::from_text(content.as_str())
}
