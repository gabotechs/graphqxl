extern crate core;

use graphqxl_parser::Spec;
use std::error::Error;

use crate::transpiler::GraphqxlTranspiler;

mod transpile_block_def;
mod transpile_description;
mod transpile_generic_block_def;
mod transpile_spec;
mod transpiler;
mod modified_ref;

pub fn transpile_spec(spec: &Spec) -> Result<Spec, Box<dyn Error>> {
    GraphqxlTranspiler::from(spec).transpile()
}
