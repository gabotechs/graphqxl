#[derive(Copy, Clone, Default)]
pub(crate) struct SynthContext {
    pub(crate) indent_lvl: usize,
    pub(crate) indent_spaces: usize,
    pub(crate) multiline: bool,
}

pub(crate) trait Synth {
    fn synth(&self, context: &SynthContext) -> String;
    fn synth_zero(&self) -> String {
        self.synth(&SynthContext::default())
    }
}
