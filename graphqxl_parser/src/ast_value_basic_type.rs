use crate::parser::{Rule, RuleError};
use crate::utils::unknown_rule_error;
use crate::{Identifier, OwnedSpan};
use pest::iterators::Pair;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum ValueBasicType {
    Int,
    Float,
    Boolean,
    String,
    Object(Identifier),
}

impl Display for ValueBasicType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                ValueBasicType::Int => "Int",
                ValueBasicType::Float => "Float",
                ValueBasicType::Boolean => "Boolean",
                ValueBasicType::String => "String",
                ValueBasicType::Object(id) => &id.id,
            }
        })
    }
}

fn _parse_value_basic_type(pair: Pair<Rule>, file: &str) -> Result<ValueBasicType, Box<RuleError>> {
    match pair.as_rule() {
        Rule::int => Ok(ValueBasicType::Int),
        Rule::float => Ok(ValueBasicType::Float),
        Rule::string => Ok(ValueBasicType::String),
        Rule::boolean => Ok(ValueBasicType::Boolean),
        Rule::object => Ok(ValueBasicType::Object(Identifier {
            id: pair.as_str().to_string(),
            span: OwnedSpan::from(pair.as_span(), file),
        })),
        _unknown => Err(unknown_rule_error(
            pair,
            "int, float, string, boolean or object",
        )),
    }
}

pub(crate) fn parse_value_basic_type(
    pair: Pair<Rule>,
    file: &str,
) -> Result<ValueBasicType, Box<RuleError>> {
    match pair.as_rule() {
        Rule::value_basic_type => _parse_value_basic_type(pair.into_inner().next().unwrap(), file),
        _unknown => Err(unknown_rule_error(pair, "value_type")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<ValueBasicType, Box<RuleError>> {
        parse_full_input(input, Rule::value_basic_type, parse_value_basic_type)
    }

    #[test]
    fn test_int() {
        assert_eq!(parse_input("Int").unwrap(), ValueBasicType::Int);
    }

    #[test]
    fn test_float() {
        assert_eq!(parse_input("Float").unwrap(), ValueBasicType::Float);
    }

    #[test]
    fn test_string() {
        assert_eq!(parse_input("String").unwrap(), ValueBasicType::String);
    }

    #[test]
    fn test_boolean() {
        assert_eq!(parse_input("Boolean").unwrap(), ValueBasicType::Boolean);
    }

    #[test]
    fn test_object_1() {
        assert_eq!(
            parse_input("IntMyType"),
            Ok(ValueBasicType::Object(Identifier::from("IntMyType")))
        );
    }

    #[test]
    fn test_object_2() {
        assert_eq!(
            parse_input("MyType"),
            Ok(ValueBasicType::Object(Identifier::from("MyType")))
        );
    }

    #[test]
    fn test_invalid_1() {
        parse_input("1DoNotStartWithNumber").unwrap_err();
    }

    #[test]
    fn test_invalid_2() {
        parse_input("no-minus-sign").unwrap_err();
    }

    #[test]
    fn test_invalid_3() {
        parse_input("no/slash/sign").unwrap_err();
    }
}
