use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueBasicData {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

pub(crate) fn _parse_basic_data(
    pair: Pair<Rule>,
) -> Result<ValueBasicData, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::int_data => Ok(ValueBasicData::Int(pair.as_str().parse::<i64>().unwrap())),
        Rule::float_data => Ok(ValueBasicData::Float(pair.as_str().parse::<f64>().unwrap())),
        Rule::string_data => Ok(ValueBasicData::String(pair.as_str().to_string())),
        Rule::boolean_data => Ok(ValueBasicData::Boolean(pair.as_str() == "true")),
        _unknown => Err(unknown_rule_error(
            pair,
            "int_data, float_data, string_data, boolean_data",
        )),
    }
}

pub(crate) fn parse_basic_data(
    pair: Pair<Rule>,
) -> Result<ValueBasicData, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::basic_data => _parse_basic_data(pair.into_inner().next().unwrap()),
        _unknown => Err(unknown_rule_error(
            pair,
            "int_data, float_data, string_data, boolean_data",
        )),
    }
}
