use graphqxl_parser::OwnedSpan;

#[derive(Clone)]
pub struct SynthConfig {
    pub indent_spaces: usize,
    pub max_one_line_args: usize,
    pub max_one_line_ors: usize,
    pub allow_multiline_values: bool,
    pub private_prefix: String,
}

impl Default for SynthConfig {
    fn default() -> Self {
        Self {
            indent_spaces: 2,
            max_one_line_args: 2,
            max_one_line_ors: 2,
            allow_multiline_values: false,
            private_prefix: "_".to_string(),
        }
    }
}

pub struct SourceMapEntry {
    pub line: usize,
    pub col: usize,
    pub start: usize,
    pub stop: usize,
    pub span: OwnedSpan,
}

#[derive(Default)]
pub(crate) struct SynthContext {
    pub(crate) result: String,
    pub(crate) source_map: Vec<SourceMapEntry>,
    pub(crate) indent_lvl: usize,
    pub(crate) offset: usize,
    pub(crate) line: usize,
    pub(crate) col: usize,
    pub(crate) config: SynthConfig,
}

impl SynthContext {
    pub(crate) fn push_indent_level(&mut self) {
        self.indent_lvl += 1
    }

    pub(crate) fn pop_indent_level(&mut self) {
        self.indent_lvl -= 1
    }

    pub(crate) fn write<'a>(&mut self, text: &'a str) -> &'a str {
        self.offset += text.len();
        self.result += text;
        text
    }

    pub(crate) fn write_with_source<'a>(&mut self, text: &'a str, span: &OwnedSpan) -> &'a str {
        let start = self.offset;
        self.write(text);
        let stop = self.offset;
        self.source_map.push(SourceMapEntry {
            line: self.line,
            col: self.col,
            start,
            stop,
            span: span.clone(),
        });
        text
    }

    pub(crate) fn write_line_jump(&mut self) {
        self.line += 1;
        self.col = 0;
        self.write("\n");
    }

    pub(crate) fn write_double_line_jump(&mut self) {
        self.write_line_jump();
        self.write_line_jump();
    }

    pub(crate) fn write_indent(&mut self, indent_lvl: usize) {
        self.write(&" ".repeat(indent_lvl * self.config.indent_spaces));
    }
}

pub(crate) trait Synth {
    fn synth(&self, context: &mut SynthContext) -> bool;
    fn synth_zero(&self) -> String {
        let mut context = SynthContext::default();
        self.synth(&mut context);
        context.result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl SynthConfig {
        pub(crate) fn indent_spaces(&self, n: usize) -> Self {
            let mut clone = self.clone();
            clone.indent_spaces = n;
            clone
        }

        pub(crate) fn max_one_line_args(&self, n: usize) -> Self {
            let mut clone = self.clone();
            clone.max_one_line_args = n;
            clone
        }

        pub(crate) fn max_one_line_ors(&self, n: usize) -> Self {
            let mut clone = self.clone();
            clone.max_one_line_ors = n;
            clone
        }

        pub(crate) fn allow_multiline_values(&self) -> Self {
            let mut clone = self.clone();
            clone.allow_multiline_values = true;
            clone
        }
    }

    impl SynthContext {
        pub(crate) fn with_indent_lvl(&mut self, lvl: usize) {
            self.indent_lvl = lvl;
        }

        pub(crate) fn with_config(&mut self, config: SynthConfig) {
            self.config = config;
        }
    }
}
