use validator::Validate;
use sqlx::MySqlPool;

use axum::{
    extract::Path, 
    http::StatusCode, 
    response::IntoResponse, 
    Extension, 
    Json
};
use chrono::{
    DateTime, 
    FixedOffset, 
    Utc, 
    Duration
};
use crate::{
    models::refresh_token::{
        CreateRefreshTokenFromInput, 
        FieldValue, 
        RefreshToken, 
        UpdateRefreshToken
    }, 
    utils::{
        input_validation::handle_validation_errors, 
        tokens::generate_refresh_token
    }
};

pub async fn refresh_tokens_index(
    Extension(pool): Extension<MySqlPool>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let q = &format!("SELECT * FROM refresh_tokens");

    let refresh_tokens = sqlx::query_as::<_, RefreshToken>(q)
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch refresh tokens from database: {}", e)))?;

    Ok((StatusCode::OK, Json(refresh_tokens)))
}

pub async fn refresh_tokens_find(
    Extension(pool): Extension<MySqlPool>, 
    Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
    Ok((StatusCode::OK, Json(fetch_refresh_token(&pool, id).await?)))
}


// Helper function for fetching refresh token.
pub async fn fetch_refresh_token(
    pool: &MySqlPool,
    id: i32
) -> Result<RefreshToken, (StatusCode, String)> {
    let q = &format!("SELECT * FROM refresh_tokens WHERE id = {id}");

    let refresh_token = sqlx::query_as::<_, RefreshToken>(q)
        .fetch_one(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch refresh token from database: {}", e)))?;

    Ok(refresh_token)
}

pub async fn refresh_tokens_create(
    Extension(pool): Extension<MySqlPool>, 
    Json(input): Json<CreateRefreshTokenFromInput>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
    input.validate().map_err(|e| {
        let error_string = handle_validation_errors(e);
        (StatusCode::BAD_REQUEST, error_string)
    })?;

    let refresh_token = create_refresh_token(&pool, &input.user_id).await?;
    
    Ok((StatusCode::CREATED, Json(refresh_token)))
}

// Helper function for creating refresh token.
pub async fn create_refresh_token(
    pool: &MySqlPool,
    user_id: &i32
) -> Result<RefreshToken, (StatusCode, String)>  {
    let offset = FixedOffset::east_opt(6 * 3600); // BST is +6 hours from UTC
    let now_in_dhaka: DateTime<FixedOffset> = Utc::now().with_timezone(&offset.unwrap());
    let expires_at = now_in_dhaka + Duration::days(7);
    let expires_at_formatted = expires_at.with_timezone(&Utc);
    let token = generate_refresh_token(pool).await;

    let q = &format!("INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES (?, ?, ?)");
    let refresh_token_id = sqlx::query(q)
        .bind(user_id)
        .bind(token)
        .bind(expires_at_formatted)
        .execute(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create refresh token in database: {}", e)))?.last_insert_id();
    
    let refresh_token = fetch_refresh_token(pool, refresh_token_id as i32).await?;

    return Ok(refresh_token);
}

pub async fn refresh_tokens_update(
    Extension(pool): Extension<MySqlPool>, 
    Path(id): Path<i32>, 
    Json(updates): Json<UpdateRefreshToken>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut query_string = "UPDATE refresh_tokens SET ".to_string();
    let mut params: Vec<String> = vec![];

    refresh_tokens_query_builder(&mut query_string, &mut params, &updates).await;
    query_string.push_str(&format!(" WHERE id = {}", id));

    sqlx::query(&query_string)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update refresh token in database: {}", e)))?;

    let refresh_token = fetch_refresh_token(&pool, id).await?;
    
    Ok((StatusCode::OK, Json(refresh_token)))
}

pub async fn refresh_tokens_query_builder(
    query: &mut String, 
    params: &mut Vec<String>, 
    updates: &UpdateRefreshToken
) {
    for (field, value) in updates.clone().into_iter() {
        match value {
            FieldValue::Token(token) => {
                if let Some(token) = token {
                    query.push_str(&format!("{} = ?, ", field));
                    params.push(token);
                }
            },
            FieldValue::ExpiresAt(expires_at) => {
                if let Some(expires_at) = expires_at {
                    query.push_str(&format!("{} = ?, ", field));
                    params.push(expires_at.to_string());
                }
            }
        }
    }
}

pub async fn refresh_tokens_delete(
    Extension(pool): Extension<MySqlPool>, 
    Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let q = &format!("DELETE FROM refresh_tokens WHERE id = {}", id);
    
    sqlx::query(q)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete refresh token in database: {}", e)))?;

    Ok((StatusCode::OK, Json("Refresh token deleted successfully".to_string())))
}