use crate::resolve_expandable_ref::resolve_expandable_ref;
use crate::utils::BlockDefStore;
use graphqxl_parser::{
    BlockDef, BlockField, Directive, Implements, ModifiedRef, OwnedSpan, ValueType,
};
use std::error::Error;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ResolvedRef {
    pub span: OwnedSpan,
    pub implements: Option<Implements>,
    pub description: String,
    pub fields: Vec<BlockField>,
    pub directives: Vec<Directive>,
}

impl ResolvedRef {
    pub(crate) fn init(value: &BlockDef) -> Self {
        Self {
            span: value.span.clone(),
            implements: value.implements.clone(),
            description: value.description.clone(),
            fields: vec![],
            directives: value.directives.clone(),
        }
    }
}

fn nullable(resolved_ref: &ResolvedRef) -> ResolvedRef {
    let mut optional_block_def = resolved_ref.clone();
    for field in optional_block_def.fields.iter_mut() {
        if let Some(value_type) = &mut field.value_type {
            if let ValueType::NonNullable(inner) = value_type {
                *value_type = inner.deref().deref().deref().clone()
            }
        }
    }
    optional_block_def
}

fn non_nullable(resolved_ref: &ResolvedRef) -> ResolvedRef {
    let mut required_block_def = resolved_ref.clone();
    for field in required_block_def.fields.iter_mut() {
        if let Some(value_type) = &mut field.value_type {
            if let ValueType::NonNullable(_) = value_type {
                // do nothing here
            } else {
                *value_type = ValueType::NonNullable(Box::new(value_type.clone()))
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

const MAX_RECURSION_DEPTH: usize = 100;

pub(crate) fn resolve_modified_ref_with_context(
    modified_ref: &ModifiedRef,
    store: &BlockDefStore,
    stack_context: ModifiedRefStackContext,
) -> Result<ResolvedRef, Box<dyn Error>> {
    if stack_context.stack_count > MAX_RECURSION_DEPTH {
        return Err(modified_ref
            .span()
            .make_error("maximum nested spread operator surpassed"));
    }
    match modified_ref {
        ModifiedRef::Required(modified_ref, _) => Ok(non_nullable(
            &resolve_modified_ref_with_context(modified_ref, store, stack_context.plus_1())?,
        )),
        ModifiedRef::Optional(modified_ref, _) => Ok(nullable(&resolve_modified_ref_with_context(
            modified_ref,
            store,
            stack_context.plus_1(),
        )?)),
        ModifiedRef::ExpandableRef(expandable_ref) => Ok(resolve_expandable_ref(
            expandable_ref,
            store,
            stack_context.plus_1(),
        )?),
    }
}

pub(crate) fn resolve_modified_ref(
    modified_ref: &ModifiedRef,
    store: &BlockDefStore,
) -> Result<ResolvedRef, Box<dyn Error>> {
    resolve_modified_ref_with_context(modified_ref, store, ModifiedRefStackContext::default())
}
