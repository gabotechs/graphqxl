use crate::synth_directive::DirectiveSynth;
use crate::synth_value_data::ValueDataSynth;
use crate::synth_value_type::ValueTypeSynth;
use crate::synths::{
    ChainSynth, MultilineListSynth, OneLineListSynth, StringSynth, Synth, SynthConfig, SynthContext,
};
use graphqxl_parser::Argument;

pub(crate) struct ArgumentsSynth(pub(crate) SynthConfig, pub(crate) Vec<Argument>);

impl Synth for ArgumentsSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let inner_synths = self
            .1
            .iter()
            .map(|argument| {
                let mut v: Vec<Box<dyn Synth>> = vec![
                    Box::new(StringSynth(argument.name.clone() + ": ")),
                    Box::new(ValueTypeSynth(argument.value_type.clone())),
                ];
                if let Some(default) = &argument.default {
                    v.push(Box::new(StringSynth::from(" = ")));
                    v.push(Box::new(ValueDataSynth(self.0, default.clone())));
                }
                for directive in argument.directives.iter() {
                    v.push(Box::new(StringSynth::from(" ")));
                    v.push(Box::new(DirectiveSynth(self.0, directive.clone())));
                }
                ChainSynth(v)
            })
            .collect();

        if self.1.len() > self.0.max_one_line_args {
            MultilineListSynth::no_suffix(&self.0, ("(", inner_synths, ")")).synth(context)
        } else {
            OneLineListSynth::comma(("(", inner_synths, ")")).synth(context)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::{Directive, ValueData};

    impl ArgumentsSynth {
        fn default(args: Vec<Argument>) -> Self {
            Self(SynthConfig::default(), args)
        }
    }

    #[test]
    fn test_one_argument() {
        let synth = ArgumentsSynth::default(vec![Argument::string("arg")]);
        assert_eq!(synth.synth_zero(), "(arg: String)")
    }

    #[test]
    fn test_two_argument() {
        let synth =
            ArgumentsSynth::default(vec![Argument::string("arg1"), Argument::string("arg2")]);
        assert_eq!(synth.synth_zero(), "(arg1: String, arg2: String)")
    }

    #[test]
    fn test_two_arguments_indent() {
        let config = SynthConfig::default().max_one_line_args(1);
        let synth = ArgumentsSynth(
            config,
            vec![Argument::string("arg"), Argument::string("arg2")],
        );
        assert_eq!(synth.synth_zero(), "(\n  arg: String\n  arg2: String\n)")
    }

    #[test]
    fn test_with_default_value() {
        let synth =
            ArgumentsSynth::default(vec![Argument::int("arg").default(ValueData::int(1).list())]);
        assert_eq!(synth.synth_zero(), "(arg: Int = [ 1 ])")
    }

    #[test]
    fn test_with_directives() {
        let synth =
            ArgumentsSynth::default(vec![Argument::int("arg").directive(Directive::build("dir"))]);
        assert_eq!(synth.synth_zero(), "(arg: Int @dir)")
    }

    #[test]
    fn test_with_default_value_with_directives() {
        let synth = ArgumentsSynth::default(vec![Argument::int("arg")
            .default(ValueData::int(1).list())
            .directive(Directive::build("dir"))]);
        assert_eq!(synth.synth_zero(), "(arg: Int = [ 1 ] @dir)")
    }
}
