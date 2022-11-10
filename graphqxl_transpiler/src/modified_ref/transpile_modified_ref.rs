use crate::modified_ref::transpile_expandable_ref::transpile_expandable_ref;
use graphqxl_parser::{BlockDef, BlockEntry, ModifiedRef, ValueType};
use std::collections::HashMap;
use std::error::Error;
use std::ops::Deref;

fn nullable(block_def: &BlockDef) -> BlockDef {
    let mut optional_block_def = block_def.clone();
    for entry in optional_block_def.entries.iter_mut() {
        if let BlockEntry::Field(field) = entry {
            if let Some(value_type) = &mut field.value_type {
                if let ValueType::NonNullable(inner) = value_type {
                    *value_type = inner.deref().deref().deref().clone()
                }
            }
        }
    }
    optional_block_def
}

fn non_nullable(block_def: &BlockDef) -> BlockDef {
    let mut required_block_def = block_def.clone();
    for entry in required_block_def.entries.iter_mut() {
        if let BlockEntry::Field(field) = entry {
            if let Some(value_type) = &mut field.value_type {
                if let ValueType::NonNullable(_) = value_type {
                    // do nothing here
                } else {
                    *value_type = ValueType::NonNullable(Box::new(value_type.clone()))
                }
            }
        }
    }
    required_block_def
}

#[derive(Default, Clone)]
pub(crate) struct ModifiedRefStackContext {
    stack_count: usize,
}

impl ModifiedRefStackContext {
    pub(crate) fn plus_1(&self) -> Self {
        let mut clone = self.clone();
        clone.stack_count += 1;
        clone
    }
}

pub(crate) fn _transpile_modified_ref(
    modified_ref: &ModifiedRef,
    store: &HashMap<String, BlockDef>,
    stack_context: ModifiedRefStackContext,
) -> Result<BlockDef, Box<dyn Error>> {
    // todo: where does this come from
    if stack_context.stack_count > 100 {
        return Err(modified_ref
            .span()
            .make_error("maximum nested spread operator surpassed"));
    }
    match modified_ref {
        ModifiedRef::Required(modified_ref, _) => Ok(non_nullable(&_transpile_modified_ref(
            modified_ref,
            store,
            stack_context.plus_1(),
        )?)),
        ModifiedRef::Optional(modified_ref, _) => Ok(nullable(&_transpile_modified_ref(
            modified_ref,
            store,
            stack_context.plus_1(),
        )?)),
        ModifiedRef::ExpandableRef(expandable_ref) => Ok(transpile_expandable_ref(
            expandable_ref,
            store,
            stack_context.plus_1(),
        )?),
    }
}

pub(crate) fn transpile_modified_ref(
    modified_ref: &ModifiedRef,
    store: &HashMap<String, BlockDef>,
) -> Result<BlockDef, Box<dyn Error>> {
    _transpile_modified_ref(modified_ref, store, ModifiedRefStackContext::default())
}
