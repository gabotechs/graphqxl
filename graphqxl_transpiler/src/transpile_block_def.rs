use crate::transpile_description::transpile_description;
use graphqxl_parser::{BlockDef, BlockDefType, BlockEntry, Identifier, OwnedSpan, Rule};
use std::collections::{HashMap, HashSet};
use std::string::ToString;

pub(crate) enum IdOrBlock {
    Id(Identifier),
    Block(BlockDef),
}

impl IdOrBlock {
    fn name(&self) -> &str {
        match self {
            IdOrBlock::Id(id) => &id.id,
            IdOrBlock::Block(block_def) => &block_def.name.id,
        }
    }

    fn span(&self) -> &OwnedSpan {
        match self {
            IdOrBlock::Id(id) => &id.span,
            IdOrBlock::Block(block_def) => &block_def.span,
        }
    }
}

pub(crate) const BLOCK_NAME: &str = "block.name";
pub(crate) const BLOCK_TYPE: &str = "block.type";

fn _transpile_block_def(
    identifier: &IdOrBlock,
    store: &HashMap<String, BlockDef>,
    stack_count: usize,
    parent_name: &str,
    parent_kind: &BlockDefType,
) -> Result<BlockDef, pest::error::Error<Rule>> {
    // todo: where does this come from
    if stack_count > 100 {
        return Err(identifier
            .span()
            .make_error("maximum nested spread operator surpassed"));
    }
    let block_def = match identifier {
        IdOrBlock::Id(id) => match store.get(&id.id) {
            Some(block_def) => block_def,
            None => {
                return Err(id.span.make_error(&format!("{} is undefined", &id.id)));
            }
        },
        IdOrBlock::Block(block_def) => block_def,
    };

    let template_string_replacements = HashMap::from([
        (BLOCK_NAME.to_string(), parent_name.to_string()),
        (BLOCK_TYPE.to_string(), format!("{}", parent_kind)),
    ]);

    let mut transpiled_block_def = block_def.clone();
    transpiled_block_def.entries.clear();

    let mut seen = HashSet::new();

    let mut evaluate_block_entry =
        |block_entry: &BlockEntry| -> Result<(), pest::error::Error<Rule>> {
            let BlockEntry::Field(field) = block_entry else {
                unreachable!()
            };
            if seen.contains(&field.name.id) {
                return Err(field.span.make_error("repeated field"));
            }
            seen.insert(field.name.id.clone());
            let mut field_clone = field.clone();
            if block_def.generic.is_none() {
                transpile_description(&mut field_clone, &template_string_replacements)?;
            }
            transpiled_block_def
                .entries
                .push(BlockEntry::Field(field_clone));

            Ok(())
        };

    for entry in block_def.entries.iter() {
        if let BlockEntry::SpreadRef(identifier) = entry {
            let mut referenced_type = _transpile_block_def(
                &IdOrBlock::Id(identifier.clone()),
                store,
                stack_count + 1,
                parent_name,
                parent_kind,
            )?;
            for imported_entry in referenced_type.entries.iter_mut() {
                evaluate_block_entry(imported_entry)?;
            }
        } else {
            evaluate_block_entry(entry)?;
        }
    }
    Ok(transpiled_block_def)
}

pub(crate) fn transpile_block_def(
    identifier: &IdOrBlock,
    store: &HashMap<String, BlockDef>,
) -> Result<BlockDef, pest::error::Error<Rule>> {
    let parent_name = identifier.name();
    let parent_kind = match identifier {
        IdOrBlock::Id(id) => match store.get(&id.id) {
            Some(block_def) => &block_def.kind,
            _ => &BlockDefType::Type,
        },
        IdOrBlock::Block(block_def) => &block_def.kind,
    };
    _transpile_block_def(identifier, store, 0, parent_name, parent_kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::BlockField;

    impl IdOrBlock {
        fn from(id: &str) -> Self {
            IdOrBlock::Id(Identifier::from(id))
        }
    }

    #[test]
    fn test_transpiles_one() {
        let block_def = BlockDef::type_def("MyType").field(BlockField::build("field").string());
        let block_def_with_spread = BlockDef::type_def("MyType2")
            .spread(&block_def.name.id)
            .field(BlockField::build("field2").string());
        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        types.insert(block_def_with_spread.name.id.clone(), block_def_with_spread);
        let transpiled = transpile_block_def(&IdOrBlock::from("MyType2"), &types).unwrap();
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
            .spread(&block_def.name.id)
            .field(BlockField::build("field2").string());
        let another_block_def_with_spread = BlockDef::type_def("MyType3")
            .spread(&block_def_with_spread.name.id)
            .field(BlockField::build("field3").string());

        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        types.insert(block_def_with_spread.name.id.clone(), block_def_with_spread);
        types.insert(
            another_block_def_with_spread.name.id.clone(),
            another_block_def_with_spread,
        );
        let transpiled = transpile_block_def(&IdOrBlock::from("MyType3"), &types).unwrap();
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
            .spread("MyType2")
            .field(BlockField::build("field").string());
        let block_def_with_spread = BlockDef::type_def("MyType2")
            .spread("MyType")
            .field(BlockField::build("field2").string());
        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        types.insert(block_def_with_spread.name.id.clone(), block_def_with_spread);
        let err = transpile_block_def(&IdOrBlock::from("MyType"), &types).unwrap_err();
        assert!(err.to_string().contains("maximum nested spread operator"))
    }

    #[test]
    fn test_does_not_allow_repeated_fields_in_the_same_type() {
        let block_def = BlockDef::type_def("MyType")
            .field(BlockField::build("field").string())
            .field(BlockField::build("field").string());

        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        let err = transpile_block_def(&IdOrBlock::from("MyType"), &types).unwrap_err();
        assert!(err.to_string().contains("repeated field"))
    }

    #[test]
    fn test_does_not_allow_repeated_fields_in_different_types() {
        let block_def = BlockDef::type_def("MyType").field(BlockField::build("field").string());

        let block_def_2 = BlockDef::type_def("MyType2")
            .spread("MyType")
            .field(BlockField::build("field").string());
        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        types.insert(block_def_2.name.id.clone(), block_def_2);
        let err = transpile_block_def(&IdOrBlock::from("MyType2"), &types).unwrap_err();
        assert!(err.to_string().contains("repeated field"))
    }

    #[test]
    fn test_undefined_spread_should_fail() {
        let block_def = BlockDef::type_def("MyType2")
            .spread("MyType")
            .field(BlockField::build("field").string());
        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        let err = transpile_block_def(&IdOrBlock::from("MyType2"), &types).unwrap_err();
        assert!(err.to_string().contains("undefined"))
    }
}
