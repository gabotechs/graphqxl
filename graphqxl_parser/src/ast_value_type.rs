use crate::ast_value_basic_type::{parse_value_basic_type, ValueBasicType};
use crate::parser::{Rule, RuleError};
use crate::utils::unknown_rule_error;
use crate::{Identifier, OwnedSpan};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Basic(ValueBasicType, OwnedSpan),
    Array(Box<ValueType>, OwnedSpan),
    NonNullable(Box<ValueType>, OwnedSpan),
}

impl ValueType {
    pub fn build(t: ValueBasicType) -> Self {
        Self::Basic(t, OwnedSpan::default())
    }

    pub fn span(&self) -> &OwnedSpan {
        match self {
            ValueType::Basic(_, span) => span,
            ValueType::Array(_, span) => span,
            ValueType::NonNullable(_, span) => span,
        }
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

    pub fn non_nullable(&self) -> Self {
        ValueType::NonNullable(Box::new(self.clone()), self.span().clone())
    }

    pub fn array(&self) -> Self {
        ValueType::Array(Box::new(self.clone()), self.span().clone())
    }

    pub fn retrieve_basic_type(&self) -> &ValueBasicType {
        match self {
            ValueType::Basic(b, _) => b,
            ValueType::Array(a, _) => ValueType::retrieve_basic_type(a),
            ValueType::NonNullable(a, _) => ValueType::retrieve_basic_type(a),
        }
    }

    pub fn replace_basic_type(&mut self, value: ValueType) -> Result<(), Box<RuleError>> {
        if let ValueType::NonNullable(_, _) = value {
            if let ValueType::NonNullable(_, _) = self {
                return Err(value.span().make_error(
                    "cannot use a non-nullable type inside another non-nullable type",
                ));
            }
        }
        match self {
            ValueType::Basic(_, _) => *self = value,
            ValueType::Array(a, _) => ValueType::replace_basic_type(a, value)?,
            ValueType::NonNullable(a, _) => ValueType::replace_basic_type(a, value)?,
        };
        Ok(())
    }
}

pub(crate) fn parse_value_type(pair: Pair<Rule>, file: &str) -> Result<ValueType, Box<RuleError>> {
    let span = OwnedSpan::from(pair.as_span(), file);
    match pair.as_rule() {
        Rule::value_type => parse_value_type(pair.into_inner().next().unwrap(), file),
        Rule::value_basic_type => Ok(ValueType::Basic(parse_value_basic_type(pair, file)?, span)),
        Rule::value_non_nullable => Ok(ValueType::NonNullable(
            Box::new(parse_value_type(pair.into_inner().next().unwrap(), file)?),
            span,
        )),
        Rule::value_array => Ok(ValueType::Array(
            Box::new(parse_value_type(pair.into_inner().next().unwrap(), file)?),
            span,
        )),
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

    fn parse_input(input: &str) -> Result<ValueType, Box<RuleError>> {
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
        parsed
            .replace_basic_type(ValueType::string().array())
            .unwrap();
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
