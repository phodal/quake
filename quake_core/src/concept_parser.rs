use regex::Regex;

#[derive(Debug)]
pub struct ConceptExpr {
    pub object: String,
    pub action: String,
    pub text: String,

    // todo: thinking in tag & at
    // pub tag: String,
    // pub at: String,
}

impl Default for ConceptExpr {
    fn default() -> Self {
        ConceptExpr {
            object: "".to_string(),
            action: "".to_string(),
            text: "".to_string()
        }
    }
}

impl ConceptExpr {
    pub fn from(text: &str) -> ConceptExpr {
        let regex = Regex::new("(?P<object>[a-z]+).(?P<action>[a-z]+):(?P<text>.*)").unwrap();

        let mut expr = ConceptExpr::default();
        if let Some(captures) = regex.captures(text) {
            expr.action = String::from(&captures["action"]);
            expr.object = String::from(&captures["object"]);
            expr.text = String::from(&captures["text"]);
        }

        expr
    }
}


#[cfg(test)]
mod tests {
    use crate::concept_parser::ConceptExpr;

    #[test]
    fn should_parse_expression() {
        let expr = ConceptExpr::from("todo.add:添加 todo 的支持");
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "add");
        assert_eq!(expr.text, "添加 todo 的支持");
    }
}
