use indexmap::IndexMap;

pub use meta_field::MetaField;

pub mod meta_field;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Author {
    name: String,
    email: String,
}

impl Author {
    pub fn new(str: String) -> Author {
        Author {
            name: str.to_string(),
            email: "".to_string(),
        }
    }
}

impl Default for Author {
    fn default() -> Self {
        Author {
            name: "".to_string(),
            email: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryDefineFields {}

impl EntryDefineFields {
    pub fn from(map: IndexMap<String, String>) -> IndexMap<String, MetaField> {
        let mut fields = IndexMap::new();
        for (key, value) in map {
            fields.insert(key, Self::parse_field_type(value));
        }

        fields
    }

    fn parse_field_type(value: String) -> MetaField {
        let field = match value.to_lowercase().as_str() {
            "text" => MetaField::Text(value),
            "title" => MetaField::Title(value),
            "flow" => MetaField::Flow(value),
            "string" => MetaField::Text(value),
            "searchable" => MetaField::Searchable("string".to_string()),
            "filterable" => MetaField::Filterable("string".to_string()),
            "date" => MetaField::Date(value),
            _ => MetaField::Unknown(value),
        };
        field
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use crate::meta::{EntryDefineFields, MetaField};

    #[test]
    fn custom_type() {
        let mut map = IndexMap::new();
        map.insert("title".to_string(), "Title".to_string());

        let fields = EntryDefineFields::from(map);

        let option = fields.get("title").unwrap();
        assert_eq!(&MetaField::Title(String::from("Title")), option)
    }
}
