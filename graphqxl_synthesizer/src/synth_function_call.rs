use crate::synth_value_data::ValueDataSynth;
use crate::synths::{ChainSynth, MultilineListSynth, OneLineListSynth, StringSynth};
use crate::{Synth, SynthContext};
use graphqxl_parser::FunctionCall;

pub(crate) struct FunctionCallSynth(pub(crate) FunctionCall);

impl Synth for FunctionCallSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        let inner_synths = self
            .0
            .inputs
            .iter()
            .map(|e| {
                ChainSynth(vec![
                    Box::new(StringSynth(format!("{}: ", e.name.id))),
                    Box::new(ValueDataSynth(e.value.clone())),
                ])
            })
            .collect();
        if self.0.inputs.len() > context.config.max_one_line_args {
            MultilineListSynth::no_suffix(("(", inner_synths, ")")).synth(context);
        } else {
            OneLineListSynth::comma(("(", inner_synths, ")")).synth(context);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SynthConfig;
    use graphqxl_parser::ValueData;

    #[test]
    fn test_one_argument() {
        let synth = FunctionCallSynth(FunctionCall::build().input("arg", ValueData::int(1)));
        assert_eq!(synth.synth_zero(), "(arg: 1)")
    }

    #[test]
    fn test_one_argument_multiline() {
        let synth = FunctionCallSynth(FunctionCall::build().input("arg", ValueData::int(1)));
        let mut context = SynthContext::default();
        context.with_config(SynthConfig::default().max_one_line_args(0));
        synth.synth(&mut context);
        assert_eq!(context.result, "(\n  arg: 1\n)")
    }

    #[test]
    fn test_multiple_arguments() {
        let synth = FunctionCallSynth(
            FunctionCall::build()
                .input("arg1", ValueData::int(1))
                .input("arg2", ValueData::float(1.0).list()),
        );
        assert_eq!(synth.synth_zero(), "(arg1: 1, arg2: [ 1.0 ])")
    }

    #[test]
    fn test_multiple_arguments_multiline() {
        let synth = FunctionCallSynth(
            FunctionCall::build()
                .input("arg1", ValueData::int(1))
                .input("arg2", ValueData::float(1.0).list()),
        );
        let mut context = SynthContext::default();
        context.with_config(SynthConfig::default().max_one_line_args(1));
        synth.synth(&mut context);
        assert_eq!(
            context.result,
            "\
(
  arg1: 1
  arg2: [ 1.0 ]
)"
        )
    }
}
