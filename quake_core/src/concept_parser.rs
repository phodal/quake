pub struct ConceptExpr {
    pub object: String,
    pub action: String,
    pub text: String,

    // todo: thinking in tag & at
    pub tag: String,
    pub at: String,
}

impl ConceptExpr {
    pub fn from(text: &str) {}
}


#[cfg(test)]
mod tests {
    use crate::concept_parser::ConceptExpr;

    #[test]
    fn should_parse_expression() {
        ConceptExpr::from("todo.add: 添加 todo 的支持 #todo #文章")
    }
}
