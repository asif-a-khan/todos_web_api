use validator::Validate;
use sqlx::MySqlPool;

use axum::{
    extract::Path, 
    http::StatusCode, 
    response::IntoResponse, 
    Extension, 
    Json
};

use crate::{
    models::api_key::{
        CreateApiKey, 
        FieldValue, 
        ApiKey, 
        UpdateApiKey
    }, 
    utils::{
        input_validation::handle_validation_errors, 
        tokens::generate_api_key
    }
};

pub async fn api_keys_index(
    Extension(pool): Extension<MySqlPool>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let q = &format!("SELECT * FROM api_keys");

    let api_keys = sqlx::query_as::<_, ApiKey>(q)
        .fetch_all(&pool)
        .await;

    if let Err(e) = api_keys {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch api keys from database: {}", e)))
    }

    Ok((StatusCode::OK, Json(api_keys.unwrap())))
}

pub async fn api_keys_find(
    Extension(pool): Extension<MySqlPool>, 
    Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
    let api_key = fetch_api_key(&pool, id).await;

    if let Err(e) = api_key {
        return Err(e)
    }

    Ok((StatusCode::OK, Json(api_key.unwrap())))
}


// Helper function for fetching api key.
pub async fn fetch_api_key(
    pool: &MySqlPool,
    id: i32
) -> Result<ApiKey, (StatusCode, String)> {
    let q = &format!("SELECT * FROM api_keys WHERE id = {id}");

    let api_key = sqlx::query_as::<_, ApiKey>(q)
        .fetch_one(pool)
        .await;

    if let Err(e) = api_key {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch api key from database: {}", e)))
    }

    Ok(api_key.unwrap())
}

pub async fn api_keys_create(
    Extension(pool): Extension<MySqlPool>, 
    Json(input): Json<CreateApiKey>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
    let validation = input.validate();

    if let Err(e) = validation {
        let error_string = handle_validation_errors(e);
        return Err((StatusCode::BAD_REQUEST, error_string))
    }

    let api_key = create_api_key(&pool, &input.client_name, &input.contact_email).await;

    if let Err(e) = api_key {
        return Err(e)
    }
    
    Ok((StatusCode::CREATED, Json(api_key.unwrap())))
}

// Helper function for creating api key.
pub async fn create_api_key(
    pool: &MySqlPool,
    client_name: &str,
    contact_email: &str
) -> Result<ApiKey, (StatusCode, String)>  {
    let api_key = generate_api_key().await;
    let q = &format!("INSERT INTO api_keys (api_key, client_name, contact_email) VALUES ({api_key}, {client_name}, {contact_email})");

    let api_key_id = sqlx::query(q)
        .execute(pool)
        .await;

    if let Err(e) = api_key_id {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create api key in database: {}", e)))
    }
    
    let api_key = fetch_api_key(pool, api_key_id.unwrap().last_insert_id() as i32).await;

    if let Err(e) = api_key {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch api key from database: {:?}", e)))
    }

    return Ok(api_key.unwrap());
}

pub async fn api_keys_update(
    Extension(pool): Extension<MySqlPool>, 
    Path(id): Path<i32>, 
    Json(updates): Json<UpdateApiKey>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut query_string = "UPDATE api_keys SET ".to_string();
    let mut params: Vec<String> = vec![];

    api_keys_update_query_builder(&mut query_string, &mut params, &updates).await;
    query_string.push_str(&format!(" WHERE id = {}", id));

    let api_key = sqlx::query(&query_string)
        .execute(&pool)
        .await;

    if let Err(e) = api_key {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update api token in database: {}", e)))
    }

    let api_key = fetch_api_key(&pool, id).await;

    if let Err(e) = api_key {
        return Err(e)
    }
    
    Ok((StatusCode::OK, Json(api_key.unwrap())))
}

pub async fn api_keys_update_query_builder(
    query: &mut String, 
    params: &mut Vec<String>, 
    updates: &UpdateApiKey
) {
    for (field, value) in updates.clone().into_iter() {
        match value {
            FieldValue::ApiKey(api_key) => {
                if let Some(api_key) = api_key {
                    query.push_str(&format!("{} = ?, ", field));
                    params.push(api_key);
                }
            },
            FieldValue::ClientName(client_name) => {
                if let Some(client_name) = client_name {
                    query.push_str(&format!("{} = ?, ", field));
                    params.push(client_name);
                }
            },
            FieldValue::ContactEmail(contact_email) => {
                if let Some(contact_email) = contact_email {
                    query.push_str(&format!("{} = ?, ", field));
                    params.push(contact_email);
                }
            },
            FieldValue::IsActive(expires_at) => {
                if let Some(expires_at) = expires_at {
                    query.push_str(&format!("{} = ?, ", field));
                    params.push(expires_at.to_string());
                }
            }
        }
    }
}

pub async fn api_keys_delete(
    Extension(pool): Extension<MySqlPool>, 
    Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let q = &format!("DELETE FROM api_keys WHERE id = {}", id);
    let api_key = sqlx::query(q)
        .execute(&pool)
        .await;

    if let Err(e) = api_key {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete api key in database: {}", e)))
    }
    
    Ok((StatusCode::OK, Json("Api key deleted successfully".to_string())))
}