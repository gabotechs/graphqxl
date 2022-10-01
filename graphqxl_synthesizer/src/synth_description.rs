use crate::synths::{Synth, SynthConfig, SynthContext};
use crate::utils::escape_non_escaped_quotes;

pub(crate) struct DescriptionSynth {
    pub(crate) text: String,
    pub(crate) is_multiline: bool,
    pub(crate) indent_spaces: usize,
}

impl DescriptionSynth {
    pub(crate) fn text(config: &SynthConfig, text: &str) -> Self {
        Self {
            text: text.to_string(),
            is_multiline: text.contains('\n'),
            indent_spaces: config.indent_spaces,
        }
    }
}

impl Synth for DescriptionSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut result = "".to_string();
        if self.text.is_empty() {
            return result;
        }
        if self.is_multiline {
            result += "\"\"\"";
            for line in self.text.split('\n') {
                result += "\n";
                result += &" ".repeat(context.indent_lvl * self.indent_spaces);
                result += &escape_non_escaped_quotes(line);
            }
            result += "\n";
            result += &" ".repeat(context.indent_lvl * self.indent_spaces);
            result += "\"\"\"";
        } else {
            result += "\"";
            result += &escape_non_escaped_quotes(&self.text);
            result += "\"";
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl DescriptionSynth {
        pub(crate) fn text_default(text: &str) -> Self {
            Self {
                text: text.to_string(),
                is_multiline: text.contains('\n'),
                indent_spaces: SynthConfig::default().indent_spaces,
            }
        }
    }

    #[test]
    fn test_synth_empty() {
        let synth = DescriptionSynth::text_default("");
        assert_eq!(synth.synth_zero(), "");
    }

    #[test]
    fn test_synth_one_line() {
        let synth = DescriptionSynth::text_default("This is one line");
        assert_eq!(synth.synth_zero(), "\"This is one line\"");
    }

    #[test]
    fn test_synth_multiline() {
        let synth = DescriptionSynth::text_default("These are two lines\nhi!");
        assert_eq!(
            synth.synth_zero(),
            "\
\"\"\"
These are two lines
hi!
\"\"\""
        )
    }

    #[test]
    fn test_synth_multiline_indented() {
        let synth = DescriptionSynth::text_default("These are two lines\nhi!");
        assert_eq!(
            synth.synth(&SynthContext::default().plus_one_indent_lvl()),
            "\
\"\"\"
  These are two lines
  hi!
  \"\"\""
        )
    }
}
