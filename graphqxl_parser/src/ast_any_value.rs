use crate::ast_value_content::{parse_value_content, ValueContent};
use crate::utils::unknown_rule_error;
use crate::Rule;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub content: ValueContent,
    pub nullable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueArray {
    pub value: Value,
    pub nullable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnyValue {
    Simple(Value),
    Array(ValueArray),
}

fn parse_value(
    pair: Pair<Rule>,
    nullable: bool,
    array: bool,
    array_nullable: bool,
) -> Result<AnyValue, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::value_nullable => {
            let inner = pair.into_inner().next().unwrap();
            let content = parse_value_content(inner).unwrap();
            let value = Value { content, nullable };
            if array {
                Ok(AnyValue::Array(ValueArray {
                    value,
                    nullable: array_nullable,
                }))
            } else {
                Ok(AnyValue::Simple(value))
            }
        }
        Rule::value => {
            let rule = pair.into_inner().next().unwrap();
            parse_value(rule, false, array, array_nullable)
        }
        Rule::value_array_nullable => {
            let rule = pair.into_inner().next().unwrap();
            parse_value(rule, nullable, true, array_nullable)
        }
        Rule::value_array => {
            let rule = pair.into_inner().next().unwrap();
            parse_value(rule, nullable, true, false)
        }
        _unknown => Err(unknown_rule_error(
            pair,
            "value_nullable, value, value_array_nullable, value_array",
        )),
    }
}

pub(crate) fn parse_any_value(pair: Pair<Rule>) -> Result<AnyValue, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::any_value => {
            let rule = pair.into_inner().next().unwrap();
            parse_value(rule, true, false, true)
        }
        _unknown => Err(unknown_rule_error(pair, "any_value")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<AnyValue, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::any_value, parse_any_value)
    }

    #[test]
    fn test_simple_nullable() {
        if let AnyValue::Simple(val) = parse_input("Int").unwrap() {
            assert_eq!(val.content, ValueContent::Int);
            assert_eq!(val.nullable, true);
        } else {
            panic!("should have been a simple value")
        }
    }

    #[test]
    fn test_simple_non_nullable() {
        if let AnyValue::Simple(val) = parse_input("Int!").unwrap() {
            assert_eq!(val.content, ValueContent::Int);
            assert_eq!(val.nullable, false);
        } else {
            panic!("should have been a simple value")
        }
    }

    #[test]
    fn test_array_nullable() {
        if let AnyValue::Array(val) = parse_input("[Int]").unwrap() {
            assert_eq!(val.value.content, ValueContent::Int);
            assert_eq!(val.value.nullable, true);
            assert_eq!(val.nullable, true);
        } else {
            panic!("should have been an array value")
        }
    }

    #[test]
    fn test_array_non_nullable() {
        if let AnyValue::Array(val) = parse_input("[Int]!").unwrap() {
            assert_eq!(val.value.content, ValueContent::Int);
            assert_eq!(val.value.nullable, true);
            assert_eq!(val.nullable, false);
        } else {
            panic!("should have been an array value")
        }
    }

    #[test]
    fn test_array_nullable_inner_value_non_nullable() {
        if let AnyValue::Array(val) = parse_input("[Int!]").unwrap() {
            assert_eq!(val.value.content, ValueContent::Int);
            assert_eq!(val.value.nullable, false);
            assert_eq!(val.nullable, true);
        } else {
            panic!("should have been an array value")
        }
    }

    #[test]
    fn test_array_non_nullable_inner_value_non_nullable() {
        if let AnyValue::Array(val) = parse_input("[Int!]!").unwrap() {
            assert_eq!(val.value.content, ValueContent::Int);
            assert_eq!(val.value.nullable, false);
            assert_eq!(val.nullable, false);
        } else {
            panic!("should have been an array value")
        }
    }
}
