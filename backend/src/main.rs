use axum::{
    Router,
    extract::Json,
    routing::{get, post},
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    //load the .env file
    dotenv().ok();
    let backend_addr = "127.0.0.1:8001";

    // /users routes
    let user_router = Router::new()
        .route("/login", post(login_users))
        .route("/register", post(register_users));

    //Create main router
    let main_router = Router::new()
        .nest("/users", user_router)
        .layer(CorsLayer::very_permissive());

    //Listen on port 8001
    let listener = TcpListener::bind(backend_addr).await.unwrap();

    println!("Server running at {}", backend_addr);

    //Serve the app using axum::serve
    axum::serve(listener, main_router).await.unwrap();
}

#[derive(Serialize)]
struct Output {
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct RegisterData {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct LoginData {
    username: String,
    password: String,
}

async fn register_users(Json(payload): Json<RegisterData>) -> Json<Output> {
    println!("{:?}", payload);

    let response = Output {
        message: String::from("Test1"),
    };

    Json(response)
}

async fn login_users(Json(payload): Json<RegisterData>) -> Json<Output> {
    println!("{:?}", payload);

    let response = Output {
        message: String::from("Test1"),
    };

    Json(response)
}
