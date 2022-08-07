use crate::synths::{Synth, SynthContext};

pub(crate) struct StringSynth(pub(crate) String);

impl Synth for StringSynth {
    fn synth(&self, _: &SynthContext) -> String {
        self.0.to_string()
    }
}

impl From<&str> for StringSynth {
    fn from(text: &str) -> Self {
        Self(text.to_string())
    }
}
