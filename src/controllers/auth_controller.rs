use argon2::{PasswordHash, PasswordVerifier};
use axum::{
    body::{self, Body}, extract::Extension, http::{header::SET_COOKIE, StatusCode}, response::{IntoResponse, Response}, Json
};
use serde::Serialize;
use sqlx::MySqlPool;
use validator::Validate;
use chrono::{
    Duration, 
    FixedOffset, 
    Utc
};
use tower_cookies::{
    cookie::time::OffsetDateTime, 
    cookie::time::Duration as TowerDuration, 
    Cookie,
}; 

use crate::{models::{
    auth::{LoginUser, LogoutUser},
    user::User
}, 
utils::{
    input_validation::handle_validation_errors, 
    tokens::{
        generate_access_token, 
        generate_refresh_token
    }
}};

#[derive(Serialize)]
struct ResponseMessage {
    message: &'static str,
}

pub async fn login(
    Extension(pool): Extension<MySqlPool>,
    Json(payload): Json<LoginUser>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    payload.validate().map_err(|errors| {
        let error_message = handle_validation_errors(errors);
        (StatusCode::BAD_REQUEST, error_message)
    })?;

    let q = format!("SELECT * FROM users WHERE username = '{}'", payload.username);

    let user_result = sqlx::query_as::<_, User>(&q)
        .fetch_one(&pool)
        .await;
    
    match user_result {
        Ok(user) => {
            let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|e| {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse password hash: {}", e))
            })?;

            if argon2::Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_ok() {
                // Successful authentication

                // Generate JWT access token
                let token = generate_access_token(&user.id, &pool).await?;

                // Generate refresh token
                let refresh_token = generate_refresh_token(&pool).await;

                // Store refresh token in the database
                let offset = FixedOffset::east_opt(6 * 3600);
                let now_dhaka = Utc::now().with_timezone(&offset.unwrap());
                let expires_at = now_dhaka + Duration::days(7);
                let expires_at_formatted = expires_at.with_timezone(&Utc);

                let q = format!("INSERT INTO refresh_tokens (token, user_id, expires_at) VALUES (?, ?, ?)");

                sqlx::query(&q)
                    .bind(&refresh_token)
                    .bind(&user.id)
                    .bind(&expires_at_formatted)
                    .execute(&pool)
                    .await
                    .map_err(|e| {
                        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to store refresh token in database: {}", e))
                    })?;

                // Create cookies for access and refresh tokens
                let access_token_cookie = Cookie::build(("access_token", token))
                    .http_only(true)
                    .path("/")
                    .build();

                let refresh_token_cookie = Cookie::build(("refresh_token", refresh_token))
                    .http_only(true)
                    .path("/auth/refresh") // Restrict to refresh route only
                    .build();

                // Build the response and attach cookies
                let mut response = (StatusCode::OK, Json(ResponseMessage {message: "Login Successful"})).into_response();

                response.headers_mut().insert(
                    SET_COOKIE,
                    access_token_cookie.to_string().parse().unwrap(),
                );
                response.headers_mut().insert(
                    SET_COOKIE,
                    refresh_token_cookie.to_string().parse().unwrap(),
                );

                Ok(response)
            } else {
                Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
            }
        }
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())),
    }
}

pub async fn logout(
    Extension(pool): Extension<MySqlPool>,
    Json(payload): Json<LogoutUser>
) -> Result <impl IntoResponse, (StatusCode, String)> {
    
    // Clear the access token cookie
    let access_token_cookie = Cookie::build(("access_token", ""))
        .http_only(true)
        .path("/")
        .expires(Some(OffsetDateTime::now_utc() - TowerDuration::seconds(1)))
        .build();

    // Clear the refresh token cookie
    let refresh_token_cookie = Cookie::build(("refresh_token", ""))
        .http_only(true)
        .path("/auth/refresh") 
        .expires(Some(OffsetDateTime::now_utc() - TowerDuration::seconds(1)))
        .build();

    // Invalidate the refresh token in the database
    let q = format!("DELETE FROM refresh_tokens WHERE user_id = '{}'", payload.user_id);
    sqlx::query(&q)
        .execute(&pool)
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to invalidate refresh token in database: {}", e))
        })?;

    // Build the response and attach cookies to clear them
    let mut response = (StatusCode::OK, Json(ResponseMessage {message: "Logout successful"})).into_response();

    response.headers_mut().insert(
        SET_COOKIE,
        access_token_cookie.to_string().parse().unwrap(),
    );
    response.headers_mut().insert(
        SET_COOKIE,
        refresh_token_cookie.to_string().parse().unwrap(),
    );

    Ok(response)
}
