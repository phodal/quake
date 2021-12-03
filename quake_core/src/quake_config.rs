/// load from `.quake`
#[derive(Debug, Serialize, Deserialize)]
pub struct QuakeConfig {
    // storage path
    pub path: String,
    // set default editor
    pub editor: String,
    pub workspace: String,
    pub search_url: String,
    pub server_location: String,
}

impl Default for QuakeConfig {
    fn default() -> Self {
        QuakeConfig {
            path: "".to_string(),
            editor: "".to_string(),
            workspace: "".to_string(),
            search_url: "".to_string(),
            server_location: "".to_string()
        }
    }
}
