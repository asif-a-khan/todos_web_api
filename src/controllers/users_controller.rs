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
use validator::{
    // Validate, 
    ValidationErrors
};
use rand::distributions::Alphanumeric;

use rand::{thread_rng, Rng};
use serde::Serialize;

use super::super::models::user::{
    User,
    CreateUser,
    CreateUserFromInput
};

use sqlx::MySqlPool;

#[derive(Serialize)]
pub struct RegisterUserResponse {
    message: String,
}

pub async fn users_index(
    Extension(pool): Extension<MySqlPool>
) -> impl IntoResponse {
	let q = "SELECT * FROM users";

	let users = sqlx::query_as::<_, User>(q)
		.fetch_all(&pool)
		.await
		.unwrap();

	(StatusCode::OK, Json(users))
}

pub async fn users_find(
	Extension(pool): Extension<MySqlPool>, 
	Path(id): Path<i32>
) -> impl IntoResponse  {
	let q = format!("SELECT * FROM users WHERE id = {}", id).to_string();

	let todo = sqlx::query_as::<_, User>(&q)
		.fetch_one(&pool)
		.await
		.unwrap();

	(StatusCode::OK, Json(todo))
}

pub async fn users_create(
    Extension(pool): Extension<MySqlPool>, 
    Json(input): Json<CreateUserFromInput>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let q = "INSERT INTO users (username, password_hash, email, phone_number, phone_number_verified, refresh_token, refresh_token_expiry) VALUES (?, ?, ?, ?, ?, ?, ?)";

    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default(); 
    let password_hash = argon2
        .hash_password(input.password.as_bytes(), &salt)
        .expect("Failed to hash password");

    let refresh_token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let new_user = CreateUser {
        username: input.username,
        password: password_hash.to_string(),
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
		.await
		.unwrap()
        .last_insert_id();

    let q2: &str = "SELECT * FROM users WHERE id = ?";

    let user = sqlx::query_as::<_, User>(q2)
        .bind(user_id)
		.fetch_one(&pool)
		.await
        .unwrap();

    Ok((StatusCode::CREATED, Json(user)))

}

pub async fn users_update () -> impl IntoResponse{
    StatusCode::OK
}

pub async fn users_delete(
	Extension(pool): Extension<MySqlPool>,
	Path(id): Path<i32>
) -> impl IntoResponse  {
	let q = "DELETE FROM users WHERE id = ?";

	let _delete = sqlx::query(q)
		.bind(id)
		.execute(&pool)
		.await
		.unwrap();

	StatusCode::OK
}

// Helper function to format validation errors
fn handle_validation_errors(errors: ValidationErrors) -> String {
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