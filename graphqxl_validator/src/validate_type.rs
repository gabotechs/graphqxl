use crate::utils::custom_error;
use graphqxl_parser::{BlockDef, BlockDefType, Rule, Spec};

pub(crate) fn validate_type(t: BlockDef, spec: Spec) -> Result<(), pest::error::Error<Rule>> {
    if t.kind != BlockDefType::Type {
        return Err(custom_error(&t.span, "expected a type"));
    }
    Ok(())
}
