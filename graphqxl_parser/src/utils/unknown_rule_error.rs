use crate::Rule;
use pest::iterators::Pair;

pub fn unknown_rule_error(pair: Pair<Rule>, expected_str: &str) -> pest::error::Error<Rule> {
    pest::error::Error::new_from_span(
        pest::error::ErrorVariant::CustomError {
            message: "cannot parse as ".to_string() + expected_str,
        },
        pair.as_span(),
    )
}
