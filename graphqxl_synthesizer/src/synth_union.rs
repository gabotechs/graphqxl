use crate::synth_description::DescriptionSynth;
use crate::synth_directive::DirectiveSynth;
use crate::synth_identifier::IdentifierSynth;
use crate::synths::{
    ChainSynth, MultilineListSynth, OneLineListSynth, PairSynth, StringSynth, Synth, SynthContext,
};
use graphqxl_parser::{Identifier, Union};

pub(crate) struct UnionSynth(pub(crate) Union);

struct UnionTypesSynth(pub(crate) Vec<Identifier>);

impl Synth for UnionTypesSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        let inner_synths = self.0.iter().map(|t| IdentifierSynth(t.clone())).collect();
        if self.0.len() > context.config.max_one_line_ors {
            MultilineListSynth::or_suffix(("", inner_synths, "")).synth(context);
        } else {
            OneLineListSynth::or(("", inner_synths, "")).synth(context);
        }
        true
    }
}

impl Synth for UnionSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        let mut v: Vec<Box<dyn Synth>> = match self.0.extend {
            true => vec![Box::new(StringSynth::from("extend "))],
            false => vec![],
        };

        v.push(Box::new(StringSynth::from("union ")));
        v.push(Box::new(IdentifierSynth(self.0.name.clone())));
        for directive in self.0.directives.iter() {
            v.push(Box::new(StringSynth::from(" ")));
            v.push(Box::new(DirectiveSynth(directive.clone())));
        }
        if !(self.0.extend && self.0.types.is_empty()) {
            v.push(Box::new(StringSynth::from(" = ")));
            v.push(Box::new(UnionTypesSynth(self.0.types.clone())));
        }

        let pair_synth = PairSynth::top_level(
            DescriptionSynth::text(self.0.description.as_str()),
            ChainSynth(v),
        );
        pair_synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SynthConfig;
    use graphqxl_parser::Directive;

    #[test]
    fn test_one_type() {
        let synth = UnionSynth(Union::build("MyUnion").type_("MyType"));
        assert_eq!(synth.synth_zero(), "union MyUnion = MyType");
    }

    #[test]
    fn test_empty_with_extension() {
        let synth = UnionSynth(Union::build("MyUnion").extend());
        assert_eq!(synth.synth_zero(), "extend union MyUnion");
    }

    #[test]
    fn test_one_type_with_extension() {
        let synth = UnionSynth(Union::build("MyUnion").type_("MyType").extend());
        assert_eq!(synth.synth_zero(), "extend union MyUnion = MyType");
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
        let mut context = SynthContext::default();
        context.with_config(SynthConfig::default().max_one_line_ors(1));
        synth.synth(&mut context);
        assert_eq!(
            context.result,
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
        let mut context = SynthContext::default();
        context.with_config(SynthConfig::default().max_one_line_ors(1));
        synth.synth(&mut context);
        assert_eq!(
            context.result,
            "\
union MyUnion @dir1 @dir2 = 
  MyType1 |
  MyType2"
        );
    }
}
