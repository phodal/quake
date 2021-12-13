use crate::parser::ast::{
    ActionDecl, Endway, LayoutComponent, Midway, Parameter, SimpleLayoutDecl, SourceUnit,
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

        layout.rows.push(row);
    }

    layout
}

fn parse_flex_child(decl: Pair<Rule>) -> Vec<LayoutComponent> {
    let mut components = vec![];
    for pair in decl.into_inner() {
        if let Rule::component_use_decl = pair.as_rule() {
            components.push(component_use_decl(pair));
        }
    }

    components
}

fn component_use_decl(decl: Pair<Rule>) -> LayoutComponent {
    let mut component = LayoutComponent::default();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::sized_empty_comp => {
                component.is_empty = true;
                component.name = "Empty".to_string();
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::digits => {
                            component.size = inner.as_str().parse().unwrap();
                        }
                        _ => {}
                    }
                }
            }
            Rule::component_flow => {}
            _ => {
                println!("{}", pair);
            }
        }
    }

    component
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
            Rule::parameters => {
                midway.from = parameters(pair);
            }
            Rule::parameter => {
                midway.end = value(pair);
            }
            Rule::from | Rule::to | Rule::lbracket | Rule::rbracket => {}
            Rule::filter_expr => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::double_quoted_string | Rule::single_quoted_string => {
                            midway.filter = string_from_pair(inner);
                        }
                        _ => {}
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
            Rule::parameters => {
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
            Rule::from | Rule::to | Rule::lbracket | Rule::rbracket => {}
            Rule::filter_expr => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::double_quoted_string | Rule::single_quoted_string => {
                            endway.filter = string_from_pair(inner);
                        }
                        _ => {}
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
    let mut action = ActionDecl::new();
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
            Rule::parameter => {
                let mut param = Parameter::default();
                param.value = value(pair);

                params.push(param)
            }
            Rule::lbracket => {}
            Rule::rbracket => {}
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
            Rule::double_quoted_string | Rule::single_quoted_string => {
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
    use crate::parser::parser::parse;

    #[test]
    fn should_parse_add_todo() {
        let unit = parse("todo.add: 添加 todo 的支持").unwrap();
        assert_eq!(1, unit.0.len());

        match &unit.0[0] {
            SourceUnitPart::Action(action) => {
                assert_eq!("add", action.action);
                assert_eq!("todo", action.object);
                assert_eq!("添加 todo 的支持", action.text);
            }
            _ => {}
        }
    }

    #[test]
    fn should_parse_update_todo() {
        let unit = parse("todo.update(1)").unwrap();
        assert_eq!(1, unit.0.len());

        match &unit.0[0] {
            SourceUnitPart::Action(action) => {
                assert_eq!("todo", action.object);
                assert_eq!("update", action.action);
                assert_eq!(1, action.parameters.len());
                assert_eq!("1", action.parameters[0].value);
            }
            _ => {}
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
        println!("{:?}", unit);
        match &unit.0[0] {
            SourceUnitPart::Transflow(decl) => {
                let flow = decl.flows[0].clone();
                match flow {
                    TransflowEnum::Midway(_) => {
                        assert!(false);
                    }
                    TransflowEnum::Endway(end) => {
                        assert_eq!(2, end.from.len());
                        assert_eq!("todo", end.from[0].value);
                        assert_eq!("blog", end.from[1].value);
                        assert_eq!("quake-calendar", end.component);
                    }
                }
            }
            _ => {
                assert!(false);
            }
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
}",
        )
        .unwrap();

        assert_eq!(1, unit.0.len());
    }
}
