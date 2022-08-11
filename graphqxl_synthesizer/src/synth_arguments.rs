use crate::synth_directive::DirectiveSynth;
use crate::synth_value_data::ValueDataSynth;
use crate::synth_value_type::ValueTypeSynth;
use crate::synths::{ChainSynth, ListSynth, StringSynth, Synth, SynthContext};
use graphqxl_parser::Argument;

pub(crate) struct ArgumentsSynth(pub(crate) Vec<Argument>);

impl Synth for ArgumentsSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let list_synth = ListSynth::from((
            "(",
            self.0
                .iter()
                .map(|argument| {
                    let mut v: Vec<Box<dyn Synth>> = vec![
                        Box::new(StringSynth(argument.name.clone() + ": ")),
                        Box::new(ValueTypeSynth(argument.value_type.clone())),
                    ];
                    if let Some(default) = &argument.default {
                        v.push(Box::new(StringSynth::from(" = ")));
                        v.push(Box::new(ValueDataSynth(default.clone())));
                    }
                    for directive in argument.directives.iter() {
                        v.push(Box::new(StringSynth::from(" ")));
                        v.push(Box::new(DirectiveSynth(directive.clone())));
                    }
                    ChainSynth(v)
                })
                .collect(),
            " ",
            ")",
        ));
        list_synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::{Directive, ValueData};

    #[test]
    fn test_one_argument() {
        let synth = ArgumentsSynth(vec![Argument::string("arg")]);
        assert_eq!(synth.synth_zero(), "(arg: String)")
    }

    #[test]
    fn test_two_argument() {
        let synth = ArgumentsSynth(vec![Argument::string("arg1"), Argument::string("arg2")]);
        assert_eq!(synth.synth_zero(), "(arg1: String arg2: String)")
    }

    #[test]
    fn test_one_argument_indent() {
        let synth = ArgumentsSynth(vec![Argument::string("arg"), Argument::string("arg2")]);

        assert_eq!(
            synth.synth_multiline(2),
            "(\n  arg: String \n  arg2: String\n)"
        )
    }

    #[test]
    fn test_with_default_value() {
        let synth = ArgumentsSynth(vec![Argument::int("arg").default(ValueData::int(1).list())]);
        assert_eq!(synth.synth_zero(), "(arg: Int = [ 1 ])")
    }

    #[test]
    fn test_with_directives() {
        let synth = ArgumentsSynth(vec![Argument::int("arg").directive(Directive::build("dir"))]);
        assert_eq!(synth.synth_zero(), "(arg: Int @dir)")
    }

    #[test]
    fn test_with_default_value_with_directives() {
        let synth = ArgumentsSynth(vec![Argument::int("arg")
            .default(ValueData::int(1).list())
            .directive(Directive::build("dir"))]);
        assert_eq!(synth.synth_zero(), "(arg: Int = [ 1 ] @dir)")
    }
}
