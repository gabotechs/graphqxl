mod utils;

use graphqxl_parser::{Rule, Spec};

pub fn validate_spec(_spec: &Spec) -> Result<(), pest::error::Error<Rule>> {
    Ok(())
}
