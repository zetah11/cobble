mod ast;
mod parser;
mod transform;

use super::Errors;
use crate::compiler::source::Span;
use crate::compiler::token::Token;
use crate::compiler::tree::Program;

pub fn parse<'src, I>(tokens: I, errors: &mut Errors) -> Program<'src>
where
    I: Iterator<Item = (Token<'src>, Span)>,
{
    let mut parser = parser::Parser::new(tokens, errors);
    let program = parser.parse();

    let mut transformer = transform::Transformer::new(errors);
    transformer.make_program(program)
}

#[cfg(test)]
mod test {
    use super::parser::Parser;
    use crate::compiler::source::Sources;
    use crate::compiler::token::lex;
    use crate::compiler::Errors;

    #[test]
    fn parser_halts_on_success() {
        let mut sources = Sources::new();
        let source = sources.add("a = voronoi a -> return".into());

        let tokens = lex(&sources, source);
        let mut errors = Errors::new();
        let mut parser = Parser::new(tokens, &mut errors);
        let _ = parser.parse();

        assert!(errors.is_empty());
    }

    #[test]
    fn parser_halts_on_complex_success() {
        let mut sources = Sources::new();
        let source = sources.add("a = (voronoi, simplex) -> math(Multiply) a -> return".into());

        let tokens = lex(&sources, source);
        let mut errors = Errors::new();
        let mut parser = Parser::new(tokens, &mut errors);
        let _ = parser.parse();

        assert!(errors.is_empty());
    }

    #[test]
    fn parser_halts_on_failure() {
        let mut sources = Sources::new();
        let source = sources.add("asjfas w309ru 2r3kj ".into());

        let tokens = lex(&sources, source);
        let mut errors = Errors::new();
        let mut parser = Parser::new(tokens, &mut errors);
        let _ = parser.parse();

        assert!(!errors.is_empty());
    }

    #[test]
    fn parser_halts_on_group_failure() {
        let mut sources = Sources::new();
        let source = sources.add("(bwuf8soil A1".into());

        let tokens = lex(&sources, source);
        let mut errors = Errors::new();
        let mut parser = Parser::new(tokens, &mut errors);
        let _ = parser.parse();

        assert!(!errors.is_empty());
    }
}
