use std::collections::HashMap;

use graphqxl_parser::{
    BlockDef, BlockEntry, GenericBlockDef, Identifier, Rule, ValueBasicType, ValueType,
};

pub(crate) fn transpile_generic_block_def(
    generic_block_def: &GenericBlockDef,
    store: &HashMap<String, BlockDef>,
) -> Result<BlockDef, pest::error::Error<Rule>> {
    let unresolved_block_def = match store.get(&generic_block_def.block_def.id) {
        Some(block_def) => block_def,
        None => {
            return Err(generic_block_def
                .block_def
                .span
                .make_error(&format!("{} is undefined", &generic_block_def.block_def.id)));
        }
    };

    let generic = match &unresolved_block_def.generic {
        Some(generic) => generic,
        None => {
            return Err(generic_block_def.block_def.span.make_error(&format!(
                "{} is not a generic block template",
                &generic_block_def.block_def.id
            )));
        }
    };

    if generic.args.len() != generic_block_def.generic_call.args.len() {
        return Err(generic_block_def
            .block_def
            .span
            .make_error("generic arguments length does not match"));
    }

    let mut generic_map = HashMap::new();
    for i in 0..generic.args.len() {
        generic_map.insert(
            &generic.args.get(i).unwrap().id,
            generic_block_def.generic_call.args.get(i).unwrap(),
        );
    }

    Ok(resolve_block_def(
        unresolved_block_def,
        &generic_block_def.name,
        generic_map,
    ))
}

fn resolve_block_def(
    unresolved_block_def: &BlockDef,
    name: &Identifier,
    generic_map: HashMap<&String, &ValueType>,
) -> BlockDef {
    let mut resolved_block_def = unresolved_block_def.clone();
    resolved_block_def.generic = None;
    resolved_block_def.name = name.clone();
    for entry in resolved_block_def.entries.iter_mut() {
        // if it is a field...
        if let BlockEntry::Field(block_field) = entry {
            // ...and has a type...
            if let Some(value_type) = &block_field.value_type {
                let basic_value_type = value_type.retrieve_basic_type();
                // ...and that type is an object...
                if let ValueBasicType::Object(object) = basic_value_type {
                    // ...which is stored in the generic map...
                    if let Some(generic_replacement) = generic_map.get(object) {
                        let replacement = *generic_replacement;
                        // ...then replace it
                        block_field.value_type = Some(replacement.clone());
                    }
                }
            }
        }
    }
    resolved_block_def
}

#[cfg(test)]
mod tests {
    use graphqxl_parser::{BlockField, Generic, GenericBlockDef, ValueType};

    use super::*;

    fn build_store(block_defs: Vec<BlockDef>) -> HashMap<String, BlockDef> {
        let mut store = HashMap::new();
        for block_def in block_defs {
            store.insert(block_def.name.id.clone(), block_def);
        }
        store
    }

    #[test]
    fn test_replaces_generic() {
        let generic = GenericBlockDef::type_def("Type", "Template", ValueType::string());
        let store = build_store(vec![BlockDef::type_def("Template")
            .generic(Generic::from("T"))
            .field(BlockField::build("field").value_type(ValueType::object("T")))]);
        let block_def = transpile_generic_block_def(&generic, &store).unwrap();

        assert_eq!(
            block_def,
            BlockDef::type_def("Type")
                .field(BlockField::build("field").value_type(ValueType::string()))
        )
    }
}
