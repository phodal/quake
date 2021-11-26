use indexmap::IndexMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct FrontMatter {
    pub fields: IndexMap<String, String>
}

impl Default for FrontMatter {
    fn default() -> Self {
        FrontMatter {
            fields: IndexMap::new()
        }
    }
}
