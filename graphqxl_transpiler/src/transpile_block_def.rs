use crate::resolve_modified_ref::resolve_modified_ref;
use crate::transpile_description::transpile_description;
use crate::utils::BlockDefStore;
use graphqxl_parser::{BlockDef, BlockEntry, Identifier};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::string::ToString;

enum IdOrBlock<'a> {
    Id(&'a Identifier),
    Block(&'a BlockDef),
}

impl<'a> IdOrBlock<'a> {
    fn block_def<'b>(&'b self, store: &'b BlockDefStore) -> Result<&'b BlockDef, Box<dyn Error>> {
        match self {
            IdOrBlock::Id(id) => match store.get(&id.id) {
                Some(block_def) => Ok(block_def),
                None => Err(id.span.make_error(&format!("{} is undefined", &id.id))),
            },
            IdOrBlock::Block(block_def) => Ok(block_def),
        }
    }
}

pub(crate) const BLOCK_NAME: &str = "block.name";
pub(crate) const BLOCK_TYPE: &str = "block.type";
pub(crate) const CUSTOM: &str = "custom";

fn transpile_block_def(
    identifier: &IdOrBlock,
    store: &BlockDefStore,
) -> Result<BlockDef, Box<dyn Error>> {
    let block_def = identifier.block_def(store)?;
    if block_def.generic.is_some() {
        return Ok(block_def.clone());
    }

    let mut transpiled_block_def = block_def.clone();
    transpiled_block_def.entries.clear();

    let mut entries_to_evaluate = vec![];

    for entry in block_def.entries.iter() {
        match entry {
            BlockEntry::SpreadRef(modified_ref) => {
                let referenced_type = resolve_modified_ref(modified_ref, store)?;
                entries_to_evaluate.extend(referenced_type.fields);
            }
            BlockEntry::Field(field) => {
                entries_to_evaluate.push(field.clone());
            }
        }
    }

    let mut seen = HashSet::new();

    let block_type = &block_def.kind;
    let mut template_string_replacements = HashMap::from([
        (BLOCK_NAME.to_string(), block_def.name.id.clone()),
        (BLOCK_TYPE.to_string(), format!("{block_type}")),
    ]);
    if let Some(variables) = &block_def.description_variables {
        for variable in variables.variables.iter() {
            template_string_replacements
                .insert(format!("{CUSTOM}.{}", variable.0), variable.1.clone());
        }
    }

    transpile_description(
        &mut transpiled_block_def,
        &template_string_replacements,
        false,
    )?;

    for field in entries_to_evaluate.iter_mut() {
        if seen.contains(&field.name.id) {
            return Err(field.span.make_error("repeated field"));
        }
        seen.insert(field.name.id.clone());
        transpile_description(field, &template_string_replacements, false)?;
        transpiled_block_def
            .entries
            .push(BlockEntry::Field(field.clone()));
    }
    Ok(transpiled_block_def)
}

pub(crate) fn transpile_block_def_by_id(
    identifier: &Identifier,
    store: &BlockDefStore,
) -> Result<BlockDef, Box<dyn Error>> {
    transpile_block_def(&IdOrBlock::Id(identifier), store)
}

pub(crate) fn transpile_block_def_by_block(
    block_def: &BlockDef,
    store: &BlockDefStore,
) -> Result<BlockDef, Box<dyn Error>> {
    transpile_block_def(&IdOrBlock::Block(block_def), store)
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::{BlockDef, BlockField, ModifiedRef};

    #[test]
    fn test_transpiles_one() {
        let block_def = BlockDef::type_def("MyType").field(BlockField::build("field").string());
        let block_def_with_spread = BlockDef::type_def("MyType2")
            .spread(ModifiedRef::build(&block_def.name.id))
            .field(BlockField::build("field2").string());
        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        types.insert(block_def_with_spread.name.id.clone(), block_def_with_spread);
        let transpiled = transpile_block_def(
            &IdOrBlock::Id(&Identifier::from("MyType2")),
            &BlockDefStore::from(&types),
        )
        .unwrap();
        assert_eq!(
            transpiled,
            BlockDef::type_def("MyType2")
                .field(BlockField::build("field").string())
                .field(BlockField::build("field2").string())
        )
    }

    #[test]
    fn test_transpiles_multiple() {
        let block_def = BlockDef::type_def("MyType").field(BlockField::build("field").string());
        let block_def_with_spread = BlockDef::type_def("MyType2")
            .spread(ModifiedRef::build(&block_def.name.id))
            .field(BlockField::build("field2").string());
        let another_block_def_with_spread = BlockDef::type_def("MyType3")
            .spread(ModifiedRef::build(&block_def_with_spread.name.id))
            .field(BlockField::build("field3").string());

        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        types.insert(block_def_with_spread.name.id.clone(), block_def_with_spread);
        types.insert(
            another_block_def_with_spread.name.id.clone(),
            another_block_def_with_spread,
        );
        let transpiled = transpile_block_def(
            &IdOrBlock::Id(&Identifier::from("MyType3")),
            &BlockDefStore::from(&types),
        )
        .unwrap();
        assert_eq!(
            transpiled,
            BlockDef::type_def("MyType3")
                .field(BlockField::build("field").string())
                .field(BlockField::build("field2").string())
                .field(BlockField::build("field3").string())
        )
    }

    #[test]
    fn test_stops_on_spread_loop() {
        let block_def = BlockDef::type_def("MyType")
            .spread(ModifiedRef::build("MyType2"))
            .field(BlockField::build("field").string());
        let block_def_with_spread = BlockDef::type_def("MyType2")
            .spread(ModifiedRef::build("MyType"))
            .field(BlockField::build("field2").string());
        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        types.insert(block_def_with_spread.name.id.clone(), block_def_with_spread);
        let err = transpile_block_def(
            &IdOrBlock::Id(&Identifier::from("MyType")),
            &BlockDefStore::from(&types),
        )
        .unwrap_err();
        assert!(err.to_string().contains("maximum nested spread operator"))
    }

    #[test]
    fn test_does_not_allow_repeated_fields_in_the_same_type() {
        let block_def = BlockDef::type_def("MyType")
            .field(BlockField::build("field").string())
            .field(BlockField::build("field").string());

        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        let err = transpile_block_def(
            &IdOrBlock::Id(&Identifier::from("MyType")),
            &BlockDefStore::from(&types),
        )
        .unwrap_err();
        assert!(err.to_string().contains("repeated field"))
    }

    #[test]
    fn test_does_not_allow_repeated_fields_in_different_types() {
        let block_def = BlockDef::type_def("MyType").field(BlockField::build("field").string());

        let block_def_2 = BlockDef::type_def("MyType2")
            .spread(ModifiedRef::build("MyType"))
            .field(BlockField::build("field").string());
        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        types.insert(block_def_2.name.id.clone(), block_def_2);
        let err = transpile_block_def(
            &IdOrBlock::Id(&Identifier::from("MyType2")),
            &BlockDefStore::from(&types),
        )
        .unwrap_err();
        assert!(err.to_string().contains("repeated field"))
    }

    #[test]
    fn test_undefined_spread_should_fail() {
        let block_def = BlockDef::type_def("MyType2")
            .spread(ModifiedRef::build("MyType"))
            .field(BlockField::build("field").string());
        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        let err = transpile_block_def(
            &IdOrBlock::Id(&Identifier::from("MyType2")),
            &BlockDefStore::from(&types),
        )
        .unwrap_err();
        assert!(err.to_string().contains("undefined"))
    }
}
