use crate::synth_block_field::BlockFieldSynth;
use crate::synth_description::DescriptionSynth;
use crate::synth_directive::DirectiveSynth;
use crate::synths::{ChainSynth, ListSynth, PairSynth, StringSynth, Synth, SynthContext};
use graphqxl_parser::{BlockDef, BlockDefType};

pub(crate) struct BlockDefSynth(pub(crate) BlockDef);

impl Synth for BlockDefSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let symbol = match self.0.kind {
            BlockDefType::Type => "type",
            BlockDefType::Input => "input",
            BlockDefType::Enum => "enum",
            BlockDefType::Interface => "interface",
        };
        let mut v: Vec<Box<dyn Synth>> = vec![Box::new(StringSynth(format!(
            "{} {} ",
            symbol, self.0.name
        )))];
        for directive in self.0.directives.iter() {
            v.push(Box::new(DirectiveSynth(directive.clone())));
            v.push(Box::new(StringSynth::from(" ")));
        }
        v.push(Box::new(ListSynth::from((
            "{",
            self.0
                .fields
                .iter()
                .map(|e| BlockFieldSynth(e.clone()))
                .collect(),
            " ",
            "}",
        ))));
        let synth = PairSynth::top_level(
            DescriptionSynth::from(self.0.description.as_str()),
            ChainSynth(v),
        );
        synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::{Argument, BlockField, Directive, ValueData};

    fn test_most_simple_block_def_factory() -> BlockDef {
        BlockDef::type_("MyType").field(BlockField::build("field").string())
    }

    #[test]
    fn test_most_simple_block_def() {
        let synth = BlockDefSynth(test_most_simple_block_def_factory());
        assert_eq!(synth.synth_zero(), "type MyType {field: String}")
    }

    fn test_with_args_block_def_factory() -> BlockDef {
        test_most_simple_block_def_factory().field(
            BlockField::build("field2")
                .string()
                .arg(Argument::string("arg1"))
                .arg(Argument::string("arg2"))
                .arg(Argument::string("arg3")),
        )
    }

    #[test]
    fn test_with_args_block_def() {
        let synth = BlockDefSynth(test_with_args_block_def_factory());
        assert_eq!(
            synth.synth_multiline(2),
            "\
type MyType {
  field: String 
  field2(
    arg1: String 
    arg2: String 
    arg3: String
  ): String
}"
        )
    }

    fn test_with_descriptions_block_def_factory() -> BlockDef {
        test_most_simple_block_def_factory().field(
            BlockField::build("field2")
                .string()
                .description("my description"),
        )
    }

    #[test]
    fn test_with_descriptions_block_def() {
        let synth = BlockDefSynth(test_with_descriptions_block_def_factory());
        assert_eq!(
            synth.synth_multiline(2),
            "\
type MyType {
  field: String 
  \"my description\"
  field2: String
}"
        )
    }

    fn test_with_directive_factory() -> BlockDef {
        test_with_args_block_def_factory()
            .directive(Directive::build("dir1").input("arg", ValueData::int(1)))
            .directive(Directive::build("dir2"))
    }

    #[test]
    fn test_with_directive() {
        let synth = BlockDefSynth(test_with_directive_factory());
        assert_eq!(
            synth.synth_multiline(2),
            "\
type MyType @dir1(
  arg: 1
) @dir2 {
  field: String 
  field2(
    arg1: String 
    arg2: String 
    arg3: String
  ): String
}"
        );
    }
}
