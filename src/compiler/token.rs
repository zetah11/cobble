use logos::Logos;

use crate::compiler::source::{SourceId, Sources, Span};

pub fn lex(sources: &Sources, id: SourceId) -> impl Iterator<Item = (Token, Span)> + '_ {
    let source = sources.get(&id);
    Token::lexer(&source.content)
        .spanned()
        .map(move |(token, range)| (token, Span::new(range, id)))
}

#[derive(Logos, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Token<'src> {
    #[regex(r"[a-z][a-zA-Z0-9_']*", |lex| lex.slice())]
    Ident(&'src str),

    #[regex(r"[A-Z][a-zA-Z0-9_']*", |lex| lex.slice())]
    Symbol(&'src str),

    #[regex(r"[+\-]?[0-9][0-9_']*(\.[0-9][0-9_']*)?([eE][+\-]?[0-9][0-9_']*)?", |lex| lex.slice())]
    Number(&'src str),

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token(",")]
    Comma,

    #[token("=")]
    Equal,

    #[token("->")]
    Pipe,

    #[regex(r"\s+", logos::skip)]
    #[regex(r"--[^\n]*", logos::skip)]
    #[error]
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::{lex, Token};
    use crate::compiler::source::{Source, Sources};

    fn test_tokens(expected: &[Token], source: impl Into<Source>) {
        let mut sources = Sources::new();
        let id = sources.add(source.into());

        let actual: Vec<_> = lex(&sources, id).map(|(token, _)| token).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_basic() {
        let source = "a aASH912_''1'231 Sy_'1 123 12.3 12e3 1.2e+3 -123e-123";
        let expected = &[
            Token::Ident("a"),
            Token::Ident("aASH912_''1'231"),
            Token::Symbol("Sy_'1"),
            Token::Number("123"),
            Token::Number("12.3"),
            Token::Number("12e3"),
            Token::Number("1.2e+3"),
            Token::Number("-123e-123"),
        ];

        test_tokens(expected, source);
    }

    #[test]
    fn lex_other() {
        let source = "({)},=->ø";
        let expected = &[
            Token::OpenParen,
            Token::OpenBrace,
            Token::CloseParen,
            Token::CloseBrace,
            Token::Comma,
            Token::Equal,
            Token::Pipe,
            Token::Invalid,
        ];

        test_tokens(expected, source);
    }

    #[test]
    fn lex_comment() {
        let source = "a = voronoi\n-- hehe\na -> return";
        let expected = &[
            Token::Ident("a"),
            Token::Equal,
            Token::Ident("voronoi"),
            Token::Ident("a"),
            Token::Pipe,
            Token::Ident("return"),
        ];

        test_tokens(expected, source);
    }
}
