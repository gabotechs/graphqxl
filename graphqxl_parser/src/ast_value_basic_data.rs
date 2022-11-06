use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueBasicData {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

pub(crate) fn _parse_basic_data(
    pair: Pair<Rule>,
) -> Result<ValueBasicData, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::int_data => Ok(ValueBasicData::Int(pair.as_str().parse::<i64>().unwrap())),
        Rule::float_data => Ok(ValueBasicData::Float(pair.as_str().parse::<f64>().unwrap())),
        Rule::string_data => Ok(ValueBasicData::String({
            let str = pair.as_str();
            str[1..str.len() - 1].to_string()
        })),
        Rule::boolean_data => Ok(ValueBasicData::Boolean(pair.as_str() == "true")),
        _unknown => Err(unknown_rule_error(
            pair,
            "int_data, float_data, string_data, boolean_data",
        )),
    }
}

pub(crate) fn parse_basic_data(
    pair: Pair<Rule>,
    _file: &str,
) -> Result<ValueBasicData, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::basic_data => _parse_basic_data(pair.into_inner().next().unwrap()),
        _unknown => Err(unknown_rule_error(pair, "basic_data")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<ValueBasicData, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::basic_data, parse_basic_data)
    }

    #[test]
    fn test_int() {
        let int = parse_input("1").unwrap();
        assert_eq!(int, ValueBasicData::Int(1));
    }

    #[test]
    fn test_invalid_int() {
        parse_input("1.").unwrap_err();
    }

    #[test]
    fn test_float() {
        let float = parse_input("12.34").unwrap();
        assert_eq!(float, ValueBasicData::Float(12.34))
    }

    #[test]
    fn test_invalid_float() {
        parse_input("12.35a").unwrap_err();
    }

    #[test]
    fn test_string() {
        let string = parse_input("\"this is a string\"").unwrap();
        assert_eq!(
            string,
            ValueBasicData::String("this is a string".to_string())
        )
    }

    #[test]
    fn test_invalid_string() {
        parse_input("\"this is not a string'").unwrap_err();
    }

    #[test]
    fn test_boolean() {
        let bool = parse_input("true").unwrap();
        assert_eq!(bool, ValueBasicData::Boolean(true));
    }

    #[test]
    fn test_invalid_boolean() {
        parse_input("\"true").unwrap_err();
    }
}
