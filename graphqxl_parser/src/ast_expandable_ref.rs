use crate::ast_generic_call::parse_generic_call;
use crate::ast_identifier::parse_identifier;
use crate::parser::{Rule, RuleError};
use crate::utils::unknown_rule_error;
use crate::{GenericCall, Identifier, OwnedSpan, ValueType};
use pest::iterators::Pair;
use std::borrow::BorrowMut;

#[derive(Debug, Clone, PartialEq)]
pub struct ExpandableRef {
    pub span: OwnedSpan,
    pub identifier: Identifier,
    pub generic_call: Option<GenericCall>,
}

impl ExpandableRef {
    pub fn from(name: &str) -> Self {
        Self {
            span: OwnedSpan::default(),
            identifier: Identifier::from(name),
            generic_call: None,
        }
    }

    pub fn generic_arg(&mut self, value_type: ValueType) -> Self {
        if let Some(call) = self.generic_call.borrow_mut() {
            call.args.push(value_type)
        } else {
            self.generic_call = Some(GenericCall::from(value_type));
        }
        self.clone()
    }
}

pub(crate) fn parse_expandable_ref(
    pair: Pair<Rule>,
    file: &str,
) -> Result<ExpandableRef, Box<RuleError>> {
    match pair.as_rule() {
        Rule::expandable_ref => {
            let span = OwnedSpan::from(pair.as_span(), file);
            let mut childs = pair.into_inner();
            let identifier = parse_identifier(childs.next().unwrap(), file)?;
            let mut generic_call: Option<GenericCall> = None;
            if let Some(child) = childs.next() {
                generic_call = Some(parse_generic_call(child, file)?);
            }

            Ok(ExpandableRef {
                span,
                identifier,
                generic_call,
            })
        }
        _unknown => Err(unknown_rule_error(pair, "expandable_ref")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<ExpandableRef, Box<RuleError>> {
        parse_full_input(input, Rule::expandable_ref, parse_expandable_ref)
    }

    #[test]
    fn test_parses_without_generic_call() {
        assert_eq!(parse_input("MyType"), Ok(ExpandableRef::from("MyType")))
    }

    #[test]
    fn test_parses_with_generic_call() {
        assert_eq!(
            parse_input("MyType<String, Int>"),
            Ok(ExpandableRef::from("MyType")
                .generic_arg(ValueType::string())
                .generic_arg(ValueType::int()))
        )
    }

    #[test]
    fn test_do_not_parses_invalid_input() {
        parse_input("MyType<String, Int").unwrap_err();
    }
}
