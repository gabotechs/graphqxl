use crate::ast_value_basic_type::{parse_value_basic_type, ValueBasicType};
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use crate::Identifier;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Basic(ValueBasicType),
    Array(Box<ValueType>),
    NonNullable(Box<ValueType>),
}

impl ValueType {
    pub fn build(t: ValueBasicType) -> Self {
        Self::Basic(t)
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

    pub fn object(identifier: Identifier) -> Self {
        Self::build(ValueBasicType::Object(identifier))
    }

    pub fn non_nullable(&mut self) -> Self {
        ValueType::NonNullable(Box::new(self.clone()))
    }

    pub fn array(&mut self) -> Self {
        ValueType::Array(Box::new(self.clone()))
    }

    pub fn retrieve_basic_type(&self) -> &ValueBasicType {
        match self {
            ValueType::Basic(b) => b,
            ValueType::Array(a) => ValueType::retrieve_basic_type(a),
            ValueType::NonNullable(a) => ValueType::retrieve_basic_type(a),
        }
    }

    pub fn replace_basic_type(&mut self, value: ValueType) {
        match self {
            ValueType::Basic(_) => *self = value,
            ValueType::Array(a) => ValueType::replace_basic_type(a, value),
            ValueType::NonNullable(a) => ValueType::replace_basic_type(a, value),
        }
    }
}

pub(crate) fn parse_value_type(
    pair: Pair<Rule>,
    file: &str,
) -> Result<ValueType, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::value_type => parse_value_type(pair.into_inner().next().unwrap(), file),
        Rule::value_basic_type => Ok(ValueType::Basic(parse_value_basic_type(pair, file)?)),
        Rule::value_non_nullable => Ok(ValueType::NonNullable(Box::new(parse_value_type(
            pair.into_inner().next().unwrap(),
            file,
        )?))),
        Rule::value_array => Ok(ValueType::Array(Box::new(parse_value_type(
            pair.into_inner().next().unwrap(),
            file,
        )?))),
        _unknown => Err(unknown_rule_error(
            pair,
            "value_type, value_array, value_non_nullable or value_basic_type",
        )),
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

    #[test]
    fn test_replaces_value() {
        let mut parsed = parse_input("[Int!]!").unwrap();
        parsed.replace_basic_type(ValueType::string().array());
        assert_eq!(
            parsed,
            ValueType::string()
                .array()
                .non_nullable()
                .array()
                .non_nullable()
        );
    }

    #[test]
    fn test_parses_super_nested_array() {
        assert_eq!(
            parse_input("[[[[Int]]]]"),
            Ok(ValueType::int().array().array().array().array())
        )
    }

    #[test]
    fn test_parses_super_nested_array_with_non_nullables() {
        assert_eq!(
            parse_input("[[[[Int!]]!]]!"),
            Ok(ValueType::int()
                .non_nullable()
                .array()
                .array()
                .non_nullable()
                .array()
                .array()
                .non_nullable())
        )
    }

    #[test]
    fn test_not_parses_double_nullable() {
        parse_input("[Int!!]").unwrap_err();
    }
}
