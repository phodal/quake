pub struct ConceptExpr {
    pub object: String,
    pub action: String,
    pub tag: String,
    pub keyword: String,
}

pub struct SearchTextParser {}

impl SearchTextParser {
    pub fn parse(text: &str) {}
}


#[cfg(test)]
mod tests {
    use crate::search_text_parser::SearchTextParser;

    #[test]
    fn it_works() {
        SearchTextParser::parse("todo")
    }
}
