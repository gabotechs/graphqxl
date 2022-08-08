use crate::synths::synth::Synth;
use crate::synths::SynthContext;
use crate::utils::is_last_iter;

pub(crate) struct ListSynth<T: Synth> {
    pub(crate) items: Vec<T>,
    pub(crate) sep: String,
    pub(crate) wrapper: (String, String),
}

impl<T: Synth> Synth for ListSynth<T> {
    fn synth(&self, context: &SynthContext) -> String {
        let mut result = self.wrapper.0.clone();
        for (is_last, item) in is_last_iter(self.items.iter()) {
            if context.multiline {
                result += "\n";
                result += &" ".repeat((context.indent_lvl + 1) * context.indent_spaces);
                result += &item.synth(&SynthContext {
                    indent_lvl: context.indent_lvl + 1,
                    ..*context
                });
                if !is_last {
                    result += &self.sep;
                }
            } else {
                result += &item.synth(context);
                if !is_last {
                    result += &self.sep
                }
            }
        }
        if context.multiline && !self.wrapper.1.is_empty() {
            result += "\n";
            result += &" ".repeat(context.indent_lvl * context.indent_spaces);
        }
        result + &self.wrapper.1
    }
}

impl<T: Synth> From<(&str, Vec<T>, &str, &str)> for ListSynth<T> {
    fn from(tuple: (&str, Vec<T>, &str, &str)) -> Self {
        Self {
            items: tuple.1,
            sep: tuple.2.to_string(),
            wrapper: (tuple.0.to_string(), tuple.3.to_string()),
        }
    }
}
