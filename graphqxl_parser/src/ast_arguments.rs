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
    pub value_type: ValueType,
    pub default: Option<ValueData>,
}

impl Argument {
    pub fn build(name: &str, t: ValueType) -> Self {
        Self {
            name: name.to_string(),
            description: "".to_string(),
            value_type: t,
            default: None,
        }
    }

    pub fn int(name: &str) -> Self {
        Self::build(name, ValueType::int())
    }

    pub fn float(name: &str) -> Self {
        Self::build(name, ValueType::float())
    }

    pub fn string(name: &str) -> Self {
        Self::build(name, ValueType::string())
    }

    pub fn boolean(name: &str) -> Self {
        Self::build(name, ValueType::boolean())
    }

    pub fn object(name: &str, object_name: &str) -> Self {
        Self::build(name, ValueType::object(object_name))
    }

    pub fn description(&mut self, description: &str) -> Self {
        self.description = description.to_string();
        self.clone()
    }

    pub fn default(&mut self, value_data: ValueData) -> Self {
        self.default = Some(value_data);
        self.clone()
    }
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
            if let Some(pair) = childs.next() {
                if let Rule::value_data = pair.as_rule() {
                    default = Some(parse_value_data(pair)?)
                }
            }
            Ok(Argument {
                name,
                description,
                value_type: value,
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
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<Vec<Argument>, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::arguments, parse_arguments)
    }

    #[test]
    fn test_accepts_description() {
        assert_eq!(
            parse_input("(\"\"\" my description \"\"\"arg: String)"),
            Ok(vec![Argument::string("arg").description("my description")])
        );
    }

    #[test]
    fn test_one_argument_is_parsed_correctly() {
        assert_eq!(
            parse_input("(arg1: String)"),
            Ok(vec![Argument::string("arg1")])
        );
    }

    #[test]
    fn test_multiple_arguments_are_parsed_correctly() {
        assert_eq!(
            parse_input("(arg1: String! arg2: [Boolean]!)"),
            Ok(vec![
                Argument::build("arg1", ValueType::string().non_nullable()),
                Argument::build("arg2", ValueType::boolean().array().non_nullable())
            ])
        );
    }

    #[test]
    fn test_default_value_for_argument_works() {
        assert_eq!(
            parse_input("(arg: String = \"default\")"),
            Ok(vec![
                Argument::string("arg").default(ValueData::string("default"))
            ])
        );
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
