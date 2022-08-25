use crate::synths::synth::Synth;
use crate::synths::{SynthContext};
use crate::utils::is_last_iter;

pub(crate) struct OneLineListSynth<T: Synth> {
    pub(crate) items: Vec<T>,
    pub(crate) sep: String,
    pub(crate) wrapper: (String, String),
}

impl<T: Synth> Synth for OneLineListSynth<T> {
    fn synth(&self, context: &SynthContext) -> String {
        let mut result = self.wrapper.0.clone();
        for (is_last, item) in is_last_iter(self.items.iter()) {
            result += &item.synth(context);
            if !is_last {
                result += &self.sep;
            }
        }
        result + &self.wrapper.1
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
