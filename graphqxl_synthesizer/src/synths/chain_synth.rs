use crate::synths::Synth;

pub(crate) struct ChainSynth {
    synths: Vec<Box<dyn Synth>>,
}

impl Synth for ChainSynth {
    fn synth(&self, indent_lvl: usize, multiline: bool) -> String {
        let mut result = "".to_string();
        for synth in self.synths.iter() {
            result += &synth.synth(indent_lvl, multiline);
        }
        result
    }
}


impl From<Vec<Box<dyn Synth>>> for ChainSynth {
    fn from(synths: Vec<Box<dyn Synth>>) -> Self {
        Self { synths }
    }
}