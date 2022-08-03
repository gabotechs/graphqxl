use crate::ast_arguments::{parse_arguments, Argument};
use crate::ast_identifier::parse_identifier;
use crate::ast_value::{parse_value, Value};
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockField {
    pub name: String,
    pub value: Option<Value>,
    pub args: Vec<Argument>,
}

fn _parse_block_field(pair: Pair<Rule>) -> Result<BlockField, pest::error::Error<Rule>> {
    // at this moment we are on [type_field|input_field], both will work
    let mut pairs = pair.into_inner();
    // at this moment we are on [identifier, args?, value?]
    let name = parse_identifier(pairs.next().unwrap())?;
    let value_or_args_or_nothing = pairs.next();
    if let Some(value_or_args) = value_or_args_or_nothing {
        let mut value = value_or_args.clone();
        let mut type_field_args = Vec::new();
        if let Rule::arguments = value_or_args.as_rule() {
            type_field_args = parse_arguments(value_or_args)?;
            value = pairs.next().unwrap();
        }
        Ok(BlockField {
            name,
            value: Some(parse_value(value)?),
            args: type_field_args,
        })
    } else {
        Ok(BlockField {
            name,
            value: None,
            args: Vec::new(),
        })
    }
}

pub(crate) fn parse_block_field(pair: Pair<Rule>) -> Result<BlockField, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::field_with_args => _parse_block_field(pair),
        Rule::field_without_args => _parse_block_field(pair),
        Rule::field_without_args_without_value => _parse_block_field(pair),
        _unknown => Err(unknown_rule_error(pair, "field")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_value::{ValueArray, ValueSimple};
    use crate::ast_value_type::ValueType;
    use crate::utils::parse_full_input;

    fn parse_with_args_input(input: &str) -> Result<BlockField, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::field_with_args, parse_block_field)
    }

    fn parse_without_args_input(input: &str) -> Result<BlockField, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::field_without_args, parse_block_field)
    }

    fn parse_without_args_without_value_input(
        input: &str,
    ) -> Result<BlockField, pest::error::Error<Rule>> {
        parse_full_input(
            input,
            Rule::field_without_args_without_value,
            parse_block_field,
        )
    }

    #[test]
    fn test_type_accept_args() {
        parse_with_args_input("field(arg1: Int, arg2: String!): String").unwrap();
    }

    #[test]
    fn test_input_do_not_accept_args() {
        parse_without_args_input("field(arg1: Int, arg2: String!): String").unwrap_err();
    }

    #[test]
    fn test_enum_do_not_accept_args() {
        parse_without_args_without_value_input("field(arg1: Int, arg2: String!): String")
            .unwrap_err();
    }

    #[test]
    fn test_parse_string_block_field() {
        let field = parse_with_args_input("field: String").unwrap();
        assert_eq!(field.name, String::from("field"));
        assert_eq!(field.args.len(), 0);
        assert_eq!(
            field.value,
            Some(Value::Simple(ValueSimple {
                nullable: true,
                content: ValueType::String
            }))
        );
    }

    #[test]
    fn test_parse_array_string_block_field() {
        let field = parse_with_args_input("field: [String!]!").unwrap();
        assert_eq!(field.name, String::from("field"));
        assert_eq!(
            field.value,
            Some(Value::Array(ValueArray {
                value: ValueSimple {
                    nullable: false,
                    content: ValueType::String
                },
                nullable: false
            }))
        );
    }

    #[test]
    fn test_parse_block_field_args_one_arg() {
        let field = parse_with_args_input("field(arg1: String): String").unwrap();
        assert_eq!(
            field.args,
            vec![Argument {
                name: "arg1".to_string(),
                value: Value::Simple(ValueSimple {
                    content: ValueType::String,
                    nullable: true
                })
            }]
        );
    }

    #[test]
    fn test_parse_block_field_args_two_args() {
        let field = parse_with_args_input("field(arg1: [String]!, arg2: Float!): String").unwrap();
        assert_eq!(
            field.args,
            vec![
                Argument {
                    name: "arg1".to_string(),
                    value: Value::Array(ValueArray {
                        value: ValueSimple {
                            content: ValueType::String,
                            nullable: true
                        },
                        nullable: false
                    })
                },
                Argument {
                    name: "arg2".to_string(),
                    value: Value::Simple(ValueSimple {
                        content: ValueType::Float,
                        nullable: false
                    })
                }
            ]
        );
    }

    #[test]
    fn test_do_not_parse_invalid() {
        parse_with_args_input("field: [String!!").unwrap_err();
    }
}
