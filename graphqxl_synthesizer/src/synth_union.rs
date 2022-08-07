use crate::synth_description::DescriptionSynth;
use crate::synths::{ListSynth, PairSynth, StringSynth, Synth, SynthContext};
use graphqxl_parser::Union;

pub(crate) struct UnionSynth(Union);

impl Synth for UnionSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let pair_synth = PairSynth::top_level(
            DescriptionSynth::from(self.0.description.as_str()),
            PairSynth::inline(
                StringSynth(format!("union {} =", self.0.name)),
                ListSynth::from((
                    "",
                    self.0
                        .types
                        .iter()
                        .map(|t| StringSynth::from(t.as_str()))
                        .collect(),
                    " | ",
                    "",
                )),
            ),
        );
        pair_synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_type() {
        let synth = UnionSynth(Union {
            name: "MyUnion".to_string(),
            description: "".to_string(),
            types: vec!["MyType".to_string()],
        });
        assert_eq!(synth.synth_zero(), "union MyUnion = MyType");
    }

    #[test]
    fn test_multiple() {
        let synth = UnionSynth(Union {
            name: "MyUnion".to_string(),
            description: "".to_string(),
            types: vec!["MyType1".to_string(), "MyType2".to_string()],
        });
        assert_eq!(synth.synth_zero(), "union MyUnion = MyType1 | MyType2");
    }

    #[test]
    fn test_with_comment() {
        let synth = UnionSynth(Union {
            name: "MyUnion".to_string(),
            description: "my description...\n..that takes two lines".to_string(),
            types: vec!["MyType1".to_string(), "MyType2".to_string()],
        });
        assert_eq!(
            synth.synth_zero(),
            "\
\"\"\"
my description...
..that takes two lines
\"\"\"
union MyUnion = MyType1 | MyType2"
        );
    }

    #[test]
    fn test_indented() {
        let synth = UnionSynth(Union {
            name: "MyUnion".to_string(),
            description: "".to_string(),
            types: vec!["MyType1".to_string(), "MyType2".to_string()],
        });
        assert_eq!(
            synth.synth(&SynthContext {
                multiline: true,
                indent_spaces: 2,
                ..Default::default()
            }),
            "\
union MyUnion = 
  MyType1 | 
  MyType2
"
        );
    }
}
