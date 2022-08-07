use graphqxl_parser::{Argument, ValueBasicType, ValueType, ValueTypeSimple};

pub fn simple_string_arg_factory(name: &str) -> Argument {
    Argument {
        name: name.to_string(),
        description: "".to_string(),
        value: ValueType::Simple(ValueTypeSimple {
            content: ValueBasicType::String,
            nullable: true,
        }),
        default: None,
    }
}
