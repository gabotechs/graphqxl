use crate::ast_identifier::parse_identifier;
use crate::utils::unknown_rule_error;
use crate::{parse_function_call, FunctionCall, FunctionInput, Rule, ValueData};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    name: String,
    call: Option<FunctionCall>,
}

impl Directive {
    pub fn build(name: &str) -> Self {
        Self {
            name: name.to_string(),
            call: None,
        }
    }

    pub fn input(&mut self, name: &str, value: ValueData) -> Self {
        let input = FunctionInput {
            name: name.to_string(),
            value,
        };
        if let Some(mut call) = self.call.clone() {
            call.inputs.push(input);
        } else {
            self.call = Some(FunctionCall {
                inputs: vec![input],
            });
        }
        self.clone()
    }
}

pub(crate) fn parse_directive(pair: Pair<Rule>) -> Result<Directive, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::directive => {
            let mut childs = pair.into_inner();
            let name = parse_identifier(childs.next().unwrap())?;
            let maybe_function_call = childs.next();
            let mut call = None;
            if let Some(function_call) = maybe_function_call {
                call = Some(parse_function_call(function_call)?);
            }
            Ok(Directive { name, call })
        }
        _unknown => Err(unknown_rule_error(pair, "directive")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<Directive, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::directive, parse_directive)
    }

    #[test]
    fn test_parses_no_input_directive() {
        assert_eq!(parse_input("@dir"), Ok(Directive::build("dir")));
    }

    #[test]
    fn test_parses_directive_with_inputs() {
        assert_eq!(
            parse_input("@dir (arg: \"data space\")"),
            Ok(Directive::build("dir").input("arg", ValueData::string("data space")))
        );
    }

    #[test]
    fn test_parse_directive_with_weird_inputs() {
        assert_eq!(
            parse_input("@dir (arg: [1, 2, 3])"),
            Ok(Directive::build("dir").input(
                "arg",
                ValueData::int(1)
                    .list()
                    .push(ValueData::int(2))
                    .push(ValueData::int(3))
            ))
        );
    }
}
