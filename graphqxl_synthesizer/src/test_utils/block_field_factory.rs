use graphqxl_parser::{BlockField, ValueBasicType, ValueType, ValueTypeSimple};

pub fn simple_block_field_synth_factory(name: &str) -> BlockField {
    BlockField {
        name: name.to_string(),
        description: "".to_string(),
        value: None,
        args: vec![],
    }
}

pub fn simple_string_block_field_synth_factory(name: &str) -> BlockField {
    BlockField {
        name: name.to_string(),
        description: "".to_string(),
        value: Some(ValueType::Simple(ValueTypeSimple {
            content: ValueBasicType::String,
            nullable: true,
        })),
        args: vec![],
    }
}
