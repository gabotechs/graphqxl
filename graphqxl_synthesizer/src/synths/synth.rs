#[derive(Copy, Clone)]
pub struct SynthConfig {
    pub indent_spaces: usize,
    pub max_one_line_args: usize,
    pub max_one_line_ors: usize,
    pub allow_multiline_values: bool,
}

impl Default for SynthConfig {
    fn default() -> Self {
        Self {
            indent_spaces: 2,
            max_one_line_args: 2,
            max_one_line_ors: 2,
            allow_multiline_values: false,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub(crate) struct SynthContext {
    pub(crate) indent_lvl: usize,
    pub(crate) config: SynthConfig,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    impl SynthConfig {
        pub(crate) fn indent_spaces(&self, n: usize) -> Self {
            let mut clone = *self;
            clone.indent_spaces = n;
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

    impl SynthContext {
        pub(crate) fn with_indent_lvl(&self, lvl: usize) -> Self {
            let mut clone = *self;
            clone.indent_lvl = lvl;
            clone
        }

        pub(crate) fn with_config(&self, config: SynthConfig) -> Self {
            let mut clone = *self;
            clone.config = config;
            clone
        }
    }
}
