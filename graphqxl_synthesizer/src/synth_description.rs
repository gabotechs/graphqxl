use crate::synths::Synth;

pub(crate) struct DescriptionSynth {
    text: String,
    indent: usize,
    pub(crate) is_multiline: bool,
}

impl From<(&str, usize)> for DescriptionSynth {
    fn from(text_indent: (&str, usize)) -> Self {
        Self {
            text: text_indent.0.to_string(),
            indent: text_indent.1,
            is_multiline: text_indent.0.contains("\n"),
        }
    }
}

impl From<&str> for DescriptionSynth {
    fn from(text: &str) -> Self {
        Self::from((text, 0))
    }
}

impl Synth for DescriptionSynth {
    fn synth(&self, indent_lvl: usize, _multiline: bool) -> String {
        let mut result = "".to_string();
        if self.text.is_empty() {
            return result;
        }
        if self.is_multiline {
            result += "\"\"\"";
            for line in self.text.split("\n") {
                result += "\n";
                result += &" ".repeat(indent_lvl * self.indent);
                result += line;
            }
            result += "\n";
            result += &" ".repeat(indent_lvl * self.indent);
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
        let synth = DescriptionSynth::from(("These are two lines\nhi!", 2));
        assert_eq!(
            synth.synth(1, false),
            "\
\"\"\"
  These are two lines
  hi!
  \"\"\""
        )
    }
}
