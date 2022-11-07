use std::collections::HashMap;

pub(crate) fn transpile_description(description: &mut String, replace: &HashMap<&str, String>) {
    let mut replaced = description.to_string();
    for (key, value) in replace.iter() {
        let replacement = replaced.replace(&format!("${key}"), value);
        replaced = replacement;
    }
    *description = replaced;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replaces_if_match() {
        let mut string = "This must be replaced: $T".to_string();
        transpile_description(
            &mut string,
            &HashMap::from([
                ("T", "Replacement".to_string()),
                ("I", "Ignored".to_string()),
            ]),
        );
        assert_eq!(string, "This must be replaced: Replacement")
    }

    #[test]
    fn test_replaces_nothing() {
        let mut string = "This must not be replaced: $T".to_string();
        transpile_description(&mut string, &HashMap::from([("I", "Ignored".to_string())]));
        assert_eq!(string, "This must not be replaced: $T")
    }
}
