use crate::compiler::source::Span;

#[derive(Debug)]
pub struct Error {
    pub at: Span,
    pub message: String,
    pub notes: Vec<String>,
    pub helps: Vec<String>,
}

impl Error {
    pub fn new(at: Span, message: impl Into<String>) -> Self {
        Self {
            at,
            message: message.into(),
            notes: Vec::new(),
            helps: Vec::new(),
        }
    }

    pub fn with_notes(self, notes: Vec<impl Into<String>>) -> Self {
        let notes = notes.into_iter().map(Into::into).collect();
        Self { notes, ..self }
    }

    pub fn with_helps(self, helps: Vec<impl Into<String>>) -> Self {
        let helps = helps.into_iter().map(Into::into).collect();
        Self { helps, ..self }
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
    pub fn cycle(&mut self) {
        let notes = vec!["this name depends upon itself, and so cannot be produced"];
        self.add(Error::new(self.span, "cycle detected").with_notes(notes));
    }

    pub fn expected_definition(&mut self) {
        self.add(Error::new(self.span, "expected a definition"));
    }

    pub fn expected_name(&mut self, symbol: Option<&str>) {
        let (notes, helps) = symbol_notes(symbol);
        self.add(
            Error::new(self.span, "expected a name")
                .with_notes(notes)
                .with_helps(helps),
        );
    }

    pub fn expected_node(&mut self) {
        self.add(Error::new(
            self.span,
            "expected a name, tuple, or initializer",
        ));
    }

    pub fn expected_statement(&mut self) {
        let notes = vec!["statements include return pipelines and assignments"];

        self.add(Error::new(self.span, "expected a statement").with_notes(notes));
    }

    pub fn expected_value(&mut self) {
        self.add(Error::new(self.span, "expected a value"));
    }

    pub fn init_non_name(&mut self, symbol: Option<&str>) {
        let (mut notes, helps) = symbol_notes(symbol);

        notes.insert(0, "only primitive names can have an initializer");

        self.add(
            Error::new(self.span, "unexpected initializer")
                .with_notes(notes)
                .with_helps(helps),
        );
    }

    pub fn unclosed_paren(&mut self) {
        self.add(Error::new(self.span, "unclosed group"));
    }

    pub fn unexpected_return(&mut self) {
        self.add(Error::new(self.span, "unexpected 'return'"));
    }

    fn add(&mut self, msg: Error) {
        self.errors.errors.push(msg);
    }
}

fn symbol_notes(symbol: Option<&str>) -> (Vec<&'static str>, Vec<String>) {
    if let Some(sym) = symbol {
        let name = symbol_to_name(sym);
        let notes = vec!["this is a symbol - although it may look like a name, it has an initial uppercase letter and serves a different purpose"];
        let helps = vec![format!("try '{name}' instead")];

        (notes, helps)
    } else {
        (vec![], vec![])
    }
}

fn symbol_to_name(sym: &str) -> String {
    let mut chars = sym.chars();
    let initial = chars.next().unwrap().to_lowercase();
    initial.chain(chars).collect()
}
