use std::env;
use dotenv::dotenv;
use chrono::{Duration, FixedOffset, Utc};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use jsonwebtoken::{
    decode, 
    encode, 
    Algorithm, 
    DecodingKey, 
    EncodingKey, 
    Header, 
    Validation
};
use uuid::Uuid;

use crate::models::{access_token::AccessToken, refresh_token::RefreshToken};

pub async fn generate_refresh_token(
    pool: &MySqlPool
) -> String {
    
    let mut refresh_token: String;
    loop {
        // Generate a 32-character alphanumeric token
        refresh_token = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let q = format!("SELECT 1 FROM refresh_tokens WHERE token = {refresh_token}");
        // Check for uniqueness in the database
        let exists = sqlx::query_as::<_, RefreshToken>(&q)
            .fetch_optional(pool)
            .await;

        if !exists.is_ok() {
            // Token is unique, break the loop
            break;
        }
    }
    refresh_token
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (usually the user ID)
    exp: usize,  // Expiration time (in seconds since the Unix epoch)
}

pub async fn generate_access_token(
    user_id: &i32,
    pool: &MySqlPool
) -> Result<String, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let mut token: String;
    let secret_key = env::var("SECRET_KEY").expect("Secret key not found");

    let offset = FixedOffset::east_opt(6 * 3600);
    let now_dhaka = Utc::now().with_timezone(&offset.unwrap());
    let expiration = now_dhaka + Duration::minutes(60);

    let claims = Claims {
        sub: user_id.to_string(), 
        exp: expiration.timestamp() as usize,
    };
    
    loop {
        token = encode(
            &Header::default(), 
            &claims,
            &EncodingKey::from_secret(secret_key.as_bytes()), 
        )?;
    
        let q = format!("SELECT 1 FROM access_tokens WHERE token = {token}");
        // Check for uniqueness in the database
        let exists = sqlx::query_as::<_, AccessToken>(&q)
            .fetch_optional(pool)
            .await;
    
        if !exists.is_ok() {
            // Token is unique, break the loop
            break;
        }
    }

    Ok(token)
}

pub async fn decode_access_token(token: &str) {
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("Secret key not found");

    let validation = Validation::new(Algorithm::HS256);

    let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());
    let result = decode::<Claims>(token, &decoding_key, &validation);

    match result {
        Ok(token_data) => {
            println!("Decoded JWT:");
            println!("Header: {:?}", token_data.header);
            println!("Claims: {:?}", token_data.claims);
        }
        Err(err) => {
            println!("Error decoding JWT: {}", err);
        }
    }
}

pub async fn generate_api_key() -> String {
    Uuid::new_v4().to_string() 
}