use std::collections::HashMap;

pub(crate) fn transpile_description(description: &str, replace: &HashMap<&str, String>) -> String {
    let mut replaced = description.to_string();
    for (key, value) in replace.iter() {
        let replacement = replaced.replace(&format!("${key}"), value);
        replaced = replacement;
    }
    replaced
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replaces_if_match() {
        assert_eq!(
            transpile_description(
                "This must be replaced: $T",
                &HashMap::from([
                    ("T", "Replacement".to_string()),
                    ("I", "Ignored".to_string())
                ])
            ),
            "This must be replaced: Replacement"
        )
    }

    #[test]
    fn test_replaces_nothing() {
        assert_eq!(
            transpile_description(
                "This must not be replaced: $T",
                &HashMap::from([("I", "Ignored".to_string())])
            ),
            "This must not be replaced: $T"
        )
    }
}
