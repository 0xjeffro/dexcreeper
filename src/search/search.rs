use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::Instant;
use crate::graph::{dynamic_graph, static_graph};
use crate::graph::static_graph::{EdgeInfo, StaticGraph};
use crate::jupiter::quote::{quote, QuoteParams, QuoteResponse};
use crate::mints::mints;
use std::collections::VecDeque;

const JUPITER_URL: &str = "http://64.130.36.228:18080";
pub fn create_static_graph() -> StaticGraph {
    // When initializing the static graph, we need to know the number of nodes
    let mut static_graph = static_graph::StaticGraph::new(6);

    // And we also need to assign the node id (from 1 to n_node) to the mint address
    let mut token2id = HashMap::new();
    let mut id2token = HashMap::new();
    token2id.insert(mints::WSOL.mint, 1);
    id2token.insert(1, mints::WSOL.mint);
    token2id.insert(mints::USDC.mint, 2);
    id2token.insert(2, mints::USDC.mint);
    token2id.insert(mints::USDT.mint, 3);
    id2token.insert(3, mints::USDT.mint);
    token2id.insert(mints::WETH.mint, 4);
    id2token.insert(4, mints::WETH.mint);
    token2id.insert(mints::FARTCOIN.mint, 5);
    id2token.insert(5, mints::FARTCOIN.mint);
    token2id.insert(mints::POPCAT.mint, 6);
    id2token.insert(6, mints::POPCAT.mint);

    // Add edges to the graph
    // WSOL <-> USDC
    static_graph.add_edge(token2id[&mints::WSOL.mint], token2id[&mints::USDC.mint], mints::WSOL.mint.to_string(), mints::USDC.mint.to_string());
    static_graph.add_edge(token2id[&mints::USDC.mint], token2id[&mints::WSOL.mint], mints::USDC.mint.to_string(), mints::WSOL.mint.to_string());
    // WSOL <-> USDT
    static_graph.add_edge(token2id[&mints::WSOL.mint], token2id[&mints::USDT.mint], mints::WSOL.mint.to_string(), mints::USDT.mint.to_string());
    static_graph.add_edge(token2id[&mints::USDT.mint], token2id[&mints::WSOL.mint], mints::USDT.mint.to_string(), mints::WSOL.mint.to_string());
    // USDC <-> USDT
    static_graph.add_edge(token2id[&mints::USDC.mint], token2id[&mints::USDT.mint], mints::USDC.mint.to_string(), mints::USDT.mint.to_string());
    static_graph.add_edge(token2id[&mints::USDT.mint], token2id[&mints::USDC.mint], mints::USDT.mint.to_string(), mints::USDC.mint.to_string());

    // SOL <-> WETH, USDC <-> WETH, USDT <-> WETH
    static_graph.add_edge(token2id[&mints::USDC.mint], token2id[&mints::WETH.mint], mints::USDC.mint.to_string(), mints::WETH.mint.to_string());
    static_graph.add_edge(token2id[&mints::WETH.mint], token2id[&mints::USDC.mint], mints::WETH.mint.to_string(), mints::USDC.mint.to_string());
    static_graph.add_edge(token2id[&mints::USDT.mint], token2id[&mints::WETH.mint], mints::USDT.mint.to_string(), mints::WETH.mint.to_string());
    static_graph.add_edge(token2id[&mints::WETH.mint], token2id[&mints::USDT.mint], mints::WETH.mint.to_string(), mints::USDT.mint.to_string());
    static_graph.add_edge(token2id[&mints::WSOL.mint], token2id[&mints::WETH.mint], mints::WSOL.mint.to_string(), mints::WETH.mint.to_string());
    static_graph.add_edge(token2id[&mints::WETH.mint], token2id[&mints::WSOL.mint], mints::WETH.mint.to_string(), mints::WSOL.mint.to_string());

    // SOL <-> FARTCOIN, USDC <-> FARTCOIN, USDT <-> FARTCOIN
    static_graph.add_edge(token2id[&mints::USDC.mint], token2id[&mints::FARTCOIN.mint], mints::USDC.mint.to_string(), mints::FARTCOIN.mint.to_string());
    static_graph.add_edge(token2id[&mints::FARTCOIN.mint], token2id[&mints::USDC.mint], mints::FARTCOIN.mint.to_string(), mints::USDC.mint.to_string());
    static_graph.add_edge(token2id[&mints::USDT.mint], token2id[&mints::FARTCOIN.mint], mints::USDT.mint.to_string(), mints::FARTCOIN.mint.to_string());
    static_graph.add_edge(token2id[&mints::FARTCOIN.mint], token2id[&mints::USDT.mint], mints::FARTCOIN.mint.to_string(), mints::USDT.mint.to_string());
    static_graph.add_edge(token2id[&mints::WSOL.mint], token2id[&mints::FARTCOIN.mint], mints::WSOL.mint.to_string(), mints::FARTCOIN.mint.to_string());
    static_graph.add_edge(token2id[&mints::FARTCOIN.mint], token2id[&mints::WSOL.mint], mints::FARTCOIN.mint.to_string(), mints::WSOL.mint.to_string());

    // SOL <-> POPCAT, USDC <-> POPCAT, USDT <-> POPCAT
    static_graph.add_edge(token2id[&mints::USDC.mint], token2id[&mints::POPCAT.mint], mints::USDC.mint.to_string(), mints::POPCAT.mint.to_string());
    static_graph.add_edge(token2id[&mints::POPCAT.mint], token2id[&mints::USDC.mint], mints::POPCAT.mint.to_string(), mints::USDC.mint.to_string());
    static_graph.add_edge(token2id[&mints::USDT.mint], token2id[&mints::POPCAT.mint], mints::USDT.mint.to_string(), mints::POPCAT.mint.to_string());
    static_graph.add_edge(token2id[&mints::POPCAT.mint], token2id[&mints::USDT.mint], mints::POPCAT.mint.to_string(), mints::USDT.mint.to_string());
    static_graph.add_edge(token2id[&mints::WSOL.mint], token2id[&mints::POPCAT.mint], mints::WSOL.mint.to_string(), mints::POPCAT.mint.to_string());
    static_graph.add_edge(token2id[&mints::POPCAT.mint], token2id[&mints::WSOL.mint], mints::POPCAT.mint.to_string(), mints::WSOL.mint.to_string());


    static_graph
}

