use crate::synths::{Synth, SynthContext};

pub(crate) struct ChainSynth(pub(crate) Vec<Box<dyn Synth>>);

impl Synth for ChainSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        let mut has_written = false;
        for synth in self.0.iter() {
            let written_flag = synth.synth(context);
            if written_flag {
                has_written = true;
            }
        }
        has_written
    }
}
