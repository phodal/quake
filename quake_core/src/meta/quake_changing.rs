use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref STATUS_CHANGE: Regex = Regex::new(
        r#"(?P<time>\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2})\s"(?P<from>.*)" -> "(?P<to>.*)""#
    )
    .unwrap();
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct QuakeChanging {
    pub from: String,
    pub to: String,
    pub changed_date: String,
}

impl QuakeChanging {
    pub fn from(text: &str) -> QuakeChanging {
        match STATUS_CHANGE.captures(text) {
            None => QuakeChanging {
                from: "".to_string(),
                to: "".to_string(),
                changed_date: "".to_string(),
            },
            Some(caps) => QuakeChanging {
                from: caps["from"].to_string(),
                to: caps["to"].to_string(),
                changed_date: caps["time"].to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::meta::quake_changing::QuakeChanging;

    #[test]
    fn from_string() {
        let text = "2021-12-09 09:40:28 \"Spike\" -> \"Todo\"";
        let changing = QuakeChanging::from(text);
        assert_eq!("2021-12-09 09:40:28", changing.changed_date);
        assert_eq!("Spike", changing.from);
        assert_eq!("Todo", changing.to);
    }
}
