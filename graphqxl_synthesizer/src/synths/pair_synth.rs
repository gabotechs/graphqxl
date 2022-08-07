use crate::synths::synth::Synth;

pub(crate) struct PairSynth<T1: Synth, T2: Synth> {
    pub(crate) indent: usize,
    pub(crate) first: T1,
    pub(crate) last: T2,
    pub(crate) line_jump_sep: bool,
}

impl<T1: Synth, T2: Synth> Synth for PairSynth<T1, T2> {
    fn synth(&self, indent_lvl: usize, multiline: bool) -> String {
        let mut result = self.first.synth(indent_lvl, multiline);
        if result.is_empty() {
            // do nothing
        } else if self.line_jump_sep {
            result += "\n";
            result += &" ".repeat(self.indent * indent_lvl);
        } else {
            result += " ";
        }
        result += &self.last.synth(indent_lvl, multiline);
        result
    }
}

impl<T1: Synth, T2: Synth> PairSynth<T1, T2> {
    pub(crate) fn top_level(first: T1, last: T2) -> Self {
        Self {
            indent: 0,
            first,
            last,
            line_jump_sep: true,
        }
    }
    
    pub(crate) fn inline(first: T1, last: T2) -> Self {
        Self {
            indent: 0,
            first,
            last,
            line_jump_sep: false
        }
    }
}
