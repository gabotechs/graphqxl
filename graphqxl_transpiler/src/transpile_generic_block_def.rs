use std::collections::HashMap;

use crate::transpile_block_def::{BLOCK_NAME, BLOCK_TYPE};
use crate::transpile_description::transpile_description;
use graphqxl_parser::{BlockDef, BlockEntry, GenericBlockDef, Rule, ValueBasicType, ValueType};

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

    resolve_block_def(unresolved_block_def, generic_block_def, generic_map)
}

fn resolve_block_def(
    unresolved_block_def: &BlockDef,
    generic_block_def: &GenericBlockDef,
    generic_map: HashMap<&String, &ValueType>,
) -> Result<BlockDef, pest::error::Error<Rule>> {
    let mut resolved_block_def = unresolved_block_def.clone();
    resolved_block_def.generic = None;
    resolved_block_def.name = generic_block_def.name.clone();

    let mut description_replacements = HashMap::from([
        (BLOCK_NAME.to_string(), generic_block_def.name.id.clone()),
        (
            BLOCK_TYPE.to_string(),
            format!("{}", generic_block_def.kind),
        ),
    ]);
    for (key, value) in generic_map.iter() {
        description_replacements.insert(
            format!("variables.{key}"),
            format!("{}", value.retrieve_basic_type()),
        );
    }

    if !generic_block_def.description.is_empty() {
        resolved_block_def.description = generic_block_def.description.clone()
    } else {
        transpile_description(&mut resolved_block_def, &description_replacements)?;
    }

    for entry in resolved_block_def.entries.iter_mut() {
        // if it is a field...
        if let BlockEntry::Field(block_field) = entry {
            transpile_description(block_field, &description_replacements)?;
            // ...and has a type...
            if let Some(value_type) = &block_field.value_type {
                let basic_value_type = value_type.retrieve_basic_type();
                // ...and that type is an object...
                if let ValueBasicType::Object(object) = basic_value_type {
                    // ...which is stored in the generic map...
                    if let Some(generic_replacement) = generic_map.get(&object.id) {
                        let replacement = *generic_replacement;
                        // ...then replace it
                        block_field.value_type = Some(replacement.clone());
                    }
                }
            }
        }
    }
    Ok(resolved_block_def)
}

#[cfg(test)]
mod tests {
    use graphqxl_parser::{BlockField, Generic, GenericBlockDef, Identifier, ValueType};

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
            .field(
                BlockField::build("field").value_type(ValueType::object(Identifier::from("T"))),
            )]);
        let block_def = transpile_generic_block_def(&generic, &store).unwrap();

        assert_eq!(
            block_def,
            BlockDef::type_def("Type")
                .field(BlockField::build("field").value_type(ValueType::string()))
        )
    }
}
