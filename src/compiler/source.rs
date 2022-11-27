use std::collections::HashMap;
use std::ops::Range;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SourceId(usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub file: SourceId,
}

impl Span {
    pub fn new(range: Range<usize>, id: SourceId) -> Self {
        Self {
            start: range.start,
            end: range.end,
            file: id,
        }
    }
}

#[derive(Debug)]
pub struct Source {
    pub content: String,
}

impl<S: Into<String>> From<S> for Source {
    fn from(content: S) -> Self {
        Self {
            content: content.into(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Sources {
    sources: HashMap<SourceId, Source>,
    count: usize,
}

impl Sources {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            count: 0,
        }
    }

    pub fn add(&mut self, source: Source) -> SourceId {
        let id = SourceId(self.count);
        self.count += 1;
        self.sources.insert(id, source);
        id
    }

    pub fn get(&self, id: &SourceId) -> &Source {
        self.sources.get(id).unwrap()
    }
}
