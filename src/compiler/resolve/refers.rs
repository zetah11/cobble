use std::collections::HashMap;

use crate::compiler::source::Span;
use crate::compiler::tree::{Node, Pipeline, Program};

type Deps<'src> = HashMap<&'src str, (Span, Vec<(&'src str, Span)>)>;

pub fn refers<'src>(program: &Program<'src>) -> Deps<'src> {
    let mut refers = Refers::new();
    refers.analyze(program);
    refers.result
}

#[derive(Debug)]
struct Refers<'src> {
    result: Deps<'src>,
}

impl<'src> Refers<'src> {
    fn new() -> Self {
        Self {
            result: HashMap::new(),
        }
    }

    fn analyze(&mut self, program: &Program<'src>) {
        for (def, (span, (pipeline, _))) in program.defs.iter() {
            let names = self.analyze_pipeline(pipeline);
            self.result.insert(def, (*span, names));
        }
    }

    fn analyze_pipeline(&self, pipeline: &Pipeline<'src>) -> Vec<(&'src str, Span)> {
        let mut res = Vec::with_capacity(pipeline.nodes.len());

        for node in pipeline.nodes.iter() {
            res.extend(self.analyze_node(node));
        }

        res.shrink_to_fit();
        res
    }

    fn analyze_node(&self, node: &(Node<'src>, Span)) -> Vec<(&'src str, Span)> {
        match &node.0 {
            Node::Invalid => vec![],
            Node::Name(name) => vec![(*name, node.1)],
            Node::Tuple(pipes) => {
                let mut res = Vec::with_capacity(pipes.len());

                for (pipe, _) in pipes.iter() {
                    res.extend(self.analyze_pipeline(pipe));
                }

                res.shrink_to_fit();
                res
            }
            Node::Init { name, .. } => {
                vec![(name.0, name.1)]
            }
        }
    }
}
