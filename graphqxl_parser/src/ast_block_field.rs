use crate::ast_arguments::{parse_arguments, Argument};
use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_identifier::parse_identifier;
use crate::ast_value_type::{parse_value_type, ValueType};
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use crate::{parse_directive, Directive};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlockField {
    pub name: String,
    pub description: String,
    pub value_type: Option<ValueType>,
    pub args: Vec<Argument>,
    pub directives: Vec<Directive>,
}

impl BlockField {
    pub fn build(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn value_type(&mut self, value_type: ValueType) -> Self {
        self.value_type = Some(value_type);
        self.clone()
    }

    pub fn int(&mut self) -> Self {
        self.value_type = Some(ValueType::int());
        self.clone()
    }

    pub fn float(&mut self) -> Self {
        self.value_type = Some(ValueType::float());
        self.clone()
    }

    pub fn string(&mut self) -> Self {
        self.value_type = Some(ValueType::string());
        self.clone()
    }

    pub fn boolean(&mut self) -> Self {
        self.value_type = Some(ValueType::boolean());
        self.clone()
    }

    pub fn object(&mut self, object_name: &str) -> Self {
        self.value_type = Some(ValueType::object(object_name));
        self.clone()
    }

    pub fn description(&mut self, description: &str) -> Self {
        self.description = description.to_string();
        self.clone()
    }

    pub fn arg(&mut self, arg: Argument) -> Self {
        self.args.push(arg);
        self.clone()
    }

    pub fn directive(&mut self, directive: Directive) -> Self {
        self.directives.push(directive);
        self.clone()
    }
}

fn _parse_block_field(pair: Pair<Rule>) -> Result<BlockField, pest::error::Error<Rule>> {
    // at this moment we are on [type_field|input_field], both will work
    let mut pairs = pair.into_inner();
    // at this moment we are on [description?, identifier, args?, value?]
    let DescriptionAndNext(description, next) = parse_description_and_continue(&mut pairs);
    let name = parse_identifier(next)?;
    let mut block_field = BlockField {
        name,
        description,
        ..Default::default()
    };
    let value_or_args_or_nothing = pairs.next();
    if let Some(value_or_args) = value_or_args_or_nothing {
        let mut value = value_or_args.clone();
        if let Rule::arguments = value_or_args.as_rule() {
            block_field.args = parse_arguments(value_or_args)?;
            value = pairs.next().unwrap();
        }
        if let Rule::value_type = value.as_rule() {
            block_field.value_type = Some(parse_value_type(value)?)
        } else if let Rule::directive = value.as_rule() {
            block_field.directives.push(parse_directive(value)?);
        }
    }
    for child in pairs {
        block_field.directives.push(parse_directive(child)?);
    }
    Ok(block_field)
}

pub(crate) fn parse_block_field(pair: Pair<Rule>) -> Result<BlockField, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::field_with_args => _parse_block_field(pair),
        Rule::field_without_args => _parse_block_field(pair),
        Rule::field_without_args_without_value => _parse_block_field(pair),
        _unknown => Err(unknown_rule_error(pair, "field")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_value_data::ValueData;
    use crate::utils::parse_full_input;

    fn parse_with_args_input(input: &str) -> Result<BlockField, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::field_with_args, parse_block_field)
    }

    fn parse_without_args_input(input: &str) -> Result<BlockField, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::field_without_args, parse_block_field)
    }

    fn parse_without_args_without_value_input(
        input: &str,
    ) -> Result<BlockField, pest::error::Error<Rule>> {
        parse_full_input(
            input,
            Rule::field_without_args_without_value,
            parse_block_field,
        )
    }

    #[test]
    fn test_type_accept_args() {
        parse_with_args_input("field(arg1: Int, arg2: String!): String").unwrap();
    }

    #[test]
    fn test_input_do_not_accept_args() {
        parse_without_args_input("field(arg1: Int, arg2: String!): String").unwrap_err();
    }

    #[test]
    fn test_enum_do_not_accept_args() {
        parse_without_args_without_value_input("field(arg1: Int, arg2: String!): String")
            .unwrap_err();
    }

    #[test]
    fn test_accepts_description() {
        assert_eq!(
            parse_with_args_input("\"\"\" my description \"\"\" field(arg: String): String"),
            Ok(BlockField::build("field")
                .string()
                .arg(Argument::string("arg"))
                .description("my description"))
        );
    }

    #[test]
    fn test_parse_string_block_field() {
        assert_eq!(
            parse_with_args_input("field: String"),
            Ok(BlockField::build("field").string())
        );
    }

    #[test]
    fn test_parse_array_string_block_field() {
        assert_eq!(
            parse_with_args_input("field: [String!]!"),
            Ok(BlockField::build("field")
                .value_type(ValueType::string().non_nullable().array().non_nullable()))
        );
    }

    #[test]
    fn test_parse_block_field_args_one_arg() {
        assert_eq!(
            parse_with_args_input("field(arg1: String): String"),
            Ok(BlockField::build("field")
                .string()
                .arg(Argument::string("arg1")))
        );
    }

    #[test]
    fn test_parse_block_field_args_two_args_and_one_default() {
        assert_eq!(
            parse_with_args_input("field(arg1: [String]! = [\"default\"], arg2: Float!): String"),
            Ok(BlockField::build("field")
                .string()
                .arg(
                    Argument::build("arg1", ValueType::string().array().non_nullable())
                        .default(ValueData::string("default").list())
                )
                .arg(Argument::build("arg2", ValueType::float().non_nullable())))
        );
    }

    #[test]
    fn test_without_args_without_value_input_accepts_directive() {
        assert_eq!(
            parse_without_args_without_value_input("field @dir1 @dir2"),
            Ok(BlockField::build("field")
                .directive(Directive::build("dir1"))
                .directive(Directive::build("dir2")))
        );
    }

    #[test]
    fn test_with_args_accepts_directives() {
        assert_eq!(
            parse_with_args_input("field: [String!]! @dir1 @dir2"),
            Ok(BlockField::build("field")
                .directive(Directive::build("dir1"))
                .directive(Directive::build("dir2"))
                .value_type(ValueType::string().non_nullable().array().non_nullable()))
        );
    }

    #[test]
    fn test_with_args_with_default_value_accepts_directives() {
        assert_eq!(
            parse_with_args_input("field(arg1: String, arg2: Int): [String!]! @dir1 @dir2"),
            Ok(BlockField::build("field")
                .arg(Argument::string("arg1"))
                .arg(Argument::int("arg2"))
                .directive(Directive::build("dir1"))
                .directive(Directive::build("dir2"))
                .value_type(ValueType::string().non_nullable().array().non_nullable()))
        );
    }

    #[test]
    fn test_do_not_parse_invalid() {
        parse_with_args_input("field: [String!!").unwrap_err();
    }
}
