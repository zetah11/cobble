use crate::compiler::source::Span;

pub type Spanned<T> = (T, Span);

#[derive(Debug)]
pub enum Ast<'src> {
    Assign(Box<Spanned<Ast<'src>>>, Box<Spanned<Ast<'src>>>),

    Pipe(Vec<Spanned<Ast<'src>>>),
    Tuple(Vec<Spanned<Ast<'src>>>),

    Init(Box<Spanned<Ast<'src>>>, Vec<Spanned<Ast<'src>>>),

    Name(&'src str),
    Number(&'src str),
    Symbol(&'src str),
}
