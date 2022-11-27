mod ast;
mod parser;

#[cfg(test)]
mod test {
    use super::parser::Parser;
    use crate::compiler::error::Errors;
    use crate::compiler::source::Sources;
    use crate::compiler::token::lex;

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
