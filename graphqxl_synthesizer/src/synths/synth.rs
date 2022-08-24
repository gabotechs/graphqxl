#[derive(Copy, Clone)]
pub(crate) struct SynthContext {
    pub(crate) indent_lvl: usize,
    pub(crate) indent_spaces: usize,
    pub(crate) multiline: bool,
    pub(crate) max_one_line_args: usize,
    pub(crate) max_one_line_ors: usize,
    pub(crate) allow_multiline_values: bool,
}

impl Default for SynthContext {
    fn default() -> Self {
        Self {
            indent_lvl: 0,
            indent_spaces: 2,
            multiline: false,
            max_one_line_args: 2,
            max_one_line_ors: 2,
            allow_multiline_values: false,
        }
    }
}

impl SynthContext {
    pub(crate) fn plus_one_indent_lvl(&self) -> Self {
        let mut clone = *self;
        clone.indent_lvl += 1;
        clone
    }

    pub(crate) fn with_indent_lvl(&self, lvl: usize) -> Self {
        let mut clone = *self;
        clone.indent_lvl = lvl;
        clone
    }

    pub(crate) fn no_multiline(&self) -> Self {
        let mut clone = *self;
        clone.multiline = false;
        clone
    }

    pub(crate) fn multiline(&self) -> Self {
        let mut clone = *self;
        clone.multiline = true;
        clone
    }

    pub(crate) fn max_one_line_args(&self, n: usize) -> Self {
        let mut clone = *self;
        clone.max_one_line_args = n;
        clone
    }

    pub(crate) fn max_one_line_ors(&self, n: usize) -> Self {
        let mut clone = *self;
        clone.max_one_line_ors = n;
        clone
    }

    pub(crate) fn allow_multiline_values(&self) -> Self {
        let mut clone = *self;
        clone.allow_multiline_values = true;
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
            ..Default::default()
        })
    }
}
