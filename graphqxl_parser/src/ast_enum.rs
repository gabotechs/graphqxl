use crate::ast_enum_field::{parse_enum_field, EnumField};
use crate::ast_identifier::parse_identifier;
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    name: String,
    fields: Vec<EnumField>,
}

pub(crate) fn parse_enum(pair: Pair<Rule>) -> Result<Enum, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::enum_def => {
            let mut pairs = pair.into_inner();
            // [identifier, enum_selection_set]
            let name = parse_identifier(pairs.next().unwrap())?;
            let enum_selection_set = pairs.next().unwrap();
            let mut enum_def = Enum {
                name,
                fields: Vec::new(),
            };
            for field_without_args_without_value in enum_selection_set.into_inner() {
                let field = parse_enum_field(field_without_args_without_value)?;
                enum_def.fields.push(field);
            }
            Ok(enum_def)
        }
        _unknown => Err(unknown_rule_error(pair, "enum_def")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<Enum, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::enum_def, parse_enum)
    }

    #[test]
    fn test_parses_enum() {
        let enum_def = parse_input("enum MyEnum { Field }").unwrap();
        assert_eq!(enum_def.name, "MyEnum");
        assert_eq!(
            enum_def.fields,
            vec![EnumField {
                name: "Field".to_string()
            }]
        );
    }

    #[test]
    fn test_parses_enum_multiple_fields() {
        let enum_def = parse_input("enum MyEnum { Field1\n field2 }").unwrap();
        assert_eq!(enum_def.name, "MyEnum");
        assert_eq!(
            enum_def.fields,
            vec![
                EnumField {
                    name: "Field1".to_string()
                },
                EnumField {
                    name: "field2".to_string()
                }
            ]
        );
    }

    #[test]
    fn test_incorrect_input_1() {
        parse_input("enum MyEnum { Field1, Field2 }").unwrap_err();
    }

    #[test]
    fn test_incorrect_input_2() {
        parse_input("enum MyEnum { Field1: String Field2 }").unwrap_err();
    }

    #[test]
    fn test_incorrect_input_3() {
        parse_input("enum MyEnum { Field1 Field2 ").unwrap_err();
    }
}
