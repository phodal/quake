use std::error::Error;
use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;
use regex::Regex;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_yaml::{Sequence, Value};

use crate::helper::quake_time::string_date_to_unix;
use crate::parser::ast::{
    MapDecl, ParameterType, SimpleLayoutDecl, SourceUnitPart, TransflowDecl, TransflowEnum,
    TransflowSource,
};
use crate::parser::errors::QuakeParserError;
use crate::parser::quake_parser::parse;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QuakeIt {
    pub actions: Vec<QuakeActionNode>,
    pub transflows: Vec<QuakeTransflowNode>,
    pub simple_layout: Vec<SimpleLayout>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

    pub fn new(name: String) -> QuakeTransflowNode {
        Self {
            name,
            routes: vec![],
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct RouteSource {
    pub url: String,
    /// GET, POST
    pub method: String,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub enum RouteTarget {
    Empty,
    Component(String),
    Temp(String),
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone, Default)]
pub struct Route {
    pub name: String,
    pub from: Vec<String>,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<Vec<MapStream>>,
    #[serde(skip_serializing)]
    pub is_end_way: bool,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct MapStream {
    pub source_type: String,
    pub source_prop: String,
    pub target_prop: String,
    pub operators: Vec<MapOperator>,
}

impl MapStream {
    pub fn new(source_prop: &str, target_prop: &str, source_type: &str) -> MapStream {
        Self {
            source_type: source_type.to_string(),
            source_prop: source_prop.to_string(),
            target_prop: target_prop.to_string(),
            operators: vec![],
        }
    }
}

impl Display for MapStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for operator in &self.operators {
            str.push_str(" | ");
            str.push_str(operator.operator.as_str());
            if !operator.params.is_empty() {
                str.push('(');
                let mut vec = vec![];
                for param in &operator.params {
                    match param {
                        ParamType::String(str) => {
                            vec.push(format!("{:?}", str));
                        }
                        ParamType::Number(num) => {
                            vec.push(num.to_string());
                        }
                    }
                }
                String::push_str(&mut str, vec.join(",").to_string().as_str());
                str.push(')');
            }
        }

        write!(f, "{:} -> {:}{:}", self.source_prop, self.target_prop, str)
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct MapOperator {
    pub operator: String,
    pub params: Vec<ParamType>,
}

impl Serialize for MapOperator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.params.len() + 1))?;
        map.serialize_entry("operator", &self.operator)?;

        let mut vec: Sequence = vec![];
        for param in &self.params {
            match param {
                ParamType::String(str) => {
                    vec.push(Value::from(str.to_string()));
                }
                ParamType::Number(num) => {
                    vec.push(Value::from(format!("{:}", num)));
                }
            }
        }
        map.serialize_entry("params", &vec)?;

        map.end()
    }
}

