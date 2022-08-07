use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_identifier::parse_identifier;
use crate::ast_value_data::{parse_value_data, ValueData};
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use crate::{parse_value_type, ValueType};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: String,
    pub description: String,
    pub value: ValueType,
    pub default: Option<ValueData>,
}

fn parse_argument(pair: Pair<Rule>) -> Result<Argument, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::argument => {
            // at this moment we are on [argument]
            let mut childs = pair.into_inner();
            let DescriptionAndNext(description, next) = parse_description_and_continue(&mut childs);
            // at this moment we are on [identifier, value]
            let name = parse_identifier(next)?;
            let value = parse_value_type(childs.next().unwrap())?;
            let mut default = None;
            let maybe_default = childs.next();
            if let Some(pair) = maybe_default {
                if let Rule::value_data = pair.as_rule() {
                    default = Some(parse_value_data(pair)?)
                }
            }
            Ok(Argument {
                name,
                description,
                value,
                default,
            })
        }
        _unknown => Err(unknown_rule_error(pair, "argument")),
    }
}

pub(crate) fn parse_arguments(pair: Pair<Rule>) -> Result<Vec<Argument>, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::arguments => {
            let mut arguments = Vec::new();
            for argument in pair.into_inner() {
                arguments.push(parse_argument(argument)?);
            }
            Ok(arguments)
        }
        _unknown => Err(unknown_rule_error(pair, "arguments")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_value_basic_data::ValueBasicData;
    use crate::ast_value_basic_type::ValueBasicType;
    use crate::ast_value_type::ValueTypeSimple;
    use crate::utils::parse_full_input;
    use crate::ValueTypeArray;

    fn parse_input(input: &str) -> Result<Vec<Argument>, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::arguments, parse_arguments)
    }

    #[test]
    fn test_accepts_description() {
        let arguments = parse_input("(\"\"\" my description \"\"\"arg: String)").unwrap();
        assert_eq!(arguments.get(0).unwrap().description, "my description")
    }

    #[test]
    fn test_one_argument_is_parsed_correctly() {
        let args = parse_input("(arg1: String)").unwrap();
        assert_eq!(
            args,
            vec![Argument {
                name: "arg1".to_string(),
                description: "".to_string(),
                value: ValueType::Simple(ValueTypeSimple {
                    content: ValueBasicType::String,
                    nullable: true
                }),
                default: None
            }]
        )
    }

    #[test]
    fn test_multiple_arguments_are_parsed_correctly() {
        let args = parse_input("(arg1: String! arg2: [Boolean]!)").unwrap();
        assert_eq!(
            args,
            vec![
                Argument {
                    name: "arg1".to_string(),
                    description: "".to_string(),
                    value: ValueType::Simple(ValueTypeSimple {
                        content: ValueBasicType::String,
                        nullable: false
                    }),
                    default: None
                },
                Argument {
                    name: "arg2".to_string(),
                    description: "".to_string(),
                    value: ValueType::Array(ValueTypeArray {
                        value: ValueTypeSimple {
                            content: ValueBasicType::Boolean,
                            nullable: true
                        },
                        nullable: false
                    }),
                    default: None
                }
            ]
        )
    }

    #[test]
    fn test_default_value_for_argument_works() {
        let args = parse_input("(arg: String = \"default\")").unwrap();
        assert_eq!(
            args,
            vec![Argument {
                name: "arg".to_string(),
                description: "".to_string(),
                value: ValueType::Simple(ValueTypeSimple {
                    content: ValueBasicType::String,
                    nullable: true
                }),
                default: Some(ValueData::Basic(ValueBasicData::String(
                    "default".to_string()
                )))
            }]
        )
    }

    #[test]
    fn test_invalid_input_no_parenthesis() {
        parse_input("arg: String)").unwrap_err();
    }
    #[test]
    fn test_invalid_input_too_much_parenthesis() {
        parse_input("((arg: Boolean))").unwrap_err();
    }
    #[test]
    fn test_invalid_input_no_two_dots() {
        parse_input("(arg1 Int, arg2: Float)").unwrap_err();
    }
    #[test]
    fn test_valid_input_accepts_no_comma() {
        parse_input("(arg1: Int arg2: Boolean)").unwrap();
    }
    #[test]
    fn test_valid_input_accepts_too_much_commas() {
        parse_input("(,arg1: Int, arg2: Float)").unwrap();
    }
}
