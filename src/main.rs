use tokio;

mod http;
mod handler;
mod executor;

#[tokio::main]
async fn main() {
    http::http_handler().await;
}