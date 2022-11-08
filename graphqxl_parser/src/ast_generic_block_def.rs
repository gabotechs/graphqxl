use pest::iterators::Pair;

use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_directive::parse_directive;
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use crate::{
    parse_generic_call, parse_identifier, BlockDefType, Directive, GenericCall, Identifier,
    OwnedSpan, ValueType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GenericBlockDef {
    pub span: OwnedSpan,
    pub description: String,
    pub kind: BlockDefType,
    pub name: Identifier,
    pub directives: Vec<Directive>,
    pub block_def: Identifier,
    pub generic_call: GenericCall,
}

impl GenericBlockDef {
    fn from(kind: BlockDefType, name: &str, block_def: &str, arg: ValueType) -> Self {
        GenericBlockDef {
            kind,
            description: "".to_string(),
            directives: vec![],
            span: OwnedSpan::default(),
            name: Identifier::from(name),
            block_def: Identifier::from(block_def),
            generic_call: GenericCall::from(arg),
        }
    }

    pub fn type_def(name: &str, block_def: &str, arg: ValueType) -> Self {
        Self::from(BlockDefType::Type, name, block_def, arg)
    }

    pub fn input_def(name: &str, block_def: &str, arg: ValueType) -> Self {
        Self::from(BlockDefType::Input, name, block_def, arg)
    }

    pub fn description(&mut self, text: &str) -> Self {
        self.description = text.to_string();
        self.clone()
    }

    pub fn arg(&mut self, arg: ValueType) -> Self {
        self.generic_call.arg(arg);
        self.clone()
    }

    pub fn directive(&mut self, directive: Directive) -> Self {
        self.directives.push(directive);
        self.clone()
    }
}

fn _parse_generic_block_def(
    kind: BlockDefType,
    pair: Pair<Rule>,
    file: &str,
) -> Result<GenericBlockDef, pest::error::Error<Rule>> {
    let span = OwnedSpan::from(pair.as_span(), file);
    let mut childs = pair.into_inner();
    let DescriptionAndNext(description, next) = parse_description_and_continue(&mut childs, file);
    let name = parse_identifier(next, file)?;
    let mut directives = vec![];
    let mut child = childs.next().unwrap();
    while let Rule::directive = &child.as_rule() {
        directives.push(parse_directive(child.clone(), file)?);
        child = childs.next().unwrap();
    }
    let block_def = parse_identifier(child, file)?;
    let generic_call = parse_generic_call(childs.next().unwrap(), file)?;

    Ok(GenericBlockDef {
        kind,
        description,
        span,
        directives,
        name,
        block_def,
        generic_call,
    })
}

pub(crate) fn parse_generic_block_def(
    pair: Pair<Rule>,
    file: &str,
) -> Result<GenericBlockDef, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::generic_type_def => _parse_generic_block_def(BlockDefType::Type, pair, file),
        Rule::generic_input_def => _parse_generic_block_def(BlockDefType::Input, pair, file),
        _unknown => Err(unknown_rule_error(
            pair,
            "generic_type_def, generic_input_def",
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::parse_full_input;

    use super::*;

    fn parse_input(input: &str) -> Result<GenericBlockDef, pest::error::Error<Rule>> {
        let rule = if input.contains("input ") {
            Rule::generic_input_def
        } else {
            Rule::generic_type_def
        };
        parse_full_input(input, rule, parse_generic_block_def)
    }

    #[test]
    fn test_parses_generic_type_def_with_one_arg() {
        assert_eq!(
            parse_input("type MyType = OtherType<String>"),
            Ok(GenericBlockDef::type_def(
                "MyType",
                "OtherType",
                ValueType::string(),
            ))
        )
    }

    #[test]
    fn test_parses_generic_type_def_with_description() {
        assert_eq!(
            parse_input("\"description\"type MyType = OtherType<String>"),
            Ok(
                GenericBlockDef::type_def("MyType", "OtherType", ValueType::string(),)
                    .description("description")
            )
        )
    }

    #[test]
    fn test_parses_generic_type_def_with_description_and_directive() {
        assert_eq!(
            parse_input("\"description\"type MyType @dir = OtherType<String>"),
            Ok(
                GenericBlockDef::type_def("MyType", "OtherType", ValueType::string(),)
                    .description("description")
                    .directive(Directive::build("dir"))
            )
        )
    }

    #[test]
    fn test_parses_generic_input_def_with_two_args() {
        assert_eq!(
            parse_input("input MyType = OtherType<String Int>"),
            Ok(
                GenericBlockDef::input_def("MyType", "OtherType", ValueType::string())
                    .arg(ValueType::int())
            )
        )
    }

    #[test]
    fn test_parses_generic_input_def_with_two_args_and_comma() {
        assert_eq!(
            parse_input("type Instanced = Template<[Float!]! Boolean>"),
            Ok(GenericBlockDef::type_def(
                "Instanced",
                "Template",
                ValueType::float().non_nullable().array().non_nullable(),
            )
            .arg(ValueType::boolean()))
        )
    }

    #[test]
    fn test_do_not_parses_without_generic_call() {
        parse_input("type MyType = OtherType").unwrap_err();
    }
}
