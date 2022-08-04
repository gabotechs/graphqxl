use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::Pair;

pub(crate) fn parse_description(pair: Pair<Rule>) -> Result<String, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::description => {
            let description = pair.into_inner().next().unwrap().as_str().to_string();
            Ok(description.trim_end().to_string())
        }
        _unknown => Err(unknown_rule_error(pair, "description")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<String, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::description, parse_description)
    }

    #[test]
    fn test_parses_description() {
        let description = parse_input("\"\"\" This is a description 123 \"\"\"").unwrap();
        assert_eq!(description, "This is a description 123");
    }

    #[test]
    fn test_invalid_input_one_double_quote_less_at_the_start() {
        parse_input("\"\" This is a description \"\"\"").unwrap_err();
    }

    #[test]
    fn test_invalid_input_one_double_quote_less_at_the_end() {
        parse_input("\"\"\" This is a description \"\"").unwrap_err();
    }
}
