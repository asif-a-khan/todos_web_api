use argon2::{
    password_hash::{
        // PasswordHash, 
        PasswordHasher, 
        // PasswordVerifier, 
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


use chrono::{Duration, Local};
// use jsonwebtoken::{encode, EncodingKey, Header};
// use validator::{
//     // Validate, 
//     // ValidationErrors
// };
use rand::distributions::Alphanumeric;

use rand::{thread_rng, Rng};
use validator::{Validate, ValidationErrors};

use crate::models::user::FieldValue;

use super::super::models::user::{
    User,
    CreateUser,
    CreateUserFromInput,
    UpdateUser
};

use sqlx::MySqlPool;

pub async fn users_index(
    Extension(pool): Extension<MySqlPool>
) -> Result<impl IntoResponse, (StatusCode, String)> {
	let q = "SELECT * FROM users";

	let users = sqlx::query_as::<_, User>(q)
		.fetch_all(&pool)
		.await;

    if let Err(e) = users {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch users from database: {}", e)))
    }

	Ok((StatusCode::OK, Json(users.unwrap())))
}

pub async fn users_find(
	Extension(pool): Extension<MySqlPool>, 
	Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
	let q = format!("SELECT * FROM users WHERE id = {}", id).to_string();

	let user = sqlx::query_as::<_, User>(&q)
		.fetch_one(&pool)
		.await;

    if let Err(e) = user {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch user from database: {}", e)))
    }

	Ok((StatusCode::OK, Json(user.unwrap())))
}

pub async fn users_create(
    Extension(pool): Extension<MySqlPool>, 
    Json(input): Json<CreateUserFromInput>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let validation = input.validate();

    if let Err(e) = validation {
        let error_string = handle_validation_errors(e);
        return Err((StatusCode::BAD_REQUEST, format!("Validation failed: {}", error_string)))
    }

    let q = "INSERT INTO users (username, password_hash, email, phone_number, phone_number_verified, refresh_token, refresh_token_expiry) VALUES (?, ?, ?, ?, ?, ?, ?)";

    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default(); 
    let password_hash = argon2.hash_password(input.password.as_bytes(), &salt);
    
    if let Err(e) = password_hash {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error hashing password: {}", e),
        ));
    }

    let refresh_token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let new_user = CreateUser {
        username: input.username,
        password: password_hash.unwrap().to_string(),
        email: input.email,
        phone_number: input.phone_number,
        phone_number_verified: false,
        refresh_token: Some(refresh_token),
        refresh_token_expiry: Some(Local::now() + Duration::hours(2)),
    };

	let user_id = sqlx::query(q)
        .bind(new_user.username)
        .bind(new_user.password)
        .bind(new_user.email)
        .bind(new_user.phone_number)
        .bind(new_user.phone_number_verified)
        .bind(new_user.refresh_token)
        .bind(new_user.refresh_token_expiry)
		.execute(&pool)
		.await;

    if let Err(e) = user_id {
        println!("Error creating user: {}", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error creating user: {}", e),
        ));
    }

    let user_id = user_id.unwrap().last_insert_id();
    let q2: &str = "SELECT * FROM users WHERE id = ?";

    let user = sqlx::query_as::<_, User>(q2)
        .bind(user_id)
		.fetch_one(&pool)
		.await
        .unwrap();

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
    let update_query = sqlx::query(&query_string)
        .execute(&pool)
        .await;

	if let Err(e) = update_query {
		return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update todos: {} {}", e, query_string)))
	}

	// Fetch the updated todo
	let user = fetch_user(&id, &pool).await;	
	if let Err(e) = user {
		return Err(e)
	}

	Ok((StatusCode::OK, Json(user.unwrap())))
}

// Find_user helper function
pub async fn fetch_user(id: &i32, pool: &MySqlPool) -> Result<User, (StatusCode, String)> {
	let q = &format!("SELECT * FROM users WHERE id = {}", id).to_string();

	let user = sqlx::query_as::<_, User>(q)
		.fetch_one(pool)
		.await;

	if let Err(e) = user {
		return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch user from database: {}", e)))
	}

	Ok(user.unwrap())
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
            },
            FieldValue::RefreshToken(val) => {
                if let Some(value) = val {
                    query.push_str(&format!("{} = '{}', ", field, value));
                    params.push(field.to_string());
                }
            },
            FieldValue::RefreshTokenExpiry(val) => {
                if let Some(value) = val {
                    query.push_str(&format!("{} = '{}', ", field, value));
                    params.push(field.to_string());
                }
            },
        }
    }
}

pub async fn users_delete(
	Extension(pool): Extension<MySqlPool>,
	Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
	let q = "DELETE FROM users WHERE id = ?";

	let delete = sqlx::query(q)
		.bind(id)
		.execute(&pool)
		.await;

    if let Err(e) = delete {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error deleting user from database: {}", e)))
    }

	Ok((StatusCode::OK, "User deleted successfully".to_string()))
}

// Helper function to format validation errors
fn handle_validation_errors(errors: ValidationErrors) -> String {
    println!("Validation errors: {:#?}", errors);
    let formatted_errors: Vec<String> = errors
        .field_errors()
        .into_iter()
        .map(|(field, errors)| {
            let error_messages: Vec<_> = errors
                .iter()
                .filter_map(|err| err.message.clone().map(|msg| msg.into_owned())) // Handle Optional message
                .collect();
            format!("{}: {}", field, error_messages.join(", "))
        })
        .collect();
    formatted_errors.join(", ")
}