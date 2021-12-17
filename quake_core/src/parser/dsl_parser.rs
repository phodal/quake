use crate::parser::ast::{
    ActionDecl, Endway, LayoutComponentNode, Midway, Parameter, SimpleLayoutDecl, SourceUnit,
    SourceUnitPart, TransflowDecl, TransflowEnum,
};
use crate::parser::errors::QuakeParserError;
use pest::iterators::Pair;
use pest::Parser;
use std::error::Error;

#[derive(Parser)]
#[grammar = "parser/quake.pest"]
struct QuakeParser;

/// parse text to SourceUnit
/// convert support:
///   - Action
///   - Transflow
pub fn parse(text: &str) -> Result<SourceUnit, Box<dyn Error>> {
    let pairs = match QuakeParser::parse(Rule::earth, text) {
        Ok(pairs) => pairs,
        Err(e) => {
            let string = format!("{:}", e);
            return Err(Box::new(QuakeParserError::new(&*string)));
        }
    };

    let mut parts = vec![];
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::action_decl => {
                    parts.push(SourceUnitPart::Action(action_decl(inner_pair)));
                }
                Rule::transflow_decl => {
                    parts.push(SourceUnitPart::Transflow(transflow_decl(inner_pair)));
                }
                Rule::layout_decl => {
                    parts.push(SourceUnitPart::SimpleLayout(layout_decl(inner_pair)));
                }
                _ => println!("rule: {}", inner_pair),
            };
        }
    }

    Ok(SourceUnit(parts))
}

fn layout_decl(decl: Pair<Rule>) -> SimpleLayoutDecl {
    let mut layout = SimpleLayoutDecl::default();
    for pair in decl.into_inner() {
        let mut row = vec![];
        match pair.as_rule() {
            Rule::flex_child => {
                row.append(&mut parse_flex_child(pair));
            }
            Rule::ident => {
                layout.name = String::from(pair.as_str());
            }
            _ => {
                println!("{}", pair);
            }
        }

        if !row.is_empty() {
            layout.rows.push(row);
        }
    }

    layout
}

fn parse_flex_child(decl: Pair<Rule>) -> Vec<LayoutComponentNode> {
    let mut components = vec![];
    for pair in decl.into_inner() {
        if let Rule::component_use_decl = pair.as_rule() {
            components.push(component_use_decl(pair));
        }
    }

    components
}

fn component_use_decl(decl: Pair<Rule>) -> LayoutComponentNode {
    let mut component = LayoutComponentNode::default();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::sized_empty_comp => {
                component.is_empty = true;
                component.name = "Empty".to_string();
                for inner in pair.into_inner() {
                    if inner.as_rule() == Rule::digits {
                        component.size = inner.as_str().parse().unwrap();
                    }
                }
            }
            Rule::component_flow => component_flow(&mut component, pair),
            _ => {
                println!("{}", pair);
            }
        }
    }

    component
}

fn component_flow(component: &mut LayoutComponentNode, pair: Pair<Rule>) {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::use_name => {
                component.name = String::from(inner.as_str());
            }
            Rule::call_flow => {
                for flow_pair in inner.into_inner() {
                    match flow_pair.as_rule() {
                        Rule::string => {
                            component.flow = Some(string_from_pair(flow_pair));
                        }
                        Rule::digits => {
                            component.size = flow_pair.as_str().parse().unwrap();
                        }
                        Rule::component_decl => {
                            for name in flow_pair.into_inner() {
                                match name.as_rule() {
                                    Rule::component_name => {
                                        component.is_pure_component = true;
                                        component.flow = Some(String::from(name.as_str()))
                                    }
                                    _ => {
                                        println!("{}", name);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Rule::l_bracket | Rule::r_bracket => {}
            _ => {
                println!("{:}", inner);
            }
        }
    }
}

fn transflow_decl(decl: Pair<Rule>) -> TransflowDecl {
    let mut transflow = TransflowDecl::default();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::transflow_expr => {
                if let Some(flow) = transflow_expr(pair) {
                    transflow.flows.push(flow);
                }
            }
            Rule::ident => {
                transflow.name = String::from(pair.as_str());
            }
            _ => {
                println!("{}", pair);
            }
        }
    }

    transflow
}

fn transflow_expr(decl: Pair<Rule>) -> Option<TransflowEnum> {
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::midway => return Some(TransflowEnum::Midway(midway(pair))),
            Rule::endway => return Some(TransflowEnum::Endway(endway(pair))),
            _ => {
                println!("{}", pair);
            }
        }
    }

    None
}

fn midway(decl: Pair<Rule>) -> Midway {
    let mut midway = Midway::default();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::entry_list => {
                midway.from = parameters(pair);
            }
            Rule::parameter => {
                midway.end = value(pair);
            }
            Rule::from | Rule::to | Rule::l_bracket | Rule::r_bracket => {}
            Rule::filter_expr => {
                for inner in pair.into_inner() {
                    if inner.as_rule() == Rule::string {
                        midway.filter = string_from_pair(inner);
                    }
                }
            }
            _ => {
                println!("{}", pair);
            }
        }
    }
    midway
}

