use graphqxl_parser::{OwnedSpan, Rule};
use pest::Span;

pub(crate) fn custom_error(span: &OwnedSpan, msg: &str) -> pest::error::Error<Rule> {
    pest::error::Error::new_from_span(
        pest::error::ErrorVariant::CustomError {
            message: msg.to_string(),
        },
        Span::from(span),
    )
}
