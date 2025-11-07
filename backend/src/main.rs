use axum::{Router, response::Html, routing::get};
use std::fs;
use tokio::net::TcpListener;

pub mod model;

#[tokio::main]
async fn main() {
    // Define a route "/" that always returns your index.html content
    let app = Router::new().route("/", get(index_html));

    let listener = TcpListener::bind("0.0.0.0:3030").await.unwrap();
    println!("Serving HTML on http://0.0.0.0:3030");

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}

// Handler that returns the HTML content
async fn index_html() -> Html<String> {
    // Read your HTML file from disk
    let html = fs::read_to_string("../frontend/index.html")
        .unwrap_or_else(|_| "<h1>Failed to load index.html</h1>".to_string());
    Html(html)
}
