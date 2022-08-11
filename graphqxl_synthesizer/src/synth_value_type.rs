use crate::synths::{Synth, SynthContext};
use graphqxl_parser::{ValueBasicType, ValueType};

pub(crate) struct ValueTypeSynth(pub(crate) ValueType);

impl Synth for ValueTypeSynth {
    fn synth(&self, _context: &SynthContext) -> String {
        match &self.0 {
            ValueType::Basic(basic) => match &basic {
                ValueBasicType::Int => "Int".to_string(),
                ValueBasicType::Float => "Float".to_string(),
                ValueBasicType::String => "String".to_string(),
                ValueBasicType::Boolean => "Boolean".to_string(),
                ValueBasicType::Object(name) => name.clone(),
            },
            ValueType::NonNullable(value_type) => {
                let synth = ValueTypeSynth(*value_type.clone());
                format!("{}!", synth.synth_zero(),)
            }
            ValueType::Array(value_type) => {
                let synth = ValueTypeSynth(*value_type.clone());
                format!("[{}]", synth.synth_zero(),)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nullable_int() {
        let synth = ValueTypeSynth(ValueType::int());
        assert_eq!(synth.synth_zero(), "Int");
    }

    #[test]
    fn test_non_nullable_int() {
        let synth = ValueTypeSynth(ValueType::int().non_nullable());
        assert_eq!(synth.synth_zero(), "Int!");
    }

    #[test]
    fn test_array_int() {
        let synth = ValueTypeSynth(ValueType::int().array());
        assert_eq!(synth.synth_zero(), "[Int]");
    }

    #[test]
    fn test_non_nullable_array_nullable_int() {
        let synth = ValueTypeSynth(ValueType::int().array().non_nullable());
        assert_eq!(synth.synth_zero(), "[Int]!");
    }

    #[test]
    fn test_non_nullable_array_non_nullable_int() {
        let synth = ValueTypeSynth(ValueType::int().non_nullable().array().non_nullable());
        assert_eq!(synth.synth_zero(), "[Int!]!");
    }

    #[test]
    fn test_non_nullable_array_non_nullable_string() {
        let synth = ValueTypeSynth(ValueType::string().non_nullable().array().non_nullable());
        assert_eq!(synth.synth_zero(), "[String!]!");
    }

    #[test]
    fn test_non_nullable_array_non_nullable_object() {
        let synth = ValueTypeSynth(
            ValueType::object("MyObject")
                .non_nullable()
                .array()
                .non_nullable(),
        );
        assert_eq!(synth.synth_zero(), "[MyObject!]!");
    }

    #[test]
    fn test_deeply_nested_array() {
        let synth = ValueTypeSynth(ValueType::int().array().array().array());
        assert_eq!(synth.synth_zero(), "[[[Int]]]");
    }

    #[test]
    fn test_deeply_nested_array_with_non_nullables() {
        let synth = ValueTypeSynth(
            ValueType::int()
                .non_nullable()
                .array()
                .non_nullable()
                .array()
                .non_nullable()
                .array(),
        );
        assert_eq!(synth.synth_zero(), "[[[Int!]!]!]");
    }
}
