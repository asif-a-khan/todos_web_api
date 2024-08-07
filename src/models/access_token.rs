use std::borrow::Cow;

use chrono::{
    DateTime,
    Local
};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::{ValidationError, ValidationErrors};

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct AccessToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expires_at: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccessToken {
    pub user_id: i32,
    pub token: String,
    pub expires_at: DateTime<Local>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccessTokenFromInput {
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateAccessToken {
    pub token: Option<String>,
    pub expires_at: Option<DateTime<Local>>
}

#[derive(Debug)]
pub enum FieldValue {
    Token(Option<String>),
    ExpiresAt(Option<DateTime<Local>>),
}

impl IntoIterator for UpdateAccessToken {
    type Item = (&'static str, FieldValue);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("token", FieldValue::Token(self.token)),
            ("expires_at", FieldValue::ExpiresAt(self.expires_at))
        ].into_iter()
    }
}

impl validator::Validate for CreateAccessTokenFromInput {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.user_id <= 0 {
            errors.add(
                "user_id",
                ValidationError::new(
                    "user_id must be greater than 0")
                    .with_message(Cow::Borrowed("User_id must be greater than 0.")
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


