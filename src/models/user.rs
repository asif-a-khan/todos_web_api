use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::{
    ValidationErrors,
    ValidationError
};

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub phone_number_verified: bool,
    pub refresh_token: Option<String>,
    pub refresh_token_expiry: Option<chrono::DateTime<Local>>
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub phone_number_verified: bool,
    pub refresh_token: Option<String>,
    pub refresh_token_expiry: Option<chrono::DateTime<Local>>
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct CreateUserFromInput {
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone_number: Option<String>
}

impl validator::Validate for CreateUser {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if self.username.is_empty() {
            errors.add("username", ValidationError::new("Username cannot be empty"));
        }

        if self.email.is_empty() {
            errors.add("email", ValidationError::new("Email cannot be empty"));
        }

        if self.password.is_empty() {
            errors.add("password", ValidationError::new("Password cannot be empty"));
        }

        if self.phone_number.is_none() {
            errors.add("phone_number", ValidationError::new("Phone number cannot be empty"));
        }

        if self.refresh_token.is_none() {
            errors.add("refresh_token", ValidationError::new("Refresh token cannot be empty"));
        }

        if self.refresh_token_expiry.is_none() {
            errors.add("refresh_token_expiry", ValidationError::new("Refresh token expiry cannot be empty"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}