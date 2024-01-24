use crate::synth_directive::DirectiveSynth;
use crate::synth_identifier::IdentifierSynth;
use crate::synth_value_data::ValueDataSynth;
use crate::synth_value_type::ValueTypeSynth;
use crate::synths::{
    ChainSynth, MultilineListSynth, OneLineListSynth, StringSynth, Synth, SynthContext,
};
use graphqxl_parser::{Argument, ArgumentDefaultValue};

pub(crate) struct ArgumentsSynth(pub(crate) Vec<Argument>);

impl Synth for ArgumentsSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        let inner_synths = self
            .0
            .iter()
            .map(|argument| {
                let mut v: Vec<Box<dyn Synth>> = vec![
                    Box::new(StringSynth(argument.name.id.clone() + ": ")),
                    Box::new(ValueTypeSynth(argument.value_type.clone())),
                ];
                if let ArgumentDefaultValue::ValueData(default) = &argument.default {
                    v.push(Box::new(StringSynth::from(" = ")));
                    v.push(Box::new(ValueDataSynth(default.clone())));
                } else if let ArgumentDefaultValue::Identifier(default) = &argument.default {
                    v.push(Box::new(StringSynth::from(" = ")));
                    v.push(Box::new(IdentifierSynth(default.clone())));
                }

                for directive in argument.directives.iter() {
                    v.push(Box::new(StringSynth::from(" ")));
                    v.push(Box::new(DirectiveSynth(directive.clone())));
                }
                ChainSynth(v)
            })
            .collect();

        if self.0.len() > context.config.max_one_line_args {
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
    use graphqxl_parser::{Directive, ValueData};

    #[test]
    fn test_one_argument() {
        let synth = ArgumentsSynth(vec![Argument::string("arg")]);
        assert_eq!(synth.synth_zero(), "(arg: String)")
    }

    #[test]
    fn test_two_argument() {
        let synth = ArgumentsSynth(vec![Argument::string("arg1"), Argument::string("arg2")]);
        assert_eq!(synth.synth_zero(), "(arg1: String, arg2: String)")
    }

    #[test]
    fn test_two_arguments_indent() {
        let synth = ArgumentsSynth(vec![Argument::string("arg"), Argument::string("arg2")]);
        let mut context = SynthContext::default();
        context.with_config(SynthConfig::default().max_one_line_args(1));
        synth.synth(&mut context);
        assert_eq!(context.result, "(\n  arg: String\n  arg2: String\n)")
    }

    #[test]
    fn test_with_default_value() {
        let synth =
            ArgumentsSynth(vec![Argument::int("arg")
                .default(ArgumentDefaultValue::ValueData(ValueData::int(1).list()))]);
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
            .default(ArgumentDefaultValue::ValueData(ValueData::int(1).list()))
            .directive(Directive::build("dir"))]);
        assert_eq!(synth.synth_zero(), "(arg: Int = [ 1 ] @dir)")
    }
}
