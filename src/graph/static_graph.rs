// This module defines a structure for representing a directed graph using ‘Chain Forward Star’
// The input should guarantee that nodes are compactly numbered from 1 to n
// Node id 0 is reserved as an empty node

pub struct StaticGraph {
    pub head: Vec<Option<usize>>, // head[i] is the index of the first edge from node i
    pub to: Vec<usize>, // to[i] is the destination node of edge i
    pub next: Vec<Option<usize>>, // next[i] is the index of the next edge from the same node as edge i
    pub edge_info: Vec<EdgeInfo>, // edge_info[i] is the information of edge i
}

pub struct EdgeInfo {
    pub input_mint: String,
    pub output_mint: String,
}

impl StaticGraph {
    pub fn new(n_node: usize) -> Self {
        Self {
            head: vec![None; n_node + 1], // head[0] is reserved for the empty node
            to: vec![0; 0],
            next: vec![None; 0],
            edge_info: vec![],
        }
    }
    
    pub fn add_edge(&mut self, from: usize, to: usize, input_mint: String, output_mint: String)
    {
        self.to.push(to);
        self.edge_info.push(EdgeInfo {
            input_mint,
            output_mint,
        });
        self.next.push(self.head[from]);
        self.head[from] = Some(self.to.len() - 1);
    }
}