use argon2::{
    password_hash::{
        PasswordHasher, 
        SaltString
    },
    Argon2,
};

use axum::{
	extract::Path, 
    http::StatusCode, 
    response::IntoResponse, 
    Extension, 
    Json
};

use validator::Validate;

use crate::{
    models::user::{
        CreateUser, 
        CreateUserFromInput, 
        FieldValue, 
        UpdateUser, 
        User
    }, 
    utils::input_validation::handle_validation_errors
};

use sqlx::MySqlPool;

pub async fn users_index(
    Extension(pool): Extension<MySqlPool>
) -> Result<impl IntoResponse, (StatusCode, String)> {
	let q = "SELECT * FROM users";

	let users = sqlx::query_as::<_, User>(q)
		.fetch_all(&pool)
		.await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch users from database: {}", e)))?;

	Ok((StatusCode::OK, Json(users)))
}

pub async fn users_find(
	Extension(pool): Extension<MySqlPool>, 
	Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
    Ok((StatusCode::OK, Json(fetch_user(&id, &pool).await?))) 
}

pub async fn users_create(
    Extension(pool): Extension<MySqlPool>, 
    Json(input): Json<CreateUserFromInput>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    input.validate().map_err(|e| {
        let error_string = handle_validation_errors(e);
        (StatusCode::BAD_REQUEST, format!("Validation failed: {}", error_string))
    })?;

    let q = "INSERT INTO users (username, password_hash, email, phone_number, phone_number_verified) VALUES (?, ?, ?, ?, ?)";

    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default(); 
    let password_hash = argon2.hash_password(input.password.as_bytes(), &salt)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error hashing password: {}", e)))?;

    let new_user = CreateUser {
        username: input.username,
        password: password_hash.to_string(),
        email: input.email,
        phone_number: input.phone_number,
        phone_number_verified: false,
    };

	let user_id = sqlx::query(q)
        .bind(new_user.username)
        .bind(new_user.password)
        .bind(new_user.email)
        .bind(new_user.phone_number)
        .bind(new_user.phone_number_verified)
		.execute(&pool)
		.await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error creating user: {}", e)))?;

    let user_id = user_id.last_insert_id() as i32;

    let user = fetch_user(&user_id, &pool).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn users_update(
    Path(id): Path<i32>,
    Extension(pool): Extension<MySqlPool>,
    Json(updates): Json<UpdateUser>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut query_string = "UPDATE users SET ".to_string();
    let mut params = Vec::new();

    // Use a helper function for query string building
    users_update_query_builder(&mut query_string, &mut params, &updates);
    
    // Remove trailing comma and space if any fields were updated
    if !params.is_empty() {
        query_string.truncate(query_string.len() - 2);
    }
    query_string.push_str(&format!(" WHERE id = {}", id));

    // Execute the query
    sqlx::query(&query_string)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update todos: {} {}", e, query_string)))?;

	// Fetch the updated todo
	let user = fetch_user(&id, &pool).await?;	

	Ok((StatusCode::OK, Json(user)))
}

// Find_user helper function
pub async fn fetch_user(id: &i32, pool: &MySqlPool) -> Result<User, (StatusCode, String)> {
	let q = &format!("SELECT * FROM users WHERE id = {}", id);

	let user = sqlx::query_as::<_, User>(q)
		.fetch_one(pool)
		.await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch user from database: {}", e)))?;

	Ok(user)
}

// Helper function for users_update.
pub fn users_update_query_builder(
    query: &mut String, 
    params: &mut Vec<String>, 
    updates: &UpdateUser
) {
    // Go through all fields provided in the JSON request body. <UpdateUser>.
    // If there is a value for it, add that value to the query string that sqlx will execute.
    for (field, item) in updates.clone().into_iter() {
        match item {
            FieldValue::Username(val) => {
                if let Some(username) = val {
                    query.push_str(&format!("{} = '{}', ", field, username));
                    params.push(field.to_string());
                }
            },
            FieldValue::Password(val) => {
                if let Some(password) = val {
                    // Hash the password before storing
                    let salt = SaltString::generate(&mut rand::thread_rng());
                    let password_hash = Argon2::default()
                        .hash_password(password.as_bytes(), &salt).unwrap();
                    query.push_str(&format!("password_hash = '{}', ", password_hash.to_string()));
                    query.push_str(&format!("{} = '{}', ", field, password));
                    params.push(field.to_string());
                }
            },
            FieldValue::Email(val) => {
                if let Some(value) = val {
                    query.push_str(&format!("{} = '{}', ", field, value));
                    params.push(field.to_string());
                }
            },
            FieldValue::PhoneNumber(val) => {
                if let Some(value) = val {
                    query.push_str(&format!("{} = '{}', ", field, value));
                    params.push(field.to_string());
                }
            },
            FieldValue::PhoneNumberVerified(val) => {
                if let Some(value) = val {
                    query.push_str(&format!("{} = '{}', ", field, value));
                    params.push(field.to_string());
                }
            }
        }
    }
}

pub async fn users_delete(
	Extension(pool): Extension<MySqlPool>,
	Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
	let q = "DELETE FROM users WHERE id = ?";

	sqlx::query(q)
		.bind(id)
		.execute(&pool)
		.await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error deleting user from database: {}", e)))?;

	Ok((StatusCode::OK, "User deleted successfully".to_string()))
}
