use crate::ast_identifier::parse_identifier;
use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use crate::OwnedSpan;
use pest::iterators::Pair;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DescriptionVariables {
    pub span: OwnedSpan,
    pub variables: HashMap<String, String>,
}

impl DescriptionVariables {
    pub fn build(variable: (&str, &str)) -> Self {
        Self {
            span: OwnedSpan::default(),
            variables: HashMap::from([(variable.0.to_string(), variable.1.to_string())]),
        }
    }

    pub fn variable(&mut self, variable: (&str, &str)) -> Self {
        self.variables
            .insert(variable.0.to_string(), variable.1.to_string());
        self.clone()
    }
}

pub(crate) fn parse_description_variables(
    pair: Pair<Rule>,
    file: &str,
) -> Result<DescriptionVariables, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::description_variables => {
            let span = OwnedSpan::from(pair.as_span(), file);
            let mut variables = HashMap::new();
            for child in pair.into_inner() {
                let mut grand_childs = child.into_inner();
                let first = grand_childs.next().unwrap();
                let second = grand_childs.next().unwrap();
                let identifier = parse_identifier(first, file)?;
                let str = second.as_str();
                let value = str[1..str.len() - 1].to_string(); // trim ""
                variables.insert(identifier.id, value);
            }

            Ok(DescriptionVariables { span, variables })
        }
        _unknown => Err(unknown_rule_error(pair, "description_variables")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<DescriptionVariables, pest::error::Error<Rule>> {
        parse_full_input(
            input,
            Rule::description_variables,
            parse_description_variables,
        )
    }

    #[test]
    fn test_parses_one_variable() {
        assert_eq!(
            parse_input("${ key: \"value\" }"),
            Ok(DescriptionVariables::build(("key", "value")))
        )
    }

    #[test]
    fn test_do_not_parses_invalid_identifier() {
        parse_input("${ 1key: \"value\" }").unwrap_err();
    }

    #[test]
    fn test_parses_two_values() {
        assert_eq!(
            parse_input("${ key: \"value\" foo: \"bar\" }"),
            Ok(DescriptionVariables::build(("key", "value")).variable(("foo", "bar")))
        )
    }

    #[test]
    fn test_do_not_parses_invalid_value() {
        parse_input("${ key: \"value\" foo: \"\"bar\" }").unwrap_err();
    }
}
