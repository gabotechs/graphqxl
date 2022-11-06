use pest::iterators::Pair;

use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use crate::{parse_identifier, Identifier, OwnedSpan};

#[derive(Debug, Clone, PartialEq)]
pub struct Generic {
    pub span: OwnedSpan,
    pub args: Vec<Identifier>,
}

impl Generic {
    pub fn from(name: &str) -> Self {
        Generic {
            span: OwnedSpan::default(),
            args: vec![Identifier::from(name)],
        }
    }

    pub fn arg(&mut self, name: &str) -> Self {
        self.args.push(Identifier::from(name));
        self.clone()
    }
}

pub(crate) fn parse_generic(
    pair: Pair<Rule>,
    file: &str,
) -> Result<Generic, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::generic => {
            let span = OwnedSpan::from(pair.as_span(), file);
            let childs = pair.into_inner();
            let mut args = Vec::new();
            for name in childs {
                args.push(parse_identifier(name, file)?);
            }

            Ok(Generic { span, args })
        }
        _unknown => Err(unknown_rule_error(pair, "generic")),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::parse_full_input;

    use super::*;

    fn parse_input(input: &str) -> Result<Generic, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::generic, parse_generic)
    }

    #[test]
    fn test_parses_one_generic() {
        assert_eq!(parse_input("<T>"), Ok(Generic::from("T")))
    }

    #[test]
    fn test_parses_two_generics() {
        assert_eq!(parse_input("<T C>"), Ok(Generic::from("T").arg("C")))
    }

    #[test]
    fn test_parses_two_generics_with_a_comma() {
        assert_eq!(parse_input("<T, C>"), Ok(Generic::from("T").arg("C")))
    }

    #[test]
    fn test_do_not_parse_incorrectly_formed_generic() {
        parse_input("T>").unwrap_err();
    }

    #[test]
    fn test_do_not_parse_incorrectly_formed_generic_2() {
        parse_input("<<T>").unwrap_err();
    }
}
