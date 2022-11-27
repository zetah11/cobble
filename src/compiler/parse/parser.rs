use super::ast::{Ast, Spanned};
use crate::compiler::error::Errors;
use crate::compiler::token::Token;

#[derive(Debug)]
pub struct Parser<'src, 'a, I> {
    tokens: I,
    errors: &'a mut Errors,

    curr: Option<Spanned<Token<'src>>>,
    prev: Option<Spanned<Token<'src>>>,
}

impl<'src, 'a, I> Parser<'src, 'a, I>
where
    I: Iterator<Item = Spanned<Token<'src>>>,
{
    pub fn new(mut it: I, errors: &'a mut Errors) -> Self {
        let curr = it.next();
        Self {
            tokens: it,
            errors,

            curr,
            prev: None,
        }
    }

    pub fn parse(&mut self) -> Vec<Spanned<Ast<'src>>> {
        self.program()
    }

    fn is_done(&self) -> bool {
        self.curr.is_none()
    }

    fn advance(&mut self) {
        self.prev = self.curr.take();
        self.curr = self.tokens.next();
    }

    fn peek(&self, matcher: impl Matcher) -> bool {
        let Some((token, _)) = self.curr.as_ref() else { return false; };
        matcher.matches(token)
    }

    fn consume(&mut self, matcher: impl Matcher) -> Option<&Spanned<Token<'src>>> {
        if self.peek(matcher) {
            self.advance();
            self.prev.as_ref()
        } else {
            None
        }
    }

    /// ```abnf
    /// program = *statement
    /// ```
    fn program(&mut self) -> Vec<Spanned<Ast<'src>>> {
        let mut res = Vec::new();

        while !self.is_done() {
            if let Some((_, span)) =
                self.consume(Not([Token::Ident(""), Token::OpenParen].as_slice()))
            {
                let span = *span;
                self.errors.at(span).expected_statement();
                continue;
            }

            res.push(self.statement());
        }

        res
    }

    /// ```abnf
    /// statement = [IDENT "="] pipeline
    /// ```
    fn statement(&mut self) -> Spanned<Ast<'src>> {
        let target = self.pipeline();

        if matches!(target.0, Ast::Name(_)) && self.consume(Token::Equal).is_some() {
            let def = self.pipeline();

            let span = target.1 + def.1;
            (Ast::Assign(Box::new(target), Box::new(def)), span)
        } else {
            target
        }
    }

    /// ```abnf
    /// pipeline = [pipeline "->"] base-expr
    /// ```
    fn pipeline(&mut self) -> Spanned<Ast<'src>> {
        let mut expr = self.base_expr();

        while self.consume(Token::Pipe).is_some() {
            let next = self.base_expr();

            expr = match expr {
                (Ast::Pipe(mut nodes), span) => {
                    let span = span + next.1;
                    nodes.push(next);
                    (Ast::Pipe(nodes), span)
                }

                (ast, span) => {
                    let new_span = span + next.1;
                    (Ast::Pipe(vec![(ast, span), next]), new_span)
                }
            }
        }

        expr
    }

    /// ```abnf
    /// base-expr  = IDENT [args]
    /// base-expr =/ NUMBER / SYMBOL
    /// base-expr =/ args
    /// ```
    fn base_expr(&mut self) -> Spanned<Ast<'src>> {
        if let Some(tok) =
            self.consume([Token::Ident(""), Token::Number(""), Token::Symbol("")].as_slice())
        {
            let name = match tok {
                (Token::Ident(name), span) => (Ast::Name(name), *span),
                (Token::Number(num), span) => return (Ast::Number(num), *span),
                (Token::Symbol(sym), span) => return (Ast::Symbol(sym), *span),
                _ => unreachable!(),
            };

            let args = self.args();
            if args.is_empty() {
                name
            } else {
                let span = args
                    .iter()
                    .map(|(_, span)| *span)
                    .reduce(|a, b| a + b)
                    .unwrap();

                (Ast::Init(Box::new(name), args), span)
            }
        } else {
            let args = self.args();
            let span = args
                .iter()
                .map(|(_, span)| *span)
                .reduce(|a, b| a + b)
                .or_else(|| self.prev.as_ref().map(|(_, span)| *span))
                .or_else(|| self.curr.as_ref().map(|(_, span)| *span))
                .unwrap();

            (Ast::Tuple(args), span)
        }
    }

    /// ```abnf
    /// args = "(" [statement *("," statement) [","]] ")"
    /// ```
    fn args(&mut self) -> Vec<Spanned<Ast<'src>>> {
        if let Some((_, opener)) = self.consume(Token::OpenParen) {
            let opener = *opener;
            let mut res = Vec::new();
            let mut closed = false;

            while !self.is_done() {
                if self.consume(Token::CloseParen).is_some() {
                    closed = true;
                    break;
                }

                res.push(self.statement());

                let _ = self.consume(Token::Comma);
            }

            if !closed {
                self.errors.at(opener).unclosed_paren();
            }

            res
        } else {
            vec![]
        }
    }
}

trait Matcher {
    fn matches(&self, token: &Token) -> bool;
}

impl Matcher for Token<'_> {
    fn matches(&self, token: &Token) -> bool {
        match (self, token) {
            (Token::Ident(..), Token::Ident(..)) => true,
            (Token::Number(..), Token::Number(..)) => true,
            (Token::Symbol(..), Token::Symbol(..)) => true,
            (t, u) if t == u => true,
            _ => false,
        }
    }
}

impl Matcher for &[Token<'_>] {
    fn matches(&self, token: &Token) -> bool {
        self.iter().any(|tok| tok.matches(token))
    }
}

struct Not<T>(T);

impl<T: Matcher> Matcher for Not<T> {
    fn matches(&self, token: &Token) -> bool {
        !self.0.matches(token)
    }
}
