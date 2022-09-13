use crate::ast_import::parse_import;
use crate::parser::{GraphqxlParser, Rule};
use crate::utils::{already_defined_error, custom_error, unknown_rule_error};
use crate::{
    parse_block_def, parse_directive_def, parse_scalar, parse_schema, parse_union, BlockDef,
    DirectiveDef, Identifier, Scalar, Schema, Union,
};
use pest::iterators::Pair;
use pest::{Parser, Span};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

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

    fn merge(&mut self, other: Spec) -> Result<(), pest::error::Error<Rule>> {
        for el in other.order.iter() {
            match el {
                DefType::Type(name) => {
                    if self.types.contains_key(&name.id) {
                        return Err(custom_error(Span::from(&name.span), "Duplicated type"));
                    }
                    self.order.push(el.clone());
                    self.types.insert(
                        name.id.to_string(),
                        other.types.get(&name.id).unwrap().clone(),
                    );
                }
                DefType::Input(name) => {
                    if self.inputs.contains_key(&name.id) {
                        return Err(custom_error(Span::from(&name.span), "Duplicated input"));
                    }
                    self.order.push(el.clone());
                    self.inputs.insert(
                        name.id.to_string(),
                        other.inputs.get(&name.id).unwrap().clone(),
                    );
                }
                DefType::Enum(name) => {
                    if self.enums.contains_key(&name.id) {
                        return Err(custom_error(Span::from(&name.span), "Duplicated enum"));
                    }
                    self.order.push(el.clone());
                    self.enums.insert(
                        name.id.to_string(),
                        other.enums.get(&name.id).unwrap().clone(),
                    );
                }
                DefType::Interface(name) => {
                    if self.interfaces.contains_key(&name.id) {
                        return Err(custom_error(Span::from(&name.span), "Duplicated interface"));
                    }
                    self.order.push(el.clone());
                    self.interfaces.insert(
                        name.id.to_string(),
                        other.interfaces.get(&name.id).unwrap().clone(),
                    );
                }
                DefType::Scalar(name) => {
                    if self.scalars.contains_key(&name.id) {
                        return Err(custom_error(Span::from(&name.span), "Duplicated scalar"));
                    }
                    self.order.push(el.clone());
                    self.scalars.insert(
                        name.id.to_string(),
                        other.scalars.get(&name.id).unwrap().clone(),
                    );
                }
                DefType::Union(name) => {
                    if self.unions.contains_key(&name.id) {
                        return Err(custom_error(Span::from(&name.span), "Duplicated union"));
                    }
                    self.order.push(el.clone());
                    self.unions.insert(
                        name.id.to_string(),
                        other.unions.get(&name.id).unwrap().clone(),
                    );
                }
                DefType::Directive(name) => {
                    if self.directives.contains_key(&name.id) {
                        return Err(custom_error(Span::from(&name.span), "Duplicated directive"));
                    }
                    self.order.push(el.clone());
                    self.directives.insert(
                        name.id.to_string(),
                        other.directives.get(&name.id).unwrap().clone(),
                    );
                }
            }
        }
        Ok(())
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

pub fn parse_spec<P: AsRef<Path>>(path: P) -> Result<Spec, Box<dyn Error>> {
    let abs_path = fs::canonicalize(path)?;

    let mut spec = Spec::new();
    let content = fs::read_to_string(&abs_path)?;
    let mut pairs = GraphqxlParser::parse(Rule::spec, &content)?;
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::spec => {
            for child in pair.into_inner() {
                if let Rule::EOI = &child.as_rule() {
                    // nothing to do here
                } else if let Rule::import = &child.as_rule() {
                    let import = parse_import(child.clone())?;
                    let file_name = if import.file_name.ends_with(".graphqxl") {
                        import.file_name
                    } else {
                        import.file_name + ".graphqxl"
                    };
                    let file_dir = abs_path.parent().unwrap();
                    let import_path = Path::new(file_dir).join(&file_name);
                    if !import_path.exists() {
                        return Err(Box::new(custom_error(
                            Span::from(&import.span),
                            format!("file {:?} does not exist", import_path).as_str(),
                        )));
                    }
                    let imported_spec = parse_spec(import_path)?;
                    spec.merge(imported_spec)?;
                } else {
                    spec.add(child)?;
                }
            }
            Ok(spec)
        }
        _unknown => Err(Box::new(unknown_rule_error(pair, "spec"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_spec_1() {
        let spec_or_err = parse_spec("test_graphqxl_files/1.graphqxl");
        if let Err(err) = spec_or_err {
            panic!("Error parsing file: {}", err)
        }
    }
}
