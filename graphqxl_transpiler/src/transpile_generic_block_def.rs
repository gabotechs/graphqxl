use std::collections::HashMap;
use std::error::Error;

use crate::modified_ref::transpile_modified_ref;
use graphqxl_parser::{BlockDef, GenericBlockDef};

pub(crate) fn transpile_generic_block_def(
    generic_block_def: &GenericBlockDef,
    store: &HashMap<String, BlockDef>,
) -> Result<BlockDef, Box<dyn Error>> {
    let mut resolved = transpile_modified_ref(&generic_block_def.modified_ref, store)?;
    resolved.name = generic_block_def.name.clone();
    resolved.kind = generic_block_def.kind.clone();
    resolved
        .directives
        .extend(generic_block_def.directives.clone());
    if !generic_block_def.description.is_empty() {
        resolved.description = generic_block_def.description.clone();
    }
    Ok(resolved)
}
