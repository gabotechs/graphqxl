pub(crate) fn append_if_no_empty(char: &str, string: &str) -> String {
    if string.is_empty() {
        string.to_string()
    } else {
        string.to_string() + char
    }
}
