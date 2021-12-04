use crate::parser::ast::SourceUnitPart;
use crate::parser::parser::parse;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionDefine {
    pub object: String,
    pub action: String,
    pub text: String,
    pub parameters: Vec<String>,
}

impl Default for ActionDefine {
    fn default() -> Self {
        ActionDefine {
            object: "".to_string(),
            action: "".to_string(),
            text: "".to_string(),
            parameters: vec![],
        }
    }
}

impl ActionDefine {
    pub fn from(text: &str) -> Result<ActionDefine, Box<dyn Error>> {
        let unit = parse(text)?;
        let mut expr = ActionDefine::default();
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

        Ok(expr)
    }

    pub fn index_from_parameter(&self) -> usize {
        let string = &self.parameters[0];
        string.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::quake_parser::ActionDefine;

    #[test]
    fn should_parse_expression() {
        let expr = ActionDefine::from("todo.add: 添加 todo 的支持").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "add");
        assert_eq!(expr.text, "添加 todo 的支持");
    }

    #[test]
    fn should_parse_update_parameter() {
        let expr = ActionDefine::from("todo.update(1)").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "update");
        assert_eq!(expr.parameters[0], "1");

        assert_eq!(1, expr.index_from_parameter());
    }

    #[test]
    fn should_parse_com() {
        let expr = ActionDefine::from("phodal_com.sync").unwrap();
        assert_eq!(expr.object, "phodal_com");
        assert_eq!(expr.action, "sync");
    }

    #[test]
    fn should_parse_double_digital() {
        let expr = ActionDefine::from("todo.update(12)").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "update");
        assert_eq!(expr.parameters[0], "12");
        assert_eq!(12, expr.index_from_parameter());
    }

    #[test]
    fn should_parse_chinese_quote() {
        let expr = ActionDefine::from("todo.update（12）").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "update");
        assert_eq!(expr.parameters[0], "12");
        assert_eq!(12, expr.index_from_parameter());
    }
}
