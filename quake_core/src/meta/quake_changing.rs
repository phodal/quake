#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct QuakeChanging {
    pub from: String,
    pub to: String,
    pub changed_date: String,
}
