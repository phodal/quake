/// load from `.quake`
#[derive(Debug, Serialize, Deserialize)]
pub struct QuakeConfig {
    // set default editor
    pub editor: String,
    pub workspace: String,
    pub search_url: String,
    pub server_location: String,
    pub port: u32,
}

impl Default for QuakeConfig {
    fn default() -> Self {
        QuakeConfig {
            editor: "".to_string(),
            workspace: "".to_string(),
            search_url: "".to_string(),
            server_location: "".to_string(),
            port: 9999,
        }
    }
}
