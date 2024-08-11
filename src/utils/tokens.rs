use std::env;
use axum::{http::StatusCode, Json};
use dotenv::dotenv;
use chrono::{
    DateTime, Duration, FixedOffset, TimeZone, Utc
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sqlx::MySqlPool;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation
};
use uuid::Uuid;

use crate::models::{access_token::AccessToken, auth::{Claims, ResponseMessage}, refresh_token::RefreshToken};

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
    user_id: &i32,
    pool: &MySqlPool
) -> Result<String, (StatusCode, String)> {
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
        ).map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to generate access token: {}", e))
        })?;

        
    
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

pub async fn decode_access_token(token: &str) -> Result<TokenData<Claims>, (StatusCode, Json<ResponseMessage>)>{
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("Secret key not found");

    let validation = Validation::new(Algorithm::HS256);

    let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());
    let result = decode::<Claims>(token, &decoding_key, &validation);

    match result {
        Ok(token_data) => Ok(token_data),
        Err(err) => {
            match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    return Err((StatusCode::UNAUTHORIZED, Json(ResponseMessage { message: "Token expired in decode token".to_string() })));
                }
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    return Err((StatusCode::UNAUTHORIZED, Json(ResponseMessage { message: format!("Error decoding JWT: {}", err) })));
                },
                _ => {
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseMessage { message: format!("Error decoding JWT: {}", err) })));
                }
            }
        }
    }
}

pub async fn generate_api_key() -> String {
    Uuid::new_v4().to_string() 
}

pub async fn time_in_dhaka(timestamp: i64) -> DateTime<FixedOffset> {
    // Convert to UTC time and time in Dhaka
    let expiration_datetime_utc = Utc.timestamp_opt(timestamp as i64, 0); 

    let offset_opt = FixedOffset::east_opt(6 * 3600);
    let offset = offset_opt.unwrap();
    let expiration_datetime_bst = expiration_datetime_utc.unwrap().with_timezone(&offset);
    expiration_datetime_bst
}