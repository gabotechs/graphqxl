pub(crate) fn escape_non_escaped_quotes(str: &str) -> String {
    str.replace("\\\"", "\"").replace('\"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escapes_quotes() {
        assert_eq!(escape_non_escaped_quotes("\""), "\\\"")
    }

    #[test]
    fn test_not_escapes_already_escaped_quote() {
        assert_eq!(escape_non_escaped_quotes("\\\""), "\\\"")
    }
}
