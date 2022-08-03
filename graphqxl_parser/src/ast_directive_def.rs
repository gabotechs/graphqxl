use crate::ast_arguments::{parse_arguments, Argument};
use crate::ast_directive_location::{parse_directive_location, DirectiveLocation};
use crate::ast_identifier::parse_identifier;
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct DirectiveDef {
    name: String,
    arguments: Vec<Argument>,
    is_repeatable: bool,
    locations: Vec<DirectiveLocation>,
}

pub(crate) fn parse_directive_def(
    pair: Pair<Rule>,
) -> Result<DirectiveDef, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::directive_def => {
            // [identifier, arguments?, repeatable?, ...locations]
            let mut childs = pair.into_inner();
            let name = parse_identifier(childs.next().unwrap())?;
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
                name,
                arguments,
                is_repeatable,
                locations,
            })
        },
        _unknown => Err(unknown_rule_error(pair, "directive_def")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<DirectiveDef, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::directive_def, parse_directive_def)
    }

    #[test]
    fn test_directive_without_arguments_without_repeatable_1_location() {
        let directive = parse_input("directive @dir on QUERY").unwrap();
        assert_eq!(directive.name, "dir");
        assert!(!directive.is_repeatable);
        assert_eq!(directive.arguments.len(), 0);
        assert_eq!(directive.locations, vec![DirectiveLocation::Query]);
    }

    #[test]
    fn test_directive_with_arguments_without_repeatable_1_location() {
        let directive = parse_input("directive @dir2(arg: String) on SUBSCRIPTION").unwrap();
        assert_eq!(directive.name, "dir2");
        assert!(!directive.is_repeatable);
        assert_eq!(directive.arguments.get(0).unwrap().name, "arg");
        assert_eq!(directive.locations, vec![DirectiveLocation::Subscription]);
    }

    #[test]
    fn test_directive_with_arguments_with_repeatable_1_location() {
        let directive =
            parse_input("directive @dir3(arg: String, arg2: [Boolean!]!) repeatable on ENUM_VALUE")
                .unwrap();
        assert_eq!(directive.name, "dir3");
        assert!(directive.is_repeatable);
        assert_eq!(directive.arguments.get(0).unwrap().name, "arg");
        assert_eq!(directive.arguments.get(1).unwrap().name, "arg2");
        assert_eq!(directive.locations, vec![DirectiveLocation::EnumValue]);
    }

    #[test]
    fn test_directive_with_arguments_with_repeatable_3_location() {
        let directive = parse_input(
            "directive @dir3(arg: String, arg2: [Boolean!]!) repeatable on UNION | ENUM | FIELD",
        )
        .unwrap();
        assert_eq!(directive.name, "dir3");
        assert!(directive.is_repeatable);
        assert_eq!(directive.arguments.get(0).unwrap().name, "arg");
        assert_eq!(directive.arguments.get(1).unwrap().name, "arg2");
        assert_eq!(
            directive.locations,
            vec![
                DirectiveLocation::Union,
                DirectiveLocation::Enum,
                DirectiveLocation::Field
            ]
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
    fn test_invalid_input_no_comma_in_args() {
        parse_input("directive @dir(arg: String arg2: Boolean) repeatable on INTERFACE")
            .unwrap_err();
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
