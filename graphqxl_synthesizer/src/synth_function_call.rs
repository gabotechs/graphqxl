use crate::synth_value_data::ValueDataSynth;
use crate::synths::{ChainSynth, MultilineListSynth, OneLineListSynth, StringSynth, SynthConfig};
use crate::{Synth, SynthContext};
use graphqxl_parser::FunctionCall;

pub(crate) struct FunctionCallSynth(pub(crate) SynthConfig, pub(crate) FunctionCall);

impl Synth for FunctionCallSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let inner_synths = self
            .1
            .inputs
            .iter()
            .map(|e| {
                ChainSynth(vec![
                    Box::new(StringSynth(format!("{}: ", e.name))),
                    Box::new(ValueDataSynth(self.0, e.value.clone())),
                ])
            })
            .collect();
        if self.1.inputs.len() > self.0.max_one_line_args {
            MultilineListSynth::no_suffix(&self.0, ("(", inner_synths, ")")).synth(context)
        } else {
            OneLineListSynth::comma(("(", inner_synths, ")")).synth(context)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::ValueData;

    impl FunctionCallSynth {
        fn default(def: FunctionCall) -> Self {
            Self(SynthConfig::default(), def)
        }
    }

    #[test]
    fn test_one_argument() {
        let synth =
            FunctionCallSynth::default(FunctionCall::build().input("arg", ValueData::int(1)));
        assert_eq!(synth.synth_zero(), "(arg: 1)")
    }

    #[test]
    fn test_one_argument_multiline() {
        let synth = FunctionCallSynth(
            SynthConfig::default().max_one_line_args(0),
            FunctionCall::build().input("arg", ValueData::int(1)),
        );
        assert_eq!(synth.synth(&SynthContext::default()), "(\n  arg: 1\n)")
    }

    #[test]
    fn test_multiple_arguments() {
        let synth = FunctionCallSynth::default(
            FunctionCall::build()
                .input("arg1", ValueData::int(1))
                .input("arg2", ValueData::float(1.0).list()),
        );
        assert_eq!(synth.synth_zero(), "(arg1: 1, arg2: [ 1.0 ])")
    }

    #[test]
    fn test_multiple_arguments_multiline() {
        let synth = FunctionCallSynth(
            SynthConfig::default().max_one_line_args(1),
            FunctionCall::build()
                .input("arg1", ValueData::int(1))
                .input("arg2", ValueData::float(1.0).list()),
        );
        assert_eq!(
            synth.synth(&SynthContext::default()),
            "\
(
  arg1: 1
  arg2: [ 1.0 ]
)"
        )
    }
}
