use axum::{routing::get, Router};

pub async fn start() {
    let app = Router::new().route("/health_check", get(|| async { "GOOD" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
