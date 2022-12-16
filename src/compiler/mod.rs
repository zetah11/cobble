pub mod graph;
pub mod parse;
pub mod resolve;
pub mod source;
pub mod token;
pub mod tree;

mod error;

pub use error::{Error, Errors};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Value<'src> {
    Number(&'src str),
    Symbol(&'src str),
}
