use crate::synth_description::DescriptionSynth;
use crate::synth_directive::DirectiveSynth;
use crate::synths::{ChainSynth, ListSynth, PairSynth, StringSynth, Synth, SynthContext};
use graphqxl_parser::Union;

pub(crate) struct UnionSynth(pub(crate) Union);

struct UnionTypesSynth(Vec<String>);

impl Synth for UnionTypesSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let list_synth = ListSynth::inline_or_multiline_suffixed((
            "",
            self.0
                .iter()
                .map(|t| StringSynth::from(t.as_str()))
                .collect(),
            (" | ", " |"),
            "",
        ));

        if self.0.len() > context.max_one_line_ors {
            list_synth.synth(&context.multiline())
        } else {
            list_synth.synth(&context.no_multiline())
        }
    }
}

impl Synth for UnionSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut v: Vec<Box<dyn Synth>> =
            vec![Box::new(StringSynth(format!("union {}", self.0.name)))];
        for directive in self.0.directives.iter() {
            v.push(Box::new(StringSynth::from(" ")));
            v.push(Box::new(DirectiveSynth(directive.clone())));
        }
        v.push(Box::new(StringSynth::from(" = ")));
        v.push(Box::new(UnionTypesSynth(self.0.types.clone())));
        let pair_synth = PairSynth::top_level(
            DescriptionSynth::from(self.0.description.as_str()),
            ChainSynth(v),
        );
        // todo: where does this 4 come from
        if self.0.types.len() > 4 {
            pair_synth.synth(&context.multiline())
        } else {
            pair_synth.synth(&context.no_multiline())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::Directive;

    #[test]
    fn test_one_type() {
        let synth = UnionSynth(Union::build("MyUnion").type_("MyType"));
        assert_eq!(synth.synth_zero(), "union MyUnion = MyType");
    }

    #[test]
    fn test_multiple() {
        let synth = UnionSynth(Union::build("MyUnion").type_("MyType1").type_("MyType2"));
        assert_eq!(synth.synth_zero(), "union MyUnion = MyType1 | MyType2");
    }

    #[test]
    fn test_with_comment() {
        let synth = UnionSynth(
            Union::build("MyUnion")
                .description("my description...\n..that takes two lines")
                .type_("MyType1")
                .type_("MyType2"),
        );
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
        let synth = UnionSynth(Union::build("MyUnion").type_("MyType1").type_("MyType2"));
        assert_eq!(
            synth.synth(&SynthContext::default().max_one_line_ors(1)),
            "\
union MyUnion = 
  MyType1 |
  MyType2"
        );
    }

    #[test]
    fn test_with_directives() {
        let synth = UnionSynth(
            Union::build("MyUnion")
                .type_("MyType1")
                .type_("MyType2")
                .directive(Directive::build("dir1"))
                .directive(Directive::build("dir2")),
        );
        assert_eq!(
            synth.synth(&SynthContext::default().max_one_line_ors(1)),
            "\
union MyUnion @dir1 @dir2 = 
  MyType1 |
  MyType2"
        );
    }
}
