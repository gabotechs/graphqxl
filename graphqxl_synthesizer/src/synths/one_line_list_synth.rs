use crate::synths::synth_context::Synth;
use crate::synths::SynthContext;
use crate::utils::is_last_iter;

pub(crate) struct OneLineListSynth<T: Synth> {
    pub(crate) items: Vec<T>,
    pub(crate) sep: String,
    pub(crate) wrapper: (String, String),
}

impl<T: Synth> Synth for OneLineListSynth<T> {
    fn synth(&self, context: &mut SynthContext) -> bool {
        context.write(&self.wrapper.0);
        for (is_last, item) in is_last_iter(self.items.iter()) {
            item.synth(context);
            if !is_last {
                context.write(&self.sep);
            }
        }
        context.write(&self.wrapper.1);
        true
    }
}

impl<T: Synth> OneLineListSynth<T> {
    pub(crate) fn comma(tuple: (&str, Vec<T>, &str)) -> Self {
        Self {
            items: tuple.1,
            sep: ", ".to_string(),
            wrapper: (tuple.0.to_string(), tuple.2.to_string()),
        }
    }

    pub(crate) fn or(tuple: (&str, Vec<T>, &str)) -> Self {
        Self {
            items: tuple.1,
            sep: " | ".to_string(),
            wrapper: (tuple.0.to_string(), tuple.2.to_string()),
        }
    }
}
