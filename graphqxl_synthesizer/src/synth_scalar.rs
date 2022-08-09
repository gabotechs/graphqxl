use crate::synth_description::DescriptionSynth;
use crate::synths::{PairSynth, StringSynth, Synth, SynthContext};
use graphqxl_parser::Scalar;

pub(crate) struct ScalarSynth(pub(crate) Scalar);

impl Synth for ScalarSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let pair_synth = PairSynth::top_level(
            DescriptionSynth::from(self.0.description.as_str()),
            StringSynth(format!("scalar {}", self.0.name)),
        );
        pair_synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