#[derive(Clone, Debug)]
pub struct BFSStatus {
    pub current_edge_id: usize, // current_edge_id is the id of the edge being processed
    pub visited: Vec<bool>, // visited[i] = true if node_i has been visited
    pub path: Vec<usize>, // the i-th edge in the path is path[i]
    pub path_tail: usize, // path_tail = the tail of the path Vec
    pub quote_response_map: HashMap<String, QuoteResponse>, // quote_response_map[mint1_mint2] = the quote response of edge mint1 -> mint2
    pub first_rsp_time: Option<Instant> // the first response time of this staus
}


impl BFSStatus {
    pub fn new(n_node: usize, max_path_len: usize) -> Self {
        Self {
            current_edge_id: 0,
            visited: vec![false; n_node + 1],
            path: vec![0; max_path_len + 1],
            path_tail: 0,
            quote_response_map: HashMap::new(),
            first_rsp_time: None,
        }
    }
}


pub async fn search(graph: StaticGraph, start_node_id: usize, start_amount: u64, max_path_len: usize) -> Option<Vec<BFSStatus>> {
    let mut queue: VecDeque<BFSStatus> = VecDeque::new();
    let mut edge_idx = match graph.head[start_node_id] {
        Some(idx) => idx,
        None => {
            eprintln!("No edge from start node {}", start_node_id);
            return None;
        },
    };


    loop {
        let to_node = graph.to[edge_idx];
        let edge_info = &graph.edge_info[edge_idx];
        let input_mint = &edge_info.input_mint;
        let output_mint = &edge_info.output_mint;
        let mut status = BFSStatus::new(graph.head.len(), max_path_len);
        // status.visited[start_node_id] = true;
        if !status.visited[to_node] {
            status.current_edge_id = edge_idx;
            status.visited[to_node] = true;
            status.path[0] = edge_idx;
            status.path_tail += 1;
            
            
            let quote_params = QuoteParams::new(
                input_mint.clone(),
                output_mint.clone(),
                start_amount,
            );
            let quote_rsp: Option<QuoteResponse> = match quote(JUPITER_URL, quote_params).await {
                Ok(response) => {
                    Some(response)
                },
                Err(e) => {
                    eprintln!("Quote Error: {}", e);
                    None
                }
            };
            
            if let Some(quote_rsp) = quote_rsp {
                status.quote_response_map.insert(format!("{}_{}", input_mint, output_mint), quote_rsp);
                status.first_rsp_time = Some(Instant::now());
                
                if status.path_tail < max_path_len {
                    queue.push_back(status);
                } else {
                    eprintln!("Path length exceeds the maximum length");
                }
            }
        }
        
        match graph.next[edge_idx] {
            Some(idx) => edge_idx = idx,
            None => break,
        }
    }
    
    let mut opportunities = vec![];
    // check while the queue is not empty
    while let Some(status) = queue.pop_front() {
        eprintln!("queue len: {}", queue.len());
        let current_to_node_id = graph.to[status.current_edge_id];
        if current_to_node_id == start_node_id {
            opportunities.push(status);
            continue;
        }
        
        let mut edge_idx = match graph.head[current_to_node_id] {
            Some(idx) => idx,
            None => {
                continue;
            },
        };
        loop {
            let to_node = graph.to[edge_idx];
            let edge_info = &graph.edge_info[edge_idx];
            let input_mint = &edge_info.input_mint;
            let output_mint = &edge_info.output_mint;
            let mut new_status = status.clone();
            
            if !new_status.visited[to_node] {
                new_status.current_edge_id = edge_idx;
                new_status.visited[to_node] = true;
                new_status.path[new_status.path_tail] = edge_idx;
                new_status.path_tail += 1;

                let quote_params = QuoteParams::new(
                    input_mint.clone(),
                    output_mint.clone(),
                    start_amount,
                );
                //println!("Quote params: {:?}", quote_params);
                let quote_rsp: Option<QuoteResponse> = match quote(JUPITER_URL, quote_params).await {
                    Ok(response) => {
                        Some(response)
                    },
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        None
                    }
                };

                if let Some(quote_rsp) = quote_rsp {
                    new_status.quote_response_map.insert(format!("{}_{}", input_mint, output_mint), quote_rsp);
                    if new_status.path_tail < max_path_len {
                        queue.push_back(new_status);
                    } else {
                        eprintln!("Path length exceeds the maximum length");
                    }
                }
            }
            
            match graph.next[edge_idx] {
                Some(idx) => edge_idx = idx,
                None => break,
            }
        }
    }
    Some(opportunities)
}