use crate::synth_description::DescriptionSynth;
use crate::synth_directive::DirectiveSynth;
use crate::synths::{ChainSynth, PairSynth, StringSynth, Synth, SynthConfig, SynthContext};
use graphqxl_parser::Scalar;

pub(crate) struct ScalarSynth(pub(crate) SynthConfig, pub(crate) Scalar);

impl Synth for ScalarSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut v: Vec<Box<dyn Synth>> =
            vec![Box::new(StringSynth(format!("scalar {}", self.1.name)))];
        for directive in self.1.directives.iter() {
            v.push(Box::new(StringSynth::from(" ")));
            v.push(Box::new(DirectiveSynth(self.0, directive.clone())));
        }
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

    impl ScalarSynth {
        fn default(def: Scalar) -> Self {
            Self(SynthConfig::default(), def)
        }
    }

    #[test]
    fn test_scalar_without_description() {
        let synth = ScalarSynth::default(Scalar::build("MyScalar"));
        assert_eq!(synth.synth_zero(), "scalar MyScalar")
    }

    #[test]
    fn test_scalar_with_description() {
        let synth = ScalarSynth::default(Scalar::build("MyScalar").description("my description"));
        assert_eq!(synth.synth_zero(), "\"my description\"\nscalar MyScalar")
    }

    #[test]
    fn test_with_directives() {
        let synth = ScalarSynth::default(
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
