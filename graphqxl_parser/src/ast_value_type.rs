use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Int,
    Float,
    Boolean,
    String,
    Object(String),
}

fn _parse_value_type(pair: Pair<Rule>) -> Result<ValueType, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::int => Ok(ValueType::Int),
        Rule::float => Ok(ValueType::Float),
        Rule::string => Ok(ValueType::String),
        Rule::boolean => Ok(ValueType::Boolean),
        Rule::object => Ok(ValueType::Object(String::from(pair.as_str()))),
        _unknown => Err(unknown_rule_error(
            pair,
            "int, float, string, boolean or object",
        )),
    }
}

pub(crate) fn parse_value_type(pair: Pair<Rule>) -> Result<ValueType, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::value_type => _parse_value_type(pair.into_inner().next().unwrap()),
        _unknown => Err(unknown_rule_error(pair, "value_type")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<ValueType, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::value_type, parse_value_type)
    }

    #[test]
    fn test_int() {
        assert_eq!(parse_input("Int").unwrap(), ValueType::Int);
    }

    #[test]
    fn test_float() {
        assert_eq!(parse_input("Float").unwrap(), ValueType::Float);
    }

    #[test]
    fn test_string() {
        assert_eq!(parse_input("String").unwrap(), ValueType::String);
    }

    #[test]
    fn test_boolean() {
        assert_eq!(parse_input("Boolean").unwrap(), ValueType::Boolean);
    }

    #[test]
    fn test_object_1() {
        if let ValueType::Object(val) = parse_input("IntMyType").unwrap() {
            assert_eq!(val, "IntMyType")
        } else {
            panic!("not an object")
        }
    }

    #[test]
    fn test_object_2() {
        if let ValueType::Object(val) = parse_input("MyType").unwrap() {
            assert_eq!(val, "MyType")
        } else {
            panic!("not an object")
        }
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
