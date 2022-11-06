use pest::iterators::Pair;

use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use crate::{parse_value_type, OwnedSpan, ValueType};

#[derive(Debug, Clone, PartialEq)]
pub struct GenericCall {
    pub span: OwnedSpan,
    pub args: Vec<ValueType>,
}

impl GenericCall {
    pub fn from(value_type: ValueType) -> Self {
        GenericCall {
            span: OwnedSpan::default(),
            args: vec![value_type],
        }
    }

    pub fn arg(&mut self, value_type: ValueType) -> Self {
        self.args.push(value_type);
        self.clone()
    }
}

pub(crate) fn parse_generic_call(
    pair: Pair<Rule>,
    file: &str,
) -> Result<GenericCall, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::generic_call => {
            let span = OwnedSpan::from(pair.as_span(), file);
            let childs = pair.into_inner();
            let mut args = Vec::new();
            for name in childs {
                args.push(parse_value_type(name, file)?);
            }

            Ok(GenericCall { span, args })
        }
        _unknown => Err(unknown_rule_error(pair, "generic_call")),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::parse_full_input;

    use super::*;

    fn parse_input(input: &str) -> Result<GenericCall, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::generic_call, parse_generic_call)
    }

    #[test]
    fn test_parses_one_generic_call() {
        assert_eq!(
            parse_input("<String!>"),
            Ok(GenericCall::from(ValueType::string().non_nullable()))
        )
    }

    #[test]
    fn test_parses_two_generic_call() {
        assert_eq!(
            parse_input("<[Int]! Boolean>"),
            Ok(GenericCall::from(ValueType::int().array().non_nullable())
                .arg(ValueType::boolean()))
        )
    }
}
