use crate::parser::ast::SourceUnitPart;
use crate::parser::quake_parser::parse;

#[derive(Debug)]
pub struct InputParser {
    pub object: String,
    pub action: String,
    pub text: String,
    pub parameters: Vec<String>
}

impl Default for InputParser {
    fn default() -> Self {
        InputParser {
            object: "".to_string(),
            action: "".to_string(),
            text: "".to_string(),
            parameters: vec![]
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

                    for parameter in action.parameters {
                        expr.parameters.push(parameter.value);
                    }
                }
            }
        }

        expr
    }

    pub fn index_from_parameter(&self) -> usize {
        let string = &self.parameters[0];
        string.parse().unwrap()
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

    #[test]
    fn should_parse_update_parameter() {
        let expr = InputParser::from("todo.update(1)");
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "update");
        assert_eq!(expr.parameters[0], "1");

        assert_eq!(1, expr.index_from_parameter());
    }


    #[test]
    fn should_parse_com() {
        let expr = InputParser::from("phodal_com.sync");
        assert_eq!(expr.object, "phodal_com");
        assert_eq!(expr.action, "sync");
    }
}
