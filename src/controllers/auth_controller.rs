use argon2::{PasswordHash, PasswordVerifier};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use chrono::{Duration, FixedOffset, Utc};
use sqlx::MySqlPool;
use validator::Validate;

use tower_cookies::{Cookie, Cookies};

use crate::{
    models::{
        auth::{LoginUser, LogoutUser, RefreshUser, ResponseMessage}, refresh_token::RefreshToken, user::User
    },
    utils::{
        input_validation::handle_validation_errors,
        tokens::{generate_access_token, generate_refresh_token},
    },
};

pub async fn login(
    Extension(pool): Extension<MySqlPool>,
    cookies: Cookies,
    Json(payload): Json<LoginUser>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    payload.validate().map_err(|errors| {
        let error_message = handle_validation_errors(errors);
        (StatusCode::BAD_REQUEST, error_message)
    })?;

    let q = format!(
        "SELECT * FROM users WHERE username = '{}'",
        payload.username
    );

    let user_result = sqlx::query_as::<_, User>(&q).fetch_one(&pool).await;

    match user_result {
        Ok(user) => {
            let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to parse password hash: {}", e),
                )
            })?;

            if argon2::Argon2::default()
                .verify_password(payload.password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                // Successful authentication

                // Generate JWT access token
                let token = generate_access_token(&user.id, &pool).await?;

                println!("Generated JWT access token: {}", token);

                // Generate refresh token
                let refresh_token = generate_refresh_token(&pool).await;

                // Store refresh token in the database
                let offset = FixedOffset::east_opt(6 * 3600);
                let now_dhaka = Utc::now().with_timezone(&offset.unwrap());
                let expires_at = now_dhaka + Duration::days(7);
                let expires_at_formatted = expires_at.with_timezone(&Utc);

                let q = format!(
                    "INSERT INTO refresh_tokens (token, user_id, expires_at) VALUES (?, ?, ?)"
                );

                sqlx::query(&q)
                    .bind(&refresh_token)
                    .bind(&user.id)
                    .bind(&expires_at_formatted)
                    .execute(&pool)
                    .await
                    .map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to store refresh token in database: {}", e),
                        )
                    })?;

                // Create cookies for access and refresh tokens
                let access_token_cookie = Cookie::build(("access_token", token))
                    .http_only(true)
                    .path("/api")
                    .build();

                let refresh_token_cookie = Cookie::build(("refresh_token", refresh_token))
                    .http_only(true)
                    .path("/api") // Restrict to refresh route only
                    .build();

                // Add the cookies to the existing cookies object
                cookies.add(access_token_cookie);
                cookies.add(refresh_token_cookie);

                // Build the response and attach cookies
                let response = (
                    StatusCode::OK,
                    Json(ResponseMessage {
                        message: "Login Successful".to_string(),
                    }),
                )
                    .into_response();

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
    cookies: Cookies,
    Json(payload): Json<LogoutUser>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Clear the access token cookie
    // Clear the refresh token cookie
    let access_token_cookie = Cookie::build(("access_token", ""))
        .http_only(true)
        .path("/api")
        .build();

    let refresh_token_cookie = Cookie::build(("refresh_token", ""))
        .http_only(true)
        .path("/api") // Restrict to refresh route only
        .build();

    cookies.add(access_token_cookie);
    cookies.add(refresh_token_cookie);

    // Invalidate the refresh token in the database
    let q = format!(
        "DELETE FROM refresh_tokens WHERE user_id = '{}'",
        payload.user_id
    );
    sqlx::query(&q).execute(&pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to invalidate refresh token in database: {}", e),
        )
    })?;

    // Build the response.
    let response = (
        StatusCode::OK,
        Json(ResponseMessage {
            message: "Logout successful".to_string(),
        }),
    )
        .into_response();

    Ok(response)
}

pub async fn refresh(
    Extension(pool): Extension<MySqlPool>,
    cookies: Cookies,
    Json(payload): Json<RefreshUser>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let refresh_token = cookies
        .get("refresh_token")
        .ok_or((StatusCode::UNAUTHORIZED, "Missing refresh token".to_string()))?
        .value()
        .to_string();

    println!("Refresh token: {}", refresh_token);
    // 1. Retrieve the refresh token from the database
    let q = &format!("SELECT * FROM refresh_tokens WHERE token = '{refresh_token}'");

    let token_data = sqlx::query_as::<_, RefreshToken>(q)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch refresh token from database: {}", e)))?;

    let token_data = token_data.ok_or((StatusCode::UNAUTHORIZED, "Invalid refresh token".to_string()))?;

    // 2. Check if the refresh token has expired
    if token_data.expires_at < Utc::now() {
        return Err((StatusCode::UNAUTHORIZED, "Refresh token expired".to_string()));
    }

    // Convert to i32
    let user_id = payload.user_id.parse::<i32>().unwrap();

    // 3. If the refresh token is valid, generate a new access token
    let new_access_token = generate_access_token(&user_id, &pool)
        .await
        .map_err(|_| {(StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate access token".to_string())})?;

    // 4. Optionally, you might want to generate a new refresh token and update it in the database
    let new_refresh_token = generate_refresh_token(&pool).await;

    sqlx::query("UPDATE refresh_tokens SET token = ?, expires_at = ? WHERE id = ?")
        .bind(&new_refresh_token)
        .bind(Utc::now() + Duration::days(30))
        .bind(&payload.user_id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update refresh token in database: {}", e)))?;

    // 5. Set the new access token as a cookie
    let access_token_cookie = Cookie::build(("access_token", new_access_token))
        .http_only(true)
        .path("/api")
        .build();
    cookies.add(access_token_cookie);

    // 6. Build the response and attach the new access token cookie
    let response = (
        StatusCode::OK,
        Json( ResponseMessage { message: "Token refreshed successfully".to_string() }),
    )
        .into_response();

    Ok(response)
}
