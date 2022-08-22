use std::fmt::{Debug, Display, Formatter};

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Serialize, Serializer};

lazy_static! {
    static ref STATUS_CHANGE: Regex = Regex::new(
        r#"(?P<time>\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2})\s"(?P<from>.*)" -> "(?P<to>.*)""#
    )
    .unwrap();
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct QuakeChange {
    pub from: String,
    pub to: String,
    pub changed_date: String,
}

impl Serialize for QuakeChange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{:}", self).as_str())
    }
}

impl Display for QuakeChange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.to.is_empty() {
            write!(f, "{:} {:?}", self.changed_date, self.from)
        } else {
            write!(f, "{:} {:?} -> {:?}", self.changed_date, self.from, self.to)
        }
    }
}

impl QuakeChange {
    pub fn from(text: &str) -> Option<QuakeChange> {
        match STATUS_CHANGE.captures(text) {
            None => None,
            Some(caps) => {
                let mut change = QuakeChange {
                    from: caps["from"].to_string(),
                    to: "".to_string(),
                    changed_date: caps["time"].to_string(),
                };

                match &caps.name("to") {
                    None => {}
                    Some(match_) => change.to = match_.as_str().to_string(),
                }

                Some(change)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::meta::quake_change::QuakeChange;

    #[test]
    fn from_string() {
        let text = "2021-12-09 09:40:28 \"Spike\" -> \"Todo\"";
        let changing = QuakeChange::from(text).unwrap();
        assert_eq!("2021-12-09 09:40:28", changing.changed_date);
        assert_eq!("Spike", changing.from);
        assert_eq!("Todo", changing.to);

        assert_eq!(text, format!("{:}", changing));
    }

    #[test]
    fn option_target() {
        let text = "2021-12-09 09:40:28 \"\" -> \"Spike\"";
        let changing = QuakeChange::from(text).unwrap();
        assert_eq!("2021-12-09 09:40:28", changing.changed_date);
        assert_eq!("", changing.from);
        assert_eq!("Spike", changing.to);

        assert_eq!(text, format!("{:}", changing));
    }
}
