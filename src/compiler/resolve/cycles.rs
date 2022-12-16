use std::collections::{HashMap, HashSet};

use super::refers::refers;
use crate::compiler::source::Span;
use crate::compiler::tree::Program;

type Graph<'src> = HashMap<&'src str, (Span, Vec<(&'src str, Span)>)>;

pub struct Cycles<'src> {
    index: usize,

    indicies: HashMap<&'src str, usize>,
    lowlinks: HashMap<&'src str, usize>,

    stack: Vec<(&'src str, Span)>,
    on_stack: HashSet<&'src str>,

    components: Vec<Vec<(&'src str, Span)>>,
}

impl<'src> Cycles<'src> {
    pub fn find(program: &Program<'src>) -> Vec<Vec<(&'src str, Span)>> {
        let graph = refers(program);
        let mut finder = Self {
            index: 0,
            indicies: HashMap::new(),
            lowlinks: HashMap::new(),
            stack: Vec::new(),
            on_stack: HashSet::new(),
            components: Vec::new(),
        };

        for name in graph.keys() {
            if !finder.indicies.contains_key(name) {
                let span = graph.get(name).unwrap().0;
                finder.connect(&graph, name, span);
            }
        }

        finder.components
    }

    fn connect(&mut self, graph: &Graph<'src>, name: &'src str, span: Span) {
        self.indicies.insert(name, self.index);
        self.lowlinks.insert(name, self.index);
        self.index += 1;

        self.stack.push((name, span));
        self.on_stack.insert(name);

        for child in graph.get(&name).map(|(_, v)| v).into_iter().flatten() {
            if !self.indicies.contains_key(&child.0) {
                self.connect(graph, child.0, child.1);
                let lowlink = *self
                    .lowlinks
                    .get(name)
                    .unwrap()
                    .min(self.lowlinks.get(child.0).unwrap());
                self.lowlinks.insert(name, lowlink);
            } else if self.on_stack.contains(child.0) {
                let lowlink = *self
                    .lowlinks
                    .get(&name)
                    .unwrap()
                    .min(self.indicies.get(child.0).unwrap());
                self.lowlinks.insert(name, lowlink);
            }
        }

        if self.lowlinks.get(&name) == self.indicies.get(&name) {
            let mut component = Vec::new();
            while let Some(child) = self.stack.pop() {
                component.push(child);
                self.on_stack.remove(child.0);

                if child.0 == name {
                    break;
                }
            }

            self.components.push(component);
        }
    }
}
