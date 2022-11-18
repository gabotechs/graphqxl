use crate::resolve_modified_ref::{
    resolve_modified_ref_with_context, ModifiedRefStackContext, ResolvedRef,
};
use crate::transpile_description::transpile_description;
use crate::utils::BlockDefStore;
use graphqxl_parser::{BlockEntry, ExpandableRef, ValueBasicType};
use std::collections::HashMap;
use std::error::Error;

const VARIABLES_PREFIX: &str = "variables";

pub(crate) fn resolve_expandable_ref(
    expandable_ref: &ExpandableRef,
    store: &BlockDefStore,
    stack_context: ModifiedRefStackContext,
) -> Result<ResolvedRef, Box<dyn Error>> {
    let referenced_block_def = match store.get(&expandable_ref.identifier.id) {
        Some(block_def) => block_def,
        None => {
            return Err(expandable_ref
                .identifier
                .span
                .make_error(&format!("{} is undefined", &expandable_ref.identifier.id)));
        }
    };

    let empty_args = vec![];
    let generic_args = match &referenced_block_def.generic {
        Some(generic) => &generic.args,
        None => &empty_args,
    };

    let generic_referenced_block_def = referenced_block_def;

    let empty_call_args = vec![];
    let generic_call_args = match &expandable_ref.generic_call {
        Some(generic_call) => &generic_call.args,
        None => &empty_call_args,
    };

    if generic_args.len() != generic_call_args.len() {
        return Err(expandable_ref.span.make_error(&format!(
            "Instantiated generic type with {} args when {} where expected",
            generic_call_args.len(),
            generic_args.len()
        )));
    }

    let mut generic_map = HashMap::new();
    for i in 0..generic_args.len() {
        generic_map.insert(
            &generic_args.get(i).unwrap().id,
            generic_call_args.get(i).unwrap(),
        );
    }

    let mut resolved_ref = ResolvedRef::init(generic_referenced_block_def);

    let mut description_replacements = HashMap::new();
    for (key, value) in generic_map.iter() {
        description_replacements.insert(
            format!("{VARIABLES_PREFIX}.{key}"),
            format!("{}", value.retrieve_basic_type()),
        );
    }

    transpile_description(&mut resolved_ref, &description_replacements, true)?;

    let mut new_fields = vec![];

    for entry in generic_referenced_block_def.entries.iter() {
        let new_entry = entry.clone();
        // if it is a field...
        match new_entry {
            BlockEntry::Field(mut block_field) => {
                transpile_description(&mut block_field, &description_replacements, true)?;
                // ...and has a type...
                if let Some(value_type) = &mut block_field.value_type {
                    let basic_value_type = value_type.retrieve_basic_type();
                    // ...and that type is an object...
                    if let ValueBasicType::Object(object) = basic_value_type {
                        // ...which is stored in the generic map...
                        if let Some(generic_replacement) = generic_map.get(&object.id) {
                            // ...then replace it
                            value_type.replace_basic_type((*generic_replacement).clone());
                        }
                    }
                }
                new_fields.push(block_field)
            }
            BlockEntry::SpreadRef(modified_ref) => {
                // NOTE: Careful here, recursive brain exploding ahead
                let resolved_ref = resolve_modified_ref_with_context(
                    &modified_ref,
                    store,
                    stack_context.plus_1(),
                )?;
                new_fields.extend_from_slice(&resolved_ref.fields);
            }
        }
    }
    resolved_ref.fields = new_fields;
    Ok(resolved_ref)
}
