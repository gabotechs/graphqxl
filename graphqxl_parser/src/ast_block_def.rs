use crate::ast_block_field::{parse_block_field, BlockField};
use crate::ast_description::parse_description;
use crate::ast_description_variables::{parse_description_variables, DescriptionVariables};
use crate::ast_identifier::{parse_identifier, Identifier};
use crate::ast_implements::{parse_implements, Implements};
use crate::ast_modified_ref::{parse_modified_ref, ModifiedRef};
use crate::parser::{Rule, RuleError};
use crate::utils::{unknown_rule_error, OwnedSpan};
use crate::{parse_directive, parse_generic, Directive, Generic};
use pest::iterators::Pair;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum BlockDefType {
    Input,
    Type,
    Enum,
    Interface,
}

impl Display for BlockDefType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BlockDefType::Input => "input",
                BlockDefType::Type => "type",
                BlockDefType::Enum => "enum",
                BlockDefType::Interface => "interface",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockEntry {
    Field(BlockField),
    SpreadRef(ModifiedRef),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockDef {
    pub span: OwnedSpan,
    pub name: Identifier,
    pub generic: Option<Generic>,
    pub implements: Option<Implements>,
    pub description: String,
    pub description_variables: Option<DescriptionVariables>,
    pub kind: BlockDefType,
    pub entries: Vec<BlockEntry>,
    pub directives: Vec<Directive>,
}

impl BlockDef {
    fn build(name: &str, kind: BlockDefType) -> Self {
        Self {
            span: OwnedSpan::default(),
            name: Identifier::from(name),
            generic: None,
            implements: None,
            description: "".to_string(),
            kind,
            description_variables: None,
            entries: Vec::new(),
            directives: Vec::new(),
        }
    }

    pub fn type_def(name: &str) -> Self {
        Self::build(name, BlockDefType::Type)
    }

    pub fn input_def(name: &str) -> Self {
        Self::build(name, BlockDefType::Input)
    }

    pub fn enum_def(name: &str) -> Self {
        Self::build(name, BlockDefType::Enum)
    }

    pub fn interface_def(name: &str) -> Self {
        Self::build(name, BlockDefType::Interface)
    }

    pub fn generic(&mut self, generic: Generic) -> Self {
        self.generic = Some(generic);
        self.clone()
    }

    pub fn implements(&mut self, implements: Implements) -> Self {
        self.implements = Some(implements);
        self.clone()
    }

    pub fn description(&mut self, description: &str) -> Self {
        self.description = description.to_string();
        self.clone()
    }

    pub fn description_variable(&mut self, variable: (&str, &str)) -> Self {
        self.description_variables = Some(match &mut self.description_variables {
            Some(current) => current.variable(variable),
            None => DescriptionVariables::build(variable),
        });
        self.clone()
    }

    pub fn field(&mut self, field: BlockField) -> Self {
        self.entries.push(BlockEntry::Field(field));
        self.clone()
    }

    pub fn spread(&mut self, modified_ref: ModifiedRef) -> Self {
        self.entries.push(BlockEntry::SpreadRef(modified_ref));
        self.clone()
    }

    pub fn directive(&mut self, directive: Directive) -> Self {
        self.directives.push(directive);
        self.clone()
    }
}

fn _parse_block_def(
    pair: Pair<Rule>,
    kind: BlockDefType,
    file: &str,
) -> Result<BlockDef, Box<RuleError>> {
    let span = OwnedSpan::from(pair.as_span(), file);
    let mut pairs = pair.into_inner();
    // [description_variables?, description?, identifier, directives*, selection_set]
    let mut pair = pairs.next().unwrap();

    let mut description_variables: Option<DescriptionVariables> = None;
    if let Rule::description_variables = pair.as_rule() {
        description_variables = Some(parse_description_variables(pair, file)?);
        pair = pairs.next().unwrap();
    }
    let mut description = "".to_string();
    if let Rule::description = pair.as_rule() {
        description = parse_description(pair, file)?;
        pair = pairs.next().unwrap();
    }

    let name = parse_identifier(pair, file)?;

    let mut entries = Vec::new();
    let mut directives = Vec::new();
    let mut generic: Option<Generic> = None;
    let mut implements: Option<Implements> = None;
    for child in pairs {
        match child.as_rule() {
            Rule::generic => {
                generic = Some(parse_generic(child, file)?);
            }
            Rule::implements => {
                implements = Some(parse_implements(child, file)?);
            }
            Rule::directive => {
                directives.push(parse_directive(child, file)?);
            }
            // this means selection_set or spread_reference
            _ => {
                for pair in child.into_inner() {
                    match pair.as_rule() {
                        Rule::spread_reference => {
                            let spread =
                                parse_modified_ref(pair.into_inner().next().unwrap(), file)?;
                            entries.push(BlockEntry::SpreadRef(spread))
                        }
                        _ => {
                            let field = parse_block_field(pair.clone(), file)?;
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
        generic,
        implements,
        description_variables,
        description,
        kind,
        entries,
        directives,
    })
}

pub(crate) fn parse_block_def(pair: Pair<Rule>, file: &str) -> Result<BlockDef, Box<RuleError>> {
    match pair.as_rule() {
        Rule::type_def => _parse_block_def(pair, BlockDefType::Type, file),
        Rule::input_def => _parse_block_def(pair, BlockDefType::Input, file),
        Rule::enum_def => _parse_block_def(pair, BlockDefType::Enum, file),
        Rule::interface_def => _parse_block_def(pair, BlockDefType::Interface, file),
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

    fn parse_input(input: &str) -> Result<BlockDef, Box<RuleError>> {
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
            Ok(BlockDef::type_def("MyType")
                .description("my description")
                .field(BlockField::build("arg").string()))
        );
    }

    #[test]
    fn test_input_description_works() {
        assert_eq!(
            parse_input("\"\"\" my description \"\"\"input MyInput { arg: String }"),
            Ok(BlockDef::input_def("MyInput")
                .description("my description")
                .field(BlockField::build("arg").string()))
        );
    }

    #[test]
    fn test_enum_description_works() {
        assert_eq!(
            parse_input("\"\"\" my description \"\"\"enum MyEnum { arg }"),
            Ok(BlockDef::enum_def("MyEnum")
                .description("my description")
                .field(BlockField::build("arg")))
        );
    }

    #[test]
    fn test_interface_description_works() {
        assert_eq!(
            parse_input("\"\"\" my description \"\"\"interface MyInterface { arg: String }"),
            Ok(BlockDef::interface_def("MyInterface")
                .description("my description")
                .field(BlockField::build("arg").string()))
        );
    }

    #[test]
    fn test_description_variables_works() {
        assert_eq!(
            parse_input("${foo: \"bar\"}type MyType { arg: String }"),
            Ok(BlockDef::type_def("MyType")
                .description_variable(("foo", "bar"))
                .field(BlockField::build("arg").string()))
        );
    }

    #[test]
    fn test_description_variables_with_description_works() {
        assert_eq!(
            parse_input("${foo: \"bar\"}\"\"\" my description \"\"\"type MyType { arg: String }"),
            Ok(BlockDef::type_def("MyType")
                .description("my description")
                .description_variable(("foo", "bar"))
                .field(BlockField::build("arg").string()))
        );
    }

    #[test]
    fn parses_empty_type() {
        assert_eq!(
            parse_input("type MyType { }"),
            Ok(BlockDef::type_def("MyType"))
        );
    }

    #[test]
    fn parses_empty_input() {
        assert_eq!(
            parse_input("input MyInput { }"),
            Ok(BlockDef::input_def("MyInput"))
        );
    }

    #[test]
    fn parses_empty_enum() {
        assert_eq!(
            parse_input("enum MyEnum { }"),
            Ok(BlockDef::enum_def("MyEnum"))
        );
    }

    #[test]
    fn parses_type_with_spread() {
        assert_eq!(
            parse_input("type MyType { field1: String ...Type field2: String }"),
            Ok(BlockDef::type_def("MyType")
                .field(BlockField::build("field1").string())
                .spread(ModifiedRef::build("Type"))
                .field(BlockField::build("field2").string()))
        );
    }

    #[test]
    fn parses_input_with_spread() {
        assert_eq!(
            parse_input("input MyInput { field1: String ...Input field2: String }"),
            Ok(BlockDef::input_def("MyInput")
                .field(BlockField::build("field1").string())
                .spread(ModifiedRef::build("Input"))
                .field(BlockField::build("field2").string()))
        );
    }

    #[test]
    fn parses_enum_with_spread() {
        assert_eq!(
            parse_input("enum MyEnum { field1 ...Enum field2 }"),
            Ok(BlockDef::enum_def("MyEnum")
                .field(BlockField::build("field1"))
                .spread(ModifiedRef::build("Enum"))
                .field(BlockField::build("field2")))
        );
    }

    #[test]
    fn parses_filled_with_spaces_type() {
        assert_eq!(
            parse_input("type MyType { field: String }"),
            Ok(BlockDef::type_def("MyType").field(BlockField::build("field").string()))
        );
    }

    #[test]
    fn parses_filled_with_line_jumps_type() {
        assert_eq!(
            parse_input("type MyType { \nfield: String\n }"),
            Ok(BlockDef::type_def("MyType").field(BlockField::build("field").string()))
        );
    }

    #[test]
    fn test_parses_enum() {
        assert_eq!(
            parse_input("enum MyEnum { Field }"),
            Ok(BlockDef::enum_def("MyEnum").field(BlockField::build("Field")))
        );
    }

    #[test]
    fn test_parses_enum_multiple_fields() {
        assert_eq!(
            parse_input("enum MyEnum { Field1\n field2 }"),
            Ok(BlockDef::enum_def("MyEnum")
                .field(BlockField::build("Field1"))
                .field(BlockField::build("field2")))
        );
    }

    #[test]
    fn test_accept_directives() {
        assert_eq!(
            parse_input("enum MyEnum @dir1 @dir2 { Field }"),
            Ok(BlockDef::enum_def("MyEnum")
                .field(BlockField::build("Field"))
                .directive(Directive::build("dir1"))
                .directive(Directive::build("dir2")))
        );
    }

    #[test]
    fn test_accept_directives_in_fields_indented() {
        assert_eq!(
            parse_input("type MyType {\n  Field1: String!\n  Field2: Int @dir\n}"),
            Ok(BlockDef::type_def("MyType")
                .field(BlockField::build("Field1").value_type(ValueType::string().non_nullable()))
                .field(
                    BlockField::build("Field2")
                        .int()
                        .directive(Directive::build("dir"))
                ))
        );
    }

    #[test]
    fn test_type_accepts_generic_arg() {
        assert_eq!(
            parse_input("type MyType<T> { field: String }"),
            Ok(BlockDef::type_def("MyType")
                .generic(Generic::from("T"))
                .field(BlockField::build("field").string()))
        )
    }

    #[test]
    fn test_type_accepts_generic_arg_and_implements() {
        assert_eq!(
            parse_input("type MyType<T> implements One & Two { field: String }"),
            Ok(BlockDef::type_def("MyType")
                .generic(Generic::from("T"))
                .implements(Implements::from("One").interface("Two"))
                .field(BlockField::build("field").string()))
        )
    }

    #[test]
    fn test_input_accepts_generic_arg() {
        assert_eq!(
            parse_input("input MyInput<T> { field: String }"),
            Ok(BlockDef::input_def("MyInput")
                .generic(Generic::from("T"))
                .field(BlockField::build("field").string()))
        )
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
