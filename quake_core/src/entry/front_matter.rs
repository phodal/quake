use indexmap::IndexMap;

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
