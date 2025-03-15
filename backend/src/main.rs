use axum::{
    Extension, Router,
    body::Body,
    extract::Json,
    response::{self, IntoResponse, Response},
    routing::{get, post},
};
use bcrypt;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row, postgres::PgPoolOptions, query};
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
mod auth;
use auth::jwt;

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
struct Message<T> {
    message: String,
    data: Option<T>,
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
) -> Response<Body> {
    let mut body: Message<String> = Message {
        message: String::new(),
        data: None,
    };

    let username_check = query("SELECT username FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&pool)
        .await;

    match username_check {
        Ok(Some(row)) => {
            body.message = String::from("That username already exists!");
            println!("Test Failed");
        }
        Ok(None) => {
            println!("Next step: HASH");
            //Change hash cost to 12-14 for production
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
                Ok(row) => {
                    let jwt = jwt::create_jwt(row.get("id"));
                    body.data = Some(jwt.unwrap());
                }
                Err(e) => {
                    eprintln!("Database Error: {}", e);
                    todo!()
                }
            }
        }
        Err(e) => {}
    }

    let response = Json(body).into_response();

    response
}

async fn login_users(Json(payload): Json<LoginData>) -> Json<String> {
    println!("{:?}", payload);

    Json("String".to_string())
}
