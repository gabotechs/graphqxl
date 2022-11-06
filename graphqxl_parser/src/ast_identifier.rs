use crate::parser::Rule;
use crate::utils::{unknown_rule_error, OwnedSpan};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Identifier {
    pub id: String,
    pub span: OwnedSpan,
}

impl From<&str> for Identifier {
    fn from(id: &str) -> Self {
        Self {
            id: id.to_string(),
            span: OwnedSpan::default(),
        }
    }
}

pub(crate) fn parse_identifier(
    pair: Pair<Rule>,
    file: &str,
) -> Result<Identifier, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::identifier => Ok(Identifier {
            id: pair.as_str().to_string(),
            span: OwnedSpan::from(pair.as_span(), file),
        }),
        _unknown => Err(unknown_rule_error(pair, "identifier")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<Identifier, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::identifier, parse_identifier)
    }

    #[test]
    fn test_correct() {
        assert_eq!(
            parse_input("Correct1").unwrap(),
            Identifier::from("Correct1")
        );
    }

    #[test]
    fn test_incorrect_1() {
        parse_input("incorr-ect").unwrap_err();
    }

    #[test]
    fn test_incorrect_2() {
        parse_input("1incorrect").unwrap_err();
    }
}
