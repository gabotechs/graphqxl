use crate::parser::{GraphqlParser, Rule};
use pest::error::ErrorVariant;
use pest::iterators::Pair;
use pest::{Parser, Position};

pub fn parse_full_input<R>(
    input: &str,
    rule: Rule,
    parser: fn(Pair<Rule>) -> Result<R, pest::error::Error<Rule>>,
) -> Result<R, pest::error::Error<Rule>> {
    let pair_or_err = GraphqlParser::parse(rule, input);
    if let Err(err) = &pair_or_err {
        eprintln!("{}", err);
    }
    let mut pair = pair_or_err?;
    let parsed = pair.next().unwrap();
    if parsed.as_str().len() < input.len() {
        return Err(pest::error::Error::new_from_pos(
            ErrorVariant::CustomError {
                message: String::from("not everything was parsed"),
            },
            Position::new(input, pair.as_str().len()).unwrap(),
        ));
    }
    let res = parser(parsed);
    if let Err(err) = &res {
        eprintln!("{}", err);
    }
    res
}
