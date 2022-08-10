use crate::synth_block_field::BlockFieldSynth;
use crate::synth_description::DescriptionSynth;
use crate::synths::{ListSynth, PairSynth, StringSynth, Synth, SynthContext};
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
        let synth = PairSynth {
            line_jump_sep: context.multiline,
            first: DescriptionSynth::from(self.0.description.as_str()),
            last: PairSynth::inline(
                StringSynth(format!("{} {}", symbol, self.0.name)),
                ListSynth::from((
                    "{",
                    self.0
                        .fields
                        .iter()
                        .map(|e| BlockFieldSynth(e.clone()))
                        .collect(),
                    " ",
                    "}",
                )),
            ),
        };
        synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::{Argument, BlockField};

    #[test]
    fn test_most_simple_block_def() {
        let synth =
            BlockDefSynth(BlockDef::type_("MyType").field(BlockField::build("field").string()));
        assert_eq!(synth.synth_zero(), "type MyType {field: String}")
    }

    #[test]
    fn test_with_args_block_def() {
        let synth = BlockDefSynth(
            BlockDef::type_("MyType")
                .field(BlockField::build("field").string())
                .field(
                    BlockField::build("field2")
                        .string()
                        .arg(Argument::string("arg1"))
                        .arg(Argument::string("arg2"))
                        .arg(Argument::string("arg3")),
                ),
        );
        assert_eq!(
            synth.synth(&SynthContext {
                multiline: true,
                indent_spaces: 2,
                ..Default::default()
            }),
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
}
