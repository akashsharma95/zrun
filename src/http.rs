
use axum::prelude::*;
use std::time::Duration;
use tower::ServiceBuilder;
use tokio;
use tower_http::compression::CompressionLayer;

use crate::handler;

pub async fn http_handler() {
    let middleware_stack = ServiceBuilder::new()
        .timeout(Duration::from_secs(30))
        .load_shed()
        .concurrency_limit(100)
        .layer(CompressionLayer::new())
        .into_inner();

    let app = route("/fn/:name", any(handler::fn_handler)).layer(middleware_stack);

    let server =
        hyper::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service());

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
