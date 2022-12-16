mod cycles;
mod refers;

use std::collections::{HashMap, HashSet};

use self::cycles::Cycles;
use super::graph::NodeId;
use crate::compiler::graph::{Graph, Node};
use crate::compiler::tree;
use crate::compiler::Errors;

pub fn resolve(program: tree::Program) -> Graph {
    Resolver::resolve(program)
}

struct Resolver<'src> {
    graph: Graph<'src>,

    cycles: HashSet<&'src str>,
    names: HashMap<&'src str, NodeId>,
}

impl<'src> Resolver<'src> {
    pub fn resolve(mut program: tree::Program<'src>) -> Graph<'src> {
        let cycles = Cycles::find(&program);

        let mut errors = Errors::new();
        let mut in_cycles = HashSet::new();

        let mut worklist = Vec::new();

        for mut cycle in cycles {
            if cycle.len() > 1 {
                let mut errored = false;
                for (name, span) in cycle.iter() {
                    if !errored {
                        errors.at(*span).cycle();
                        errored = true;
                    }

                    in_cycles.insert(*name);
                }
            }

            if let Some((name, _)) = cycle.pop() {
                worklist.push(name);
            }
        }

        let mut resolver = Self {
            graph: Graph::new(),
            cycles: in_cycles,
            names: HashMap::new(),
        };

        for name in worklist {
            if let Some((_, (pipeline, _))) = program.defs.remove(&name) {
                let node = resolver.resolve_pipeline(pipeline);
                resolver.names.insert(name, node);
            }
        }

        resolver.graph
    }

    fn resolve_pipeline(&mut self, pipeline: tree::Pipeline<'src>) -> NodeId {
        let mut result = None;

        for node in pipeline.nodes {
            let id = self.resolve_node(node.0);

            if let Some(prev) = result {
                self.graph.add_edge(prev, id);
            }

            result = Some(id);
        }

        result.unwrap()
    }

    fn resolve_node(&mut self, node: tree::Node<'src>) -> NodeId {
        match node {
            tree::Node::Init { .. } => todo!(),
            tree::Node::Invalid => self.graph.add_node(Node("invalid")),
            tree::Node::Name(name) if self.cycles.contains(&name) => {
                self.graph.add_node(Node("invalid"))
            }
            tree::Node::Name(name) => match self.names.get(&name) {
                Some(id) => *id,
                None => self.graph.add_node(Node(name)),
            },
            tree::Node::Tuple(_) => todo!(),
        }
    }
}
