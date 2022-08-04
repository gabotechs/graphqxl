use crate::ast_value_basic_type::{parse_value_basic_type, ValueBasicType};
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct ValueSimple {
    pub content: ValueBasicType,
    pub nullable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueArray {
    pub value: ValueSimple,
    pub nullable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Simple(ValueSimple),
    Array(ValueArray),
}

fn _parse_value_type(
    pair: Pair<Rule>,
    nullable: bool,
    array: bool,
    array_nullable: bool,
) -> Result<ValueType, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::value_nullable => {
            let inner = pair.into_inner().next().unwrap();
            let content = parse_value_basic_type(inner).unwrap();
            let value = ValueSimple { content, nullable };
            if array {
                Ok(ValueType::Array(ValueArray {
                    value,
                    nullable: array_nullable,
                }))
            } else {
                Ok(ValueType::Simple(value))
            }
        }
        Rule::value_non_nullable => {
            let rule = pair.into_inner().next().unwrap();
            _parse_value_type(rule, false, array, array_nullable)
        }
        Rule::value_array_nullable => {
            let rule = pair.into_inner().next().unwrap();
            _parse_value_type(rule, nullable, true, array_nullable)
        }
        Rule::value_array_non_nullable => {
            let rule = pair.into_inner().next().unwrap();
            _parse_value_type(rule, nullable, true, false)
        }
        _unknown => Err(unknown_rule_error(
            pair,
            "value_nullable, value, value_array_nullable, value_array",
        )),
    }
}

pub(crate) fn parse_value_type(pair: Pair<Rule>) -> Result<ValueType, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::value_type => {
            let rule = pair.into_inner().next().unwrap();
            _parse_value_type(rule, true, false, true)
        }
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
    fn test_simple_nullable() {
        if let ValueType::Simple(val) = parse_input("Int").unwrap() {
            assert_eq!(val.content, ValueBasicType::Int);
            assert!(val.nullable);
        } else {
            panic!("should have been a simple value")
        }
    }

    #[test]
    fn test_simple_non_nullable() {
        if let ValueType::Simple(val) = parse_input("Int!").unwrap() {
            assert_eq!(val.content, ValueBasicType::Int);
            assert!(!val.nullable);
        } else {
            panic!("should have been a simple value")
        }
    }

    #[test]
    fn test_array_nullable() {
        if let ValueType::Array(val) = parse_input("[Int]").unwrap() {
            assert_eq!(val.value.content, ValueBasicType::Int);
            assert!(val.value.nullable);
            assert!(val.nullable);
        } else {
            panic!("should have been an array value")
        }
    }

    #[test]
    fn test_array_non_nullable() {
        if let ValueType::Array(val) = parse_input("[Int]!").unwrap() {
            assert_eq!(val.value.content, ValueBasicType::Int);
            assert!(val.value.nullable);
            assert!(!val.nullable);
        } else {
            panic!("should have been an array value")
        }
    }

    #[test]
    fn test_array_nullable_inner_value_non_nullable() {
        if let ValueType::Array(val) = parse_input("[Int!]").unwrap() {
            assert_eq!(val.value.content, ValueBasicType::Int);
            assert!(!val.value.nullable);
            assert!(val.nullable);
        } else {
            panic!("should have been an array value")
        }
    }

    #[test]
    fn test_array_non_nullable_inner_value_non_nullable() {
        if let ValueType::Array(val) = parse_input("[Int!]!").unwrap() {
            assert_eq!(val.value.content, ValueBasicType::Int);
            assert!(!val.value.nullable);
            assert!(!val.nullable);
        } else {
            panic!("should have been an array value")
        }
    }
}
