use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_identifier::parse_identifier;
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use crate::{parse_value_type, ValueType};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: String,
    pub description: String,
    pub value: ValueType,
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
            Ok(Argument {
                name,
                description,
                value,
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
    use crate::ast_value_basic_type::ValueBasicType;
    use crate::ast_value_type::ValueSimple;
    use crate::utils::parse_full_input;
    use crate::ValueArray;

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
                value: ValueType::Simple(ValueSimple {
                    content: ValueBasicType::String,
                    nullable: true
                })
            }]
        )
    }

    #[test]
    fn test_multiple_arguments_are_parsed_correctly() {
        let args = parse_input("(arg1: String!, arg2: [Boolean]!)").unwrap();
        assert_eq!(
            args,
            vec![
                Argument {
                    name: "arg1".to_string(),
                    description: "".to_string(),
                    value: ValueType::Simple(ValueSimple {
                        content: ValueBasicType::String,
                        nullable: false
                    })
                },
                Argument {
                    name: "arg2".to_string(),
                    description: "".to_string(),
                    value: ValueType::Array(ValueArray {
                        value: ValueSimple {
                            content: ValueBasicType::Boolean,
                            nullable: true
                        },
                        nullable: false
                    })
                }
            ]
        )
    }

    #[test]
    fn test_invalid_input_1() {
        parse_input("arg: String)").unwrap_err();
    }
    #[test]
    fn test_invalid_input_2() {
        parse_input("((arg: Boolean))").unwrap_err();
    }
    #[test]
    fn test_invalid_input_3() {
        parse_input("(arg1: Int arg2: Boolean)").unwrap_err();
    }
    #[test]
    fn test_invalid_input_4() {
        parse_input("(arg1 Int, arg2: Float)").unwrap_err();
    }
    #[test]
    fn test_invalid_input_5() {
        parse_input("(,arg1: Int, arg2: Float)").unwrap_err();
    }
}
