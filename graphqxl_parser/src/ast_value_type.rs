use crate::ast_value_basic_type::{parse_value_basic_type, ValueBasicType};
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct ValueTypeSimple {
    pub value_type: ValueBasicType,
    pub nullable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueTypeArray {
    pub value_type: ValueTypeSimple,
    pub nullable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Simple(ValueTypeSimple),
    Array(ValueTypeArray),
}

impl ValueType {
    pub fn build(t: ValueBasicType) -> Self {
        Self::Simple(ValueTypeSimple {
            value_type: t,
            nullable: true,
        })
    }

    pub fn int() -> Self {
        Self::build(ValueBasicType::Int)
    }

    pub fn float() -> Self {
        Self::build(ValueBasicType::Float)
    }

    pub fn string() -> Self {
        Self::build(ValueBasicType::String)
    }

    pub fn boolean() -> Self {
        Self::build(ValueBasicType::Boolean)
    }

    pub fn object(name: &str) -> Self {
        Self::build(ValueBasicType::Object(name.to_string()))
    }

    pub fn non_nullable(&mut self) -> Self {
        match self {
            ValueType::Simple(simple) => simple.nullable = false,
            ValueType::Array(array) => array.nullable = false,
        }
        self.clone()
    }

    pub fn array(&mut self) -> Self {
        if let ValueType::Simple(simple) = self {
            ValueType::Array(ValueTypeArray {
                value_type: simple.clone(),
                nullable: true,
            })
        } else {
            self.clone()
        }
    }
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
            let value = ValueTypeSimple {
                value_type: content,
                nullable,
            };
            if array {
                Ok(ValueType::Array(ValueTypeArray {
                    value_type: value,
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
        assert_eq!(parse_input("Int"), Ok(ValueType::int()))
    }

    #[test]
    fn test_simple_non_nullable() {
        assert_eq!(parse_input("Int!"), Ok(ValueType::int().non_nullable()));
    }

    #[test]
    fn test_array_nullable() {
        assert_eq!(parse_input("[Int]"), Ok(ValueType::int().array()));
    }

    #[test]
    fn test_array_non_nullable() {
        assert_eq!(
            parse_input("[Int]!"),
            Ok(ValueType::int().array().non_nullable())
        );
    }

    #[test]
    fn test_array_nullable_inner_value_non_nullable() {
        assert_eq!(
            parse_input("[Int!]"),
            Ok(ValueType::int().non_nullable().array())
        );
    }

    #[test]
    fn test_array_non_nullable_inner_value_non_nullable() {
        assert_eq!(
            parse_input("[Int!]!"),
            Ok(ValueType::int().non_nullable().array().non_nullable())
        );
    }
}
