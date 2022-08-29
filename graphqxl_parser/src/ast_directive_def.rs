use crate::ast_arguments::{parse_arguments, Argument};
use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_directive_location::{parse_directive_location, DirectiveLocation};
use crate::ast_identifier::{parse_identifier, Identifier};
use crate::parser::Rule;
use crate::utils::{unknown_rule_error, OwnedSpan};
use pest::iterators::Pair;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DirectiveDef {
    pub span: OwnedSpan,
    pub name: Identifier,
    pub description: String,
    pub arguments: Vec<Argument>,
    pub is_repeatable: bool,
    pub locations: Vec<DirectiveLocation>,
}

impl DirectiveDef {
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

    pub fn repeatable(&mut self) -> Self {
        self.is_repeatable = true;
        self.clone()
    }

    pub fn arg(&mut self, arg: Argument) -> Self {
        self.arguments.push(arg);
        self.clone()
    }

    pub fn location(&mut self, location: DirectiveLocation) -> Self {
        self.locations.push(location);
        self.clone()
    }
}

pub(crate) fn parse_directive_def(
    pair: Pair<Rule>,
) -> Result<DirectiveDef, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::directive_def => {
            let span = OwnedSpan::from(pair.as_span());
            // [identifier, arguments?, repeatable?, ...locations]
            let mut childs = pair.into_inner();
            let DescriptionAndNext(description, next) = parse_description_and_continue(&mut childs);
            let name = parse_identifier(next)?;
            let mut next = childs.next().unwrap();
            let mut arguments = Vec::new();
            if let Rule::arguments = next.as_rule() {
                arguments = parse_arguments(next)?;
                next = childs.next().unwrap();
            }
            let mut is_repeatable = false;
            if let Rule::directive_repeatable = next.as_rule() {
                is_repeatable = true;
                next = childs.next().unwrap();
            }
            let mut locations = Vec::new();
            let mut seen_locations = HashSet::new();
            if let Rule::directive_location = next.as_rule() {
                seen_locations.insert(next.as_str());
                locations.push(parse_directive_location(next)?)
            }
            for child in childs {
                if seen_locations.contains(child.as_str()) {
                    return Err(pest::error::Error::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: "repeated location ".to_string() + child.as_str(),
                        },
                        child.as_span(),
                    ));
                } else {
                    seen_locations.insert(child.as_str());
                }
                locations.push(parse_directive_location(child)?);
            }
            Ok(DirectiveDef {
                span,
                name,
                description,
                arguments,
                is_repeatable,
                locations,
            })
        }
        _unknown => Err(unknown_rule_error(pair, "directive_def")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;
    use crate::ValueType;

    fn parse_input(input: &str) -> Result<DirectiveDef, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::directive_def, parse_directive_def)
    }

    #[test]
    fn test_accepts_description() {
        assert_eq!(
            parse_input("\"\"\" my description \"\"\"directive @dir on QUERY"),
            Ok(DirectiveDef::build("dir")
                .description("my description")
                .location(DirectiveLocation::Query))
        );
    }

    #[test]
    fn test_directive_without_arguments_without_repeatable_1_location() {
        assert_eq!(
            parse_input("directive @dir on QUERY"),
            Ok(DirectiveDef::build("dir").location(DirectiveLocation::Query))
        );
    }

    #[test]
    fn test_directive_with_arguments_without_repeatable_1_location() {
        assert_eq!(
            parse_input("directive @dir2(arg: String) on SUBSCRIPTION"),
            Ok(DirectiveDef::build("dir2")
                .arg(Argument::string("arg"))
                .location(DirectiveLocation::Subscription))
        );
    }

    #[test]
    fn test_directive_with_arguments_with_repeatable_1_location() {
        assert_eq!(
            parse_input("directive @dir3(arg: String, arg2: [Boolean!]!) repeatable on ENUM_VALUE"),
            Ok(DirectiveDef::build("dir3")
                .arg(Argument::string("arg"))
                .arg(Argument::build(
                    "arg2",
                    ValueType::boolean().non_nullable().array().non_nullable()
                ))
                .repeatable()
                .location(DirectiveLocation::EnumValue))
        );
    }

    #[test]
    fn test_directive_with_arguments_with_repeatable_3_location() {
        assert_eq!(
            parse_input(
                "directive @dir4(arg: String, arg2: [Boolean!]!) repeatable on UNION | ENUM | FIELD",
            ),
            Ok(DirectiveDef::build("dir4")
                .arg(Argument::string("arg"))
                .arg(Argument::build(
                    "arg2",
                    ValueType::boolean().non_nullable().array().non_nullable()
                ))
                .repeatable()
                .location(DirectiveLocation::Union)
                .location(DirectiveLocation::Enum)
                .location(DirectiveLocation::Field)
            )
        );
    }

    #[test]
    fn test_directive_no_repeated_locations() {
        parse_input("directive @dir on UNION | ENUM | FIELD | UNION").unwrap_err();
    }

    #[test]
    fn test_invalid_input_directiv() {
        parse_input("directiv @dir(arg: String) repeatable on INTERFACE").unwrap_err();
    }

    #[test]
    fn test_invalid_input_repeatable_after_on() {
        parse_input("directive @dir on repeatable INTERFACE").unwrap_err();
    }

    #[test]
    fn test_invalid_input_bad_location() {
        parse_input("directive @dir(arg: String) repeatable on INTERFAE").unwrap_err();
    }

    #[test]
    fn test_invalid_input_no_on() {
        parse_input("directive @dir(arg: String) repeatable").unwrap_err();
    }
}
