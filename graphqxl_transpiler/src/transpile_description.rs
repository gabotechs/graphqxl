use graphqxl_parser::{BlockDef, BlockField, OwnedSpan, Rule};
use regex::{escape, Regex};
use std::collections::HashMap;

pub(crate) trait TemplateDescription {
    fn get_description(&self) -> &str;
    fn mutate_description(&mut self, new_description: &str);
    fn owned_span(&self) -> &OwnedSpan;
}

impl TemplateDescription for BlockField {
    fn get_description(&self) -> &str {
        &self.description
    }

    fn mutate_description(&mut self, new_description: &str) {
        self.description = new_description.to_string();
    }

    fn owned_span(&self) -> &OwnedSpan {
        &self.span
    }
}

impl TemplateDescription for BlockDef {
    fn get_description(&self) -> &str {
        &self.description
    }

    fn mutate_description(&mut self, new_description: &str) {
        self.description = new_description.to_string();
    }

    fn owned_span(&self) -> &OwnedSpan {
        &self.span
    }
}

pub(crate) fn transpile_description<T: TemplateDescription>(
    with_template_description: &mut T,
    replace: &HashMap<String, String>,
) -> Result<(), pest::error::Error<Rule>> {
    let any_template: Regex = Regex::new(r"\$\{\{.*}}").unwrap();

    let mut replaced = with_template_description.get_description().to_string();
    if replaced.is_empty() {
        return Ok(());
    }
    for (key, value) in replace.iter() {
        let escaped_key = escape(key);

        let re_or_err = Regex::new(&format!(r#"\$\{{\{{ *{escaped_key} *\}}\}}"#));
        let re = match re_or_err {
            Ok(re) => re,
            Err(err) => {
                return Err(with_template_description
                    .owned_span()
                    .make_error(&err.to_string()));
            }
        };
        replaced = re.replace_all(&replaced, value).to_string();
    }
    if any_template.is_match(&replaced) {
        return Err(with_template_description
            .owned_span()
            .make_error("Not all the template variables where resolved"));
    }
    with_template_description.mutate_description(&replaced);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestString(String, OwnedSpan);

    impl TemplateDescription for TestString {
        fn get_description(&self) -> &str {
            &self.0
        }

        fn mutate_description(&mut self, new_description: &str) {
            self.0 = new_description.to_string();
        }

        fn owned_span(&self) -> &OwnedSpan {
            &self.1
        }
    }

    impl From<&str> for TestString {
        fn from(text: &str) -> Self {
            TestString(text.to_string(), OwnedSpan::default())
        }
    }

    #[test]
    fn test_replaces_if_match() {
        let mut string = TestString::from("This must be replaced: ${{ T }}");
        let result = transpile_description(
            &mut string,
            &HashMap::from([
                ("T".to_string(), "Replacement".to_string()),
                ("I".to_string(), "Ignored".to_string()),
            ]),
        );
        match result {
            Ok(_) => assert_eq!(string.0, "This must be replaced: Replacement"),
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn test_replaces_nothing() {
        let mut string = TestString::from("This must not be replaced: ${{ T }}");
        let result = transpile_description(
            &mut string,
            &HashMap::from([("I".to_string(), "Ignored".to_string())]),
        );
        match result {
            Ok(_) => panic!("should have failed"),
            Err(err) => assert_eq!(
                err.variant.message(),
                ":0 Not all the template variables where resolved"
            ),
        }
    }
}
