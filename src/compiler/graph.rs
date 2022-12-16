use std::collections::{HashMap, HashSet};

use bimap::BiMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NodeId(usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Node<'name>(pub &'name str);

#[derive(Debug, Default)]
pub struct Graph<'name> {
    nodes: BiMap<NodeId, Node<'name>>,
    incoming: HashMap<NodeId, HashSet<NodeId>>,
    outgoing: HashMap<NodeId, HashSet<NodeId>>,

    count: usize,
}

impl<'name> Graph<'name> {
    pub fn new() -> Self {
        Self {
            nodes: BiMap::new(),
            incoming: HashMap::new(),
            outgoing: HashMap::new(),

            count: 0,
        }
    }

    pub fn add_node(&mut self, node: Node<'name>) -> NodeId {
        if let Some(id) = self.nodes.get_by_right(&node) {
            *id
        } else {
            let id = self.reserve();
            self.nodes.insert(id, node);
            id
        }
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId) {
        self.incoming.entry(to).or_default().insert(from);
        self.outgoing.entry(from).or_default().insert(to);
    }

    pub fn add_reserved(&mut self, id: NodeId, node: Node<'name>) {
        assert!(self.nodes.insert(id, node).did_overwrite());
    }

    pub fn reserve(&mut self) -> NodeId {
        let id = NodeId(self.count);
        self.count += 1;
        id
    }
}
