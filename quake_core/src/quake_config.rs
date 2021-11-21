/// load from `.quake`
#[derive(Debug, Deserialize)]
pub struct QuakeConfig {
    // storage path
    pub path: String,
    // set default editor
    pub editor: String
}
