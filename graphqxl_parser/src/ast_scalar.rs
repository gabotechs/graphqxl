use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_identifier::parse_identifier;
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Scalar {
    pub name: String,
    pub description: String,
}

pub(crate) fn parse_scalar(pair: Pair<Rule>) -> Result<Scalar, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::scalar_def => {
            let DescriptionAndNext(description, next) =
                parse_description_and_continue(&mut pair.into_inner());
            let name = parse_identifier(next)?;
            Ok(Scalar { name, description })
        }
        _unknown => Err(unknown_rule_error(pair, "scalar_def")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<Scalar, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::scalar_def, parse_scalar)
    }

    #[test]
    fn test_accepts_description() {
        let scalar = parse_input("\"\"\" my description \"\"\" scalar MyScalar").unwrap();
        assert_eq!(scalar.description, "my description")
    }

    #[test]
    fn test_parses_scalar() {
        let scalar = parse_input("scalar MyScalar").unwrap();
        assert_eq!(scalar.name, "MyScalar")
    }

    #[test]
    fn test_invalid_input_scaalar() {
        parse_input("scaalar MyScalar").unwrap_err();
    }

    #[test]
    fn test_invalid_input_bad_identifier() {
        parse_input("scalar 1MyScalar").unwrap_err();
    }

    #[test]
    fn test_invalid_input_no_arguments() {
        parse_input("scalar MyScalar(arg1: String)").unwrap_err();
    }
}
