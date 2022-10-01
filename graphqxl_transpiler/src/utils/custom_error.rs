use graphqxl_parser::{OwnedSpan, Rule};

pub(crate) fn custom_error(span: &OwnedSpan, msg: &str) -> pest::error::Error<Rule> {
    let mut err = span.err_placeholder.clone();
    err.variant = pest::error::ErrorVariant::CustomError {
        message: msg.to_string(),
    };
    err
}
