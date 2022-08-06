use crate::parser::Rule;
use crate::utils::{already_defined_error, unknown_rule_error};
use crate::{
    parse_block_def, parse_directive_def, parse_scalar, parse_union, BlockDef, DirectiveDef,
    Scalar, Union,
};
use pest::iterators::Pair;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Spec {
    types: HashMap<String, BlockDef>,
    inputs: HashMap<String, BlockDef>,
    enums: HashMap<String, BlockDef>,
    interfaces: HashMap<String, BlockDef>,
    scalars: HashMap<String, Scalar>,
    unions: HashMap<String, Union>,
    directives: HashMap<String, DirectiveDef>,
}

impl Spec {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, pair: Pair<Rule>) -> Result<(), pest::error::Error<Rule>> {
        match pair.as_rule() {
            Rule::type_def => {
                let block_def = parse_block_def(pair.clone())?;
                let name = block_def.name.as_str();
                if self.types.contains_key(name) {
                    Err(already_defined_error(pair, "type", name))
                } else {
                    self.types.insert(name.to_string(), block_def);
                    Ok(())
                }
            }
            Rule::input_def => {
                let block_def = parse_block_def(pair.clone())?;
                let name = block_def.name.as_str();
                if self.inputs.contains_key(name) {
                    Err(already_defined_error(pair, "input", name))
                } else {
                    self.inputs.insert(name.to_string(), block_def);
                    Ok(())
                }
            }
            Rule::enum_def => {
                let block_def = parse_block_def(pair.clone())?;
                let name = block_def.name.as_str();
                if self.enums.contains_key(name) {
                    Err(already_defined_error(pair, "enum", name))
                } else {
                    self.enums.insert(name.to_string(), block_def);
                    Ok(())
                }
            }
            Rule::interface_def => {
                let block_def = parse_block_def(pair.clone())?;
                let name = block_def.name.as_str();
                if self.interfaces.contains_key(name) {
                    Err(already_defined_error(pair, "interface", name))
                } else {
                    self.interfaces.insert(name.to_string(), block_def);
                    Ok(())
                }
            }
            Rule::scalar_def => {
                let scalar = parse_scalar(pair.clone())?;
                let name = scalar.name.as_str();
                if self.scalars.contains_key(name) {
                    Err(already_defined_error(pair, "scalar", name))
                } else {
                    self.scalars.insert(name.to_string(), scalar);
                    Ok(())
                }
            }
            Rule::union_def => {
                let union = parse_union(pair.clone())?;
                let name = union.name.as_str();
                if self.unions.contains_key(name) {
                    Err(already_defined_error(pair, "union", name))
                } else {
                    self.unions.insert(name.to_string(), union);
                    Ok(())
                }
            }
            Rule::directive_def => {
                let directive = parse_directive_def(pair.clone())?;
                let name = directive.name.as_str();
                if self.directives.contains_key(name) {
                    Err(already_defined_error(pair, "directive", name))
                } else {
                    self.directives.insert(name.to_string(), directive);
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
        let content = fs::read_to_string("test_graphqxl_files/1.graphql").unwrap();
        let spec_or_err = parse_input(content.as_str());
        if let Err(err) = spec_or_err {
            panic!("Error parsing file: {:?}", err)
        }
    }
}
