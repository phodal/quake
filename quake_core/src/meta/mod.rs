use indexmap::IndexMap;

pub use meta_property::MetaProperty;

pub mod meta_property;
pub mod quake_change;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Author {
    name: String,
    email: String,
}

impl Author {
    pub fn new(str: String) -> Author {
        Author {
            name: str,
            email: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryDefineProperties {}

impl EntryDefineProperties {
    pub fn from(map: IndexMap<String, String>) -> IndexMap<String, MetaProperty> {
        let mut fields = IndexMap::new();
        for (key, value) in map {
            fields.insert(key, Self::parse_property_type(value));
        }

        fields
    }

    fn parse_property_type(value: String) -> MetaProperty {
        let field = match value.to_lowercase().as_str() {
            "text" => MetaProperty::Text(value),
            "title" => MetaProperty::Title(value),
            "flow" => MetaProperty::Flow(value),
            "string" => MetaProperty::Text(value),
            "file" => MetaProperty::File(value),
            "searchable" => MetaProperty::Searchable("string".to_string()),
            "filterable" => MetaProperty::Filterable("string".to_string()),
            "date" => MetaProperty::Date(value),
            _ => MetaProperty::Unknown(value),
        };
        field
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use crate::meta::{EntryDefineProperties, MetaProperty};

    #[test]
    fn custom_type() {
        let mut map = IndexMap::new();
        map.insert("title".to_string(), "Title".to_string());

        let properties = EntryDefineProperties::from(map);

        let option = properties.get("title").unwrap();
        assert_eq!(&MetaProperty::Title(String::from("Title")), option)
    }
}
