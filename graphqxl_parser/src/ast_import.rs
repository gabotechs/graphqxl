use crate::parser::{Rule, RuleError};
use crate::utils::unknown_rule_error;
use crate::OwnedSpan;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Import {
    pub file_name: String,
    pub span: OwnedSpan,
}

impl From<&str> for Import {
    fn from(file_name: &str) -> Self {
        Self {
            file_name: file_name.to_string(),
            span: OwnedSpan::default(),
        }
    }
}

pub(crate) fn parse_import(pair: Pair<Rule>, file: &str) -> Result<Import, Box<RuleError>> {
    match pair.as_rule() {
        Rule::import => {
            let span = OwnedSpan::from(pair.as_span(), file);
            let rule = pair.into_inner().next().unwrap();

            Ok(Import {
                file_name: rule.as_str().trim_matches('\"').to_string(),
                span,
            })
        }
        _ => Err(unknown_rule_error(pair, "import")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<Import, Box<RuleError>> {
        parse_full_input(input, Rule::import, parse_import)
    }

    #[test]
    fn test_parses_import() {
        assert_eq!(
            parse_input("import \"my_file\"").unwrap(),
            Import::from("my_file")
        )
    }

    #[test]
    fn test_does_not_parse_invalid_import() {
        parse_input("import my_file").unwrap_err();
    }
}
