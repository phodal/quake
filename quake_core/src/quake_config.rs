/// load from `.quake`
#[derive(Debug, Deserialize)]
pub struct QuakeConfig {
    // storage path
    pub path: String,
    // set default editor
    pub editor: String
}

impl Default for QuakeConfig {
    fn default() -> Self {
        QuakeConfig {
            path: "".to_string(),
            editor: "".to_string()
        }
    }
}
