use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct EnumField {
    pub(crate) name: String,
}

pub(crate) fn parse_enum_field(pair: Pair<Rule>) -> Result<EnumField, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::field_without_args_without_value => {
            let mut pairs = pair.into_inner();
            let identifier = pairs.next().unwrap().as_str();
            Ok(EnumField {
                name: identifier.to_string(),
            })
        }
        _unknown => Err(unknown_rule_error(pair, "field_without_args_without_value")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<EnumField, pest::error::Error<Rule>> {
        parse_full_input(
            input,
            Rule::field_without_args_without_value,
            parse_enum_field,
        )
    }

    #[test]
    fn test_field_is_parsed() {
        let field = parse_input("field").unwrap();
        assert_eq!(field.name, "field");
    }

    #[test]
    fn test_field_do_not_accept_args() {
        parse_input("field(arg: String!)").unwrap_err();
    }

    #[test]
    fn test_field_do_not_accept_values() {
        parse_input("field: [Float]").unwrap_err();
    }
}
