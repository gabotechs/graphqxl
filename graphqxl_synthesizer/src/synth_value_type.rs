use crate::synth_identifier::IdentifierSynth;
use crate::synths::{Synth, SynthContext};
use graphqxl_parser::{ValueBasicType, ValueType};

pub(crate) struct ValueTypeSynth(pub(crate) ValueType);

impl Synth for ValueTypeSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        match &self.0 {
            ValueType::Basic(basic) => match &basic {
                ValueBasicType::Int => {
                    context.write("Int");
                    true
                }
                ValueBasicType::Float => {
                    context.write("Float");
                    true
                }
                ValueBasicType::String => {
                    context.write("String");
                    true
                }
                ValueBasicType::Boolean => {
                    context.write("Boolean");
                    true
                }
                ValueBasicType::Object(name) => {
                    IdentifierSynth(name.clone()).synth(context);
                    true
                }
            },
            ValueType::NonNullable(value_type) => {
                ValueTypeSynth(*value_type.clone()).synth(context);
                context.write("!");
                true
            }
            ValueType::Array(value_type) => {
                context.write("[");
                ValueTypeSynth(*value_type.clone()).synth(context);
                context.write("]");
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::Identifier;

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
            ValueType::object(Identifier::from("MyObject"))
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
