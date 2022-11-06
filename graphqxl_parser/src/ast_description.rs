use crate::parser::Rule;
use crate::utils::unknown_rule_error;
use pest::iterators::{Pair, Pairs};

pub(crate) fn parse_description(
    pair: Pair<Rule>,
    _file: &str,
) -> Result<String, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::description => {
            let mut result = "".to_string();
            let trimmed = pair.as_str().trim_matches('\"');
            for line in trimmed.split('\n') {
                result += line.trim();
                result += "\n";
            }
            Ok(result.trim().to_string())
        }
        _unknown => Err(unknown_rule_error(pair, "description")),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DescriptionAndNext<'a>(pub(crate) String, pub(crate) Pair<'a, Rule>);

pub(crate) fn parse_description_and_continue<'a>(
    pairs: &mut Pairs<'a, Rule>,
    file: &str,
) -> DescriptionAndNext<'a> {
    let mut pair = pairs.next().unwrap();
    let mut description = "".to_string();
    if let Rule::description = pair.as_rule() {
        description = parse_description(pair, file).unwrap();
        pair = pairs.next().unwrap();
    }
    DescriptionAndNext(description, pair)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<String, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::description, parse_description)
    }

    #[test]
    fn test_parses_one_line_description() {
        let description = parse_input("\"This is a \\ndescription 123 \"").unwrap();
        assert_eq!(description, "This is a \\ndescription 123");
    }

    #[test]
    fn test_invalid_one_line_input_line_jump_beginning() {
        parse_input("\" \nThis is a description 123 \"").unwrap_err();
    }

    #[test]
    fn test_invalid_one_line_input_line_jump_middle() {
        parse_input("\" This is a \ndescription 123 \"").unwrap_err();
    }

    #[test]
    fn test_parses_multiline_description() {
        let description = parse_input("\"\"\" This is a \ndescription 123 \"\"\"").unwrap();
        assert_eq!(description, "This is a\ndescription 123");
    }

    #[test]
    fn test_multiline_description_accepts_double_quote() {
        let description = parse_input("\"\"\" This is a \"description 123 \"\"\"").unwrap();
        assert_eq!(description, "This is a \"description 123");
    }

    #[test]
    fn test_parses_multiline_description_trimming_indent_spaces() {
        let description = parse_input("\"\"\" This is a \n    description 123 \"\"\"").unwrap();
        assert_eq!(description, "This is a\ndescription 123");
    }

    #[test]
    fn test_invalid_multiline_input_one_double_quote_less_at_the_start() {
        parse_input("\"\" This is a description \"\"\"").unwrap_err();
    }

    #[test]
    fn test_invalid_multiline_input_one_double_quote_less_at_the_end() {
        parse_input("\"\"\" This is a description \"\"").unwrap_err();
    }
}
