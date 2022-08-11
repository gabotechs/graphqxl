use crate::ast_identifier::parse_identifier;
use crate::utils::unknown_rule_error;
use crate::{parse_value_data, Rule, ValueData};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionInput {
    pub name: String,
    pub value: ValueData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub inputs: Vec<FunctionInput>,
}

impl FunctionCall {
    pub fn build() -> Self {
        Self { inputs: Vec::new() }
    }

    pub fn input(&mut self, name: &str, value: ValueData) -> Self {
        self.inputs.push(FunctionInput {
            name: name.to_string(),
            value,
        });
        self.clone()
    }
}

pub(crate) fn parse_function_call(
    pair: Pair<Rule>,
) -> Result<FunctionCall, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::function_call => {
            let mut inputs = Vec::new();
            for function_input in pair.into_inner() {
                // [identifier, value_data]
                let mut childs = function_input.into_inner();
                let name = parse_identifier(childs.next().unwrap())?;
                let value = parse_value_data(childs.next().unwrap())?;
                inputs.push(FunctionInput { name, value })
            }
            Ok(FunctionCall { inputs })
        }
        _unknown => Err(unknown_rule_error(pair, "function_call")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<FunctionCall, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::function_call, parse_function_call)
    }

    #[test]
    fn test_parses_one_input() {
        assert_eq!(
            parse_input("(arg: \"data\")"),
            Ok(FunctionCall::build().input("arg", ValueData::string("data")))
        );
    }

    #[test]
    fn test_parses_two_inputs() {
        assert_eq!(
            parse_input("(arg: 1.0, arg2: true)"),
            Ok(FunctionCall::build()
                .input("arg", ValueData::float(1.0))
                .input("arg2", ValueData::boolean(true)))
        );
    }

    #[test]
    fn test_nested_object() {
        assert_eq!(
            parse_input("(simple: 1, complex: { a: 3 b: [{ c: \"data\" }] })"),
            Ok(FunctionCall::build()
                .input("simple", ValueData::int(1))
                .input(
                    "complex",
                    ValueData::string("data")
                        .to_object("c")
                        .list()
                        .to_object("b")
                        .insert("a", ValueData::int(3))
                ))
        );
    }
}
