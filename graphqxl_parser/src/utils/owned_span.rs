use crate::Rule;
use pest::Span;

#[derive(Clone, Debug)]
pub struct OwnedSpan {
    pub err_placeholder: pest::error::Error<Rule>,
    pub input: String,
    pub start: usize,
    pub end: usize,
}

impl Default for OwnedSpan {
    fn default() -> Self {
        let err = pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: "".to_string(),
            },
            Span::new("", 0, 0).unwrap(),
        );
        Self {
            err_placeholder: err,
            input: "".to_string(),
            start: 0,
            end: 0,
        }
    }
}

// FIXME: this implementation is only for the tests, It should be behind a #[cfg(Test)],
//  but then I don't know how to make the real implementation available only for no test
impl PartialEq for OwnedSpan {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<'a> From<Span<'a>> for OwnedSpan {
    fn from(span: Span<'a>) -> Self {
        Self {
            err_placeholder: pest::error::Error::new_from_span(
                pest::error::ErrorVariant::CustomError {
                    message: "".to_string(),
                },
                span,
            ),
            input: span.as_str().to_string(),
            start: span.start(),
            end: span.end(),
        }
    }
}

impl<'a> From<&'a OwnedSpan> for Span<'a> {
    fn from(span: &'a OwnedSpan) -> Self {
        Span::new(span.input.as_str(), span.start, span.end).unwrap()
    }
}
