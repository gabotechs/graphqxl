use crate::utils::custom_error;
use graphqxl_parser::{BlockDef, BlockEntry, Identifier, Rule};
use std::collections::{HashMap, HashSet};

pub(crate) fn transpile_block_def(
    identifier: &Identifier,
    store: &HashMap<String, BlockDef>,
    stack_count: usize,
) -> Result<BlockDef, pest::error::Error<Rule>> {
    // todo: where does this come from
    if stack_count > 100 {
        return Err(custom_error(
            &identifier.span,
            "maximum nested spread operator surpassed",
        ));
    }
    let block_def_option = store.get(&identifier.id);
    if block_def_option.is_none() {
        return Err(custom_error(
            &identifier.span,
            &format!("{} is undefined", &identifier.id),
        ));
    }
    let block_def = block_def_option.unwrap();

    let mut transpiled_block_def = block_def.clone();
    transpiled_block_def.entries.clear();

    let mut seen = HashSet::new();

    let mut evaluate_block_entry =
        |block_entry: &BlockEntry| -> Result<(), pest::error::Error<Rule>> {
            if let BlockEntry::Field(field) = block_entry {
                if seen.contains(&field.name.id) {
                    return Err(custom_error(&field.span, "repeated field"));
                } else {
                    seen.insert(field.name.id.clone());
                    transpiled_block_def.entries.push(block_entry.clone());
                };
                Ok(())
            } else {
                unreachable!()
            }
        };

    for entry in block_def.entries.iter() {
        if let BlockEntry::SpreadRef(identifier) = entry {
            let referenced_type = transpile_block_def(identifier, store, stack_count + 1)?;
            for imported_entry in referenced_type.entries.iter() {
                evaluate_block_entry(imported_entry)?;
            }
        } else {
            evaluate_block_entry(entry)?;
        }
    }
    Ok(transpiled_block_def)
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::BlockField;

    #[test]
    fn test_transpiles_one() {
        let block_def = BlockDef::type_("MyType").field(BlockField::build("field").string());
        let block_def_with_spread = BlockDef::type_("MyType2")
            .spread(&block_def.name.id)
            .field(BlockField::build("field2").string());
        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        types.insert(block_def_with_spread.name.id.clone(), block_def_with_spread);
        let transpiled = transpile_block_def(&Identifier::from("MyType2"), &types, 0).unwrap();
        assert_eq!(
            transpiled,
            BlockDef::type_("MyType2")
                .field(BlockField::build("field").string())
                .field(BlockField::build("field2").string())
        )
    }

    #[test]
    fn test_transpiles_multiple() {
        let block_def = BlockDef::type_("MyType").field(BlockField::build("field").string());
        let block_def_with_spread = BlockDef::type_("MyType2")
            .spread(&block_def.name.id)
            .field(BlockField::build("field2").string());
        let another_block_def_with_spread = BlockDef::type_("MyType3")
            .spread(&block_def_with_spread.name.id)
            .field(BlockField::build("field3").string());

        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        types.insert(block_def_with_spread.name.id.clone(), block_def_with_spread);
        types.insert(
            another_block_def_with_spread.name.id.clone(),
            another_block_def_with_spread,
        );
        let transpiled = transpile_block_def(&Identifier::from("MyType3"), &types, 0).unwrap();
        assert_eq!(
            transpiled,
            BlockDef::type_("MyType3")
                .field(BlockField::build("field").string())
                .field(BlockField::build("field2").string())
                .field(BlockField::build("field3").string())
        )
    }

    #[test]
    fn test_stops_on_spread_loop() {
        let block_def = BlockDef::type_("MyType")
            .spread("MyType2")
            .field(BlockField::build("field").string());
        let block_def_with_spread = BlockDef::type_("MyType2")
            .spread("MyType")
            .field(BlockField::build("field2").string());
        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        types.insert(block_def_with_spread.name.id.clone(), block_def_with_spread);
        let err = transpile_block_def(&Identifier::from("MyType"), &types, 0).unwrap_err();
        assert!(err.to_string().contains("maximum nested spread operator"))
    }

    #[test]
    fn test_does_not_allow_repeated_fields_in_the_same_type() {
        let block_def = BlockDef::type_("MyType")
            .field(BlockField::build("field").string())
            .field(BlockField::build("field").string());

        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        let err = transpile_block_def(&Identifier::from("MyType"), &types, 0).unwrap_err();
        assert!(err.to_string().contains("repeated field"))
    }

    #[test]
    fn test_does_not_allow_repeated_fields_in_different_types() {
        let block_def = BlockDef::type_("MyType").field(BlockField::build("field").string());

        let block_def_2 = BlockDef::type_("MyType2")
            .spread("MyType")
            .field(BlockField::build("field").string());
        let mut types = HashMap::new();
        types.insert(block_def.name.id.clone(), block_def);
        types.insert(block_def_2.name.id.clone(), block_def_2);
        let err = transpile_block_def(&Identifier::from("MyType2"), &types, 0).unwrap_err();
        assert!(err.to_string().contains("repeated field"))
    }
}
