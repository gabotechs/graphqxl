use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_identifier::{parse_identifier, Identifier};
use crate::parser::{Rule, RuleError};
use crate::utils::{unknown_rule_error, OwnedSpan};
use crate::{parse_directive, Directive};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Scalar {
    pub extend: bool,
    pub span: OwnedSpan,
    pub name: Identifier,
    pub description: String,
    pub directives: Vec<Directive>,
}

impl Scalar {
    pub fn build(name: &str) -> Self {
        Self {
            name: Identifier::from(name),
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

    pub fn extend(&mut self) -> Self {
        self.extend = true;
        self.clone()
    }
}

fn _parse_scalar(pair: Pair<Rule>, file: &str, extend: bool) -> Result<Scalar, Box<RuleError>> {
            let span = OwnedSpan::from(pair.as_span(), file);
            let mut childs = pair.into_inner();
            let DescriptionAndNext(description, next) =
                parse_description_and_continue(&mut childs, file);
            let name = parse_identifier(next.unwrap(), file)?;
            let mut directives = Vec::new();
            for child in childs {
                directives.push(parse_directive(child, file)?);
            }
            Ok(Scalar {
                extend,
                span,
                name,
                description,
                directives,
            })
}

pub(crate) fn parse_scalar(pair: Pair<Rule>, file: &str) -> Result<Scalar, Box<RuleError>> {
    match pair.as_rule() {
        Rule::scalar_def => _parse_scalar(pair, file, false),
        Rule::scalar_ext => _parse_scalar(pair, file, true),
        _unknown => Err(unknown_rule_error(pair, "scalar_def")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<Scalar, Box<RuleError>> {
        parse_full_input(input, Rule::def, parse_scalar)
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
    fn test_parses_scalar_extension() {
        assert_eq!(
            parse_input("extend scalar MyScalar"),
            Ok(Scalar::build("MyScalar").extend())
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
