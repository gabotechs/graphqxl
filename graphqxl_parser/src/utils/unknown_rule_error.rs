use crate::parser::Rule;
use pest::iterators::Pair;

pub(crate) fn unknown_rule_error(pair: Pair<Rule>, expected_str: &str) -> pest::error::Error<Rule> {
    let rule = pair.as_rule();
    pest::error::Error::new_from_span(
        pest::error::ErrorVariant::CustomError {
            message: format!("cannot parse {:?} as {}", rule, expected_str),
        },
        pair.as_span(),
    )
}
