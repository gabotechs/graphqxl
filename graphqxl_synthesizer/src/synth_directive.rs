use crate::synth_function_call::FunctionCallSynth;
use crate::synths::{ChainSynth, StringSynth};
use crate::{Synth, SynthContext};
use graphqxl_parser::Directive;

pub(crate) struct DirectiveSynth(pub(crate) Directive);

impl Synth for DirectiveSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut v: Vec<Box<dyn Synth>> = vec![Box::new(StringSynth(
            "@".to_string() + self.0.name.as_str(),
        ))];
        if let Some(call) = &self.0.call {
            v.push(Box::new(FunctionCallSynth(call.clone())));
        }

        let synth = ChainSynth(v);
        synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::ValueData;

    #[test]
    fn test_directive_with_no_inputs() {
        let synth = DirectiveSynth(Directive::build("dir"));
        assert_eq!(synth.synth_zero(), "@dir")
    }

    #[test]
    fn test_directive_with_one_input() {
        let synth = DirectiveSynth(Directive::build("dir").input("arg", ValueData::string("data")));
        assert_eq!(synth.synth_zero(), "@dir( arg: \"data\" )")
    }

    #[test]
    fn test_directive_with_multiple_inputs() {
        let synth = DirectiveSynth(
            Directive::build("dir")
                .input("arg", ValueData::string("data"))
                .input("arg2", ValueData::boolean(true).to_object("bool").list()),
        );
        assert_eq!(
            synth.synth_zero(),
            "@dir( arg: \"data\", arg2: [ { bool: true } ] )"
        )
    }

    #[test]
    fn test_directive_with_multiple_inputs_multiline() {
        let synth = DirectiveSynth(
            Directive::build("dir")
                .input("arg", ValueData::string("data"))
                .input("arg2", ValueData::boolean(true).to_object("bool").list()),
        );
        assert_eq!(
            synth.synth_multiline(2),
            "\
@dir(
  arg: \"data\"
  arg2: [
    {
      bool: true
    }
  ]
)"
        )
    }
}
