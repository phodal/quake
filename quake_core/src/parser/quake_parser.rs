use pest::iterators::Pair;
use pest::Parser;
use crate::parser::ast::{ActionDecl, SourceUnit, SourceUnitPart};

#[derive(Parser)]
#[grammar = "parser/quake.pest"]
struct QuakeParser;

pub fn parse(text: &str) -> SourceUnit {
    let pairs = QuakeParser::parse(Rule::earth, text).unwrap_or_else(|e| panic!("{}", e));

    let mut parts = vec![];
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::action_decl => {
                    let decl = action_decl(inner_pair);
                    parts.push(SourceUnitPart::Action(decl));
                }
                _ => println!("rule: {}", inner_pair)
            };
        }
    }

    SourceUnit(parts)
}

fn action_decl(decl: Pair<Rule>) -> ActionDecl {
    let mut action = ActionDecl::new();
    for pair in decl.into_inner() {
        match pair.as_rule() {
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
                println!("rule: {}", pair);
            }
        }
    }

    action
}


#[cfg(test)]
mod tests {
    use crate::parser::ast::SourceUnitPart;
    use crate::parser::quake_parser::parse;

    #[test]
    fn should_parse_expression() {
        let unit = parse("todo.add: 添加 todo 的支持");
        assert_eq!(1, unit.0.len());

        match &unit.0[0] {
            SourceUnitPart::Action(action) => {
                assert_eq!("add", action.action);
                assert_eq!("todo", action.object);
                assert_eq!("添加 todo 的支持", action.text);
            }
        }

    }
}