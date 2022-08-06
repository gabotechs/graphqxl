use crate::parser::Rule;
use pest::iterators::Pair;

pub(crate) fn already_defined_error(
    pair: Pair<Rule>,
    kind: &str,
    name: &str,
) -> pest::error::Error<Rule> {
    pest::error::Error::new_from_span(
        pest::error::ErrorVariant::CustomError {
            message: kind.to_string() + " \"" + name + "\" is already defined",
        },
        pair.as_span(),
    )
}
