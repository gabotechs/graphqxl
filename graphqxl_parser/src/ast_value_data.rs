use crate::ast_identifier::parse_identifier;
use crate::ast_value_basic_data::{parse_basic_data, ValueBasicData};
use crate::parser::{Rule, RuleError};
use crate::utils::unknown_rule_error;
use indexmap::IndexMap;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueData {
    Basic(ValueBasicData),
    List(Vec<ValueData>),
    Object(IndexMap<String, ValueData>),
}

impl ValueData {
    pub fn build(data: ValueBasicData) -> Self {
        ValueData::Basic(data)
    }

    pub fn int(int: i64) -> Self {
        Self::build(ValueBasicData::Int(int))
    }

    pub fn float(float: f64) -> Self {
        Self::build(ValueBasicData::Float(float))
    }

    pub fn string(string: &str) -> Self {
        Self::build(ValueBasicData::String(string.to_string()))
    }

    pub fn boolean(boolean: bool) -> Self {
        Self::build(ValueBasicData::Boolean(boolean))
    }

    pub fn list(&mut self) -> Self {
        ValueData::List(vec![self.clone()])
    }

    pub fn to_object(&self, name: &str) -> Self {
        ValueData::Object(IndexMap::from([(name.to_string(), self.clone())]))
    }

    pub fn push(&mut self, other: Self) -> Self {
        if let ValueData::List(data) = self {
            data.push(other);
        }
        self.clone()
    }

    pub fn insert(&mut self, key: &str, other: Self) -> Self {
        if let ValueData::Object(data) = self {
            data.insert(key.to_string(), other);
        }
        self.clone()
    }
}

pub(crate) fn parse_value_data(pair: Pair<Rule>, file: &str) -> Result<ValueData, Box<RuleError>> {
    match pair.as_rule() {
        Rule::value_data => parse_value_data(pair.into_inner().next().unwrap(), file),
        Rule::object_data => {
            let mut data = IndexMap::new();
            for entry in pair.into_inner() {
                let mut childs = entry.into_inner();
                let name = parse_identifier(childs.next().unwrap(), file)?;
                let value = parse_value_data(childs.next().unwrap(), file)?;
                data.insert(name.id, value);
            }
            Ok(ValueData::Object(data))
        }
        Rule::list_data => {
            let mut data = Vec::new();
            for entry in pair.into_inner() {
                let value = parse_value_data(entry, file)?;
                data.push(value);
            }
            Ok(ValueData::List(data))
        }
        Rule::basic_data => Ok(ValueData::Basic(parse_basic_data(pair, file)?)),
        _unknown => Err(unknown_rule_error(pair, "value_data")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<ValueData, Box<RuleError>> {
        parse_full_input(input, Rule::value_data, parse_value_data)
    }

    #[test]
    fn test_simple() {
        assert_eq!(parse_input("1.0"), Ok(ValueData::float(1.0)));
    }

    #[test]
    fn test_list() {
        assert_eq!(
            parse_input("[1, 2]"),
            Ok(ValueData::int(1).list().push(ValueData::int(2)))
        );
    }

    #[test]
    fn test_object() {
        assert_eq!(
            parse_input("{ a: 1, b: 2.0, c: \"\", d: false}"),
            Ok(ValueData::int(1)
                .to_object("a")
                .insert("b", ValueData::float(2.0))
                .insert("c", ValueData::string(""))
                .insert("d", ValueData::boolean(false)))
        );
    }

    #[test]
    fn test_nested_object() {
        assert_eq!(
            parse_input("{ a: { b: { c: { d: true }} } }"),
            Ok(ValueData::boolean(true)
                .to_object("d")
                .to_object("c")
                .to_object("b")
                .to_object("a"))
        );
    }

    #[test]
    fn test_nested_invalid_object() {
        parse_input("{ a: { b: { c { d: true }} } }").unwrap_err();
    }

    #[test]
    fn test_nested_list() {
        let mut one_two = ValueData::int(1).list().push(ValueData::int(2));
        let mut three_four = ValueData::int(3).list().push(ValueData::int(4));

        assert_eq!(
            parse_input("[[[1, 2]], [[3, 4]]]"),
            Ok(one_two.list().list().push(three_four.list()))
        );
    }
}
