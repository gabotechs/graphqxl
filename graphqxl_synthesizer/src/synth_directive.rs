use crate::synth_function_call::FunctionCallSynth;
use crate::synths::{ChainSynth, StringSynth, SynthConfig};
use crate::{Synth, SynthContext};
use graphqxl_parser::Directive;

pub(crate) struct DirectiveSynth(pub(crate) SynthConfig, pub(crate) Directive);

impl Synth for DirectiveSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut v: Vec<Box<dyn Synth>> = vec![Box::new(StringSynth(
            "@".to_string() + self.1.name.as_str(),
        ))];
        if let Some(call) = &self.1.call {
            v.push(Box::new(FunctionCallSynth(self.0, call.clone())));
        }

        let synth = ChainSynth(v);
        synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::ValueData;

    impl DirectiveSynth {
        fn default(def: Directive) -> Self {
            Self(SynthConfig::default(), def)
        }
    }

    #[test]
    fn test_directive_with_no_inputs() {
        let synth = DirectiveSynth::default(Directive::build("dir"));
        assert_eq!(synth.synth_zero(), "@dir")
    }

    #[test]
    fn test_directive_with_one_input() {
        let synth = DirectiveSynth::default(
            Directive::build("dir").input("arg", ValueData::string("data")),
        );
        assert_eq!(synth.synth_zero(), "@dir(arg: \"data\")")
    }

    #[test]
    fn test_directive_with_multiple_inputs() {
        let synth = DirectiveSynth::default(
            Directive::build("dir")
                .input("arg", ValueData::string("data"))
                .input("arg2", ValueData::boolean(true).to_object("bool").list()),
        );
        assert_eq!(
            synth.synth_zero(),
            "@dir(arg: \"data\", arg2: [ { bool: true } ])"
        )
    }

    #[test]
    fn test_directive_with_multiple_inputs_multiline() {
        let synth = DirectiveSynth::default(
            Directive::build("dir")
                .input("arg", ValueData::string("data"))
                .input("arg1", ValueData::int(1))
                .input("arg2", ValueData::boolean(true).to_object("bool").list()),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
@dir(
  arg: \"data\"
  arg1: 1
  arg2: [ { bool: true } ]
)"
        )
    }
}
