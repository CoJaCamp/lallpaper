use axum::{Extension, Json, http::StatusCode};
use axum_extra::extract::CookieJar;
use bcrypt::bcrypt;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, query};

use crate::utils::jwt::encode_access_token;

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

async fn register(
    jar: CookieJar,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<RegisterData>,
) -> Result<CookieJar, StatusCode> {
    let username = &payload.username;

    let taken_name = query("SELECT username FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if taken_name.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let hashed_password =
        bcrypt::hash(&payload.password, 12).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_user = query(
        "INSERT INTO users (username, hashed_password) VALUES ($1, $2) RETURNING id, username",
    )
    .bind(username)
    .bind(hashed_password)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (access_token, access_token_exp) = encode_access_token(new_user.get("id"), new_user.get("username"))
}
