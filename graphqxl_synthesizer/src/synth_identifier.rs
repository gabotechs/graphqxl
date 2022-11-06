use crate::synths::{Synth, SynthContext};
use graphqxl_parser::Identifier;

pub(crate) struct IdentifierSynth(pub(crate) Identifier);

impl Synth for IdentifierSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        context.write(&self.0.id);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synth_identifier() {
        let synth = IdentifierSynth(Identifier::from("MyType"));
        assert_eq!(synth.synth_zero(), "MyType")
    }
}
