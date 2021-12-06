#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SectionVO {
    pub source_url: String,
    pub id: String,
    pub created_date_time: String,
    pub display_name: String,
    pub last_modified_date_time: String,
    pub parent_name: String,
    pub pages: Vec<PageVO>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageVO {
    pub source_url: String,
    pub id: String,
    pub created_date_time: String,
    pub last_modified_date_time: Option<String>,
    pub title: String,
    pub content_url: String,
}
