// This module defines a structure for representing a directed graph using ‘Chain Forward Star’
// The input should guarantee that nodes are compactly numbered from 1 to n
// Node id 0 is reserved as an empty node

pub struct GraphStructure {
    pub head: Vec<Option<usize>>, // head[i] is the index of the first edge from node i
    pub to: Vec<usize>, // to[i] is the destination node of edge i
    pub next: Vec<Option<usize>>, // next[i] is the index of the next edge from the same node as edge i
}

impl GraphStructure {
    pub fn new(n_node: usize) -> Self {
        Self {
            head: vec![None; n_node + 1], // head[0] is reserved for the empty node
            to: vec![0; 0],
            next: vec![None; 0],
        }
    }
    
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.to.push(to);
        self.next.push(self.head[from]);
        self.head[from] = Some(self.to.len() - 1);
    }
}