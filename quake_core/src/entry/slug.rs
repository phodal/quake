use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SPECIAL: Regex = Regex::new("[\\s_-]+").unwrap(); // swap any length of whitespace, underscore, hyphen characters with a single _
    static ref LEADING: Regex = Regex::new("^-+|-+$").unwrap();
}

pub fn slugify(text: &str) -> String {
    let lower = text.trim().to_lowercase();
    let result = SPECIAL.replace_all(lower.as_str(), "-").to_string();
    let result = LEADING.replace_all(result.as_str(), "").to_string();

    result
        .replace([' ', '?', '#', ':'], "-")
        .replace([',', '。'], "")
        .replace("-/-", "")
        .replace('/', "")
        .replace("——", "-")
}

#[cfg(test)]
mod tests {
    use crate::entry::slug::slugify;

    #[test]
    fn chinese_slug() {
        let string = slugify("你無可奈何asd fsadf+");
        assert_eq!("你無可奈何asd-fsadf+", string);
    }

    #[test]
    fn leading_slash() {
        let string = slugify("-love");
        assert_eq!("love", string);
    }
}
