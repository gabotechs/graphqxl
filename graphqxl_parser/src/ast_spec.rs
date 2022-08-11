use crate::parser::Rule;
use crate::utils::{already_defined_error, unknown_rule_error};
use crate::{
    parse_block_def, parse_directive_def, parse_scalar, parse_schema, parse_union, BlockDef,
    DirectiveDef, Scalar, Schema, Union,
};
use pest::iterators::Pair;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum DefType {
    Type(String),
    Input(String),
    Enum(String),
    Interface(String),
    Scalar(String),
    Union(String),
    Directive(String),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Spec {
    pub types: HashMap<String, BlockDef>,
    pub inputs: HashMap<String, BlockDef>,
    pub enums: HashMap<String, BlockDef>,
    pub interfaces: HashMap<String, BlockDef>,
    pub scalars: HashMap<String, Scalar>,
    pub unions: HashMap<String, Union>,
    pub directives: HashMap<String, DirectiveDef>,
    pub order: Vec<DefType>,
    pub schema: Schema,
    schema_already_defined: bool,
}

impl Spec {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, pair: Pair<Rule>) -> Result<(), pest::error::Error<Rule>> {
        match pair.as_rule() {
            Rule::schema_def => {
                if self.schema_already_defined {
                    Err(pest::error::Error::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: "schema is defined multiple times".to_string(),
                        },
                        pair.as_span(),
                    ))
                } else {
                    self.schema_already_defined = true;
                    self.schema = parse_schema(pair)?;
                    Ok(())
                }
            }
            Rule::type_def => {
                let block_def = parse_block_def(pair.clone())?;
                let name = block_def.name.as_str();
                if self.types.contains_key(name) {
                    Err(already_defined_error(pair, "type", name))
                } else {
                    let name = name.to_string();
                    self.types.insert(name.clone(), block_def);
                    self.order.push(DefType::Type(name));
                    Ok(())
                }
            }
            Rule::input_def => {
                let block_def = parse_block_def(pair.clone())?;
                let name = block_def.name.as_str();
                if self.inputs.contains_key(name) {
                    Err(already_defined_error(pair, "input", name))
                } else {
                    let name = name.to_string();
                    self.inputs.insert(name.clone(), block_def);
                    self.order.push(DefType::Input(name));
                    Ok(())
                }
            }
            Rule::enum_def => {
                let block_def = parse_block_def(pair.clone())?;
                let name = block_def.name.as_str();
                if self.enums.contains_key(name) {
                    Err(already_defined_error(pair, "enum", name))
                } else {
                    let name = name.to_string();
                    self.enums.insert(name.clone(), block_def);
                    self.order.push(DefType::Enum(name));
                    Ok(())
                }
            }
            Rule::interface_def => {
                let block_def = parse_block_def(pair.clone())?;
                let name = block_def.name.as_str();
                if self.interfaces.contains_key(name) {
                    Err(already_defined_error(pair, "interface", name))
                } else {
                    let name = name.to_string();
                    self.interfaces.insert(name.clone(), block_def);
                    self.order.push(DefType::Interface(name));
                    Ok(())
                }
            }
            Rule::scalar_def => {
                let scalar = parse_scalar(pair.clone())?;
                let name = scalar.name.as_str();
                if self.scalars.contains_key(name) {
                    Err(already_defined_error(pair, "scalar", name))
                } else {
                    let name = name.to_string();
                    self.scalars.insert(name.clone(), scalar);
                    self.order.push(DefType::Scalar(name));
                    Ok(())
                }
            }
            Rule::union_def => {
                let union = parse_union(pair.clone())?;
                let name = union.name.as_str();
                if self.unions.contains_key(name) {
                    Err(already_defined_error(pair, "union", name))
                } else {
                    let name = name.to_string();
                    self.unions.insert(name.clone(), union);
                    self.order.push(DefType::Union(name));
                    Ok(())
                }
            }
            Rule::directive_def => {
                let directive = parse_directive_def(pair.clone())?;
                let name = directive.name.as_str();
                if self.directives.contains_key(name) {
                    Err(already_defined_error(pair, "directive", name))
                } else {
                    let name = name.to_string();
                    self.directives.insert(name.clone(), directive);
                    self.order.push(DefType::Directive(name));
                    Ok(())
                }
            }
            _unknown => Err(unknown_rule_error(
                pair,
                "type, input, enum, interface, scalar, union, directive",
            )),
        }
    }
}

pub fn parse_spec(pair: Pair<Rule>) -> Result<Spec, pest::error::Error<Rule>> {
    let mut spec = Spec::new();
    match pair.as_rule() {
        Rule::spec => {
            for child in pair.into_inner() {
                if let Rule::EOI = &child.as_rule() {
                    continue;
                }
                spec.add(child)?;
            }
            Ok(spec)
        }
        _unknown => Err(unknown_rule_error(pair, "spec")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;
    use std::fs;

    fn parse_input(input: &str) -> Result<Spec, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::spec, parse_spec)
    }

    #[test]
    fn test_parses_spec_1() {
        let content = fs::read_to_string("test_graphqxl_files/1.graphqxl").unwrap();
        let spec_or_err = parse_input(content.as_str());
        if let Err(err) = spec_or_err {
            panic!("Error parsing file: {:?}", err)
        }
    }
}
