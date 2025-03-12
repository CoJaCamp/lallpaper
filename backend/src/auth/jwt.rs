use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env; // For environment variables (e.g., secret key)

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,   // User ID
    pub exp: usize, // Expiration timestamp
}

pub fn create_jwt(user_id: i32) -> Result<String, jsonwebsoken::errors::Error> {}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebsoken::errors::Error> {}
