use dexcreeper::graph::schedule_update;

#[tokio::main]
async fn main() {
    schedule_update::schedule_update().await;
}
