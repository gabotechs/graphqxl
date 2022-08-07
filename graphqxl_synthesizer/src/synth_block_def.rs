use crate::synth_block_field::BlockFieldSynth;
use crate::synth_description::DescriptionSynth;
use crate::synths::{ListSynth, PairSynth, StringSynth, Synth, SynthContext};
use graphqxl_parser::{BlockDef, BlockDefType};

pub(crate) struct BlockDefSynth(BlockDef);

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

    #[test]
    fn test_most_simple_block_def() {}
}
