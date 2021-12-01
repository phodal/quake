use indexmap::IndexMap;

use crate::model::meta_field::MetaField;

pub mod meta_field;
pub mod meta_action;
pub mod meta_config;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Author {
    name: String,
    email: String,
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
pub struct CustomType {
    pub fields: IndexMap<String, MetaField>,
}

impl CustomType {
    pub fn from(map: IndexMap<String, String>) -> CustomType {
        let mut fields = IndexMap::new();
        for (key, value) in map {
            fields.insert(key, Self::parse_field_type(value));
        }

        CustomType {
            fields,
        }
    }

    pub fn field(&self, text: &str) -> Option<&MetaField> {
        self.fields.get(text)
    }

    fn parse_field_type(value: String) -> MetaField {
        let field = match value.as_str() {
            "Text" => {
                MetaField::Text(value)
            }
            "Title" => {
                MetaField::Title(value)
            }
            "Tagged" => {
                let tags = vec![];
                MetaField::Tagged(tags)
            }
            "Author" => {
                let author = Author::default();
                MetaField::Author(author)
            }
            "Date" => {
                MetaField::Date(value)
            }
            _ => {
                MetaField::Unknown(value)
            }
        };
        field
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use crate::model::CustomType;
    use crate::model::meta_field::MetaField;

    #[test]
    fn custom_type() {
        let mut map = IndexMap::new();
        map.insert("title".to_string(), "Title".to_string());

        let custom_type = CustomType::from(map);

        let option = custom_type.fields.get("title").unwrap();
        assert_eq!(&MetaField::Title(String::from("Title")), option)
    }
}
