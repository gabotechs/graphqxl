use crate::synths::{Synth, SynthConfig, SynthContext};
use crate::utils::escape_non_escaped_quotes;

pub(crate) struct DescriptionSynth {
    pub(crate) text: String,
    pub(crate) is_multiline: bool,
}

impl DescriptionSynth {
    pub(crate) fn text(text: &str) -> Self {
        Self {
            text: text.to_string(),
            is_multiline: text.contains('\n'),
        }
    }
}

impl Synth for DescriptionSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        if self.text.is_empty() {
            return false;
        }
        if self.is_multiline {
            context.write("\"\"\"");
            for line in self.text.split('\n') {
                context.write_line_jump();
                context.write(&" ".repeat(context.indent_lvl * context.config.indent_spaces));
                context.write(&escape_non_escaped_quotes(line));
            }
            context.write_line_jump();
            context.write(&" ".repeat(context.indent_lvl * context.config.indent_spaces));
            context.write("\"\"\"");
        } else {
            context.write("\"");
            context.write(&escape_non_escaped_quotes(&self.text));
            context.write("\"");
        }
        true
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
        let mut context = SynthContext::default();
        context.push_indent_level();
        synth.synth(&mut context);
        assert_eq!(
            context.result,
            "\
\"\"\"
  These are two lines
  hi!
  \"\"\""
        )
    }
}
