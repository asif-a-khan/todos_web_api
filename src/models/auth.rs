use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::{ValidationError, ValidationErrors};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,  
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LogoutUser {
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct RefreshUser {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct ResponseMessage {
    pub message: String,
}

impl validator::Validate for LoginUser {
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

        if self.password.is_empty() {
            errors.add(
                "password",
                ValidationError::new(
                    "Password cannot be empty")
                        .with_message(Cow::Borrowed("Password cannot be empty.")
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

impl validator::Validate for LogoutUser {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if self.user_id.is_empty() {
            errors.add(
                "user id is empty", 
                ValidationError::new(
                    "User ID cannot be empty")
                        .with_message(Cow::Borrowed("User ID cannot be empty.")
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