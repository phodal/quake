/// load from `.quake`
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QuakeConfig {
    // set default editor
    pub editor: String,
    pub workspace: String,
    pub search_url: String,
    pub server_location: String,
    pub port: u32,
}
