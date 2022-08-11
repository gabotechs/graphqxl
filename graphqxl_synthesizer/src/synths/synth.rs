#[derive(Copy, Clone, Default)]
pub(crate) struct SynthContext {
    pub(crate) indent_lvl: usize,
    pub(crate) indent_spaces: usize,
    pub(crate) multiline: bool,
}

impl SynthContext {
    pub(crate) fn plus_one_indent_lvl(&self) -> Self {
        let mut clone = *self;
        clone.indent_lvl += 1;
        clone
    }
}

pub(crate) trait Synth {
    fn synth(&self, context: &SynthContext) -> String;
    fn synth_zero(&self) -> String {
        self.synth(&SynthContext::default())
    }
    fn synth_multiline(&self, indent: usize) -> String {
        self.synth(&SynthContext {
            multiline: true,
            indent_spaces: indent,
            ..Default::default()
        })
    }

    fn synth_multiline_offset(&self, indent: usize, indent_start: usize) -> String {
        self.synth(&SynthContext {
            multiline: true,
            indent_spaces: indent,
            indent_lvl: indent_start,
        })
    }
}
