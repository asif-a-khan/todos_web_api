use axum::{
    extract::Request, 
    http::StatusCode, 
    middleware::Next, 
    response::Response, 
    Extension, Json,
};
use chrono::{
    TimeZone, 
    Utc
};
use sqlx::MySqlPool;
use tower_cookies::Cookies;

use crate::{
    models::auth::ResponseMessage, 
    utils::tokens::decode_access_token
};

pub struct AuthToken(pub String);

// Middleware function to check authentication
pub async fn check_token_auth(
    Extension(_pool): Extension<MySqlPool>,
    cookies: Cookies,
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ResponseMessage>)> {
    // 1. Retrieve the cookie from the request
    let cookie = cookies
        .get("access_token")
        .map(|c| c.value().to_string());

    // 2. Verify the cookie if there is one.
    if let Some(cookie) = cookie {
        // 3. Check if the token is expired
            let token_data = decode_access_token(&cookie).await?;
            let timestamp = token_data.claims.exp;
            // Convert to UTC time and time in Dhaka
            let expiration_datetime_utc = Utc.timestamp_opt(timestamp as i64, 0); 

            // Get current time
            let current_time = Utc::now();

            if current_time > expiration_datetime_utc.unwrap() {
                return Err((StatusCode::UNAUTHORIZED, Json(ResponseMessage { message: "Token expired".to_string() })));
            }
    } else {
        return Err((StatusCode::UNAUTHORIZED, Json(ResponseMessage { message: "Missing token".to_string() })))
    }

    // 4. If the token valid and not expired, return the next middleware
    Ok(next.run(req).await)

}

pub async fn api_key_auth(
    Extension(pool): Extension<MySqlPool>,
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // 1. Extract API key from the request header (adjust if needed)
    let api_key = req
        .headers()
        .get("X-Api-Key") // Assuming you're using a custom header for the API key
        .and_then(|header| header.to_str().ok())
        .map(|key| key.to_string());

    let api_key = api_key.ok_or((
        StatusCode::UNAUTHORIZED,
        "Missing or invalid API key".to_string(),
    ))?;

    // 2. Validate API key against the database
    let is_valid = sqlx::query("SELECT 1 FROM api_keys WHERE api_key = ? AND is_active = true")
        .bind(&api_key)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to validate API key: {}", e)))?;

    if is_valid.is_some() {
        // 3. If the API key is valid, proceed to the next middleware/handler
        Ok(next.run(req).await)
    } else {
        // 4. If the API key is invalid, return an error response
        Err((StatusCode::UNAUTHORIZED, "Invalid or inactive API key".to_string()))
    }
}



pub async fn main_response_mapper(res: Response) -> Response {
    res
}
