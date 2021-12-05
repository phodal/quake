use std::error::Error;

use crate::parser::ast::SourceUnitPart;
use crate::parser::errors::QuakeParserError;
use crate::parser::parser::parse;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuakeIt {
    pub actions: Vec<QuakeAction>,
    pub transflows: Vec<Transflow>,
}

impl Default for QuakeIt {
    fn default() -> Self {
        QuakeIt {
            actions: vec![],
            transflows: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transflow {
    pub routes: Vec<Route>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub from: Vec<String>,
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuakeAction {
    pub object: String,
    pub action: String,
    pub text: String,
    pub parameters: Vec<String>,
}

impl Default for QuakeAction {
    fn default() -> Self {
        QuakeAction {
            object: "".to_string(),
            action: "".to_string(),
            text: "".to_string(),
            parameters: vec![],
        }
    }
}

impl QuakeAction {
    pub fn action_from_text(text: &str) -> Result<QuakeAction, Box<dyn Error>> {
        let it = quake(text)?;
        if it.actions.is_empty() {
            return Err(Box::new(QuakeParserError::new("not match action")));
        }

        Ok(it.actions[0].clone())
    }

    pub fn index_from_parameter(&self) -> usize {
        let string = &self.parameters[0];
        string.parse().unwrap()
    }
}

pub fn quake(text: &str) -> Result<QuakeIt, Box<dyn Error>> {
    let mut quakes = QuakeIt::default();
    let unit = parse(text)?;

    for part in unit.0 {
        match part {
            SourceUnitPart::Action(decl) => {
                let mut action = QuakeAction::default();

                action.action = decl.action;
                action.object = decl.object;
                action.text = decl.text;

                for parameter in decl.parameters {
                    action.parameters.push(parameter.value);
                }

                quakes.actions.push(action);
            }
            SourceUnitPart::Transflow(_transflow) => {}
        }
    }

    Ok(quakes)
}

#[cfg(test)]
mod tests {
    use crate::parser::quake::QuakeAction;

    #[test]
    fn should_parse_expression() {
        let expr = QuakeAction::action_from_text("todo.add: 添加 todo 的支持").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "add");
        assert_eq!(expr.text, "添加 todo 的支持");
    }

    #[test]
    fn should_parse_update_parameter() {
        let expr = QuakeAction::action_from_text("todo.update(1)").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "update");
        assert_eq!(expr.parameters[0], "1");

        assert_eq!(1, expr.index_from_parameter());
    }

    #[test]
    fn should_parse_com() {
        let expr = QuakeAction::action_from_text("phodal_com.sync").unwrap();
        assert_eq!(expr.object, "phodal_com");
        assert_eq!(expr.action, "sync");
    }

    #[test]
    fn should_parse_double_digital() {
        let expr = QuakeAction::action_from_text("todo.update(12)").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "update");
        assert_eq!(expr.parameters[0], "12");
        assert_eq!(12, expr.index_from_parameter());
    }

    #[test]
    fn should_parse_chinese_quote() {
        let expr = QuakeAction::action_from_text("todo.update（12）").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "update");
        assert_eq!(expr.parameters[0], "12");
        assert_eq!(12, expr.index_from_parameter());
    }

    #[test]
    fn should_create_transflow() {
        let define = "define { from('todo','blog').to(<quake-calendar>); }";
        // let expr = QuakeAction::action_from_text(define).unwrap();
        // println!("{:?}", expr);
    }
}
