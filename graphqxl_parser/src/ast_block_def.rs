use crate::ast_block_field::{parse_block_field, BlockField};
use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_identifier::{parse_identifier, Identifier};
use crate::parser::Rule;
use crate::utils::{unknown_rule_error, OwnedSpan};
use crate::{parse_directive, Directive};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum BlockDefType {
    Input,
    Type,
    Enum,
    Interface,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockEntry {
    Field(BlockField),
    SpreadRef(Identifier),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockDef {
    pub span: OwnedSpan,
    pub name: Identifier,
    pub description: String,
    pub kind: BlockDefType,
    pub entries: Vec<BlockEntry>,
    pub directives: Vec<Directive>,
}

impl BlockDef {
    fn build(name: &str, kind: BlockDefType) -> Self {
        Self {
            span: OwnedSpan::default(),
            name: Identifier::from(name),
            description: "".to_string(),
            kind,
            entries: Vec::new(),
            directives: Vec::new(),
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
        self.entries.push(BlockEntry::Field(field));
        self.clone()
    }

    pub fn spread(&mut self, identifier: &str) -> Self {
        self.entries
            .push(BlockEntry::SpreadRef(Identifier::from(identifier)));
        self.clone()
    }

    pub fn directive(&mut self, directive: Directive) -> Self {
        self.directives.push(directive);
        self.clone()
    }
}

fn _parse_type_or_input(
    pair: Pair<Rule>,
    kind: BlockDefType,
) -> Result<BlockDef, pest::error::Error<Rule>> {
    let span = OwnedSpan::from(pair.as_span());
    let mut pairs = pair.into_inner();
    // [description?, identifier, directives*, selection_set]
    let DescriptionAndNext(description, next) = parse_description_and_continue(&mut pairs);
    let name = parse_identifier(next)?;

    let mut entries = Vec::new();
    let mut directives = Vec::new();
    for child in pairs {
        match child.as_rule() {
            Rule::directive => {
                directives.push(parse_directive(child)?);
            }
            // this means selection_set or spread_reference
            _ => {
                for pair in child.into_inner() {
                    match pair.as_rule() {
                        Rule::spread_reference => {
                            let spread = parse_identifier(pair.into_inner().next().unwrap())?;
                            entries.push(BlockEntry::SpreadRef(spread))
                        }
                        _ => {
                            let field = parse_block_field(pair.clone())?;
                            entries.push(BlockEntry::Field(field));
                        }
                    }
                }
            }
        }
    }
    Ok(BlockDef {
        span,
        name,
        description,
        kind,
        entries,
        directives,
    })
}

pub(crate) fn parse_block_def(pair: Pair<Rule>) -> Result<BlockDef, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::type_def => _parse_type_or_input(pair, BlockDefType::Type),
        Rule::input_def => _parse_type_or_input(pair, BlockDefType::Input),
        Rule::enum_def => _parse_type_or_input(pair, BlockDefType::Enum),
        Rule::interface_def => _parse_type_or_input(pair, BlockDefType::Interface),
        _unknown => Err(unknown_rule_error(
            pair,
            "type_def, input_def, enum_def or interface_def",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;
    use crate::ValueType;

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
    fn parses_type_with_spread() {
        assert_eq!(
            parse_input("type MyType { field1: String ...Type field2: String }"),
            Ok(BlockDef::type_("MyType")
                .field(BlockField::build("field1").string())
                .spread("Type")
                .field(BlockField::build("field2").string()))
        );
    }

    #[test]
    fn parses_input_with_spread() {
        assert_eq!(
            parse_input("input MyInput { field1: String ...Input field2: String }"),
            Ok(BlockDef::input("MyInput")
                .field(BlockField::build("field1").string())
                .spread("Input")
                .field(BlockField::build("field2").string()))
        );
    }

    #[test]
    fn parses_enum_with_spread() {
        assert_eq!(
            parse_input("enum MyEnum { field1 ...Enum field2 }"),
            Ok(BlockDef::enum_("MyEnum")
                .field(BlockField::build("field1"))
                .spread("Enum")
                .field(BlockField::build("field2")))
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
    fn test_accept_directives() {
        assert_eq!(
            parse_input("enum MyEnum @dir1 @dir2 { Field }"),
            Ok(BlockDef::enum_("MyEnum")
                .field(BlockField::build("Field"))
                .directive(Directive::build("dir1"))
                .directive(Directive::build("dir2")))
        );
    }

    #[test]
    fn test_accept_directives_in_fields_indented() {
        assert_eq!(
            parse_input("type MyType {\n  Field1: String!\n  Field2: Int @dir\n}"),
            Ok(BlockDef::type_("MyType")
                .field(BlockField::build("Field1").value_type(ValueType::string().non_nullable()))
                .field(
                    BlockField::build("Field2")
                        .int()
                        .directive(Directive::build("dir"))
                ))
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
