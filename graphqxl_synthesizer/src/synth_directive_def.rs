use crate::synth_arguments::ArgumentsSynth;
use crate::synth_description::DescriptionSynth;
use crate::synth_identifier::IdentifierSynth;
use crate::synths::{
    ChainSynth, MultilineListSynth, OneLineListSynth, PairSynth, StringSynth, Synth, SynthContext,
};
use graphqxl_parser::{DirectiveDef, DirectiveLocation};

pub(crate) struct DirectiveDefSynth(pub(crate) DirectiveDef);

fn print_directive_location(directive_location: &DirectiveLocation) -> String {
    let string = match directive_location {
        DirectiveLocation::Query => "QUERY",
        DirectiveLocation::Mutation => "MUTATION",
        DirectiveLocation::Subscription => "SUBSCRIPTION",
        DirectiveLocation::FieldDefinition => "FIELD_DEFINITION",
        DirectiveLocation::Field => "FIELD",
        DirectiveLocation::FragmentDefinition => "FRAGMENT_DEFINITION",
        DirectiveLocation::FragmentSpread => "FRAGMENT_SPREAD",
        DirectiveLocation::InlineFragment => "INLINE_FRAGMENT",
        DirectiveLocation::Schema => "SCHEMA",
        DirectiveLocation::Scalar => "SCALAR",
        DirectiveLocation::Object => "OBJECT",
        DirectiveLocation::ArgumentDefinition => "ARGUMENT_DEFINITION",
        DirectiveLocation::Interface => "INTERFACE",
        DirectiveLocation::Union => "UNION",
        DirectiveLocation::EnumValue => "ENUM_VALUE",
        DirectiveLocation::Enum => "ENUM",
        DirectiveLocation::InputObject => "INPUT_OBJECT",
        DirectiveLocation::InputFieldDefinition => "INPUT_FIELD_DEFINITION",
        DirectiveLocation::VariableDefinition => "VARIABLE_DEFINITION",
    };
    string.to_string()
}

struct DirectiveLocationSynth(pub(crate) Vec<DirectiveLocation>);

impl Synth for DirectiveLocationSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        let inner_synths = self
            .0
            .iter()
            .map(|t| StringSynth(print_directive_location(t)))
            .collect();
        if self.0.len() > context.config.max_one_line_ors {
            MultilineListSynth::or_suffix(("", inner_synths, "")).synth(context);
        } else {
            OneLineListSynth::or(("", inner_synths, "")).synth(context);
        }
        true
    }
}

impl Synth for DirectiveDefSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        let synth = PairSynth::top_level(
            DescriptionSynth::text(&self.0.description.as_str()),
            ChainSynth({
                let mut v: Vec<Box<dyn Synth>> = vec![
                    Box::new(StringSynth::from("directive @")),
                    Box::new(IdentifierSynth(self.0.name.clone())),
                    Box::new(StringSynth::from(" on ")),
                    Box::new(DirectiveLocationSynth(self.0.locations.clone())),
                ];
                if self.0.is_repeatable {
                    v.insert(2, Box::new(StringSynth::from(" repeatable")));
                }
                if !self.0.arguments.is_empty() {
                    v.insert(2, Box::new(ArgumentsSynth(self.0.arguments.clone())));
                }
                v
            }),
        );
        synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SynthConfig;
    use graphqxl_parser::Argument;

    #[test]
    fn test_most_simple_directive_def() {
        let synth = DirectiveDefSynth(DirectiveDef::build("dir").location(DirectiveLocation::Enum));
        assert_eq!(synth.synth_zero(), "directive @dir on ENUM");
    }

    #[test]
    fn test_with_description() {
        let synth = DirectiveDefSynth(
            DirectiveDef::build("dir")
                .description("my description")
                .location(DirectiveLocation::Enum),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
\"my description\"
directive @dir on ENUM"
        );
    }

    #[test]
    fn test_with_description_with_args() {
        let synth = DirectiveDefSynth(
            DirectiveDef::build("dir")
                .description("my description")
                .arg(Argument::string("arg"))
                .location(DirectiveLocation::Enum),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
\"my description\"
directive @dir(arg: String) on ENUM"
        );
    }

    #[test]
    fn test_with_description_with_args_repeatable() {
        let synth = DirectiveDefSynth(
            DirectiveDef::build("dir")
                .description("my description")
                .arg(Argument::string("arg"))
                .repeatable()
                .location(DirectiveLocation::Enum),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
\"my description\"
directive @dir(arg: String) repeatable on ENUM"
        );
    }

    #[test]
    fn test_with_description_with_args_repeatable_multiple_locations() {
        let synth = DirectiveDefSynth(
            DirectiveDef::build("dir")
                .description("my description")
                .arg(Argument::string("arg"))
                .repeatable()
                .location(DirectiveLocation::Enum)
                .location(DirectiveLocation::ArgumentDefinition)
                .location(DirectiveLocation::Interface),
        );
        let mut context = SynthContext::default();
        context.with_config(SynthConfig::default().max_one_line_ors(3));
        synth.synth(&mut context);
        assert_eq!(
            context.result,
            "\
\"my description\"
directive @dir(arg: String) repeatable on ENUM | ARGUMENT_DEFINITION | INTERFACE"
        );
    }

    #[test]
    fn test_with_description_with_args_repeatable_multiple_locations_multiline() {
        let synth = DirectiveDefSynth(
            DirectiveDef::build("dir")
                .description("my description")
                .arg(Argument::string("arg"))
                .repeatable()
                .location(DirectiveLocation::Enum)
                .location(DirectiveLocation::ArgumentDefinition)
                .location(DirectiveLocation::Interface),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
\"my description\"
directive @dir(arg: String) repeatable on 
  ENUM |
  ARGUMENT_DEFINITION |
  INTERFACE"
        );
    }
}
