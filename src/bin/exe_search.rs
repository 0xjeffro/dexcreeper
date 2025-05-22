use dexcreeper::search::search;

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let graph = search::create_static_graph();
    let results = search::search(graph, 1, 1000000000, 4).await;
    let end = start.elapsed();
    println!("{:?}", end);
}