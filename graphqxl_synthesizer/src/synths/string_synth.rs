use crate::synths::{Synth, SynthContext};

pub(crate) struct StringSynth(pub(crate) String);

impl Synth for StringSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        if self.0.is_empty() {
            false
        } else {
            context.write(&self.0);
            true
        }
    }
}

impl From<&str> for StringSynth {
    fn from(text: &str) -> Self {
        Self(text.to_string())
    }
}
