use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "quake.pest"]
struct QuakeParser;

pub fn parse(text: &str) {
    let pairs = QuakeParser::parse(Rule::earth, text).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::action_decl => {
                    action_decl(inner_pair);
                }
                _ => println!("rule: {}", inner_pair)
            };
        }
    }
}

fn action_decl(decl: Pair<Rule>) {
    for pair in decl.into_inner() {
        match pair.as_rule() {
            Rule::object => {

            }
            _ => {
                println!("rule: {}", pair);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn should_parse_expression() {
        parse("todo.add: 添加 todo 的支持");
    }
}