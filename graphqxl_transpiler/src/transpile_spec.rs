use crate::transpile_block_def::{transpile_block_def_by_block, transpile_block_def_by_id};
use crate::transpile_generic_block_def::transpile_generic_block_def;
use crate::transpiler::GraphqxlTranspiler;
use graphqxl_parser::{DefType, Spec};
use std::error::Error;

impl<'a> GraphqxlTranspiler<'a> {
    pub(crate) fn transpile(&mut self) -> Result<Spec, Box<dyn Error>> {
        let mut target = Spec::default();

        for def in self.source.order.iter() {
            match def {
                DefType::Type(name) => {
                    let transpiled = transpile_block_def_by_id(name, &self.types_store)?;
                    if transpiled.generic.is_none() {
                        target.types.insert(name.id.clone(), transpiled);
                        target.order.push(DefType::Type(name.clone()));
                    } 
                }
                DefType::GenericType(name) => {
                    let generic_type =
                        if let Some(generic_type) = self.source.generic_types.get(&name.id) {
                            generic_type
                        } else {
                            return Err(name.span.make_error("generic type not found"));
                        };
                    let resolved = transpile_generic_block_def(generic_type, &self.types_store)?;
                    let transpiled = transpile_block_def_by_block(&resolved, &self.types_store)?;
                    self.types_store.insert(name.id.clone(), transpiled.clone());
                    target.types.insert(name.id.clone(), transpiled);
                    target.order.push(DefType::Type(name.clone()));
                }
                DefType::Input(name) => {
                    let transpiled = transpile_block_def_by_id(&name, &self.inputs_store)?;
                    if transpiled.generic.is_none() {
                        target.inputs.insert(name.id.clone(), transpiled);
                        target.order.push(DefType::Input(name.clone()));
                    }
                }
                DefType::GenericInput(name) => {
                    let generic_input =
                        if let Some(generic_input) = self.source.generic_inputs.get(&name.id) {
                            generic_input
                        } else {
                            return Err(name.span.make_error("generic input not found"));
                        };
                    let resolved = transpile_generic_block_def(generic_input, &self.inputs_store)?;
                    let transpiled = transpile_block_def_by_block(&resolved, &self.inputs_store)?;
                    self.inputs_store
                        .insert(name.id.clone(), transpiled.clone());
                    target.inputs.insert(name.id.clone(), transpiled);
                    target.order.push(DefType::Input(name.clone()));
                }
                DefType::Enum(name) => {
                    let transpiled = transpile_block_def_by_id(name, &self.source.enums)?;
                    target.enums.insert(name.id.clone(), transpiled);
                    target.order.push(DefType::Enum(name.clone()));
                }
                DefType::Interface(name) => {
                    let transpiled = transpile_block_def_by_id(name, &self.source.interfaces)?;
                    target.interfaces.insert(name.id.clone(), transpiled);
                    target.order.push(DefType::Interface(name.clone()));
                }
                DefType::Scalar(name) => {
                    let transpiled = self.source.scalars.get(&name.id).unwrap();
                    target.scalars.insert(name.id.clone(), transpiled.clone());
                    target.order.push(DefType::Scalar(name.clone()));
                }
                DefType::Union(name) => {
                    let transpiled = self.source.unions.get(&name.id).unwrap();
                    target.unions.insert(name.id.clone(), transpiled.clone());
                    target.order.push(DefType::Union(name.clone()));
                }
                DefType::Directive(name) => {
                    let transpiled = self.source.directives.get(&name.id).unwrap();
                    target
                        .directives
                        .insert(name.id.clone(), transpiled.clone());
                    target.order.push(DefType::Directive(name.clone()));
                }
            }
        }
        target.schema = self.source.schema.clone();
        Ok(target)
    }
}
