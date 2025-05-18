// The dynamic attribute layer of the graph

use std::sync::{Arc, RwLock};
use crate::graph::topology::GraphTopology;
use crate::jupiter::quote::QuoteResponse;

pub struct GraphWithAttr {
    pub topology: Arc<GraphTopology>,
    pub start_node: usize,
    pub start_amount: f64,
    pub attr: Vec<Arc<RwLock<EdgeAttribute>>>,
}

pub struct EdgeAttribute {
    pub quote_response: Option<QuoteResponse>,
}

impl EdgeAttribute {
    pub fn new() -> Self {
        Self {
            quote_response: None
        }
    }
}

impl GraphWithAttr {
    pub fn new(topology: Arc<GraphTopology>, start_node: usize, start_amount: f64) -> Self {
        let n_edge = topology.to.len();
        Self {
            topology,
            start_node,
            start_amount,
            attr: vec![Arc::new(RwLock::new(EdgeAttribute::new())); n_edge],
        }
    }
}

