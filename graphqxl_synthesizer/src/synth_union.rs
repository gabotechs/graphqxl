use crate::synth_description::DescriptionSynth;
use crate::synths::{ListSynth, PairSynth, StringSynth, Synth};
use graphqxl_parser::Union;

pub(crate) struct UnionSynth {
    indent: usize,
    union: Union,
}

impl Synth for UnionSynth {
    fn synth(&self, _indent_lvl: usize, multiline: bool) -> String {
        let pair_synth = PairSynth::top_level(
            DescriptionSynth::from((self.union.description.as_str(), 0)),
            PairSynth {
                first: StringSynth(format!("union {} =", self.union.name)),
                last: ListSynth {
                    indent: self.indent,
                    items: self
                        .union
                        .types
                        .iter()
                        .map(|t| StringSynth::from(t.as_str()))
                        .collect(),
                    sep: " | ".to_string(),
                    wrapper: ("".to_string(), "".to_string()),
                },
                line_jump_sep: false,
                indent: 0,
            },
        );
        pair_synth.synth(0, multiline)
    }
}

impl From<Union> for UnionSynth {
    fn from(union: Union) -> Self {
        Self { union, indent: 0 }
    }
}

impl From<(Union, usize)> for UnionSynth {
    fn from(union_indent: (Union, usize)) -> Self {
        Self {
            union: union_indent.0,
            indent: union_indent.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_type() {
        let synth = UnionSynth::from(Union {
            name: "MyUnion".to_string(),
            description: "".to_string(),
            types: vec!["MyType".to_string()],
        });
        assert_eq!(synth.synth_zero(), "union MyUnion = MyType");
    }

    #[test]
    fn test_multiple() {
        let synth = UnionSynth::from(Union {
            name: "MyUnion".to_string(),
            description: "".to_string(),
            types: vec!["MyType1".to_string(), "MyType2".to_string()],
        });
        assert_eq!(synth.synth_zero(), "union MyUnion = MyType1 | MyType2");
    }

    #[test]
    fn test_with_comment() {
        let synth = UnionSynth::from(Union {
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
        let synth = UnionSynth::from((
            Union {
                name: "MyUnion".to_string(),
                description: "".to_string(),
                types: vec!["MyType1".to_string(), "MyType2".to_string()],
            },
            2,
        ));
        assert_eq!(
            synth.synth(0, true),
            "\
union MyUnion = 
  MyType1 | 
  MyType2
"
        );
    }
}
