use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_identifier::parse_identifier;
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use crate::{parse_directive, Directive};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Scalar {
    pub name: String,
    pub description: String,
    pub directives: Vec<Directive>,
}

impl Scalar {
    pub fn build(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn description(&mut self, description: &str) -> Self {
        self.description = description.to_string();
        self.clone()
    }

    pub fn directive(&mut self, directive: Directive) -> Self {
        self.directives.push(directive);
        self.clone()
    }
}

pub(crate) fn parse_scalar(pair: Pair<Rule>) -> Result<Scalar, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::scalar_def => {
            let mut childs = pair.into_inner();
            let DescriptionAndNext(description, next) = parse_description_and_continue(&mut childs);
            let name = parse_identifier(next)?;
            let mut directives = Vec::new();
            for child in childs {
                directives.push(parse_directive(child)?);
            }
            Ok(Scalar {
                name,
                description,
                directives,
            })
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
        assert_eq!(
            parse_input("\"\"\" my description \"\"\" scalar MyScalar"),
            Ok(Scalar::build("MyScalar").description("my description"))
        );
    }

    #[test]
    fn test_parses_scalar() {
        assert_eq!(
            parse_input("scalar MyScalar"),
            Ok(Scalar::build("MyScalar"))
        );
    }

    #[test]
    fn test_accepts_directives() {
        assert_eq!(
            parse_input("scalar MyScalar @dir1 @dir2"),
            Ok(Scalar::build("MyScalar")
                .directive(Directive::build("dir1"))
                .directive(Directive::build("dir2")))
        );
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
