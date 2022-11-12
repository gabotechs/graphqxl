use crate::transpile_block_def::{transpile_block_def_by_block, transpile_block_def_by_id};
use crate::transpile_generic_block_def::transpile_generic_block_def;
use crate::utils::BlockDefStore;
use graphqxl_parser::{DefType, Spec};
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct TranspileSpecOptions {
    pub private_prefix: String,
}

pub fn transpile_spec(spec: &Spec, options: &TranspileSpecOptions) -> Result<Spec, Box<dyn Error>> {
    let mut target = Spec::default();
    let mut transpiled_store = HashMap::new();

    for def in spec.order.iter() {
        let types_block_def_store = BlockDefStore::from((
            &spec.types,
            &transpiled_store,
            &spec.interfaces,
            &spec.inputs,
        ));

        let inputs_block_def_store =
            BlockDefStore::from((&spec.inputs, &transpiled_store, &spec.types));

        let enums_block_def_store = BlockDefStore::from(&spec.enums);

        let interfaces_block_def_store = BlockDefStore::from(&spec.interfaces);

        match def {
            DefType::Type(name) => {
                if name.id.starts_with(&options.private_prefix) {
                    continue;
                }
                let transpiled = transpile_block_def_by_id(name, &types_block_def_store)?;
                if transpiled.generic.is_none() {
                    target.types.insert(name.id.clone(), transpiled);
                    target.order.push(DefType::Type(name.clone()));
                }
            }
            DefType::GenericType(name) => {
                let generic_type = if let Some(generic_type) = spec.generic_types.get(&name.id) {
                    generic_type
                } else {
                    return Err(name.span.make_error("generic type not found"));
                };
                let resolved = transpile_generic_block_def(generic_type, &types_block_def_store)?;
                let transpiled = transpile_block_def_by_block(&resolved, &types_block_def_store)?;
                transpiled_store.insert(name.id.clone(), transpiled.clone());
                target.types.insert(name.id.clone(), transpiled);
                target.order.push(DefType::Type(name.clone()));
            }
            DefType::Input(name) => {
                if name.id.starts_with(&options.private_prefix) {
                    continue;
                }
                let transpiled = transpile_block_def_by_id(name, &inputs_block_def_store)?;
                if transpiled.generic.is_none() {
                    target.inputs.insert(name.id.clone(), transpiled);
                    target.order.push(DefType::Input(name.clone()));
                }
            }
            DefType::GenericInput(name) => {
                let generic_input = if let Some(generic_input) = spec.generic_inputs.get(&name.id) {
                    generic_input
                } else {
                    return Err(name.span.make_error("generic input not found"));
                };
                let resolved = transpile_generic_block_def(generic_input, &inputs_block_def_store)?;
                let transpiled = transpile_block_def_by_block(&resolved, &inputs_block_def_store)?;
                transpiled_store.insert(name.id.clone(), transpiled.clone());
                target.inputs.insert(name.id.clone(), transpiled);
                target.order.push(DefType::Input(name.clone()));
            }
            DefType::Enum(name) => {
                let transpiled = transpile_block_def_by_id(name, &enums_block_def_store)?;
                target.enums.insert(name.id.clone(), transpiled);
                target.order.push(DefType::Enum(name.clone()));
            }
            DefType::Interface(name) => {
                let transpiled = transpile_block_def_by_id(name, &interfaces_block_def_store)?;
                target.interfaces.insert(name.id.clone(), transpiled);
                target.order.push(DefType::Interface(name.clone()));
            }
            DefType::Scalar(name) => {
                let transpiled = spec.scalars.get(&name.id).unwrap();
                target.scalars.insert(name.id.clone(), transpiled.clone());
                target.order.push(DefType::Scalar(name.clone()));
            }
            DefType::Union(name) => {
                let transpiled = spec.unions.get(&name.id).unwrap();
                target.unions.insert(name.id.clone(), transpiled.clone());
                target.order.push(DefType::Union(name.clone()));
            }
            DefType::Directive(name) => {
                let transpiled = spec.directives.get(&name.id).unwrap();
                target
                    .directives
                    .insert(name.id.clone(), transpiled.clone());
                target.order.push(DefType::Directive(name.clone()));
            }
        }
    }
    target.schema = spec.schema.clone();
    Ok(target)
}
