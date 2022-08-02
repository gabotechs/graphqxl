use crate::ast_value::{parse_value, Value};
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeFieldArg {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockField {
    pub name: String,
    pub value: Value,
    pub args: Vec<TypeFieldArg>,
}

fn _parse_block_field(pair: Pair<Rule>) -> Result<BlockField, pest::error::Error<Rule>> {
    // at this moment we are on [type_field|input_field], both will work
    let mut pairs = pair.into_inner();
    // at this moment we are on [identifier, args?, value]
    let identifier = pairs.next().unwrap();
    let value_or_args = pairs.next().unwrap();
    let mut value = value_or_args.clone();
    let mut type_field_args = Vec::new();
    if let Rule::type_field_args = value_or_args.as_rule() {
        for type_field_arg in value_or_args.into_inner() {
            // each arg is [type_field_arg]
            let mut identifier_value = type_field_arg.into_inner();
            // at this moment we are on [identifier, value]
            let identifier = identifier_value.next().unwrap().as_str();
            let value = parse_value(identifier_value.next().unwrap())?;
            type_field_args.push(TypeFieldArg {
                name: identifier.to_string(),
                value,
            })
        }
        value = pairs.next().unwrap();
    }
    Ok(BlockField {
        name: identifier.as_str().into(),
        value: parse_value(value)?,
        args: type_field_args,
    })
}

pub(crate) fn parse_block_field(pair: Pair<Rule>) -> Result<BlockField, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::type_field => _parse_block_field(pair),
        Rule::input_field => _parse_block_field(pair),
        _unknown => Err(unknown_rule_error(pair, "field")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_value::{ValueArray, ValueSimple};
    use crate::ast_value_content::ValueContent;
    use crate::utils::parse_full_input;

    fn parse_type_input(input: &str) -> Result<BlockField, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::type_field, parse_block_field)
    }
    fn parse_input_input(input: &str) -> Result<BlockField, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::input_field, parse_block_field)
    }

    #[test]
    fn test_type_accept_args() {
        parse_type_input("field(arg1: Int, arg2: String!): String").unwrap();
    }

    #[test]
    fn test_input_do_not_accept_args() {
        parse_input_input("field(arg1: Int, arg2: String!): String").unwrap_err();
    }

    #[test]
    fn test_parse_string_block_field() {
        let field = parse_type_input("field: String").unwrap();
        assert_eq!(field.name, String::from("field"));
        assert_eq!(
            field.value,
            Value::Simple(ValueSimple {
                nullable: true,
                content: ValueContent::String
            })
        );
    }

    #[test]
    fn test_parse_array_string_block_field() {
        let field = parse_type_input("field: [String!]!").unwrap();
        assert_eq!(field.name, String::from("field"));
        assert_eq!(
            field.value,
            Value::Array(ValueArray {
                value: ValueSimple {
                    nullable: false,
                    content: ValueContent::String
                },
                nullable: false
            })
        );
    }

    #[test]
    fn test_parse_block_field_args_one_arg() {
        let field = parse_type_input("field(arg1: String): String").unwrap();
        assert_eq!(
            field.args,
            vec![TypeFieldArg {
                name: "arg1".to_string(),
                value: Value::Simple(ValueSimple {
                    content: ValueContent::String,
                    nullable: true
                })
            }]
        );
    }

    #[test]
    fn test_parse_block_field_args_two_args() {
        let field = parse_type_input("field(arg1: [String]!, arg2: Float!): String").unwrap();
        assert_eq!(
            field.args,
            vec![
                TypeFieldArg {
                    name: "arg1".to_string(),
                    value: Value::Array(ValueArray {
                        value: ValueSimple {
                            content: ValueContent::String,
                            nullable: true
                        },
                        nullable: false
                    })
                },
                TypeFieldArg {
                    name: "arg2".to_string(),
                    value: Value::Simple(ValueSimple {
                        content: ValueContent::Float,
                        nullable: false
                    })
                }
            ]
        );
    }

    #[test]
    fn test_do_not_parse_invalid() {
        parse_type_input("field: [String!!").unwrap_err();
    }
}
