use std::path::PathBuf;
use crate::action::entry_sets::Entrysets;

#[get("/<entry_type>")]
pub(crate) fn entry(entry_type: &str) -> String {
    let path = PathBuf::from("_fixtures").join(entry_type);
    let result = Entrysets::jsonify(&path);
    result.unwrap()
}
