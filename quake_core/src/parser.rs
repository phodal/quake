use pest::Parser;

#[derive(Parser)]
#[grammar = "quake.pest"]
struct QuakeParser;

fn parse(text: &str) {
    let pairs = QuakeParser::parse(Rule::earth, text).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                _ => println!("{}", inner_pair)
            };
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