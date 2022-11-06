use crate::synths::synth_context::Synth;
use crate::synths::{SynthConfig, SynthContext};

pub(crate) struct PairSynth<T1: Synth, T2: Synth> {
    pub(crate) first: T1,
    pub(crate) last: T2,
    pub(crate) line_jump_sep: bool,
}

impl<T1: Synth, T2: Synth> Synth for PairSynth<T1, T2> {
    fn synth(&self, context: &mut SynthContext) -> bool {
        let has_written_first = self.first.synth(context);
        if !has_written_first {
            // do nothing
        } else if self.line_jump_sep {
            context.write_line_jump();
            context.write(&" ".repeat(context.config.indent_spaces * context.indent_lvl));
        } else {
            context.write(" ");
        }
        let has_written_last = self.last.synth(context);
        has_written_last || has_written_first
    }
}

impl<T1: Synth, T2: Synth> PairSynth<T1, T2> {
    pub(crate) fn top_level(first: T1, last: T2) -> Self {
        Self {
            first,
            last,
            line_jump_sep: true,
        }
    }
}