impl MapOperator {
    pub fn params_stringify(&self) -> Vec<String> {
        let mut vec = vec![];
        for param in &self.params {
            match param {
                ParamType::String(str) => {
                    vec.push(format!("{:?}", str));
                }
                ParamType::Number(num) => {
                    vec.push(format!("{:}", num));
                }
            }
        }

        vec
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum ParamType {
    String(String),
    Number(usize),
}

lazy_static! {
    static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
}

impl<'de> Deserialize<'de> for ParamType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        if NUMBER.is_match(&s) {
            let num: usize = s.parse::<usize>().unwrap();
            Ok(ParamType::Number(num))
        } else {
            Ok(ParamType::String(s))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct QuakeActionNode {
    pub object: String,
    pub action: String,
    pub text: String,
    pub parameters: Vec<String>,
}

impl QuakeActionNode {
    /// QuakeAction will only process one by one in current
    /// so, just return first action
    pub fn from_text(text: &str) -> Result<QuakeActionNode, Box<dyn Error>> {
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SimpleLayout {
    pub name: String,
    pub rows: Vec<LayoutRow>,
}

impl SimpleLayout {
    pub fn new(name: String) -> Self {
        Self { name, rows: vec![] }
    }
    pub fn from_text(text: &str) -> Result<SimpleLayout, Box<dyn Error>> {
        let it = quake(text)?;
        if it.simple_layout.is_empty() {
            return Err(Box::new(QuakeParserError::new("not match action")));
        }

        Ok(it.simple_layout[0].clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct LayoutRow {
    pub columns: Vec<LayoutComponent>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct LayoutComponent {
    pub(crate) name: String,
    pub(crate) is_empty: bool,
    pub(crate) flow: String,
    pub(crate) size: i32,
    pub is_pure_component: bool,
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
                action.parameters = param_types_to_string_vec(&decl.parameters);

                quakes.actions.push(action);
            }
            SourceUnitPart::Transflow(decl) => {
                let transflow = build_transflow(decl);
                quakes.transflows.push(transflow);
            }
            SourceUnitPart::SimpleLayout(decl) => {
                let layout = build_simple_layout(decl);
                quakes.simple_layout.push(layout);
            }
        }
    }

    Ok(quakes)
}

fn build_simple_layout(decl: SimpleLayoutDecl) -> SimpleLayout {
    let mut layout = SimpleLayout::new(decl.name);

    for column_node in decl.rows {
        let mut row = LayoutRow::default();
        for node in column_node {
            row.columns.push(LayoutComponent {
                name: node.name.to_string(),
                is_empty: node.is_empty,
                is_pure_component: node.is_pure_component,
                flow: node.flow.unwrap_or_else(|| "".to_string()),
                size: node.size,
            });
        }

        layout.rows.push(row);
    }

    layout
}

fn build_transflow(decl: TransflowDecl) -> QuakeTransflowNode {
    let mut transflow = QuakeTransflowNode::new(decl.name);
    transflow.routes = decl
        .flows
        .iter()
        .map(|flow_decl| {
            let mut route = Route::default();
            match flow_decl {
                TransflowEnum::Midway(way) => {
                    route.to = way.end.clone();
                    match &way.from {
                        TransflowSource::EntryTypes(params) => {
                            route.from = param_types_to_string_vec(params);
                        }
                        TransflowSource::RestUrl(_) => {}
                        _ => {}
                    }

                    route.filter = replace_rule(&way.filter);
                    if way.map.is_some() {
                        route.map = Some(streams_from_ast(way.map.as_ref().unwrap()));
                    }

                    route.naming();
                }
                TransflowEnum::Endway(way) => {
                    route.to = way.component.clone();
                    route.is_end_way = true;
                    match &way.from {
                        TransflowSource::EntryTypes(params) => {
                            route.from = param_types_to_string_vec(params);
                        }
                        TransflowSource::RestUrl(_) => {}
                        _ => {}
                    }

                    if way.map.is_some() {
                        route.map = Some(streams_from_ast(way.map.as_ref().unwrap()));
                    }

                    route.filter = replace_rule(&way.filter);

                    route.naming();
                }
            }
            route
        })
        .collect::<Vec<Route>>();

    transflow
}

fn param_types_to_string_vec(params: &[ParameterType]) -> Vec<String> {
    let mut from = vec![];
    for param in params {
        match param {
            ParameterType::String(str) => {
                from.push(str.clone());
            }
            ParameterType::Number(num) => {
                from.push(num.to_string());
            }
        }
    }
    from
}

fn streams_from_ast(map_decl: &MapDecl) -> Vec<MapStream> {
    let mut streams = vec![];
    for stream in &map_decl.streams {
        let mut map_stream = MapStream::new(
            &stream.source_prop,
            &stream.target_prop,
            &stream.source_type,
        );

        for pipe in &stream.pipes {
            let mut operator = MapOperator::default();
            operator.operator = pipe.operator.clone();
            for param in &pipe.params {
                match param {
                    ParameterType::String(string) => {
                        operator.params.push(ParamType::String(string.clone()));
                    }
                    ParameterType::Number(number) => {
                        operator.params.push(ParamType::Number(*number));
                    }
                }
            }
            map_stream.operators.push(operator);
        }
        streams.push(map_stream);
    }

    streams
}

fn replace_rule(filter: &Option<String>) -> Option<String> {
    filter.as_ref().map(|str| string_date_to_unix(str))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::parser::quake::QuakeActionNode;
    use crate::quake::{MapOperator, MapStream, ParamType, QuakeTransflowNode, SimpleLayout};
    use crate::transflow::Transflow;

    #[test]
    fn should_parse_expression() {
        let expr = QuakeActionNode::from_text("todo.add: 添加 todo 的支持").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "add");
        assert_eq!(expr.text, "添加 todo 的支持");
    }

    #[test]
    fn should_parse_update_parameter() {
        let expr = QuakeActionNode::from_text("todo.update(1)").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "update");
        assert_eq!(expr.parameters[0], "1");

        assert_eq!(1, expr.index_from_parameter());
    }

    #[test]
    fn should_parse_com() {
        let expr = QuakeActionNode::from_text("phodal_com.sync").unwrap();
        assert_eq!(expr.object, "phodal_com");
        assert_eq!(expr.action, "sync");
    }

    #[test]
    fn should_parse_double_digital() {
        let expr = QuakeActionNode::from_text("todo.update(12)").unwrap();
        assert_eq!(expr.object, "todo");
        assert_eq!(expr.action, "update");
        assert_eq!(expr.parameters[0], "12");
        assert_eq!(12, expr.index_from_parameter());
    }

    #[test]
    fn should_parse_chinese_quote() {
        let expr = QuakeActionNode::from_text("todo.update（12）").unwrap();
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
        assert!(expr.routes[0].is_end_way);
        assert_eq!("quake-calendar", expr.routes[0].to);
        assert_eq!("show_calendar", expr.name);
    }

    #[test]
    fn should_create_transflows() {
        let define =
            "transflow show_calendar { from('todo','blog').to('record'), from('record').to(<quake-calendar>); }";
        let expr = QuakeTransflowNode::from_text(define).unwrap();
        assert_eq!(2, expr.routes.len());
        assert!(!expr.routes[0].is_end_way);
        assert_eq!("record", expr.routes[0].to);

        assert_eq!("record", expr.routes[1].from[0]);
        assert!(expr.routes[1].is_end_way);
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

    #[test]
    fn should_parse_filter() {
        let define = "transflow show_calendar {
         from('todo','blog').to(<quake-calendar>).filter('created_date > 2021.01.01 AND created_date < 2021.12.31') 
}";
        let expr = QuakeTransflowNode::from_text(define).unwrap();
        assert_eq!(
            expr.routes[0].filter.as_ref().unwrap(),
            "created_date > 1609459200 AND created_date < 1640908800"
        );
    }

    #[test]
    fn should_parse_filter_map() {
        let define = "transflow show_calendar {
         from('todo','blog').to(<quake-calendar>)
           .filter('created_date > 2021.01.01 AND created_date < 2021.12.31')
           .map('blog.content => content | substring(1, 150)'); 
}";

        let expr = QuakeTransflowNode::from_text(define).unwrap();
        let map_stream = expr.routes[0].map.as_ref().unwrap();

        assert_eq!(
            format!("{:}", map_stream[0]),
            "blog.content -> content | substring(1,150)"
        );
        assert_eq!(
            map_stream[0],
            MapStream {
                source_type: "blog".to_string(),
                source_prop: "blog.content".to_string(),
                target_prop: "content".to_string(),
                operators: vec![MapOperator {
                    operator: "substring".to_string(),
                    params: vec![ParamType::Number(1), ParamType::Number(150)]
                }]
            }
        )
    }

    #[test]
    fn should_parse_layout() {
        let define = "layout Dashboard {
--------------------------
|      Calendar(flow(\"show_calendar\"), 12x)  |
--------------------------
| Empty(2x) | Timeline(flow(\"show_timeline\"), 8x) | Empty(2x) |
--------------------------
}";
        let layout = SimpleLayout::from_text(define).unwrap();
        let str = format!("{:?}", layout);
        assert_eq!(str, "SimpleLayout { name: \"Dashboard\", rows: [LayoutRow { columns: [LayoutComponent { name: \"Calendar\", is_empty: false, flow: \"show_calendar\", size: 12, is_pure_component: false }] }, LayoutRow { columns: [LayoutComponent { name: \"Empty\", is_empty: true, flow: \"\", size: 2, is_pure_component: false }, LayoutComponent { name: \"Timeline\", is_empty: false, flow: \"show_timeline\", size: 8, is_pure_component: false }, LayoutComponent { name: \"Empty\", is_empty: true, flow: \"\", size: 2, is_pure_component: false }] }] }");
    }

    #[test]
    fn deserializer_yaml() {
        let fixtures = PathBuf::from("../").join("_fixtures");
        let path = fixtures.join("transflows").join("transflows.yaml");

        let string = fs::read_to_string(path).unwrap();
        let flows: Vec<Transflow> = serde_yaml::from_str(&*string).unwrap();

        println!("{:?}", flows);
    }
}
