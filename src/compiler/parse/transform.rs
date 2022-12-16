use std::collections::HashMap;

use super::ast::Ast;
use crate::compiler::tree::{Node, Pipeline, Program, Spanned, Value};
use crate::compiler::Errors;

#[derive(Debug)]
pub struct Transformer<'a> {
    errors: &'a mut Errors,
}

impl<'a> Transformer<'a> {
    pub fn new(errors: &'a mut Errors) -> Self {
        Self { errors }
    }

    pub fn make_program<'src>(&mut self, ast: Vec<Spanned<Ast<'src>>>) -> Program<'src> {
        let mut defs = HashMap::new();
        let mut results = Vec::new();

        for ast in ast {
            match ast.0 {
                Ast::Assign(target, value) => {
                    let (name, name_span) = match *target {
                        (Ast::Name(name), span) => (name, span),

                        (Ast::Symbol(sym), span) => {
                            self.errors.at(span).expected_name(Some(sym));
                            continue;
                        }

                        (_, span) => {
                            self.errors.at(span).expected_name(None);
                            continue;
                        }
                    };

                    let value = self.make_pipeline(*value);

                    if name == "return" {
                        results.push(value);
                    } else {
                        defs.insert(name, (name_span, value));
                    }
                }

                _ => {
                    self.errors.at(ast.1).expected_definition();
                }
            }
        }

        Program { defs, results }
    }

    fn make_pipeline<'src>(&mut self, ast: Spanned<Ast<'src>>) -> Spanned<Pipeline<'src>> {
        let span = ast.1;
        let nodes = match ast.0 {
            Ast::Pipe(nodes) => nodes.into_iter().map(|ast| self.make_node(ast)).collect(),
            _ => vec![self.make_node(ast)],
        };

        (Pipeline { nodes }, span)
    }

    fn make_node<'src>(&mut self, ast: Spanned<Ast<'src>>) -> Spanned<Node<'src>> {
        let span = ast.1;
        let node = match ast.0 {
            Ast::Name("return") => {
                self.errors.at(span).unexpected_return();
                Node::Invalid
            }
            Ast::Name(name) => Node::Name(name),
            Ast::Init(target, args) => {
                let name = match *target {
                    (Ast::Name(name), span) => (name, span),
                    (Ast::Symbol(sym), this) => {
                        self.errors.at(this).init_non_name(Some(sym));
                        return (Node::Invalid, span);
                    }
                    (_, this) => {
                        self.errors.at(this).init_non_name(None);
                        return (Node::Invalid, span);
                    }
                };

                self.make_init(name, args)
            }

            Ast::Tuple(nodes) => {
                let pipelines = nodes
                    .into_iter()
                    .map(|ast| self.make_pipeline(ast))
                    .collect();
                Node::Tuple(pipelines)
            }

            Ast::Assign(..) | Ast::Number(..) | Ast::Pipe(..) | Ast::Symbol(..) => {
                self.errors.at(span).expected_node();
                Node::Invalid
            }
        };

        (node, span)
    }

    fn make_init<'src>(
        &mut self,
        name: Spanned<&'src str>,
        asts: Vec<Spanned<Ast<'src>>>,
    ) -> Node<'src> {
        let mut positional = Vec::new();
        let mut named = HashMap::new();

        for ast in asts {
            match ast.0 {
                Ast::Assign(target, value) => {
                    let (name, name_span) = match *target {
                        (Ast::Name(name), span) => (name, span),
                        (Ast::Symbol(sym), span) => {
                            self.errors.at(span).expected_name(Some(sym));
                            continue;
                        }
                        (_, span) => {
                            self.errors.at(span).expected_name(None);
                            continue;
                        }
                    };

                    if let Some(value) = self.make_value(*value) {
                        if named.insert(name, (name_span, value)).is_some() {
                            self.errors.at(ast.1).repeated_name_init();
                        }
                    }
                }

                _ => {
                    if let Some(value) = self.make_value(ast) {
                        positional.push(value);
                    }
                }
            }
        }

        Node::Init {
            name,
            positional,
            named,
        }
    }

    fn make_value<'src>(&mut self, ast: Spanned<Ast<'src>>) -> Option<Spanned<Value<'src>>> {
        let span = ast.1;
        let value = match ast.0 {
            Ast::Number(num) => Value::Number(num),
            Ast::Symbol(sym) => Value::Symbol(sym),

            _ => {
                self.errors.at(span).expected_value();
                return None;
            }
        };

        Some((value, span))
    }
}
