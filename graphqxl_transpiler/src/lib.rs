mod transpile_block_def;
mod utils;

use crate::transpile_block_def::transpile_block_def;
use graphqxl_parser::{DefType, Rule, Spec};

pub fn transpile_spec(spec: &Spec) -> Result<Spec, pest::error::Error<Rule>> {
    let mut target = Spec::default();

    for def in spec.order.iter() {
        match def {
            DefType::Type(name) => {
                let transpiled = transpile_block_def(name, &spec.types, 0)?;
                target.types.insert(name.id.clone(), transpiled);
                target.order.push(DefType::Type(name.clone()));
            }
            DefType::Input(name) => {
                let transpiled = transpile_block_def(name, &spec.inputs, 0)?;
                target.inputs.insert(name.id.clone(), transpiled);
                target.order.push(DefType::Input(name.clone()));
            }
            DefType::Enum(name) => {
                let transpiled = transpile_block_def(name, &spec.enums, 0)?;
                target.enums.insert(name.id.clone(), transpiled);
                target.order.push(DefType::Enum(name.clone()));
            }
            DefType::Interface(name) => {
                let transpiled = transpile_block_def(name, &spec.interfaces, 0)?;
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
    Ok(target)
}
