use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

lazy_static! {
    static ref ENTRY_LINK_RE: Regex =
        Regex::new(r#"^(?P<type>[^#|:]+):(?P<id>\d+)(#(?P<section>.+?))??(\|(?P<label>.+?))??(\s["'“](?P<title>.+?)["'”])??$"#).unwrap();
}

/// one markdown file link to other markdown file
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Default, Clone)]
pub struct PageReference {
    pub(crate) entry_type: String,
    pub(crate) entry_id: String,
    pub(crate) entry_title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) section: Option<String>,
}

impl PageReference {
    #[allow(clippy::all)]
    pub fn from_str(text: &str) -> PageReference {
        let captures = match ENTRY_LINK_RE.captures(text) {
            None => return PageReference::default(),
            Some(capts) => capts,
        };

        let entry_type = captures.name("type").map(|v| v.as_str()).unwrap_or("");
        let entry_id = captures.name("id").map(|v| v.as_str()).unwrap_or("");
        let entry_title = captures.name("title").map(|v| v.as_str()).unwrap_or("");

        let label = captures.name("label").map(|v| v.as_str().to_string());
        let section = captures.name("section").map(|v| v.as_str().to_string());

        PageReference {
            entry_type: entry_type.to_string(),
            entry_id: entry_id.to_string(),
            entry_title: entry_title.to_string(),
            label,
            section,
        }
    }

    pub fn display(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for PageReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self
            .label
            .as_ref()
            .map(|text| format!("|{:}", text))
            .unwrap_or_else(|| "".to_string());
        let section = self
            .section
            .as_ref()
            .map(|text| format!("#{:}", text))
            .unwrap_or_else(|| "".to_string());

        write!(
            f,
            "{}:{}{}{} \"{}\"",
            self.entry_type, self.entry_id, section, label, self.entry_title
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::markdown::entry_reference::PageReference;

    #[test]
    fn parse_quake_down_link() {
        let text = r#"note:0001#Heading|Label "file name""#;
        let reference = PageReference::from_str(text);

        assert_eq!("note", reference.entry_type);
        assert_eq!("file name", reference.entry_title);
        assert_eq!("0001", reference.entry_id);

        assert_eq!("Label", reference.label.unwrap());
        assert_eq!("Heading", reference.section.unwrap());
    }

    #[test]
    fn parse_normal() {
        let text = r#"note:0001 "file name""#;
        let reference = PageReference::from_str(text);

        assert_eq!("note", reference.entry_type);
        assert_eq!("file name", reference.entry_title);
        assert_eq!("0001", reference.entry_id);

        assert!(reference.label.is_none());
        assert!(reference.section.is_none());
    }

    #[test]
    fn print_reference() {
        let text = r#"note:0001 "file name""#;
        let reference = PageReference::from_str(text);
        assert_eq!(text, reference.to_string());

        let with_title = r#"note:0001#heading "file name""#;
        let title_ref = PageReference::from_str(with_title);
        assert_eq!(with_title, title_ref.to_string());

        let with_label = r#"note:0001#heading|label "file name""#;
        let label_ref = PageReference::from_str(with_label);
        assert_eq!(with_label, label_ref.to_string());

        let only_label = r#"Note:0001|Label "file name""#;
        let only_label_ref = PageReference::from_str(only_label);
        assert_eq!(only_label, only_label_ref.to_string());
    }
}
