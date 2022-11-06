use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use crate::{parse_identifier, Identifier, OwnedSpan};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Implements {
    pub span: OwnedSpan,
    pub interfaces: Vec<Identifier>,
}

impl Implements {
    pub fn from(name: &str) -> Self {
        Self {
            span: OwnedSpan::default(),
            interfaces: vec![Identifier::from(name)],
        }
    }

    pub fn interface(&mut self, name: &str) -> Self {
        self.interfaces.push(Identifier::from(name));
        self.clone()
    }
}

pub(crate) fn parse_implements(
    pair: Pair<Rule>,
    file: &str,
) -> Result<Implements, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::implements => {
            let span = OwnedSpan::from(pair.as_span(), file);
            let mut interfaces = Vec::new();
            for child in pair.into_inner() {
                interfaces.push(parse_identifier(child, file)?);
            }

            Ok(Implements { span, interfaces })
        }
        _unknown => Err(unknown_rule_error(pair, "implements")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<Implements, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::implements, parse_implements)
    }

    #[test]
    fn test_parses_one_implement() {
        assert_eq!(parse_input("implements One"), Ok(Implements::from("One")))
    }

    #[test]
    fn test_parses_multiple_implements() {
        assert_eq!(
            parse_input("implements One & Two & Three"),
            Ok(Implements::from("One").interface("Two").interface("Three"))
        )
    }

    #[test]
    fn test_do_not_parses_with_incorrect_keyword() {
        parse_input("implemeents One").unwrap_err();
    }

    #[test]
    fn test_do_not_parses_with_no_and_operator() {
        parse_input("implements One Two").unwrap_err();
    }
}
