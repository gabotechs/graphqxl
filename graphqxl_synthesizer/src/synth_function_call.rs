use crate::synth_value_data::ValueDataSynth;
use crate::utils::is_last_iter;
use crate::{Synth, SynthContext};
use graphqxl_parser::FunctionCall;

pub(crate) struct FunctionCallSynth(pub(crate) FunctionCall);

impl Synth for FunctionCallSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut summed = "(".to_string();
        for (is_last, input) in is_last_iter(self.0.inputs.iter()) {
            if context.multiline {
                summed += "\n";
                summed += " "
                    .repeat(context.indent_spaces * (context.indent_lvl + 1))
                    .as_str();
            } else {
                summed += " "
            }
            summed += input.name.as_str();
            summed += ": ";
            summed += ValueDataSynth(input.value.clone())
                .synth(&context.plus_one_indent_lvl())
                .as_str();
            if !is_last && !context.multiline {
                summed += ","
            }
        }
        if context.multiline {
            summed += "\n";
            summed += " "
                .repeat(context.indent_spaces * context.indent_lvl)
                .as_str();
        } else {
            summed += " ";
        }
        summed + ")"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::ValueData;

    #[test]
    fn test_one_argument() {
        let synth = FunctionCallSynth(FunctionCall::build().input("arg", ValueData::int(1)));
        assert_eq!(synth.synth_zero(), "( arg: 1 )")
    }

    #[test]
    fn test_one_argument_multiline() {
        let synth = FunctionCallSynth(FunctionCall::build().input("arg", ValueData::int(1)));
        assert_eq!(synth.synth_multiline(2), "(\n  arg: 1\n)")
    }

    #[test]
    fn test_multiple_arguments() {
        let synth = FunctionCallSynth(
            FunctionCall::build()
                .input("arg1", ValueData::int(1))
                .input("arg2", ValueData::float(1.0).list()),
        );
        assert_eq!(synth.synth_zero(), "( arg1: 1, arg2: [ 1.0 ] )")
    }

    #[test]
    fn test_multiple_arguments_multiline() {
        let synth = FunctionCallSynth(
            FunctionCall::build()
                .input("arg1", ValueData::int(1))
                .input("arg2", ValueData::float(1.0).list()),
        );
        assert_eq!(
            synth.synth_multiline(2),
            "\
(
  arg1: 1
  arg2: [
    1.0
  ]
)"
        )
    }
}
