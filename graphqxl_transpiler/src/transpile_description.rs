use crate::resolve_modified_ref::ResolvedRef;
use graphqxl_parser::{BlockDef, BlockField, OwnedSpan};
use regex::{escape, Regex};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
struct Struct;

pub(crate) trait TemplateDescription {
    fn get_description(&self) -> &str;
    fn mutate_description(&mut self, new_description: &str);
    fn owned_span(&self) -> &OwnedSpan;
}

macro_rules! impl_template_description {
    ($structure: ty) => {
        impl TemplateDescription for $structure {
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
    };
}

impl_template_description!(BlockField);
impl_template_description!(BlockDef);
impl_template_description!(ResolvedRef);

pub(crate) fn transpile_description<T: TemplateDescription>(
    with_template_description: &mut T,
    replace: &HashMap<String, String>,
    allow_missing_replacements: bool,
) -> Result<(), Box<dyn Error>> {
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
    if !allow_missing_replacements && any_template.is_match(&replaced) {
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

    struct TestString {
        description: String,
        span: OwnedSpan,
    }

    impl_template_description!(TestString);

    impl From<&str> for TestString {
        fn from(text: &str) -> Self {
            TestString {
                description: text.to_string(),
                span: OwnedSpan::default(),
            }
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
            false,
        );
        match result {
            Ok(_) => assert_eq!(string.description, "This must be replaced: Replacement"),
            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    fn test_replaces_nothing() {
        let mut string = TestString::from("This must not be replaced: ${{ T }}");
        let result = transpile_description(
            &mut string,
            &HashMap::from([("I".to_string(), "Ignored".to_string())]),
            false,
        );
        match result {
            Ok(_) => panic!("should have failed"),
            Err(err) => {
                assert!(format!("{}", err)
                    .ends_with(":0 Not all the template variables where resolved"),)
            }
        }
    }
}
