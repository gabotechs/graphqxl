use crate::synth_arguments::ArgumentsSynth;
use crate::synth_description::DescriptionSynth;
use crate::synths::{ChainSynth, ListSynth, PairSynth, StringSynth, Synth, SynthContext};
use graphqxl_parser::{DirectiveDef, DirectiveLocation};

pub(crate) struct DirectiveSynth(pub(crate) DirectiveDef);

fn print_directive_location(directive_location: &DirectiveLocation) -> String {
    match directive_location {
        DirectiveLocation::Query => "QUERY".to_string(),
        DirectiveLocation::Mutation => "MUTATION".to_string(),
        DirectiveLocation::Subscription => "SUBSCRIPTION".to_string(),
        DirectiveLocation::FieldDefinition => "FIELD_DEFINITION".to_string(),
        DirectiveLocation::Field => "FIELD".to_string(),
        DirectiveLocation::FragmentDefinition => "FRAGMENT_DEFINITION".to_string(),
        DirectiveLocation::FragmentSpread => "FRAGMENT_SPREAD".to_string(),
        DirectiveLocation::InlineFragment => "INLINE_FRAGMENT".to_string(),
        DirectiveLocation::Schema => "SCHEMA".to_string(),
        DirectiveLocation::Scalar => "SCALAR".to_string(),
        DirectiveLocation::Object => "OBJECT".to_string(),
        DirectiveLocation::ArgumentDefinition => "ARGUMENT_DEFINITION".to_string(),
        DirectiveLocation::Interface => "INTERFACE".to_string(),
        DirectiveLocation::Union => "UNION".to_string(),
        DirectiveLocation::EnumValue => "ENUM_VALUE".to_string(),
        DirectiveLocation::Enum => "ENUM".to_string(),
        DirectiveLocation::InputObject => "INPUT_OBJECT".to_string(),
        DirectiveLocation::InputFieldDefinition => "INPUT_FIELD_DEFINITION".to_string(),
        DirectiveLocation::VariableDefinition => "VARIABLE_DEFINITION".to_string(),
    }
}

impl Synth for DirectiveSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let synth = PairSynth::top_level(
            DescriptionSynth::from(self.0.description.as_str()),
            ChainSynth({
                let mut v: Vec<Box<dyn Synth>> = vec![
                    Box::new(StringSynth(format!("directive @{}", self.0.name))),
                    Box::new(StringSynth::from(" on ")),
                    Box::new(ListSynth::from((
                        "",
                        self.0
                            .locations
                            .iter()
                            .map(|t| StringSynth(print_directive_location(t)))
                            .collect(),
                        " | ",
                        "",
                    ))),
                ];
                if self.0.is_repeatable {
                    v.insert(1, Box::new(StringSynth::from(" repeatable")));
                }
                if !self.0.arguments.is_empty() {
                    v.insert(1, Box::new(ArgumentsSynth(self.0.arguments.clone())));
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
    use crate::test_utils::simple_string_arg_factory;

    fn directive_factory(name: &str) -> DirectiveDef {
        DirectiveDef {
            name: name.to_string(),
            description: "".to_string(),
            is_repeatable: false,
            arguments: vec![],
            locations: vec![DirectiveLocation::Enum],
        }
    }

    #[test]
    fn test_most_simple_directive() {
        let synth = DirectiveSynth(directive_factory("dir"));
        assert_eq!(synth.synth_zero(), "directive @dir on ENUM");
    }

    #[test]
    fn test_with_description() {
        let mut synth = DirectiveSynth(directive_factory("dir"));
        synth.0.description = "my description".to_string();
        assert_eq!(
            synth.synth_zero(),
            "\
\"my description\"
directive @dir on ENUM"
        );
    }

    #[test]
    fn test_with_description_with_args() {
        let mut synth = DirectiveSynth(directive_factory("dir"));
        synth.0.description = "my description".to_string();
        synth.0.arguments = vec![simple_string_arg_factory("arg")];
        assert_eq!(
            synth.synth_zero(),
            "\
\"my description\"
directive @dir(arg: String) on ENUM"
        );
    }

    #[test]
    fn test_with_description_with_args_repeatable() {
        let mut synth = DirectiveSynth(directive_factory("dir"));
        synth.0.description = "my description".to_string();
        synth.0.arguments = vec![simple_string_arg_factory("arg")];
        synth.0.is_repeatable = true;
        assert_eq!(
            synth.synth_zero(),
            "\
\"my description\"
directive @dir(arg: String) repeatable on ENUM"
        );
    }

    #[test]
    fn test_with_description_with_args_repeatable_multiple_locations() {
        let mut synth = DirectiveSynth(directive_factory("dir"));
        synth.0.description = "my description".to_string();
        synth.0.arguments = vec![simple_string_arg_factory("arg")];
        synth.0.is_repeatable = true;
        synth
            .0
            .locations
            .push(DirectiveLocation::ArgumentDefinition);
        synth.0.locations.push(DirectiveLocation::Interface);
        assert_eq!(
            synth.synth_zero(),
            "\
\"my description\"
directive @dir(arg: String) repeatable on ENUM | ARGUMENT_DEFINITION | INTERFACE"
        );
    }

    #[test]
    fn test_with_description_with_args_repeatable_multiple_locations_multiline() {
        let mut synth = DirectiveSynth(directive_factory("dir"));
        synth.0.description = "my description".to_string();
        synth.0.arguments = vec![simple_string_arg_factory("arg")];
        synth.0.is_repeatable = true;
        synth
            .0
            .locations
            .push(DirectiveLocation::ArgumentDefinition);
        synth.0.locations.push(DirectiveLocation::Interface);
        assert_eq!(
            synth.synth(&SynthContext {
                indent_spaces: 2,
                multiline: true,
                ..Default::default()
            }),
            "\
\"my description\"
directive @dir(
  arg: String
) repeatable on 
  ENUM | 
  ARGUMENT_DEFINITION | 
  INTERFACE
"
        );
    }
}
