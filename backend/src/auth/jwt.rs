use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub user_id: i32, //user id
    pub username: String,
    pub user_roles: Vec<String>,
    pub exp: u64,
    pub token_type: TokenType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub user_id: i32,
    pub exp: u64,
    pub token_type: TokenType,
}

pub fn encode_access_token(user_id: i32, username: String, user_roles: Vec<String>) -> Result<(String, u64), jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");

    let expiration = Utc::now().checked_add_signed(Duration::hours(1)).expect("valid timestamp").timestamp() as u64;

    let access_claim = AccessClaims{
        user_id,
        username,
        user_roles,
        exp: expiration,
        token_type: TokenType::Access,
    }

    let new_token = encode(&Header::default(), &access_claim, &EncodingKey::from_secret(&secret.as_bytes()))?;

    Ok((new_token, expiration))
}

pub fn decode_access_token(token: &str) -> Result<TokenData<AccessClaims>, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");

    let token_data:TokenData<AccessClaims> = decode(&token, &DecodingKey::from_secret(&secret.as_bytes()), &Validation::default())?;

    Ok(token_data)
}
