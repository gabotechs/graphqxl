use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_identifier::{parse_identifier, Identifier};
use crate::parser::{Rule, RuleError};
use crate::utils::{unknown_rule_error, OwnedSpan};
use crate::{parse_directive, Directive};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Union {
    pub span: OwnedSpan,
    pub name: Identifier,
    pub description: String,
    pub types: Vec<Identifier>,
    pub directives: Vec<Directive>,
}

impl Union {
    pub fn build(name: &str) -> Self {
        Self {
            name: Identifier::from(name),
            ..Default::default()
        }
    }

    pub fn description(&mut self, description: &str) -> Self {
        self.description = description.to_string();
        self.clone()
    }

    pub fn type_(&mut self, type_: &str) -> Self {
        self.types.push(Identifier::from(type_));
        self.clone()
    }

    pub fn directive(&mut self, directive: Directive) -> Self {
        self.directives.push(directive);
        self.clone()
    }
}

pub(crate) fn parse_union(pair: Pair<Rule>, file: &str) -> Result<Union, Box<RuleError>> {
    match pair.as_rule() {
        Rule::union_def => {
            let span = OwnedSpan::from(pair.as_span(), file);
            let mut childs = pair.into_inner();
            // [description?, identifier, ...types]
            let DescriptionAndNext(description, next) =
                parse_description_and_continue(&mut childs, file);
            let name = parse_identifier(next, file)?;
            let mut types = Vec::new();
            let mut directives = Vec::new();
            for child in childs {
                if let Rule::directive = child.as_rule() {
                    directives.push(parse_directive(child, file)?);
                } else {
                    let name = parse_identifier(child.clone(), file)?;
                    types.push(name);
                }
            }
            Ok(Union {
                span,
                name,
                description,
                types,
                directives,
            })
        }
        _unknown => Err(unknown_rule_error(pair, "union_def")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;
    use crate::ValueData;

    fn parse_input(input: &str) -> Result<Union, Box<RuleError>> {
        parse_full_input(input, Rule::union_def, parse_union)
    }

    #[test]
    fn test_accepts_description() {
        assert_eq!(
            parse_input("\"\"\" my description \"\"\"union MyUnion = Type1"),
            Ok(Union::build("MyUnion")
                .description("my description")
                .type_("Type1"))
        );
    }

    #[test]
    fn test_parses_1_type_union() {
        assert_eq!(
            parse_input("union MyUnion = Type1"),
            Ok(Union::build("MyUnion").type_("Type1"))
        );
    }

    #[test]
    fn test_parses_3_type_union() {
        assert_eq!(
            parse_input("union UnionType = Type1 | Type2|Type3"),
            Ok(Union::build("UnionType")
                .type_("Type1")
                .type_("Type2")
                .type_("Type3"))
        );
    }

    #[test]
    fn test_accepts_directives() {
        assert_eq!(
            parse_input("union UnionType @dir(input: 1) =Type1|Type2"),
            Ok(Union::build("UnionType")
                .directive(Directive::build("dir").input("input", ValueData::int(1)))
                .type_("Type1")
                .type_("Type2"))
        );
    }

    #[test]
    fn test_invalid_input_not_an_equal() {
        parse_input("union UnionType: Type1 | Type2").unwrap_err();
    }

    #[test]
    fn test_invalid_input_not_a_correct_or_operator() {
        parse_input("union UnionType = Type1, Type2").unwrap_err();
    }
}
