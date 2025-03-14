//use chrono::{Duration, Utc}; Only needed for experation
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // User ID
                  // pub exp: usize, //!!For the sake of simplicity the experation has been omitted, add back if using in production!!
}

pub fn create_jwt(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
    let claim = Claims { sub: user_id };

    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_bytes()),
    );

    token
}

pub fn validate_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");

    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    decoded
}
