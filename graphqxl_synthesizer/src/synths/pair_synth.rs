use crate::synths::synth::Synth;
use crate::synths::{SynthConfig, SynthContext};

pub(crate) struct PairSynth<T1: Synth, T2: Synth> {
    pub(crate) first: T1,
    pub(crate) last: T2,
    pub(crate) line_jump_sep: bool,
    pub(crate) indent_spaces: usize,
}

impl<T1: Synth, T2: Synth> Synth for PairSynth<T1, T2> {
    fn synth(&self, context: &SynthContext) -> String {
        let mut result = self.first.synth(context);
        if result.is_empty() {
            // do nothing
        } else if self.line_jump_sep {
            result += "\n";
            result += &" ".repeat(self.indent_spaces * context.indent_lvl);
        } else {
            result += " ";
        }
        result += &self.last.synth(context);
        result
    }
}

impl<T1: Synth, T2: Synth> PairSynth<T1, T2> {
    pub(crate) fn top_level(config: &SynthConfig, first: T1, last: T2) -> Self {
        Self {
            indent_spaces: config.indent_spaces,
            first,
            last,
            line_jump_sep: true,
        }
    }
}
