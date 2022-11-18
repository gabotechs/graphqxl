use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_identifier::{parse_identifier, Identifier};
use crate::ast_value_data::{parse_value_data, ValueData};
use crate::parser::{Rule, RuleError};
use crate::utils::{unknown_rule_error, OwnedSpan};
use crate::{parse_directive, parse_value_type, Directive, ValueType};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub span: OwnedSpan,
    pub name: Identifier,
    pub description: String,
    pub value_type: ValueType,
    pub default: Option<ValueData>,
    pub directives: Vec<Directive>,
}

impl Argument {
    pub fn build(name: &str, t: ValueType) -> Self {
        Self {
            span: OwnedSpan::default(),
            name: Identifier::from(name),
            description: "".to_string(),
            value_type: t,
            default: None,
            directives: Vec::new(),
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

    pub fn object(name: &str, identifier: Identifier) -> Self {
        Self::build(name, ValueType::object(identifier))
    }

    pub fn description(&mut self, description: &str) -> Self {
        self.description = description.to_string();
        self.clone()
    }

    pub fn default(&mut self, value_data: ValueData) -> Self {
        self.default = Some(value_data);
        self.clone()
    }

    pub fn directive(&mut self, directive: Directive) -> Self {
        self.directives.push(directive);
        self.clone()
    }
}

fn parse_argument(pair: Pair<Rule>, file: &str) -> Result<Argument, Box<RuleError>> {
    match pair.as_rule() {
        Rule::argument => {
            let span = OwnedSpan::from(pair.as_span(), file);
            // at this moment we are on [argument]
            let mut childs = pair.into_inner();
            let DescriptionAndNext(description, next) =
                parse_description_and_continue(&mut childs, file);
            // at this moment we are on [identifier, value]
            let name = parse_identifier(next, file)?;
            let value = parse_value_type(childs.next().unwrap(), file)?;
            let mut default = None;
            let mut directives = Vec::new();
            if let Some(pair) = childs.next() {
                if let Rule::value_data = pair.as_rule() {
                    default = Some(parse_value_data(pair, file)?)
                }
                for directive in childs {
                    directives.push(parse_directive(directive, file)?);
                }
            }
            Ok(Argument {
                span,
                name,
                description,
                value_type: value,
                default,
                directives,
            })
        }
        _unknown => Err(unknown_rule_error(pair, "argument")),
    }
}

pub(crate) fn parse_arguments(
    pair: Pair<Rule>,
    file: &str,
) -> Result<Vec<Argument>, Box<RuleError>> {
    match pair.as_rule() {
        Rule::arguments => {
            let mut arguments = Vec::new();
            for argument in pair.into_inner() {
                arguments.push(parse_argument(argument, file)?);
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

    fn parse_input(input: &str) -> Result<Vec<Argument>, Box<RuleError>> {
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
    fn test_accept_directives() {
        assert_eq!(
            parse_input("(arg: String = \"default\" @dir1 @dir2)"),
            Ok(vec![Argument::string("arg")
                .default(ValueData::string("default"))
                .directive(Directive::build("dir1"))
                .directive(Directive::build("dir2"))])
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
