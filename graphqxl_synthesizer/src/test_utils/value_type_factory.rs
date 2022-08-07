use graphqxl_parser::{ValueBasicType, ValueType, ValueTypeSimple};

pub fn simple_string_value_type_factory() -> ValueType {
    ValueType::Simple(ValueTypeSimple {
        content: ValueBasicType::String,
        nullable: true,
    })
}
