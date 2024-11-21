use std::{env, time::{Duration, SystemTime, UNIX_EPOCH}};

use jsonwebtoken::{decode, encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: usize, // Expiration date as UNIX timestamp
}

const secret: &[u8] = env::var("JWT_SECRET").unwrap().as_bytes();

pub fn generate_token(user_id: &str) -> String {
    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(60 * 60 * 24)) // One day
        .expect("valid timestamp")
        .duration_since(UNIX_EPOCH)
        .expect("valid timestamp")
        .as_secs();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default, &claims, &EncodingKey::from_secret(secret))
        .expect("Failed to generate token.")
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &EncodingKey::from_secret(secret),
        &jsonwebtoken::Validation::default(),
    )
    .map(|data| data.claims)
}