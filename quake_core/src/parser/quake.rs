use std::error::Error;

use crate::parser::ast::{SourceUnitPart, TransflowDecl, TransflowEnum};
use crate::parser::errors::QuakeParserError;
use crate::parser::parser::parse;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuakeIt {
    pub actions: Vec<QuakeActionNode>,
    pub transflows: Vec<QuakeTransflowNode>,
}

impl Default for QuakeIt {
    fn default() -> Self {
        QuakeIt {
            actions: vec![],
            transflows: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuakeTransflowNode {
    pub name: String,
    pub routes: Vec<Route>,
}

impl QuakeTransflowNode {
    /// return first [QuakeTransflowNode] from text
    pub fn from_text(text: &str) -> Result<QuakeTransflowNode, Box<dyn Error>> {
        let it = quake(text)?;
        if it.transflows.is_empty() {
            return Err(Box::new(QuakeParserError::new("not match transflows")));
        }

        Ok(it.transflows[0].clone())
    }
}

impl Default for QuakeTransflowNode {
    fn default() -> Self {
        QuakeTransflowNode {
            name: "".to_string(),
            routes: vec![],
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct Route {
    pub name: String,
    pub from: Vec<String>,
    pub to: String,
    #[serde(skip_serializing)]
    pub is_end_way: bool,
}

impl Default for Route {
    fn default() -> Self {
        Route {
            name: "".to_string(),
            from: vec![],
            to: "".to_string(),
            is_end_way: false,
        }
    }
}

impl Route {
    pub fn naming(&mut self) {
        self.name = format!(
            "from_{:}_to_{:}",
            self.from.join("_"),
            self.to.replace("-", "_")
        );
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuakeActionNode {
    pub object: String,
    pub action: String,
    pub text: String,
    pub parameters: Vec<String>,
}

impl Default for QuakeActionNode {
    fn default() -> Self {
        QuakeActionNode {
            object: "".to_string(),
            action: "".to_string(),
            text: "".to_string(),
            parameters: vec![],
        }
    }
}

impl QuakeActionNode {
    /// QuakeAction will only process one by one in current
    /// so, just return first action
    pub fn action_from_text(text: &str) -> Result<QuakeActionNode, Box<dyn Error>> {
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

/// parse pure text to `QuakeIt` collections which include all
/// - QuakeAction        , the action for handle data in Quake
/// - QuakeTransflowNode , the data transform in Quake
pub fn quake(text: &str) -> Result<QuakeIt, Box<dyn Error>> {
    let mut quakes = QuakeIt::default();
    let unit = parse(text)?;

    for part in unit.0 {
        match part {
            SourceUnitPart::Action(decl) => {
                let mut action = QuakeActionNode::default();

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

fn build_transflow(decl: TransflowDecl) -> QuakeTransflowNode {
    let mut transflow = QuakeTransflowNode::default();
    transflow.name = decl.name;
    transflow.routes = decl
        .flows
        .iter()
        .map(|flow_decl| {
            let mut route = Route::default();
            match flow_decl {
                TransflowEnum::Midway(way) => {
                    route.to = way.end.clone();
                    for param in &way.from {
                        route.from.push(param.value.clone())
                    }

                    route.naming();
                }
                TransflowEnum::Endway(way) => {
                    route.to = way.component.clone();
                    route.is_end_way = true;
                    for param in &way.from {
                        route.from.push(param.value.clone())
                    }

                    route.naming();
                }
            }
            route
        })
        .collect::<Vec<Route>>();
    transflow
}

#[cfg(test)]
mod tests {
    use crate::parser::quake::QuakeActionNode;
    use crate::quake::QuakeTransflowNode;

    #[test]
    fn should_parse_expression() {
        let expr = QuakeActionNode::action_from_text("todo.add: 添加 todo 的支持").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "add");
        assert_eq!(expr.text, "添加 todo 的支持");
    }

    #[test]
    fn should_parse_update_parameter() {
        let expr = QuakeActionNode::action_from_text("todo.update(1)").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "update");
        assert_eq!(expr.parameters[0], "1");

        assert_eq!(1, expr.index_from_parameter());
    }

    #[test]
    fn should_parse_com() {
        let expr = QuakeActionNode::action_from_text("phodal_com.sync").unwrap();
        assert_eq!(expr.object, "phodal_com");
        assert_eq!(expr.action, "sync");
    }

    #[test]
    fn should_parse_double_digital() {
        let expr = QuakeActionNode::action_from_text("todo.update(12)").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "update");
        assert_eq!(expr.parameters[0], "12");
        assert_eq!(12, expr.index_from_parameter());
    }

    #[test]
    fn should_parse_chinese_quote() {
        let expr = QuakeActionNode::action_from_text("todo.update（12）").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "update");
        assert_eq!(expr.parameters[0], "12");
        assert_eq!(12, expr.index_from_parameter());
    }

    #[test]
    fn should_create_transflow() {
        let define = "transflow show_calendar { from('todo','blog').to(<quake-calendar>); }";
        let expr = QuakeTransflowNode::from_text(define).unwrap();
        assert_eq!(1, expr.routes.len());
        assert_eq!(true, expr.routes[0].is_end_way);
        assert_eq!("quake-calendar", expr.routes[0].to);
        assert_eq!("show_calendar", expr.name);
    }

    #[test]
    fn should_create_transflows() {
        let define =
            "transflow show_calendar { from('todo','blog').to('record'), from('record').to(<quake-calendar>); }";
        let expr = QuakeTransflowNode::from_text(define).unwrap();
        assert_eq!(2, expr.routes.len());
        assert_eq!(false, expr.routes[0].is_end_way);
        assert_eq!("record", expr.routes[0].to);

        assert_eq!("record", expr.routes[1].from[0]);
        assert_eq!(true, expr.routes[1].is_end_way);
        assert_eq!("quake-calendar", expr.routes[1].to);
        assert_eq!("show_calendar", expr.name);
    }

    #[test]
    fn should_create_route_func_name() {
        let define = "transflow show_calendar { from('todo','blog').to(<quake-calendar>); }";
        let expr = QuakeTransflowNode::from_text(define).unwrap();
        let route = expr.routes[0].clone();

        assert_eq!("from_todo_blog_to_quake_calendar", route.name);
    }
}
