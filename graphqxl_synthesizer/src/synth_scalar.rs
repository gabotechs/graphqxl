use crate::synth_description::DescriptionSynth;
use crate::synth_directive::DirectiveSynth;
use crate::synths::{ChainSynth, PairSynth, StringSynth, Synth, SynthConfig, SynthContext};
use graphqxl_parser::Scalar;

pub(crate) struct ScalarSynth(pub(crate) Scalar);

impl Synth for ScalarSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut v: Vec<Box<dyn Synth>> =
            vec![Box::new(StringSynth(format!("scalar {}", self.0.name.id)))];
        for directive in self.0.directives.iter() {
            v.push(Box::new(StringSynth::from(" ")));
            v.push(Box::new(DirectiveSynth(directive.clone())));
        }
        let pair_synth = PairSynth::top_level(
            &context.config,
            DescriptionSynth::text(&context.config, &self.0.description.as_str()),
            ChainSynth(v),
        );
        pair_synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::Directive;

    #[test]
    fn test_scalar_without_description() {
        let synth = ScalarSynth(Scalar::build("MyScalar"));
        assert_eq!(synth.synth_zero(), "scalar MyScalar")
    }

    #[test]
    fn test_scalar_with_description() {
        let synth = ScalarSynth(Scalar::build("MyScalar").description("my description"));
        assert_eq!(synth.synth_zero(), "\"my description\"\nscalar MyScalar")
    }

    #[test]
    fn test_with_directives() {
        let synth = ScalarSynth(
            Scalar::build("MyScalar")
                .description("my description")
                .directive(Directive::build("dir1")),
        );
        assert_eq!(
            synth.synth_zero(),
            "\"my description\"\nscalar MyScalar @dir1"
        )
    }
}
