use crate::parser::ast::SourceUnitPart;
use crate::parser::quake_parser::parse;

#[derive(Debug)]
pub struct InputParser {
    pub object: String,
    pub action: String,
    pub text: String,
}

impl Default for InputParser {
    fn default() -> Self {
        InputParser {
            object: "".to_string(),
            action: "".to_string(),
            text: "".to_string()
        }
    }
}

impl InputParser {
    pub fn from(text: &str) -> InputParser {
        let unit = parse(text);
        let mut expr = InputParser::default();
        for part in unit.0 {
            match part {
                SourceUnitPart::Action(action) => {
                    expr.action = action.action;
                    expr.object = action.object;
                    expr.text = action.text;
                }
            }
        }

        expr
    }
}


#[cfg(test)]
mod tests {
    use crate::input_parser::InputParser;

    #[test]
    fn should_parse_expression() {
        let expr = InputParser::from("todo.add: 添加 todo 的支持");
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "add");
        assert_eq!(expr.text, "添加 todo 的支持");
    }
}
