use crate::synth_value_type::ValueTypeSynth;
use crate::synths::{ListSynth, PairSynth, StringSynth, Synth};
use graphqxl_parser::Argument;

pub(crate) struct ArgumentsSynth {
    indent: usize,
    arguments: Vec<Argument>,
}

impl Synth for ArgumentsSynth {
    fn synth(&self, indent_lvl: usize, multiline: bool) -> String {
        let list_synth = ListSynth {
            wrapper: ("(".to_string(), ")".to_string()),
            sep: ", ".to_string(),
            indent: self.indent,
            items: self
                .arguments
                .iter()
                .map(|argument| {
                    PairSynth::inline(
                        StringSynth(argument.name.clone() + ":"),
                        ValueTypeSynth(argument.value.clone()),
                    )
                    // todo: missing default
                })
                .collect(),
        };
        list_synth.synth(indent_lvl, multiline)
    }
}

impl From<Vec<Argument>> for ArgumentsSynth {
    fn from(arguments: Vec<Argument>) -> Self {
        ArgumentsSynth {
            indent: 0,
            arguments,
        }
    }
}

impl From<(Vec<Argument>, usize)> for ArgumentsSynth {
    fn from(arguments_indent: (Vec<Argument>, usize)) -> Self {
        ArgumentsSynth {
            indent: arguments_indent.1,
            arguments: arguments_indent.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::{ValueBasicType, ValueType, ValueTypeSimple};

    fn simple_string_arg_factory(name: &str) -> Argument {
        Argument {
            name: name.to_string(),
            description: "".to_string(),
            value: ValueType::Simple(ValueTypeSimple {
                content: ValueBasicType::String,
                nullable: true,
            }),
            default: None,
        }
    }

    #[test]
    fn test_one_argument() {
        let synth = ArgumentsSynth::from(vec![simple_string_arg_factory("arg")]);

        assert_eq!(synth.synth_zero(), "(arg: String)")
    }

    #[test]
    fn test_two_argument() {
        let synth = ArgumentsSynth::from(vec![
            simple_string_arg_factory("arg1"),
            simple_string_arg_factory("arg2"),
        ]);

        assert_eq!(synth.synth_zero(), "(arg1: String, arg2: String)")
    }

    #[test]
    fn test_one_argument_indent() {
        let synth = ArgumentsSynth::from((vec![simple_string_arg_factory("arg")], 2));

        assert_eq!(synth.synth(0, true), "(\n  arg: String\n)")
    }
}
