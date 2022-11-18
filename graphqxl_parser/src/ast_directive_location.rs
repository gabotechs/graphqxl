use crate::parser::{Rule, RuleError};
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub enum DirectiveLocation {
    Query,
    Mutation,
    Subscription,
    FieldDefinition,
    Field,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,
    Schema,
    Scalar,
    Object,
    ArgumentDefinition,
    Interface,
    Union,
    EnumValue,
    Enum,
    InputObject,
    InputFieldDefinition,
    VariableDefinition,
}

pub(crate) fn parse_directive_location(
    pair: Pair<Rule>,
    _file: &str,
) -> Result<DirectiveLocation, Box<RuleError>> {
    match pair.as_rule() {
        Rule::directive_location => {
            let location = pair.as_str();
            match location {
                "QUERY" => Ok(DirectiveLocation::Query),
                "MUTATION" => Ok(DirectiveLocation::Mutation),
                "SUBSCRIPTION" => Ok(DirectiveLocation::Subscription),
                "FIELD_DEFINITION" => Ok(DirectiveLocation::FieldDefinition),
                "FIELD" => Ok(DirectiveLocation::Field),
                "FRAGMENT_DEFINITION" => Ok(DirectiveLocation::FragmentDefinition),
                "FRAGMENT_SPREAD" => Ok(DirectiveLocation::FragmentSpread),
                "INLINE_FRAGMENT" => Ok(DirectiveLocation::InlineFragment),
                "SCHEMA" => Ok(DirectiveLocation::Schema),
                "SCALAR" => Ok(DirectiveLocation::Scalar),
                "OBJECT" => Ok(DirectiveLocation::Object),
                "ARGUMENT_DEFINITION" => Ok(DirectiveLocation::ArgumentDefinition),
                "INTERFACE" => Ok(DirectiveLocation::Interface),
                "UNION" => Ok(DirectiveLocation::Union),
                "ENUM_VALUE" => Ok(DirectiveLocation::EnumValue),
                "ENUM" => Ok(DirectiveLocation::Enum),
                "INPUT_OBJECT" => Ok(DirectiveLocation::InputObject),
                "INPUT_FIELD_DEFINITION" => Ok(DirectiveLocation::InputFieldDefinition),
                "VARIABLE_DEFINITION" => Ok(DirectiveLocation::VariableDefinition),
                _ => Err(Box::new(pest::error::Error::new_from_span(
                    pest::error::ErrorVariant::CustomError {
                        message: "unknown directive location ".to_string() + location,
                    },
                    pair.as_span(),
                ))),
            }
        }
        _unknown => Err(unknown_rule_error(pair, "directive_location")),
    }
}
