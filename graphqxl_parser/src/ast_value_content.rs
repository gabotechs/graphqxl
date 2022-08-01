use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueContent {
    Int,
    Float,
    Boolean,
    String,
    Object(String),
}

fn _parse_value_content(pair: Pair<Rule>) -> Result<ValueContent, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::int => Ok(ValueContent::Int),
        Rule::float => Ok(ValueContent::Float),
        Rule::string => Ok(ValueContent::String),
        Rule::boolean => Ok(ValueContent::Boolean),
        Rule::object => Ok(ValueContent::Object(String::from(pair.as_str()))),
        _unknown => Err(unknown_rule_error(
            pair,
            "int, float, string, boolean or object",
        )),
    }
}

pub(crate) fn parse_value_content(
    pair: Pair<Rule>,
) -> Result<ValueContent, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::value_content => _parse_value_content(pair.into_inner().next().unwrap()),
        _unknown => Err(unknown_rule_error(pair, "value_content")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<ValueContent, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::value_content, parse_value_content)
    }

    #[test]
    fn test_int() {
        assert_eq!(parse_input("Int").unwrap(), ValueContent::Int);
    }

    #[test]
    fn test_float() {
        assert_eq!(parse_input("Float").unwrap(), ValueContent::Float);
    }

    #[test]
    fn test_string() {
        assert_eq!(parse_input("String").unwrap(), ValueContent::String);
    }

    #[test]
    fn test_boolean() {
        assert_eq!(parse_input("Boolean").unwrap(), ValueContent::Boolean);
    }

    #[test]
    fn test_object_1() {
        if let ValueContent::Object(val) = parse_input("IntMyType").unwrap() {
            assert_eq!(val, "IntMyType")
        } else {
            panic!("not an object")
        }
    }

    #[test]
    fn test_object_2() {
        if let ValueContent::Object(val) = parse_input("MyType").unwrap() {
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