fn endway(decl: Pair<Rule>) -> Endway {
    let mut endway = Endway::default();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::entry_list => {
                endway.from = parameters(pair);
            }
            Rule::component_decl => {
                for name in pair.into_inner() {
                    match name.as_rule() {
                        Rule::component_name => endway.component = String::from(name.as_str()),
                        _ => {
                            println!("{}", name);
                        }
                    }
                }
            }
            Rule::from | Rule::to | Rule::l_bracket | Rule::r_bracket => {}
            Rule::filter_expr => {
                for inner in pair.into_inner() {
                    if inner.as_rule() == Rule::string {
                        endway.filter = string_from_pair(inner);
                    }
                }
            }
            _ => {
                println!("{}", pair);
            }
        }
    }

    endway
}

fn action_decl(decl: Pair<Rule>) -> ActionDecl {
    let mut action = ActionDecl::default();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::parameters => {
                action.parameters = parameters(pair);
            }
            Rule::action => {
                action.action = String::from(pair.as_str());
            }
            Rule::object => {
                action.object = String::from(pair.as_str());
            }
            Rule::text => {
                action.text = String::from(pair.as_str());
            }
            _ => {
                println!("{}", pair);
            }
        }
    }

    action
}

fn parameters(decl: Pair<Rule>) -> Vec<Parameter> {
    let mut params = vec![];
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::parameter => params.push(Parameter { value: value(pair) }),
            Rule::l_bracket => {}
            Rule::r_bracket => {}
            _ => {
                println!("{}", pair);
            }
        }
    }

    params
}

fn value(decl: Pair<Rule>) -> String {
    let mut value: String = "".to_string();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::string => {
                value = string_from_pair(pair);
            }
            _ => {
                value = String::from(pair.as_str());
            }
        }
    }

    value
}

fn string_from_pair(pair: Pair<Rule>) -> String {
    replace_string_markers(pair.as_str())
}

pub fn replace_string_markers(input: &str) -> String {
    match input.chars().next().unwrap() {
        '"' => input.replace('"', ""),
        '\'' => input.replace('\'', ""),
        '`' => input.replace('`', ""),
        _ => unreachable!("error: {:?}", input),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ast::{SourceUnitPart, TransflowEnum};
    use crate::parser::dsl_parser::parse;

    #[test]
    fn should_parse_add_todo() {
        let unit = parse("todo.add: 添加 todo 的支持").unwrap();
        assert_eq!(1, unit.0.len());

        if let SourceUnitPart::Action(action) = &unit.0[0] {
            assert_eq!("add", action.action);
            assert_eq!("todo", action.object);
            assert_eq!("添加 todo 的支持", action.text);
        }
    }

    #[test]
    fn should_parse_update_todo() {
        let unit = parse("todo.update(1)").unwrap();
        assert_eq!(1, unit.0.len());

        if let SourceUnitPart::Action(action) = &unit.0[0] {
            assert_eq!("todo", action.object);
            assert_eq!("update", action.action);
            assert_eq!(1, action.parameters.len());
            assert_eq!("1", action.parameters[0].value);
        }
    }

    #[test]
    fn should_parse_com() {
        let unit = parse("phodal_com.sync").unwrap();
        assert_eq!(1, unit.0.len());
    }

    #[test]
    fn should_parse_flow() {
        let unit =
            parse("transflow show_calendar { from('todo','blog').to(<quake-calendar>); }").unwrap();

        match &unit.0[0] {
            SourceUnitPart::Transflow(decl) => {
                let flow = decl.flows[0].clone();
                match flow {
                    TransflowEnum::Midway(_) => assert!(false),
                    TransflowEnum::Endway(end) => {
                        assert_eq!(2, end.from.len());
                        assert_eq!("todo", end.from[0].value);
                        assert_eq!("blog", end.from[1].value);
                        assert_eq!("quake-calendar", end.component);
                    }
                }
            }
            _ => panic!(),
        }
    }

    #[test]
    fn should_parse_simple_layout() {
        let unit = parse(
            "layout Dashboard {
--------------------------
|      Calendar(flow(\"show_calendar\"), 12x)  |
--------------------------
| Empty(2x) | Timeline(flow(\"show_timeline\"), 8x) | Empty(2x) |
--------------------------
| Graph(<graph-network>, 12x)          |
--------------------------
}",
        )
        .unwrap();

        assert_eq!(1, unit.0.len());
        println!("{:?}", unit);
        if let SourceUnitPart::SimpleLayout(layout) = &unit.0[0] {
            assert_eq!(layout.name, "Dashboard");
            assert_eq!(3, layout.rows.len());

            assert_eq!(1, layout.rows[0].len());
            assert_eq!("Calendar", &layout.rows[0][0].name);
            assert_eq!("show_calendar", layout.rows[0][0].flow.as_ref().unwrap());
            assert_eq!(12, layout.rows[0][0].size);
            assert_eq!(false, layout.rows[0][0].is_empty);

            assert_eq!(3, layout.rows[1].len());
            assert_eq!("Empty", &layout.rows[1][0].name);
            assert_eq!(true, layout.rows[1][0].is_empty);
            assert_eq!(2, layout.rows[1][0].size);
        }
    }

    #[test]
    fn should_parse_rest_uri_source() {
        let define = "transflow show_calendar { from(rest('https://quake.inherd.org')).to(<quake-calendar>); }";
        let unit = parse(define).unwrap();

        println!("{:?}", unit);
    }
}
