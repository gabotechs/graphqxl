use crate::synth_arguments::ArgumentsSynth;
use crate::synth_description::DescriptionSynth;
use crate::synth_value_type::ValueTypeSynth;
use crate::synths::{ChainSynth, PairSynth, StringSynth, Synth};
use graphqxl_parser::BlockField;

pub(crate) struct BlockFieldSynth {
    indent: usize,
    block_field: BlockField,
}

impl Synth for BlockFieldSynth {
    fn synth(&self, indent_lvl: usize, multiline: bool) -> String {
        let pair = PairSynth {
            indent: self.indent,
            line_jump_sep: multiline,
            first: DescriptionSynth::from(self.block_field.description.as_str()),
            last: ChainSynth::from({
                let mut v: Vec<Box<dyn Synth>> =
                    vec![Box::new(StringSynth::from(self.block_field.name.as_str()))];
                if !self.block_field.args.is_empty() {
                    v.push(Box::new(ArgumentsSynth::from((
                        self.block_field.args.clone(),
                        self.indent,
                    ))));
                }
                if let Some(value_type) = &self.block_field.value {
                    v.push(Box::new(StringSynth::from(": ")));
                    v.push(Box::new(ValueTypeSynth(value_type.clone())));
                }
                // todo: add default
                v
            }),
        };
        pair.synth(indent_lvl, multiline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{simple_string_arg_factory, simple_string_value_type_factory};

    fn block_field_synth_factory() -> BlockFieldSynth {
        BlockFieldSynth {
            indent: 2,
            block_field: BlockField {
                name: "field".to_string(),
                description: "".to_string(),
                value: None,
                args: vec![],
            },
        }
    }

    #[test]
    fn test_no_description_no_args_no_type() {
        let synth = block_field_synth_factory();
        assert_eq!(synth.synth_zero(), "field");
    }

    #[test]
    fn test_description_no_args_no_type() {
        let mut synth = block_field_synth_factory();
        synth.block_field.description = "my description".to_string();
        assert_eq!(synth.synth_zero(), "\"my description\" field");
    }

    #[test]
    fn test_multiline_description_no_args_no_type() {
        let mut synth = block_field_synth_factory();
        synth.block_field.description = "my multiline\n description".to_string();
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
        let mut synth = block_field_synth_factory();
        synth.block_field.description = "my description".to_string();
        synth.block_field.value = Some(simple_string_value_type_factory());
        assert_eq!(synth.synth_zero(), "\"my description\" field: String");
    }

    #[test]
    fn test_description_args_type() {
        let mut synth = block_field_synth_factory();
        synth.block_field.description = "my description".to_string();
        synth.block_field.value = Some(simple_string_value_type_factory());
        synth
            .block_field
            .args
            .push(simple_string_arg_factory("arg"));
        assert_eq!(
            synth.synth_zero(),
            "\"my description\" field(arg: String): String"
        );
    }

    #[test]
    fn test_description_args_type_multiline() {
        let mut synth = block_field_synth_factory();
        synth.block_field.description = "my description".to_string();
        synth.block_field.value = Some(simple_string_value_type_factory());
        synth
            .block_field
            .args
            .push(simple_string_arg_factory("arg"));
        assert_eq!(
            synth.synth(0, true),
            "\
\"my description\"
field(
  arg: String
): String"
        );
    }

    #[test]
    fn test_description_multiple_args_type_multiline_and_indent() {
        let mut synth = block_field_synth_factory();
        synth.block_field.description = "my description".to_string();
        synth.block_field.value = Some(simple_string_value_type_factory());
        synth
            .block_field
            .args
            .push(simple_string_arg_factory("arg1"));
        synth
            .block_field
            .args
            .push(simple_string_arg_factory("arg2"));
        assert_eq!(
            synth.synth(2, true),
            "\
\"my description\"
    field(
      arg1: String, 
      arg2: String
    ): String"
        );
    }
}
