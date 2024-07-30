use std::borrow::Cow;

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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>, // Allow password update (handle hashing separately)
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub phone_number_verified: Option<bool>,
    pub refresh_token: Option<String>,
    pub refresh_token_expiry: Option<chrono::DateTime<Local>>
}

#[derive(Debug)]
pub enum FieldValue {
    Username(Option<String>),
    Password(Option<String>),
    Email(Option<String>),
    PhoneNumber(Option<String>),
    PhoneNumberVerified(Option<bool>),
    RefreshToken(Option<String>),
    RefreshTokenExpiry(Option<chrono::DateTime<Local>>),

}

impl IntoIterator for UpdateUser {
    type Item = (&'static str, FieldValue); // Item is a tuple of (field_name, value)
    type IntoIter = std::vec::IntoIter<Self::Item>; // Use a Vec to hold the pairs

    fn into_iter(self) -> Self::IntoIter {
        vec![ 
            ("username", FieldValue::Username(self.username)),
            ("password", FieldValue::Password(self.password)),
            ("email", FieldValue::Email(self.email)),
            ("phone_number", FieldValue::PhoneNumber(self.phone_number)),
            ("phone_number_verified", FieldValue::PhoneNumberVerified(self.phone_number_verified)),
            ("refresh_token", FieldValue::RefreshToken(self.refresh_token)),
            ("refresh_token_expiry", FieldValue::RefreshTokenExpiry(self.refresh_token_expiry)),
        ].into_iter()
    }
}

impl validator::Validate for CreateUserFromInput {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if self.username.is_empty() {
            errors.add(
                "username", 
                ValidationError::new(
                    "Username cannot be empty")
                        .with_message(Cow::Borrowed("Username cannot be empty.")
                )
            );
        }

        if self.email.is_empty() {
            errors.add(
                "email",
                ValidationError::new(
                    "Email cannot be empty")
                        .with_message(Cow::Borrowed("Email cannot be empty.")
                )
            );
        }

        if self.password.is_empty() {
            errors.add(
                "password",
                ValidationError::new(
                    "Password cannot be empty")
                        .with_message(Cow::Borrowed("Password cannot be empty.")
                )
            );
        }

        if self.phone_number.is_none() {
            errors.add(
                "phone_number",
                ValidationError::new(
                    "Phone number cannot be empty")
                        .with_message(Cow::Borrowed("Phone number cannot be empty.")
                )
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}