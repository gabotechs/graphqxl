use crate::synths::{Synth, SynthContext};

pub(crate) struct DescriptionSynth {
    text: String,
    pub(crate) is_multiline: bool,
}

impl From<&str> for DescriptionSynth {
    fn from(text: &str) -> Self {
        Self {
            text: text.to_string(),
            is_multiline: text.contains('\n'),
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
                result += &" ".repeat(context.indent_lvl * context.indent_spaces);
                result += line;
            }
            result += "\n";
            result += &" ".repeat(context.indent_lvl * context.indent_spaces);
            result += "\"\"\"";
        } else {
            result += "\"";
            result += &self.text;
            result += "\"";
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synth_empty() {
        let synth = DescriptionSynth::from("");
        assert_eq!(synth.synth_zero(), "");
    }

    #[test]
    fn test_synth_one_line() {
        let synth = DescriptionSynth::from("This is one line");
        assert_eq!(synth.synth_zero(), "\"This is one line\"");
    }

    #[test]
    fn test_synth_multiline() {
        let synth = DescriptionSynth::from("These are two lines\nhi!");
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
        let synth = DescriptionSynth::from("These are two lines\nhi!");
        assert_eq!(
            synth.synth(&SynthContext {
                indent_spaces: 2,
                indent_lvl: 1,
                ..Default::default()
            }),
            "\
\"\"\"
  These are two lines
  hi!
  \"\"\""
        )
    }
}
