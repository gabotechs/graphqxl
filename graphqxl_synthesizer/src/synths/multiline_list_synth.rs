use crate::synths::synth::Synth;
use crate::synths::{SynthConfig, SynthContext};
use crate::utils::is_last_iter;

pub(crate) struct MultilineListSynth<T: Synth> {
    pub(crate) indent_spaces: usize,
    pub(crate) items: Vec<T>,
    pub(crate) suffix: String,
    pub(crate) wrapper: (String, String),
}

impl<T: Synth> Synth for MultilineListSynth<T> {
    fn synth(&self, context: &SynthContext) -> String {
        let mut result = self.wrapper.0.clone();
        for (is_last, item) in is_last_iter(self.items.iter()) {
            result += "\n";
            result += &" ".repeat((context.indent_lvl + 1) * self.indent_spaces);
            result += &item.synth(&context.plus_one_indent_lvl());
            if !is_last {
                result += &self.suffix;
            }
        }
        if !self.wrapper.1.is_empty() {
            result += "\n";
            result += &" ".repeat(context.indent_lvl * self.indent_spaces);
            result += &self.wrapper.1;
        }
        result
    }
}

impl<T: Synth> MultilineListSynth<T> {
    pub(crate) fn no_suffix(config: &SynthConfig, tuple: (&str, Vec<T>, &str)) -> Self {
        Self {
            indent_spaces: config.indent_spaces,
            items: tuple.1,
            suffix: "".to_string(),
            wrapper: (tuple.0.to_string(), tuple.2.to_string()),
        }
    }

    pub(crate) fn or_suffix(config: &SynthConfig, tuple: (&str, Vec<T>, &str)) -> Self {
        Self {
            indent_spaces: config.indent_spaces,
            items: tuple.1,
            suffix: " |".to_string(),
            wrapper: (tuple.0.to_string(), tuple.2.to_string()),
        }
    }
}
