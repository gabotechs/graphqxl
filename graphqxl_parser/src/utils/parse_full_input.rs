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
    let input = input.trim_end();
    let mut pair = pair_or_err?;
    let parsed = pair.next().unwrap();
    let _parsed_str = parsed.as_str();
    let parsed_len = parsed.as_str().len();
    let input_len = input.len();
    if parsed_len < input_len {
        let err = pest::error::Error::new_from_pos(
            ErrorVariant::CustomError {
                message: "not everything was parsed: ".to_string() + &input[parsed_len..],
            },
            Position::new(input, pair.as_str().len()).unwrap(),
        );
        eprintln!("{}", err);
        return Err(err);
    }
    let res = parser(parsed);
    if let Err(err) = &res {
        eprintln!("{}", err);
    }
    res
}
