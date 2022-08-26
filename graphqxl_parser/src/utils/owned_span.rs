use pest::Span;

#[derive(Clone, Default, Debug)]
pub struct OwnedSpan {
    pub input: String,
    pub start: usize,
    pub end: usize,
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
            input: span.as_str().to_string(),
            start: span.start(),
            end: span.end(),
        }
    }
}

impl From<(&str, usize, usize)> for OwnedSpan {
    fn from(spec: (&str, usize, usize)) -> Self {
        Self {
            input: spec.0.to_string(),
            start: spec.1,
            end: spec.2,
        }
    }
}

impl<'a> From<&'a OwnedSpan> for Span<'a> {
    fn from(span: &'a OwnedSpan) -> Self {
        Span::new(span.input.as_str(), span.start, span.end).unwrap()
    }
}
