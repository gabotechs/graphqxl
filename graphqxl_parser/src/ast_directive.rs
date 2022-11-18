use crate::ast_identifier::{parse_identifier, Identifier};
use crate::parser::{Rule, RuleError};
use crate::utils::{unknown_rule_error, OwnedSpan};
use crate::{parse_function_call, FunctionCall, FunctionInput, ValueData};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Directive {
    pub span: OwnedSpan,
    pub name: Identifier,
    pub call: Option<FunctionCall>,
}

impl Directive {
    pub fn build(name: &str) -> Self {
        Self {
            name: Identifier::from(name),
            ..Default::default()
        }
    }

    pub fn input(&mut self, name: &str, value: ValueData) -> Self {
        let input = FunctionInput {
            span: Default::default(),
            name: Identifier::from(name),
            value,
        };
        if let Some(call) = self.call.as_mut() {
            call.inputs.push(input);
        } else {
            self.call = Some(FunctionCall {
                span: Default::default(),
                inputs: vec![input],
            });
        }
        self.clone()
    }
}

pub(crate) fn parse_directive(pair: Pair<Rule>, file: &str) -> Result<Directive, Box<RuleError>> {
    match pair.as_rule() {
        Rule::directive => {
            let span = OwnedSpan::from(pair.as_span(), file);
            let mut childs = pair.into_inner();
            let name = parse_identifier(childs.next().unwrap(), file)?;
            let maybe_function_call = childs.next();
            let mut call = None;
            if let Some(function_call) = maybe_function_call {
                call = Some(parse_function_call(function_call, file)?);
            }
            Ok(Directive { span, name, call })
        }
        _unknown => Err(unknown_rule_error(pair, "directive")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<Directive, Box<RuleError>> {
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
