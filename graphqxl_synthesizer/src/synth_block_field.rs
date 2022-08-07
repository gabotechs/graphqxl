use crate::synth_arguments::ArgumentsSynth;
use crate::synth_description::DescriptionSynth;
use crate::synth_value_type::ValueTypeSynth;
use crate::synths::{ChainSynth, PairSynth, StringSynth, Synth, SynthContext};
use graphqxl_parser::BlockField;

pub(crate) struct BlockFieldSynth(pub(crate) BlockField);

impl Synth for BlockFieldSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let synth = PairSynth {
            line_jump_sep: context.multiline,
            first: DescriptionSynth::from(self.0.description.as_str()),
            last: ChainSynth({
                let mut v: Vec<Box<dyn Synth>> = vec![Box::new(StringSynth(self.0.name.clone()))];
                if !self.0.args.is_empty() {
                    v.push(Box::new(ArgumentsSynth(self.0.args.clone())));
                }
                if let Some(value_type) = &self.0.value {
                    v.push(Box::new(StringSynth::from(": ")));
                    v.push(Box::new(ValueTypeSynth(value_type.clone())));
                }
                // todo: add default
                v
            }),
        };
        synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        simple_block_field_synth_factory, simple_string_arg_factory,
        simple_string_value_type_factory,
    };

    #[test]
    fn test_no_description_no_args_no_type() {
        let synth = BlockFieldSynth(simple_block_field_synth_factory("field"));
        assert_eq!(synth.synth_zero(), "field");
    }

    #[test]
    fn test_description_no_args_no_type() {
        let mut synth = BlockFieldSynth(simple_block_field_synth_factory("field"));
        synth.0.description = "my description".to_string();
        assert_eq!(synth.synth_zero(), "\"my description\" field");
    }

    #[test]
    fn test_multiline_description_no_args_no_type() {
        let mut synth = BlockFieldSynth(simple_block_field_synth_factory("field"));
        synth.0.description = "my multiline\n description".to_string();
        assert_eq!(
            synth.synth_zero(),
            "\
\"\"\"
my multiline
 description
\"\"\" field"
        );
    }

    #[test]
    fn test_description_no_args_type() {
        let mut synth = BlockFieldSynth(simple_block_field_synth_factory("field"));
        synth.0.description = "my description".to_string();
        synth.0.value = Some(simple_string_value_type_factory());
        assert_eq!(synth.synth_zero(), "\"my description\" field: String");
    }

    #[test]
    fn test_description_args_type() {
        let mut synth = BlockFieldSynth(simple_block_field_synth_factory("field"));
        synth.0.description = "my description".to_string();
        synth.0.value = Some(simple_string_value_type_factory());
        synth.0.args.push(simple_string_arg_factory("arg"));
        assert_eq!(
            synth.synth_zero(),
            "\"my description\" field(arg: String): String"
        );
    }

    #[test]
    fn test_description_args_type_multiline() {
        let mut synth = BlockFieldSynth(simple_block_field_synth_factory("field"));
        synth.0.description = "my description".to_string();
        synth.0.value = Some(simple_string_value_type_factory());
        synth.0.args.push(simple_string_arg_factory("arg"));
        assert_eq!(
            synth.synth(&SynthContext {
                multiline: true,
                indent_spaces: 2,
                indent_lvl: 0,
                ..Default::default()
            }),
            "\
\"my description\"
field(
  arg: String
): String"
        );
    }

    #[test]
    fn test_description_multiple_args_type_multiline_and_indent() {
        let mut synth = BlockFieldSynth(simple_block_field_synth_factory("field"));
        synth.0.description = "my description".to_string();
        synth.0.value = Some(simple_string_value_type_factory());
        synth.0.args.push(simple_string_arg_factory("arg1"));
        synth.0.args.push(simple_string_arg_factory("arg2"));
        assert_eq!(
            synth.synth(&SynthContext {
                multiline: true,
                indent_spaces: 2,
                indent_lvl: 2,
                ..Default::default()
            }),
            "\
\"my description\"
    field(
      arg1: String 
      arg2: String
    ): String"
        );
    }
}
