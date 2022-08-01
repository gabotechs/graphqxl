use crate::ast_field::{parse_field, Field};
use crate::utils::unknown_rule_error;
use crate::Rule;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum BlockDefType {
    Input,
    Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockDef {
    pub name: String,
    pub kind: BlockDefType,
    pub fields: HashMap<String, Field>,
}

fn _parse_type_or_input(
    mut pairs: Pairs<Rule>,
    kind: BlockDefType,
) -> Result<BlockDef, pest::error::Error<Rule>> {
    // at this point pairs is [identifier, selection_set]...
    let name = pairs.next().unwrap().as_str().to_string();
    let mut fields = HashMap::new();
    // ...so we need to move into selection_set to iter the inner fields
    let pairs = pairs.next().unwrap().into_inner();
    for pair in pairs {
        let field = parse_field(pair)?;
        fields.insert(field.name.clone(), field);
    }
    Ok(BlockDef { name, kind, fields })
}

pub(crate) fn parse_block_def(pair: Pair<Rule>) -> Result<BlockDef, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::type_def => _parse_type_or_input(pair.into_inner(), BlockDefType::Type),
        Rule::input_def => _parse_type_or_input(pair.into_inner(), BlockDefType::Input),
        _unknown => Err(unknown_rule_error(pair, "type_def or input_def")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_type_input(input: &str) -> Result<BlockDef, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::type_def, parse_block_def)
    }

    fn parse_input_input(input: &str) -> Result<BlockDef, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::input_def, parse_block_def)
    }

    #[test]
    fn parses_empty_type() {
        let t = parse_type_input("type MyType { }").unwrap();
        assert_eq!(t.name, "MyType");
        assert_eq!(t.kind, BlockDefType::Type);
        assert_eq!(t.fields.len(), 0);
    }

    #[test]
    fn parses_empty_input() {
        let t = parse_input_input("input MyInput { }").unwrap();
        assert_eq!(t.name, "MyInput");
        assert_eq!(t.kind, BlockDefType::Input);
        assert_eq!(t.fields.len(), 0);
    }

    #[test]
    fn parses_filled_with_spaces_type() {
        let t = parse_type_input("type MyType { field: String }").unwrap();
        assert_eq!(t.name, "MyType");
        assert_eq!(t.kind, BlockDefType::Type);
        assert_eq!(t.fields.len(), 1);
        assert_ne!(t.fields.get("field"), None);
    }

    #[test]
    fn parses_filled_with_line_jumps_type() {
        let t = parse_type_input("type MyType { \nfield: String\n }").unwrap();
        assert_eq!(t.name, "MyType");
        assert_eq!(t.kind, BlockDefType::Type);
        assert_eq!(t.fields.len(), 1);
        assert_ne!(t.fields.get("field"), None);
    }

    #[test]
    fn do_not_parse_invalid_input_1() {
        parse_type_input("type MyType { field: String- }").unwrap_err();
    }

    #[test]
    fn do_not_parse_invalid_input_2() {
        parse_type_input("type MyType { fi'eld: String }").unwrap_err();
    }

    #[test]
    fn do_not_parse_invalid_input_3() {
        parse_type_input("type MyT-ype { field: String }").unwrap_err();
    }

    #[test]
    fn do_not_parse_invalid_input_4() {
        parse_type_input("type_ MyType { field: String }").unwrap_err();
    }
}
