use crate::utils::custom_error;
use graphqxl_parser::{BlockDef, BlockEntry, Identifier, Rule};
use std::collections::HashMap;

pub(crate) fn transpile_block_def(
    identifier: &Identifier,
    store: &HashMap<String, BlockDef>,
    stack_count: usize,
) -> Result<BlockDef, pest::error::Error<Rule>> {
    // todo: where does this come from
    if stack_count > 100 {
        return Err(custom_error(
            &identifier.span,
            "maximum nested spread operator surpassed",
        ));
    }
    let block_def_option = store.get(&identifier.id);
    if block_def_option.is_none() {
        return Err(custom_error(
            &identifier.span,
            &format!("{} is undefined", &identifier.id),
        ));
    }
    let block_def = block_def_option.unwrap();

    let mut transpiled_block_def = block_def.clone();
    transpiled_block_def.entries.clear();
    for entry in block_def.entries.iter() {
        if let BlockEntry::SpreadRef(identifier) = entry {
            let referenced_type = transpile_block_def(identifier, store, stack_count + 1)?;
            for imported_entry in referenced_type.entries.iter() {
                transpiled_block_def.entries.push(imported_entry.clone())
            }
        } else {
            transpiled_block_def.entries.push(entry.clone());
        }
    }
    Ok(transpiled_block_def)
}
