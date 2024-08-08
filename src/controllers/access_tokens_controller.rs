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
    models::access_token::{
        AccessToken, CreateAccessTokenFromInput, FieldValue, UpdateAccessToken
    }, 
    utils::{
        input_validation::handle_validation_errors, 
        tokens::generate_access_token
    }
};

pub async fn access_tokens_index(
    Extension(pool): Extension<MySqlPool>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let q = &format!("SELECT * FROM access_tokens");

    let access_tokens = sqlx::query_as::<_, AccessToken>(q)
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch access tokens from database: {}", e)))?;

    Ok((StatusCode::OK, Json(access_tokens)))
}

pub async fn access_tokens_find(
    Extension(pool): Extension<MySqlPool>, 
    Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
    Ok((StatusCode::OK, Json(fetch_access_token(&pool, id).await?)))
}

// Helper function for fetching access token.
pub async fn fetch_access_token(
    pool: &MySqlPool,
    id: i32
) -> Result<AccessToken, (StatusCode, String)> {
    let q = &format!("SELECT * FROM access_tokens WHERE id = {id}");

    let access_token = sqlx::query_as::<_, AccessToken>(q)
        .fetch_one(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch access token from database: {}", e)))?;

    Ok(access_token)
}

pub async fn access_tokens_create(
    Extension(pool): Extension<MySqlPool>, 
    Json(input): Json<CreateAccessTokenFromInput>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
    input.validate().map_err(|e| {
        let error_string = handle_validation_errors(e);
        (StatusCode::BAD_REQUEST, error_string)
    })?;

    let access_token = create_access_token(&pool, &input.user_id).await?;
    
    Ok((StatusCode::CREATED, Json(access_token)))
}

// Helper function for creating access token.
pub async fn create_access_token(
    pool: &MySqlPool,
    user_id: &i32
) -> Result<AccessToken, (StatusCode, String)>  {
    let offset = FixedOffset::east_opt(6 * 3600); // BST is +6 hours from UTC
    let now_in_dhaka: DateTime<FixedOffset> = Utc::now().with_timezone(&offset.unwrap());
    let expires_at = now_in_dhaka + Duration::days(7);
    let expires_at_formatted = expires_at.with_timezone(&Utc);
    let token = generate_access_token(user_id, pool).await?;
    let q = &format!("INSERT INTO access_tokens (user_id, token, expires_at) VALUES (?, ?, ?)");

    let access_token_id = sqlx::query(q)
        .bind(user_id)
        .bind(token)
        .bind(expires_at_formatted)
        .execute(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create access token in database: {:?}", e)))?;
    
    let access_token = fetch_access_token(pool, access_token_id.last_insert_id() as i32).await?;

    Ok(access_token)
}

pub async fn access_tokens_update(
    Extension(pool): Extension<MySqlPool>, 
    Path(id): Path<i32>, 
    Json(updates): Json<UpdateAccessToken>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut query_string = "UPDATE access_tokens SET ".to_string();
    let mut params: Vec<String> = vec![];
    
    access_tokens_query_builder(&mut query_string, &mut params, &updates).await;
    query_string.push_str(&format!(" WHERE id = {}", id));

    sqlx::query(&query_string)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update access token in database: {}", e)))?;

    let access_token = fetch_access_token(&pool, id).await?;
    
    Ok((StatusCode::OK, Json(access_token)))
}

pub async fn access_tokens_query_builder(
    query: &mut String, 
    params: &mut Vec<String>, 
    updates: &UpdateAccessToken
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

pub async fn access_tokens_delete(
    Extension(pool): Extension<MySqlPool>, 
    Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let q = &format!("DELETE FROM access_tokens WHERE id = {}", id);

    sqlx::query(q)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete access token in database: {}", e)))?;
    
    Ok((StatusCode::OK, Json("Access token deleted successfully".to_string())))
}