use std::borrow::Cow;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::{ValidationError, ValidationErrors};


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: i32,
    pub api_key: String,
    pub client_name: String,
    pub contact_email: String,
    pub is_active: bool,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiKey {
    pub client_name: String,
    pub contact_email: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateApiKey {
    pub api_key: Option<String>,
    pub client_name: Option<String>,
    pub contact_email: Option<String>,
    pub is_active: Option<bool>
}

#[derive(Debug)]
pub enum FieldValue {
    ApiKey(Option<String>),
    ClientName(Option<String>),
    ContactEmail(Option<String>),
    IsActive(Option<bool>),
}

impl IntoIterator for UpdateApiKey {
    type Item = (&'static str, FieldValue);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("api_key", FieldValue::ApiKey(self.api_key)),
            ("client_name", FieldValue::ClientName(self.client_name)),
            ("contact_email", FieldValue::ContactEmail(self.contact_email)),
            ("is_active", FieldValue::IsActive(self.is_active))
        ].into_iter()
    }
}

impl validator::Validate for CreateApiKey {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.client_name.is_empty() {
            errors.add(
                "client_name",
                ValidationError::new(
                    "client name is required")
                    .with_message(Cow::Borrowed("Client Name is required.")
                )
            );
        }
        if self.contact_email.is_empty() {
            errors.add(
                "contact_email",
                ValidationError::new(
                    "client email is required")
                    .with_message(Cow::Borrowed("Client Email is required.")
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