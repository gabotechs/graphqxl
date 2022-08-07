use crate::synth_description::DescriptionSynth;
use crate::synths::{PairSynth, StringSynth, Synth};
use graphqxl_parser::Scalar;

pub(crate) struct SynthScalar {
    scalar: Scalar,
}

impl Synth for SynthScalar {
    fn synth(&self, indent_lvl: usize, multiline: bool) -> String {
        let pair_synth = PairSynth::top_level(
            DescriptionSynth::from(self.scalar.description.as_str()),
            StringSynth(format!("scalar {}", self.scalar.name)),
        );
        pair_synth.synth(indent_lvl, multiline)
    }
}

impl From<Scalar> for SynthScalar {
    fn from(scalar: Scalar) -> Self {
        Self { scalar }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_without_description() {
        let synth = SynthScalar::from(Scalar {
            name: "MyScalar".to_string(),
            description: "".to_string(),
        });
        assert_eq!(synth.synth_zero(), "scalar MyScalar")
    }

    #[test]
    fn test_scalar_with_description() {
        let synth = SynthScalar::from(Scalar {
            name: "MyScalar".to_string(),
            description: "my description".to_string(),
        });
        assert_eq!(synth.synth_zero(), "\"my description\"\nscalar MyScalar")
    }
}
