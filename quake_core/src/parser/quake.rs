use std::error::Error;

use crate::parser::ast::{SourceUnitPart, TransflowDecl, TransflowEnum};
use crate::parser::errors::QuakeParserError;
use crate::parser::parser::parse;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuakeIt {
    pub actions: Vec<QuakeAction>,
    pub transflows: Vec<QuakeTransflow>,
}

impl Default for QuakeIt {
    fn default() -> Self {
        QuakeIt {
            actions: vec![],
            transflows: vec![],
        }
    }
}

impl QuakeTransflow {
    pub fn from_text(text: &str) -> Result<QuakeTransflow, Box<dyn Error>> {
        let it = quake(text)?;
        if it.transflows.is_empty() {
            return Err(Box::new(QuakeParserError::new("not match transflows")));
        }

        Ok(it.transflows[0].clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuakeTransflow {
    pub routes: Vec<Route>,
}

impl Default for QuakeTransflow {
    fn default() -> Self {
        QuakeTransflow { routes: vec![] }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Route {
    pub from: Vec<String>,
    pub to: String,
    pub is_component: bool,
}

impl Default for Route {
    fn default() -> Self {
        Route {
            from: vec![],
            to: "".to_string(),
            is_component: false,
        }
    }
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
            SourceUnitPart::Transflow(decl) => {
                let transflow = build_transflow(decl);
                quakes.transflows.push(transflow);
            }
        }
    }

    Ok(quakes)
}

fn build_transflow(decl: TransflowDecl) -> QuakeTransflow {
    let mut transflow = QuakeTransflow::default();
    transflow.routes = decl
        .flows
        .iter()
        .map(|flow_decl| {
            let mut route = Route::default();
            match flow_decl {
                TransflowEnum::Midway(way) => {
                    route.to = way.end.clone();
                    route.from = way
                        .from
                        .iter()
                        .map(|param| param.value.clone())
                        .collect::<Vec<String>>()
                }
                TransflowEnum::Endway(way) => {
                    route.to = way.component.clone();
                    route.is_component = true;
                    route.from = way
                        .from
                        .iter()
                        .map(|param| param.value.clone())
                        .collect::<Vec<String>>()
                }
            }
            route
        })
        .collect::<Vec<Route>>();
    transflow
}

#[cfg(test)]
mod tests {
    use crate::parser::quake::QuakeAction;
    use crate::quake::QuakeTransflow;

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
        let expr = QuakeTransflow::from_text(define).unwrap();
        assert_eq!(1, expr.routes.len());
        assert_eq!(true, expr.routes[0].is_component);
        assert_eq!("quake-calendar", expr.routes[0].to);
    }
}
