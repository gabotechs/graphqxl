use crate::ast_value_type::{parse_value_type, ValueType};
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct ValueSimple {
    pub content: ValueType,
    pub nullable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueArray {
    pub value: ValueSimple,
    pub nullable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Simple(ValueSimple),
    Array(ValueArray),
}

fn _parse_value(
    pair: Pair<Rule>,
    nullable: bool,
    array: bool,
    array_nullable: bool,
) -> Result<Value, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::value_nullable => {
            let inner = pair.into_inner().next().unwrap();
            let content = parse_value_type(inner).unwrap();
            let value = ValueSimple { content, nullable };
            if array {
                Ok(Value::Array(ValueArray {
                    value,
                    nullable: array_nullable,
                }))
            } else {
                Ok(Value::Simple(value))
            }
        }
        Rule::value_non_nullable => {
            let rule = pair.into_inner().next().unwrap();
            _parse_value(rule, false, array, array_nullable)
        }
        Rule::value_array_nullable => {
            let rule = pair.into_inner().next().unwrap();
            _parse_value(rule, nullable, true, array_nullable)
        }
        Rule::value_array_non_nullable => {
            let rule = pair.into_inner().next().unwrap();
            _parse_value(rule, nullable, true, false)
        }
        _unknown => Err(unknown_rule_error(
            pair,
            "value_nullable, value, value_array_nullable, value_array",
        )),
    }
}

pub(crate) fn parse_value(pair: Pair<Rule>) -> Result<Value, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::value => {
            let rule = pair.into_inner().next().unwrap();
            _parse_value(rule, true, false, true)
        }
        _unknown => Err(unknown_rule_error(pair, "any_value")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<Value, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::value, parse_value)
    }

    #[test]
    fn test_simple_nullable() {
        if let Value::Simple(val) = parse_input("Int").unwrap() {
            assert_eq!(val.content, ValueType::Int);
            assert!(val.nullable);
        } else {
            panic!("should have been a simple value")
        }
    }

    #[test]
    fn test_simple_non_nullable() {
        if let Value::Simple(val) = parse_input("Int!").unwrap() {
            assert_eq!(val.content, ValueType::Int);
            assert!(!val.nullable);
        } else {
            panic!("should have been a simple value")
        }
    }

    #[test]
    fn test_array_nullable() {
        if let Value::Array(val) = parse_input("[Int]").unwrap() {
            assert_eq!(val.value.content, ValueType::Int);
            assert!(val.value.nullable);
            assert!(val.nullable);
        } else {
            panic!("should have been an array value")
        }
    }

    #[test]
    fn test_array_non_nullable() {
        if let Value::Array(val) = parse_input("[Int]!").unwrap() {
            assert_eq!(val.value.content, ValueType::Int);
            assert!(val.value.nullable);
            assert!(!val.nullable);
        } else {
            panic!("should have been an array value")
        }
    }

    #[test]
    fn test_array_nullable_inner_value_non_nullable() {
        if let Value::Array(val) = parse_input("[Int!]").unwrap() {
            assert_eq!(val.value.content, ValueType::Int);
            assert!(!val.value.nullable);
            assert!(val.nullable);
        } else {
            panic!("should have been an array value")
        }
    }

    #[test]
    fn test_array_non_nullable_inner_value_non_nullable() {
        if let Value::Array(val) = parse_input("[Int!]!").unwrap() {
            assert_eq!(val.value.content, ValueType::Int);
            assert!(!val.value.nullable);
            assert!(!val.nullable);
        } else {
            panic!("should have been an array value")
        }
    }
}
