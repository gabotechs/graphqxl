use crate::ast_identifier::parse_identifier;
use crate::ast_value_basic_data::{parse_basic_data, ValueBasicData};
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueData {
    Basic(ValueBasicData),
    List(Vec<ValueData>),
    Object(HashMap<String, ValueData>),
}

pub(crate) fn parse_value_data(pair: Pair<Rule>) -> Result<ValueData, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::value_data => parse_value_data(pair.into_inner().next().unwrap()),
        Rule::object_data => {
            let mut data = HashMap::new();
            for entry in pair.into_inner() {
                let mut childs = entry.into_inner();
                let name = parse_identifier(childs.next().unwrap())?;
                let value = parse_value_data(childs.next().unwrap())?;
                data.insert(name, value);
            }
            Ok(ValueData::Object(data))
        }
        Rule::list_data => {
            let mut data = Vec::new();
            for entry in pair.into_inner() {
                let value = parse_value_data(entry)?;
                data.push(value);
            }
            Ok(ValueData::List(data))
        }
        Rule::basic_data => Ok(ValueData::Basic(parse_basic_data(pair)?)),
        _unknown => Err(unknown_rule_error(pair, "value_data")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<ValueData, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::value_data, parse_value_data)
    }

    #[test]
    fn test_simple() {
        let value = parse_input("1.0").unwrap();
        assert_eq!(value, ValueData::Basic(ValueBasicData::Float(1.0)))
    }

    #[test]
    fn test_list() {
        let value = parse_input("[1, 2]").unwrap();
        assert_eq!(
            value,
            ValueData::List(vec![
                ValueData::Basic(ValueBasicData::Int(1)),
                ValueData::Basic(ValueBasicData::Int(2))
            ])
        )
    }

    #[test]
    fn test_object() {
        let value = parse_input("{ a: 1, b: 2.0, c: \"\", d: false}").unwrap();
        assert_eq!(
            value,
            ValueData::Object(HashMap::from([
                ("a".to_string(), ValueData::Basic(ValueBasicData::Int(1))),
                (
                    "b".to_string(),
                    ValueData::Basic(ValueBasicData::Float(2.0))
                ),
                (
                    "c".to_string(),
                    ValueData::Basic(ValueBasicData::String("".into()))
                ),
                (
                    "d".to_string(),
                    ValueData::Basic(ValueBasicData::Boolean(false))
                ),
            ]))
        )
    }

    #[test]
    fn test_nested_object() {
        let value = parse_input("{ a: { b: { c: { d: true }} } }").unwrap();
        assert_eq!(
            value,
            ValueData::Object(HashMap::from([(
                "a".to_string(),
                ValueData::Object(HashMap::from([(
                    "b".to_string(),
                    ValueData::Object(HashMap::from([(
                        "c".to_string(),
                        ValueData::Object(HashMap::from([(
                            "d".to_string(),
                            ValueData::Basic(ValueBasicData::Boolean(true))
                        )]))
                    )]))
                )]))
            )]))
        )
    }

    #[test]
    fn test_nested_invalid_object() {
        parse_input("{ a: { b: { c { d: true }} } }").unwrap_err();
    }

    #[test]
    fn test_nested_list() {
        let value = parse_input("[[[1, 2]], [[3, 4]]]").unwrap();
        assert_eq!(
            value,
            ValueData::List(vec![
                ValueData::List(vec![ValueData::List(vec![
                    ValueData::Basic(ValueBasicData::Int(1)),
                    ValueData::Basic(ValueBasicData::Int(2))
                ])]),
                ValueData::List(vec![ValueData::List(vec![
                    ValueData::Basic(ValueBasicData::Int(3)),
                    ValueData::Basic(ValueBasicData::Int(4))
                ])])
            ])
        )
    }
}
