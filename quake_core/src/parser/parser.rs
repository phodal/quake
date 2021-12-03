use crate::parser::ast::{ActionDecl, Parameter, SourceUnit, SourceUnitPart};
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
                _ => println!("rule: {}", inner_pair),
            };
        }
    }

    Ok(SourceUnit(parts))
}

fn action_decl(decl: Pair<Rule>) -> ActionDecl {
    let mut action = ActionDecl::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::parameters => {
                action.parameters.push(parameters(pair));
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

fn parameters(decl: Pair<Rule>) -> Parameter {
    let mut parameter = Parameter::default();
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::parameter => {
                parameter.value = String::from(pair.as_str());
            }
            Rule::s_quote => {}
            Rule::e_quote => {}
            _ => {
                println!("{}", pair);
            }
        }
    }

    parameter
}

#[cfg(test)]
mod tests {
    use crate::parser::ast::SourceUnitPart;
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
        }
    }

    #[test]
    fn should_parse_com() {
        let unit = parse("phodal_com.sync").unwrap();
        assert_eq!(1, unit.0.len());
    }
}
