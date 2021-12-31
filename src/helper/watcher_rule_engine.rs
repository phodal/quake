use std::ffi::OsStr;
use std::path::Path;

// by rules
// 1. match quake entry: file by suffix for start with index
// 2. match file for engine?
// generate
pub fn event_to_rule(path: &Path) {
    if let Some(_ext) = get_extension(path) {
        // todo: file to rule
    }
}

fn get_extension(path: &Path) -> Option<&str> {
    path.extension().and_then(OsStr::to_str)
}
