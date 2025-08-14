use axum::{
    Extension, Router,
    body::Body,
    extract::Json,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use bcrypt;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row, postgres::PgPoolOptions, query};
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
mod api;
mod utils;

#[tokio::main]
async fn main() {
    let backend_addr = "127.0.0.1:8001";

    //load the .env file
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create DB Pool");

    // /users routes
    let user_router = Router::new();

    //Create main router
    let main_router = Router::new()
        .nest("/users", user_router)
        .layer(Extension(pool))
        .layer(CorsLayer::very_permissive());

    //Listen on port 8001
    let listener = TcpListener::bind(backend_addr).await.unwrap();

    println!("Server running at {}", backend_addr);

    //Serve the app using axum::serve
    axum::serve(listener, main_router).await.unwrap();
}
