use graphqxl_parser::{BlockField, ValueBasicType, ValueType, ValueTypeSimple};

pub fn simple_block_field_synth_factory(name: &str) -> BlockField {
    BlockField {
        name: name.to_string(),
        description: "".to_string(),
        value_type: None,
        args: vec![],
    }
}

pub fn simple_string_block_field_synth_factory(name: &str) -> BlockField {
    BlockField {
        name: name.to_string(),
        description: "".to_string(),
        value_type: Some(ValueType::Simple(ValueTypeSimple {
            value_type: ValueBasicType::String,
            nullable: true,
        })),
        args: vec![],
    }
}
