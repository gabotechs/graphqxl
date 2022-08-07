use crate::synths::{Synth, SynthContext};

pub(crate) struct ChainSynth(pub(crate) Vec<Box<dyn Synth>>);

impl Synth for ChainSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut result = "".to_string();
        for synth in self.0.iter() {
            result += &synth.synth(context);
        }
        result
    }
}
