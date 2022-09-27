use graphqxl_parser::{DefType, Rule, Spec};

use crate::transpile_block_def::{transpile_block_def, IdOrBlock};
use crate::transpile_generic_block_def::transpile_generic_block_def;
use crate::utils::custom_error;

mod transpile_block_def;
mod transpile_generic_block_def;
mod utils;

pub fn transpile_spec(spec: &Spec) -> Result<Spec, pest::error::Error<Rule>> {
    let mut target = Spec::default();

    for def in spec.order.iter() {
        match def {
            DefType::Type(name) => {
                let transpiled = transpile_block_def(IdOrBlock::Id(name.clone()), &spec.types, 0)?;
                target.types.insert(name.id.clone(), transpiled);
                target.order.push(DefType::Type(name.clone()));
            }
            DefType::GenericType(name) => {
                let generic_type = if let Some(generic_type) = spec.generic_types.get(&name.id) {
                    generic_type
                } else {
                    return Err(custom_error(&name.span, "generic type not found"));
                };
                let resolved = transpile_generic_block_def(generic_type, &spec.types)?;
                let transpiled = transpile_block_def(IdOrBlock::Block(resolved), &spec.types, 0)?;
                target.types.insert(name.id.clone(), transpiled);
                target.order.push(DefType::Type(name.clone()));
            }
            DefType::Input(name) => {
                let transpiled = transpile_block_def(IdOrBlock::Id(name.clone()), &spec.inputs, 0)?;
                target.inputs.insert(name.id.clone(), transpiled);
                target.order.push(DefType::Input(name.clone()));
            }
            DefType::Enum(name) => {
                let transpiled = transpile_block_def(IdOrBlock::Id(name.clone()), &spec.enums, 0)?;
                target.enums.insert(name.id.clone(), transpiled);
                target.order.push(DefType::Enum(name.clone()));
            }
            DefType::GenericInput(name) => {
                let generic_input = if let Some(generic_input) = spec.generic_inputs.get(&name.id) {
                    generic_input
                } else {
                    return Err(custom_error(&name.span, "generic input not found"));
                };
                let resolved = transpile_generic_block_def(generic_input, &spec.inputs)?;
                let transpiled = transpile_block_def(IdOrBlock::Block(resolved), &spec.types, 0)?;
                target.types.insert(name.id.clone(), transpiled);
                target.order.push(DefType::Type(name.clone()));
            }
            DefType::Interface(name) => {
                let transpiled =
                    transpile_block_def(IdOrBlock::Id(name.clone()), &spec.interfaces, 0)?;
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
