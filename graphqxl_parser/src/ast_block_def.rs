use crate::ast_block_field::{parse_block_field, BlockField};
use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_identifier::parse_identifier;
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::error::ErrorVariant;
use pest::iterators::{Pair, Pairs};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum BlockDefType {
    Input,
    Type,
    Enum,
    Interface,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockDef {
    pub name: String,
    pub description: String,
    pub kind: BlockDefType,
    pub fields: Vec<BlockField>,
}

impl BlockDef {
    fn build(name: &str, kind: BlockDefType) -> Self {
        Self {
            name: name.to_string(),
            description: "".to_string(),
            kind,
            fields: Vec::new(),
        }
    }

    pub fn type_(name: &str) -> Self {
        Self::build(name, BlockDefType::Type)
    }

    pub fn input(name: &str) -> Self {
        Self::build(name, BlockDefType::Input)
    }

    pub fn enum_(name: &str) -> Self {
        Self::build(name, BlockDefType::Enum)
    }

    pub fn interface(name: &str) -> Self {
        Self::build(name, BlockDefType::Interface)
    }

    pub fn description(&mut self, description: &str) -> Self {
        self.description = description.to_string();
        self.clone()
    }

    pub fn field(&mut self, field: BlockField) -> Self {
        self.fields.push(field);
        self.clone()
    }
}

fn _parse_type_or_input(
    mut pairs: Pairs<Rule>,
    kind: BlockDefType,
) -> Result<BlockDef, pest::error::Error<Rule>> {
    // [description?, identifier, selection_set]
    let DescriptionAndNext(description, next) = parse_description_and_continue(&mut pairs);
    let name = parse_identifier(next)?;
    let selection_set = pairs.next().unwrap();
    let mut fields = Vec::new();
    let mut seen_fields = HashSet::new();
    for pair in selection_set.into_inner() {
        let field = parse_block_field(pair.clone())?;
        if seen_fields.contains(&field.name) {
            return Err(pest::error::Error::new_from_span(
                ErrorVariant::CustomError {
                    message: "duplicate field ".to_string() + &field.name,
                },
                pair.as_span(),
            ));
        } else {
            seen_fields.insert(field.name.clone());
        }
        fields.push(field);
    }
    Ok(BlockDef {
        name,
        description,
        kind,
        fields,
    })
}

pub(crate) fn parse_block_def(pair: Pair<Rule>) -> Result<BlockDef, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::type_def => _parse_type_or_input(pair.into_inner(), BlockDefType::Type),
        Rule::input_def => _parse_type_or_input(pair.into_inner(), BlockDefType::Input),
        Rule::enum_def => _parse_type_or_input(pair.into_inner(), BlockDefType::Enum),
        Rule::interface_def => _parse_type_or_input(pair.into_inner(), BlockDefType::Interface),
        _unknown => Err(unknown_rule_error(pair, "type_def or input_def")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<BlockDef, pest::error::Error<Rule>> {
        let rule = if input.contains("input ") {
            Rule::input_def
        } else if input.contains("type ") {
            Rule::type_def
        } else if input.contains("enum ") {
            Rule::enum_def
        } else if input.contains("interface ") {
            Rule::interface_def
        } else {
            Rule::type_def
        };
        parse_full_input(input, rule, parse_block_def)
    }

    #[test]
    fn test_type_def_accepts_field_args() {
        parse_input("type MyType { field(arg1: String!): Boolean }").unwrap();
    }

    #[test]
    fn test_interface_def_accepts_field_args() {
        parse_input("interface MyType { field(arg1: String!): Boolean }").unwrap();
    }

    #[test]
    fn test_input_def_do_not_accept_field_args() {
        parse_input("input MyInput { field(arg1: String!): Boolean }").unwrap_err();
    }

    #[test]
    fn test_enum_def_do_not_accept_values() {
        parse_input("enum MyType { field: Boolean }").unwrap_err();
    }

    #[test]
    fn test_type_description_works() {
        assert_eq!(
            parse_input("\"\"\" my description \"\"\"type MyType { arg: String }"),
            Ok(BlockDef::type_("MyType")
                .description("my description")
                .field(BlockField::build("arg").string()))
        );
    }

    #[test]
    fn test_input_description_works() {
        assert_eq!(
            parse_input("\"\"\" my description \"\"\"input MyInput { arg: String }"),
            Ok(BlockDef::input("MyInput")
                .description("my description")
                .field(BlockField::build("arg").string()))
        );
    }

    #[test]
    fn test_enum_description_works() {
        assert_eq!(
            parse_input("\"\"\" my description \"\"\"enum MyEnum { arg }"),
            Ok(BlockDef::enum_("MyEnum")
                .description("my description")
                .field(BlockField::build("arg")))
        );
    }

    #[test]
    fn test_interface_description_works() {
        assert_eq!(
            parse_input("\"\"\" my description \"\"\"interface MyInterface { arg: String }"),
            Ok(BlockDef::interface("MyInterface")
                .description("my description")
                .field(BlockField::build("arg").string()))
        );
    }

    #[test]
    fn parses_empty_type() {
        assert_eq!(
            parse_input("type MyType { }"),
            Ok(BlockDef::type_("MyType"))
        );
    }

    #[test]
    fn parses_empty_input() {
        assert_eq!(
            parse_input("input MyInput { }"),
            Ok(BlockDef::input("MyInput"))
        );
    }

    #[test]
    fn parses_empty_enum() {
        assert_eq!(
            parse_input("enum MyEnum { }"),
            Ok(BlockDef::enum_("MyEnum"))
        );
    }

    #[test]
    fn parses_filled_with_spaces_type() {
        assert_eq!(
            parse_input("type MyType { field: String }"),
            Ok(BlockDef::type_("MyType").field(BlockField::build("field").string()))
        );
    }

    #[test]
    fn parses_filled_with_line_jumps_type() {
        assert_eq!(
            parse_input("type MyType { \nfield: String\n }"),
            Ok(BlockDef::type_("MyType").field(BlockField::build("field").string()))
        );
    }

    #[test]
    fn test_parses_enum() {
        assert_eq!(
            parse_input("enum MyEnum { Field }"),
            Ok(BlockDef::enum_("MyEnum").field(BlockField::build("Field")))
        );
    }

    #[test]
    fn test_parses_enum_multiple_fields() {
        assert_eq!(
            parse_input("enum MyEnum { Field1\n field2 }"),
            Ok(BlockDef::enum_("MyEnum")
                .field(BlockField::build("Field1"))
                .field(BlockField::build("field2")))
        );
    }

    #[test]
    fn test_incorrect_input_no_different_field_types() {
        parse_input("enum MyEnum { Field1: String Field2 }").unwrap_err();
    }

    #[test]
    fn test_incorrect_input_no_last_close_keys() {
        parse_input("enum MyEnum { Field1 Field2 ").unwrap_err();
    }

    #[test]
    fn do_not_parse_invalid_input_bad_type() {
        parse_input("type MyType { field: String- }").unwrap_err();
    }

    #[test]
    fn do_not_parse_invalid_input_bad_field() {
        parse_input("type MyType { fi'eld: String }").unwrap_err();
    }

    #[test]
    fn do_not_parse_invalid_input_bad_type_identifier() {
        parse_input("type MyT-ype { field: String }").unwrap_err();
    }

    #[test]
    fn do_not_parse_invalid_input_bad_initial_symbol() {
        parse_input("type_ MyType { field: String }").unwrap_err();
    }

    #[test]
    fn do_not_parse_invalid_input_type_and_name_without_space() {
        parse_input("typeMyType { field: String }").unwrap_err();
    }
}
