use crate::ast_expandable_ref::{parse_expandable_ref, ExpandableRef};
use crate::parser::{Rule, RuleError};
use crate::utils::unknown_rule_error;
use crate::OwnedSpan;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum ModifiedRef {
    Required(Box<ModifiedRef>, OwnedSpan),
    Optional(Box<ModifiedRef>, OwnedSpan),
    ExpandableRef(ExpandableRef),
}

impl ModifiedRef {
    pub fn span(&self) -> &OwnedSpan {
        match self {
            ModifiedRef::Required(_, span) => span,
            ModifiedRef::Optional(_, span) => span,
            ModifiedRef::ExpandableRef(r) => &r.span,
        }
    }
}

impl ModifiedRef {
    pub fn build(name: &str) -> Self {
        Self::ExpandableRef(ExpandableRef::from(name))
    }

    pub fn expandable_ref(expandable_ref: ExpandableRef) -> Self {
        Self::ExpandableRef(expandable_ref)
    }

    pub fn optional(&mut self) -> Self {
        ModifiedRef::Optional(Box::new(self.clone()), OwnedSpan::default())
    }

    pub fn required(&mut self) -> Self {
        ModifiedRef::Required(Box::new(self.clone()), OwnedSpan::default())
    }
}

pub(crate) fn parse_modified_ref(
    pair: Pair<Rule>,
    file: &str,
) -> Result<ModifiedRef, Box<RuleError>> {
    match pair.as_rule() {
        Rule::modified_ref => {
            let span = OwnedSpan::from(pair.as_span(), file);
            let mut childs = pair.into_inner();
            let first = childs.next().unwrap();
            match first.as_rule() {
                Rule::required_modifier => {
                    let second = childs.next().unwrap();
                    Ok(ModifiedRef::Required(
                        Box::new(parse_modified_ref(second, file)?),
                        span,
                    ))
                }
                Rule::optional_modifier => {
                    let second = childs.next().unwrap();
                    Ok(ModifiedRef::Optional(
                        Box::new(parse_modified_ref(second, file)?),
                        span,
                    ))
                }
                Rule::expandable_ref => Ok(ModifiedRef::ExpandableRef(parse_expandable_ref(
                    first, file,
                )?)),
                _unknown => Err(unknown_rule_error(
                    first,
                    "required_modifier, optional_modifier or expandable_ref",
                )),
            }
        }
        _unknown => Err(unknown_rule_error(pair, "modified_ref")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;
    use crate::ValueType;

    fn parse_input(input: &str) -> Result<ModifiedRef, Box<RuleError>> {
        parse_full_input(input, Rule::modified_ref, parse_modified_ref)
    }

    #[test]
    fn test_parses_simple_expandable_ref() {
        assert_eq!(parse_input("MyType"), Ok(ModifiedRef::build("MyType")))
    }

    #[test]
    fn test_parses_optional_modified_ref() {
        assert_eq!(
            parse_input("Optional<MyType>"),
            Ok(ModifiedRef::build("MyType").optional())
        )
    }

    #[test]
    fn test_parses_required_modified_ref() {
        assert_eq!(
            parse_input("Required<MyType>"),
            Ok(ModifiedRef::build("MyType").required())
        )
    }

    #[test]
    fn test_parses_required_modified_ref_with_generic() {
        assert_eq!(
            parse_input("Required<MyType<[String]>>"),
            Ok(ModifiedRef::expandable_ref(
                ExpandableRef::from("MyType").generic_arg(ValueType::string().array())
            )
            .required())
        )
    }
}
