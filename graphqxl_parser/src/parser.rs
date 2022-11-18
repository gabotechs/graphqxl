#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GraphqxlParser;

pub type RuleError = pest::error::Error<Rule>;
