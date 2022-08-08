extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast_arguments;
mod ast_block_def;
mod ast_block_field;
mod ast_description;
mod ast_directive_def;
mod ast_directive_location;
mod ast_identifier;
mod ast_scalar;
mod ast_spec;
mod ast_union;
mod ast_value_basic_data;
mod ast_value_basic_type;
mod ast_value_data;
mod ast_value_type;
mod parser;
mod utils;

pub use ast_arguments::*;
pub use ast_block_def::*;
pub use ast_block_field::*;
use pest::Parser;
// pub use ast_description::*;
pub use ast_directive_def::*;
pub use ast_directive_location::*;
// pub use ast_identifier::*;
use crate::parser::{GraphqlParser, Rule};
pub use ast_scalar::*;
pub use ast_spec::*;
pub use ast_union::*;
pub use ast_value_basic_data::*;
pub use ast_value_basic_type::*;
pub use ast_value_data::*;
pub use ast_value_type::*;

pub fn parse_graphqxl(input: &str) -> Result<Spec, pest::error::Error<Rule>> {
    let mut pairs = GraphqlParser::parse(Rule::spec, input)?;
    if let Some(pair) = pairs.next() {
        parse_spec(pair)
    } else {
        Ok(Spec::default())
    }
}
