use std::collections::HashMap;

use crate::compiler::source::Span;

pub type Spanned<T> = (T, Span);

#[derive(Debug)]
pub struct Program<'src> {
    pub defs: HashMap<&'src str, (Span, Spanned<Pipeline<'src>>)>,
    pub results: Vec<Spanned<Pipeline<'src>>>,
}

#[derive(Debug)]
pub struct Pipeline<'src> {
    pub nodes: Vec<Spanned<Node<'src>>>,
}

#[derive(Debug)]
pub enum Node<'src> {
    Name(&'src str),

    Init {
        name: Spanned<&'src str>,
        positional: Vec<Spanned<Value<'src>>>,
        named: HashMap<&'src str, (Span, Spanned<Value<'src>>)>,
    },

    Tuple(Vec<Spanned<Pipeline<'src>>>),

    Invalid,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Value<'src> {
    Number(&'src str),
    Symbol(&'src str),
}
