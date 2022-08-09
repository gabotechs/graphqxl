use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_identifier::parse_identifier;
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Union {
    pub name: String,
    pub description: String,
    pub types: Vec<String>,
}

impl Union {
    pub fn build(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: "".to_string(),
            types: Vec::new(),
        }
    }

    pub fn description(&mut self, description: &str) -> Self {
        self.description = description.to_string();
        self.clone()
    }

    pub fn type_(&mut self, type_: &str) -> Self {
        self.types.push(type_.to_string());
        self.clone()
    }
}

pub(crate) fn parse_union(pair: Pair<Rule>) -> Result<Union, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::union_def => {
            let mut childs = pair.into_inner();
            // [description?, identifier, ...types]
            let DescriptionAndNext(description, next) = parse_description_and_continue(&mut childs);
            let name = parse_identifier(next)?;
            let mut types = Vec::new();
            let mut seen_types = HashSet::new();
            for child in childs {
                let name = parse_identifier(child.clone())?;
                if seen_types.contains(name.as_str()) {
                    return Err(pest::error::Error::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: "repeated type ".to_string() + name.as_str(),
                        },
                        child.as_span(),
                    ));
                } else {
                    seen_types.insert(name.clone());
                    types.push(name);
                }
            }
            Ok(Union {
                name,
                description,
                types,
            })
        }
        _unknown => Err(unknown_rule_error(pair, "union_def")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<Union, pest::error::Error<Rule>> {
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
    fn test_invalid_input_repeated_types() {
        parse_input("union UnionType = Type1 | Type2|Type3 |Type2").unwrap_err();
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
