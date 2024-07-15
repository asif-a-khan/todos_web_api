use axum::{
	extract::Path, 
	http::StatusCode, 
	response::IntoResponse, 
	Json,
	Extension
};

use jsonwebtoken::{encode, EncodingKey, Header};
use validator::{Validate, ValidationErrors};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use super::super::models::user::User;
use sqlx::MySqlPool;

pub async fn register_user(
    Json(payload): Json<User>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 1. Input Validation:
    if let Err(errors) = payload.validate() {
        let error_message = handle_validation_errors(errors); // Helper function for formatting errors
        return Err((StatusCode::BAD_REQUEST, error_message));
    }

    // 2. Password Hashing:
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &[])
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing failed".to_string()))?;

    // 3. Generate a refresh token:
    let refresh_token: String = thread_rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect();

    // 4. Create User Struct:
    let user = User {
        id: 0, 
        username: payload.username,
        password_hash: password_hash.to_string(),
        email: payload.email,
        phone_number: payload.phone_number,
        phone_number_verified: false,
        refresh_token: Some(refresh_token),
        refresh_token_expiry: Some(Utc::now() + Duration::days(30)), // Set expiration date
    };

    // 5. Insert User into Database:
    let query_result = sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash, email, phone_number, refresh_token, refresh_token_expiry)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        user.username,
        user.password_hash,
        user.email,
        user.phone_number,
        user.phone_number_verified,
        user.refresh_token,
        user.refresh_token_expiry
    )
    .execute(&pool)
    .await;

    // 6. Handle Database Errors:
    match query_result {
        Ok(_) => Ok((StatusCode::CREATED, "User registered successfully".to_string())),
        Err(sqlx::Error::Database(err)) => {
            if err.code() == Some("23000".to_string()) { // Duplicate entry
                Err((StatusCode::CONFLICT, "Username or email already taken".to_string()))
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", err)))
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Unexpected error: {}", e))),
    }
}

// Helper function to format validation errors
fn handle_validation_errors(errors: ValidationErrors) -> String {
    let formatted_errors: Vec<String> = errors
        .field_errors()
        .into_iter()
        .map(|(field, error)| {
            format!("{}: {:?}", field, error.code)
        })
        .collect();
    formatted_errors.join(", ")
}