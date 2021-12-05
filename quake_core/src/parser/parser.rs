use crate::parser::ast::{
    ActionDecl, Endway, Midway, Parameter, SourceUnit, SourceUnitPart, Transflow, TransflowDecl,
};
use crate::parser::errors::QuakeParserError;
use pest::iterators::Pair;
use pest::Parser;
use std::error::Error;

#[derive(Parser)]
#[grammar = "parser/quake.pest"]
struct QuakeParser;

pub fn parse(text: &str) -> Result<SourceUnit, Box<dyn Error>> {
    let pairs = match QuakeParser::parse(Rule::earth, text) {
        Ok(pairs) => pairs,
        Err(e) => {
            let string = format!("{:?}", e);
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
                _ => println!("rule: {}", inner_pair),
            };
        }
    }

    Ok(SourceUnit(parts))
}

fn transflow_decl(decl: Pair<Rule>) -> TransflowDecl {
    let mut action = TransflowDecl::default();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::transflow_expr => {
                if let Some(flow) = transflow_expr(pair) {
                    action.flows.push(flow);
                }
            }
            _ => {
                println!("{}", pair);
            }
        }
    }
    action
}

fn transflow_expr(decl: Pair<Rule>) -> Option<Transflow> {
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::midway => return Some(Transflow::Midway(midway(pair))),
            Rule::endway => return Some(Transflow::Endway(endway(pair))),
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
                midway.end = String::from(pair.as_str());
            }
            Rule::from => {}
            Rule::to => {}
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
            Rule::from => {}
            Rule::to => {}
            Rule::s_quote => {}
            Rule::e_quote => {}
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
            Rule::s_quote => {}
            Rule::e_quote => {}
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
    use crate::parser::ast::{SourceUnitPart, Transflow};
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
        let unit = parse("define { from('todo','blog').to(<quake-calendar>); }").unwrap();
        println!("{:?}", unit);
        match &unit.0[0] {
            SourceUnitPart::Transflow(decl) => {
                let flow = decl.flows[0].clone();
                match flow {
                    Transflow::Midway(_) => {
                        assert!(false);
                    }
                    Transflow::Endway(end) => {
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
}
