use crate::ast_value::{parse_value, AnyValue};
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub value: AnyValue,
}

pub(crate) fn parse_field(pair: Pair<Rule>) -> Result<Field, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::field => {
            let mut pairs = pair.into_inner();
            let identifier = pairs.next().unwrap();
            let value = pairs.next().unwrap();
            return Ok(Field {
                name: identifier.as_str().into(),
                value: parse_value(value)?,
            });
        }
        _unknown => Err(unknown_rule_error(pair, "field")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_value::{Value, ValueArray};
    use crate::ast_value_content::ValueContent;
    use crate::parser::GraphqlParser;
    use pest::error::InputLocation;
    use pest::Parser;

    #[test]
    fn test_parse_string_field() {
        let input = "field: String";
        let mut pair = GraphqlParser::parse(Rule::field, input).unwrap();
        let field = parse_field(pair.next().unwrap()).unwrap();
        assert_eq!(field.name, String::from("field"));
        assert_eq!(
            field.value,
            AnyValue::Simple(Value {
                nullable: true,
                content: ValueContent::String
            })
        );
    }

    #[test]
    fn test_parse_array_string_field() {
        let input = "field: [String!]!";
        let mut pair = GraphqlParser::parse(Rule::field, input).unwrap();
        let field = parse_field(pair.next().unwrap()).unwrap();
        assert_eq!(field.name, String::from("field"));
        assert_eq!(
            field.value,
            AnyValue::Array(ValueArray {
                value: Value {
                    nullable: false,
                    content: ValueContent::String
                },
                nullable: false
            })
        );
    }

    #[test]
    fn test_do_not_parse_invalid() {
        let input = "field: [String!!";
        let err = GraphqlParser::parse(Rule::field, input).unwrap_err();
        assert_eq!(err.location, InputLocation::Pos(8))
    }
}
