mod server;
mod workflow;

#[tokio::main]
async fn main() {
    server::start().await;
}
