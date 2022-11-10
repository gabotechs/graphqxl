use graphqxl_parser::{BlockDef, Spec};
use std::collections::HashMap;

pub(crate) struct GraphqxlTranspiler<'a> {
    pub(crate) source: &'a Spec,
    pub(crate) types_store: HashMap<String, BlockDef>,
    pub(crate) inputs_store: HashMap<String, BlockDef>,
}

impl<'a> From<&'a Spec> for GraphqxlTranspiler<'a> {
    fn from(source: &'a Spec) -> Self {
        let mut types_store = source.types.clone();
        types_store.extend(source.interfaces.clone());
        let inputs_store = source.inputs.clone();

        Self {
            source,
            types_store,
            inputs_store,
        }
    }
}
