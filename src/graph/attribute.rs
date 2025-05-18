// The dynamic attribute layer of the graph

use std::future;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use tokio::task::JoinSet;
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
    pub last_updated: Instant, // last update time in milliseconds
}

impl EdgeAttribute {
    pub fn new() -> Self {
        Self {
            quote_response: None,
            // For convenience of initialization, set a smaller time, 30 minutes ago
            // So that when update_edge_attr is called for the first time, all edges will be updated
            last_updated: Instant::now() - std::time::Duration::from_secs(1800),
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

    pub async fn update_edge_attr<F, Fut>(&mut self, min_millis: u128, max_concurrency: usize, update_fn: F)
        -> Vec<Result<(), Box<dyn std::error::Error + Send>>>
    where
        F: Fn(usize, Arc<RwLock<EdgeAttribute>>) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Result<QuoteResponse, Box<dyn std::error::Error + Send>>> + Send + 'static,
    {
        // for each edge, check whether the current time > the last update time + min_millis
        let mut edges_to_update: Vec<(usize, Instant)> = Vec::with_capacity(self.attr.len());
        for (i, attr) in self.attr.iter().enumerate() {
            if let Ok(attr_guard) = attr.read() {
                let last_update = attr_guard.last_updated;
                if last_update.elapsed().as_millis() > min_millis {
                    edges_to_update.push((i, last_update));
                }
            }
        }
        edges_to_update.sort_by(|a, b| a.1.cmp(&b.1));

        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrency));
        let mut join_set = JoinSet::new();
        for (edge_idx, last_update) in edges_to_update {
            let attr = self.attr[edge_idx].clone();
            let semaphore = semaphore.clone();
            let update_fn = update_fn.clone();
            
            // spawn a task to update the edge attribute
            join_set.spawn(async move {
                // acquire a permit from the semaphore
                let _permit = semaphore.clone().acquire_owned().await.unwrap();
                
                // call the update function
                match update_fn(edge_idx, attr.clone()).await {
                    Ok(quote_response) => {
                        // update the last update time
                        if let Ok(mut guard) = attr.write() {
                            guard.quote_response = Some(quote_response);
                            guard.last_updated = Instant::now();
                        }
                        Ok(())
                    },
                    Err(e) => {
                        eprintln!("Error updating edge {}: {}", edge_idx, e);
                        Err(e)
                    }
                }
            });
        }

        // collect the result
        let mut results = Vec::new();
        while let Some(join_result) = join_set.join_next().await {
            match join_result {
                Ok(task_result) => results.push(task_result),
                Err(e) => eprintln!("Task panicked: {}", e)
            }
        }
        let success_count = results.iter().filter(|r| r.is_ok()).count();
        println!("Updated {}/{} edges successfully", success_count, results.len());
        results
    }
}