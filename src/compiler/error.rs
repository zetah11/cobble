use crate::compiler::source::Span;

#[derive(Debug)]
pub struct Error {
    pub at: Span,
    pub message: String,
    pub notes: Vec<String>,
}

impl Error {
    pub fn new(at: Span, message: impl Into<String>) -> Self {
        Self {
            at,
            message: message.into(),
            notes: Vec::new(),
        }
    }

    pub fn with_notes(self, notes: Vec<impl Into<String>>) -> Self {
        let notes = notes.into_iter().map(Into::into).collect();
        Self { notes, ..self }
    }
}

#[derive(Debug, Default)]
pub struct Errors {
    pub errors: Vec<Error>,
}

impl Errors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub(super) fn at(&mut self, span: Span) -> ErrorAdder {
        ErrorAdder { errors: self, span }
    }
}

pub(super) struct ErrorAdder<'a> {
    errors: &'a mut Errors,
    span: Span,
}

impl ErrorAdder<'_> {
    pub fn expected_statement(&mut self) {
        let notes = vec!["statements include return pipelines and assignments"];

        self.add(Error::new(self.span, "expected a statement").with_notes(notes));
    }

    pub fn unclosed_paren(&mut self) {
        self.add(Error::new(self.span, "unclosed group"));
    }

    fn add(&mut self, msg: Error) {
        self.errors.errors.push(msg);
    }
}
