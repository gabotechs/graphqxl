use crate::Rule;
use pest::Span;

#[derive(Clone, Debug)]
pub struct OwnedSpan {
    pub err_placeholder: pest::error::Error<Rule>,
    pub file: String,
    pub line: usize,
    pub col: usize,
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
            file: "".to_string(),
            line: 0,
            col: 0,
            start: 0,
            end: 0,
        }
    }
}

impl OwnedSpan {
    pub fn make_error(&self, msg: &str) -> pest::error::Error<Rule> {
        let mut err = self.err_placeholder.clone();
        err.variant = pest::error::ErrorVariant::CustomError {
            message: msg.to_string(),
        };
        err
    }
}

// FIXME: this implementation is only for the tests, It should be behind a #[cfg(Test)],
//  but then I don't know how to make the real implementation available only for no test
impl PartialEq for OwnedSpan {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<'a> OwnedSpan {
    pub fn from(span: Span<'a>, file: &str) -> Self {
        let (line, col) = span.start_pos().line_col();
        Self {
            err_placeholder: pest::error::Error::new_from_span(
                pest::error::ErrorVariant::CustomError {
                    message: "".to_string(),
                },
                span,
            ),
            file: file.to_string(),
            line,
            col,
            input: span.as_str().to_string(),
            start: span.start(),
            end: span.end(),
        }
    }
}
