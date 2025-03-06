use axum::{Router, routing::get};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    //load the .env file
    dotenv().ok();
    let backend_addr = "127.0.0.1:8001";

    let app = Router::new().route("/", get(|| async { "Hellow, World!" }));

    let listener = tokio::net::TcpListener::bind(&backend_addr).await.unwrap();
    println!("Listening on {}", &backend_addr);

    axum::serve(listener, app).await.unwrap();
}
