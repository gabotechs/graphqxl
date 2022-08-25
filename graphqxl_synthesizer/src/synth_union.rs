use crate::synth_description::DescriptionSynth;
use crate::synth_directive::DirectiveSynth;
use crate::synths::{
    ChainSynth, MultilineListSynth, OneLineListSynth, PairSynth, StringSynth, Synth, SynthConfig,
    SynthContext,
};
use graphqxl_parser::Union;

pub(crate) struct UnionSynth(pub(crate) SynthConfig, pub(crate) Union);

struct UnionTypesSynth(pub(crate) SynthConfig, pub(crate) Vec<String>);

impl Synth for UnionTypesSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let inner_synths = self
            .1
            .iter()
            .map(|t| StringSynth::from(t.as_str()))
            .collect();
        if self.1.len() > self.0.max_one_line_ors {
            MultilineListSynth::or_suffix(&self.0, ("", inner_synths, "")).synth(context)
        } else {
            OneLineListSynth::or(("", inner_synths, "")).synth(context)
        }
    }
}

impl Synth for UnionSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut v: Vec<Box<dyn Synth>> =
            vec![Box::new(StringSynth(format!("union {}", self.1.name)))];
        for directive in self.1.directives.iter() {
            v.push(Box::new(StringSynth::from(" ")));
            v.push(Box::new(DirectiveSynth(self.0, directive.clone())));
        }
        v.push(Box::new(StringSynth::from(" = ")));
        v.push(Box::new(UnionTypesSynth(self.0, self.1.types.clone())));
        let pair_synth = PairSynth::top_level(
            &self.0,
            DescriptionSynth::text(&self.0, self.1.description.as_str()),
            ChainSynth(v),
        );
        pair_synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::Directive;

    impl UnionSynth {
        fn default(def: Union) -> Self {
            Self(SynthConfig::default(), def)
        }
    }

    #[test]
    fn test_one_type() {
        let synth = UnionSynth::default(Union::build("MyUnion").type_("MyType"));
        assert_eq!(synth.synth_zero(), "union MyUnion = MyType");
    }

    #[test]
    fn test_multiple() {
        let synth = UnionSynth::default(Union::build("MyUnion").type_("MyType1").type_("MyType2"));
        assert_eq!(synth.synth_zero(), "union MyUnion = MyType1 | MyType2");
    }

    #[test]
    fn test_with_comment() {
        let synth = UnionSynth::default(
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
        let synth = UnionSynth(
            SynthConfig::default().max_one_line_ors(1),
            Union::build("MyUnion").type_("MyType1").type_("MyType2"),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
union MyUnion = 
  MyType1 |
  MyType2"
        );
    }

    #[test]
    fn test_with_directives() {
        let synth = UnionSynth(
            SynthConfig::default().max_one_line_ors(1),
            Union::build("MyUnion")
                .type_("MyType1")
                .type_("MyType2")
                .directive(Directive::build("dir1"))
                .directive(Directive::build("dir2")),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
union MyUnion @dir1 @dir2 = 
  MyType1 |
  MyType2"
        );
    }
}
