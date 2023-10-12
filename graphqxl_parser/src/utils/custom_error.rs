use crate::parser::{Rule, RuleError};
use pest::iterators::Pair;

pub(crate) fn custom_error(pair: Pair<Rule>, message: &str) -> Box<RuleError> {
    Box::new(pest::error::Error::new_from_span(
        pest::error::ErrorVariant::CustomError {
            message: message.into(),
        },
        pair.as_span(),
    ))
}
