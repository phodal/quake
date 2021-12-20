use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ENTRY_LINK_RE: Regex =
        Regex::new(r#"^(?P<type>[^#|:]+):(?P<id>\d{1,})??(#(?P<section>.+?))??(\|(?P<label>.+?))??\s"(?P<title>[^"]+)"$"#).unwrap();
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct EntryReference {
    entry_type: String,
    entry_id: String,
    entry_title: String,
    label: Option<String>,
    section: Option<String>,
}

impl EntryReference {
    #[allow(clippy::all)]
    pub fn from_str(text: &str) -> EntryReference {
        let captures = ENTRY_LINK_RE
            .captures(text)
            .expect("note link regex didn't match - bad input?");

        let entry_type = captures.name("type").map(|v| v.as_str()).unwrap_or("");
        let entry_id = captures.name("id").map(|v| v.as_str()).unwrap_or("");
        let entry_title = captures.name("title").map(|v| v.as_str()).unwrap_or("");

        let label = captures.name("label").map(|v| v.as_str().to_string());
        let section = captures.name("section").map(|v| v.as_str().to_string());

        EntryReference {
            entry_type: entry_type.to_string(),
            entry_id: entry_id.to_string(),
            entry_title: entry_title.to_string(),
            label,
            section,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::markdown::entry_reference::EntryReference;

    #[test]
    fn parse_quake_down_link() {
        let text = r#"note:0001#Heading|Label "file name""#;
        let reference = EntryReference::from_str(text);

        assert_eq!("note", reference.entry_type);
        assert_eq!("file name", reference.entry_title);
        assert_eq!("0001", reference.entry_id);

        assert_eq!("Label", reference.label.unwrap());
        assert_eq!("Heading", reference.section.unwrap());
    }

    #[test]
    fn parse_normal() {
        let text = r#"note:0001 "file name""#;
        let reference = EntryReference::from_str(text);

        assert_eq!("note", reference.entry_type);
        assert_eq!("file name", reference.entry_title);
        assert_eq!("0001", reference.entry_id);

        assert!(reference.label.is_none());
        assert!(reference.section.is_none());
    }
}
