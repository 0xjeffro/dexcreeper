use std::collections::HashMap;
use std::sync::Arc;
use crate::graph::{dynamic_graph, static_graph};
use crate::jupiter::quote::{quote, QuoteParams};
use crate::mints::mints;

pub fn create_graph() -> dynamic_graph::DynamicGraph {
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

    let dynamic_graph = dynamic_graph::DynamicGraph::new(Arc::from(static_graph), 1, 1000000000);

    dynamic_graph
}

pub async fn schedule_update() {
    // Create the graph
    let mut graph = create_graph();
    let interval = std::time::Duration::from_millis(10);
    let min_millis = 50;
    let max_concurrency = 50;
    let update_fn = |start_amount: u64, input_mint: String, output_mint: String| {
        async move {
            let jupiter_url = "http://64.130.36.228:18080";
            let quote_params = QuoteParams::new(
                input_mint,
                output_mint,
                start_amount,
            );

            // Use a different approach to make the error Send-compatible
            match quote(jupiter_url, quote_params).await {
                Ok(response) => Ok(response),
                Err(e) => {
                    // Convert the error to a Send-compatible error
                    let error_message = format!("Quote error: {}", e);
                    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)) as Box<dyn std::error::Error + Send>)
                }
            }
        }
    };
    loop {
        // Await the future to properly handle it
        let results = graph.update_edge_attr(min_millis, max_concurrency, update_fn).await;

        let success_count = results.iter().filter(|r| r.is_ok()).count();
        eprintln!("Edge update completed: {}/{} successful", success_count, results.len());
        tokio::time::sleep(interval).await;
    }
}