use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sqlx::MySqlPool;
use jsonwebtoken::{
    encode, 
    EncodingKey, 
    Header
};

use crate::models::refresh_token::RefreshToken;

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

pub async fn generate_access_token(
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

        let q = format!("SELECT 1 FROM access_tokens WHERE token = {refresh_token}");
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

pub async fn generate_api_key(
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

        let q = format!("SELECT 1 FROM api_keys WHERE api_key = {refresh_token}");
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