use crate::synths::synth_context::Synth;
use crate::synths::SynthContext;
use crate::utils::is_last_iter;

pub(crate) struct MultilineListSynth<T: Synth> {
    pub(crate) items: Vec<T>,
    pub(crate) suffix: String,
    pub(crate) wrapper: (String, String),
}

impl<T: Synth> Synth for MultilineListSynth<T> {
    fn synth(&self, context: &mut SynthContext) -> bool {
        context.write(&self.wrapper.0);
        for (is_last, item) in is_last_iter(self.items.iter()) {
            context.write_line_jump();
            context.write_indent(context.indent_lvl + 1);
            context.push_indent_level();
            item.synth(context);
            context.pop_indent_level();
            if !is_last {
                context.write(&self.suffix);
            }
        }
        if !self.wrapper.1.is_empty() {
            context.write_line_jump();
            context.write_indent(context.indent_lvl);
            context.write(&self.wrapper.1);
        }
        true
    }
}

impl<T: Synth> MultilineListSynth<T> {
    pub(crate) fn no_suffix(tuple: (&str, Vec<T>, &str)) -> Self {
        Self {
            items: tuple.1,
            suffix: "".to_string(),
            wrapper: (tuple.0.to_string(), tuple.2.to_string()),
        }
    }

    pub(crate) fn or_suffix(tuple: (&str, Vec<T>, &str)) -> Self {
        Self {
            items: tuple.1,
            suffix: " |".to_string(),
            wrapper: (tuple.0.to_string(), tuple.2.to_string()),
        }
    }
}
