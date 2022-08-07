use crate::synth_value_type::ValueTypeSynth;
use crate::synths::{ListSynth, PairSynth, StringSynth, Synth, SynthContext};
use graphqxl_parser::Argument;

pub(crate) struct ArgumentsSynth(pub(crate) Vec<Argument>);

impl Synth for ArgumentsSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let list_synth = ListSynth::from((
            "(",
            self.0
                .iter()
                .map(|argument| {
                    PairSynth::inline(
                        StringSynth(argument.name.clone() + ":"),
                        ValueTypeSynth(argument.value.clone()),
                    )
                    // todo: missing default
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
    use crate::test_utils::simple_string_arg_factory;

    #[test]
    fn test_one_argument() {
        let synth = ArgumentsSynth(vec![simple_string_arg_factory("arg")]);

        assert_eq!(synth.synth_zero(), "(arg: String)")
    }

    #[test]
    fn test_two_argument() {
        let synth = ArgumentsSynth(vec![
            simple_string_arg_factory("arg1"),
            simple_string_arg_factory("arg2"),
        ]);

        assert_eq!(synth.synth_zero(), "(arg1: String arg2: String)")
    }

    #[test]
    fn test_one_argument_indent() {
        let synth = ArgumentsSynth(vec![
            simple_string_arg_factory("arg"),
            simple_string_arg_factory("arg2"),
        ]);

        assert_eq!(
            synth.synth(&SynthContext {
                indent_spaces: 2,
                multiline: true,
                ..Default::default()
            }),
            "(\n  arg: String \n  arg2: String\n)"
        )
    }
}
