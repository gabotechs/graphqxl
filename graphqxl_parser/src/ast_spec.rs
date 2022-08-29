use crate::parser::Rule;
use crate::utils::{already_defined_error, unknown_rule_error};
use crate::{parse_block_def, parse_directive_def, parse_scalar, parse_schema, parse_union, BlockDef, DirectiveDef, Scalar, Schema, Union, Identifier};
use pest::iterators::Pair;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum DefType {
    Type(Identifier),
    Input(Identifier),
    Enum(Identifier),
    Interface(Identifier),
    Scalar(Identifier),
    Union(Identifier),
    Directive(Identifier),
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
                let id = block_def.name.clone();
                if self.types.contains_key(&id.id) {
                    Err(already_defined_error(pair, "type", &id.id))
                } else {
                    self.types.insert(id.id.clone(), block_def);
                    self.order.push(DefType::Type(id));
                    Ok(())
                }
            }
            Rule::input_def => {
                let block_def = parse_block_def(pair.clone())?;
                let id = block_def.name.clone();
                if self.inputs.contains_key(&id.id) {
                    Err(already_defined_error(pair, "input", &id.id))
                } else {
                    self.inputs.insert(id.id.clone(), block_def);
                    self.order.push(DefType::Input(id));
                    Ok(())
                }
            }
            Rule::enum_def => {
                let block_def = parse_block_def(pair.clone())?;
                let id = block_def.name.clone();
                if self.enums.contains_key(&id.id) {
                    Err(already_defined_error(pair, "enum", &id.id))
                } else {
                    self.enums.insert(id.id.clone(), block_def);
                    self.order.push(DefType::Enum(id));
                    Ok(())
                }
            }
            Rule::interface_def => {
                let block_def = parse_block_def(pair.clone())?;
                let id = block_def.name.clone();
                if self.interfaces.contains_key(&id.id) {
                    Err(already_defined_error(pair, "interface", &id.id))
                } else {
                    self.interfaces.insert(id.id.clone(), block_def);
                    self.order.push(DefType::Interface(id));
                    Ok(())
                }
            }
            Rule::scalar_def => {
                let scalar = parse_scalar(pair.clone())?;
                let id = scalar.name.clone();
                if self.scalars.contains_key(&id.id) {
                    Err(already_defined_error(pair, "scalar", &id.id))
                } else {
                    self.scalars.insert(id.id.clone(), scalar);
                    self.order.push(DefType::Scalar(id));
                    Ok(())
                }
            }
            Rule::union_def => {
                let union = parse_union(pair.clone())?;
                let id = union.name.clone();
                if self.unions.contains_key(&id.id) {
                    Err(already_defined_error(pair, "union", &id.id))
                } else {
                    self.unions.insert(id.id.clone(), union);
                    self.order.push(DefType::Union(id));
                    Ok(())
                }
            }
            Rule::directive_def => {
                let directive = parse_directive_def(pair.clone())?;
                let id = directive.name.clone();
                if self.directives.contains_key(&id.id) {
                    Err(already_defined_error(pair, "directive", &id.id))
                } else {
                    self.directives.insert(id.id.clone(), directive);
                    self.order.push(DefType::Directive(id));
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
