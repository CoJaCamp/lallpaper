use axum::{
    Extension, Router,
    extract::Json,
    routing::{get, post},
};
use bcrypt;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions, query};
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

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
    let user_router = Router::new()
        .route("/login", post(login_users))
        .route("/register", post(register_users));

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

async fn register_users(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<RegisterData>,
) -> Json<Output> {
    let mut response = Output {
        message: String::new(),
    };

    let username_check = query("SELECT username FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&pool)
        .await;

    match username_check {
        Ok(Some(row)) => {
            response.message = String::from("That username already exists!");
            println!("Test Failed");
        }
        Ok(None) => {
            println!("Next step: HASH");
            //change hash cost to 12-14 for production
            let hashed_password = match bcrypt::hash(payload.password, 8) {
                Ok(hash) => hash,
                Err(e) => {
                    eprintln!("Error hashing password: {}", e);
                    todo!()
                }
            };
            println!("{}", hashed_password);
            let new_user =
                query("INSERT INTO users (username, hashed_password) VALUES ($1, $2) RETURNING id")
                    .bind(&payload.username)
                    .bind(hashed_password)
                    .fetch_one(&pool)
                    .await;

            match new_user {
                Ok(id) => {
                    println!("id is here")
                }
                Err(e) => {
                    eprintln!("Database Error: {}", e);
                    todo!()
                }
            }
        }
        Err(e) => {}
    }

    Json(response)
}

async fn login_users(Json(payload): Json<LoginData>) -> Json<Output> {
    println!("{:?}", payload);

    let response = Output {
        message: String::from("Test1"),
    };

    Json(response)
}
