use crate::synth_arguments::ArgumentsSynth;
use crate::synth_description::DescriptionSynth;
use crate::synth_directive::DirectiveSynth;
use crate::synth_identifier::IdentifierSynth;
use crate::synth_value_type::ValueTypeSynth;
use crate::synths::{ChainSynth, PairSynth, StringSynth, Synth, SynthContext};
use graphqxl_parser::BlockField;

pub(crate) struct BlockFieldSynth(pub(crate) BlockField);

impl Synth for BlockFieldSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        let synth = PairSynth {
            line_jump_sep: true,
            first: DescriptionSynth::text(&self.0.description),
            last: ChainSynth({
                let mut v: Vec<Box<dyn Synth>> =
                    vec![Box::new(IdentifierSynth(self.0.name.clone()))];
                if !self.0.args.is_empty() {
                    v.push(Box::new(ArgumentsSynth(self.0.args.clone())));
                }
                if let Some(value_type) = &self.0.value_type {
                    v.push(Box::new(StringSynth::from(": ")));
                    v.push(Box::new(ValueTypeSynth(value_type.clone())));
                }
                for directive in self.0.directives.iter() {
                    v.push(Box::new(StringSynth::from(" ")));
                    v.push(Box::new(DirectiveSynth(directive.clone())));
                }
                v
            }),
        };
        synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::{Argument, Directive, ValueType};

    #[test]
    fn test_no_description_no_args_no_type() {
        let synth = BlockFieldSynth(BlockField::build("field"));
        assert_eq!(synth.synth_zero(), "field");
    }

    #[test]
    fn test_description_no_args_no_type() {
        let synth = BlockFieldSynth(BlockField::build("field").description("my description"));
        assert_eq!(synth.synth_zero(), "\"my description\"\nfield");
    }

    #[test]
    fn test_multiline_description_no_args_no_type() {
        let synth =
            BlockFieldSynth(BlockField::build("field").description("my multiline\n description"));
        assert_eq!(
            synth.synth_zero(),
            "\
\"\"\"
my multiline
 description
\"\"\"
field"
        );
    }

    #[test]
    fn test_description_no_args_type() {
        let synth = BlockFieldSynth(
            BlockField::build("field")
                .string()
                .description("my description"),
        );
        assert_eq!(synth.synth_zero(), "\"my description\"\nfield: String");
    }

    #[test]
    fn test_description_args_type() {
        let synth = BlockFieldSynth(
            BlockField::build("field")
                .string()
                .description("my description")
                .value_type(ValueType::string())
                .arg(Argument::string("arg")),
        );
        assert_eq!(
            synth.synth_zero(),
            "\"my description\"\nfield(arg: String): String"
        );
    }

    #[test]
    fn test_description_args_type_multiline() {
        let synth = BlockFieldSynth(
            BlockField::build("field")
                .string()
                .description("my description")
                .value_type(ValueType::string())
                .arg(Argument::string("arg")),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
\"my description\"
field(arg: String): String"
        );
    }
    #[test]
    fn test_description_multiple_args_type_multiline_and_indent_and_directive() {
        let synth = BlockFieldSynth(
            BlockField::build("field")
                .string()
                .description("my description")
                .value_type(ValueType::string())
                .arg(Argument::string("arg1"))
                .arg(Argument::string("arg2"))
                .directive(Directive::build("dir1")),
        );
        let mut context = SynthContext::default();
        context.with_indent_lvl(2);
        synth.synth(&mut context);
        assert_eq!(
            context.result,
            "\
\"my description\"
    field(arg1: String, arg2: String): String @dir1"
        );
    }
}
