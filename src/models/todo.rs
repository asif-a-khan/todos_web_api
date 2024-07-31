use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::{ValidationError, ValidationErrors};

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Todo {
	pub id: i32,
    pub user_id: i32,
	pub description: String,
	pub done: bool
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct CreateTodo {
    pub user_id: i32,
	pub description: String,
	pub done: bool
}

#[derive(FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTodo {
    pub description: Option<String>,
    pub done: Option<bool>
}

#[derive(Debug)]
pub enum FieldValue {
	Description(Option<String>),
	Done(Option<bool>),
}

impl IntoIterator for UpdateTodo {
	type Item = (&'static str, FieldValue);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		vec![
			("description", FieldValue::Description(self.description)),
			("done", FieldValue::Done(self.done)),
		].into_iter()
	}
}

impl validator::Validate for CreateTodo {
	fn validate(&self) -> Result<(), ValidationErrors> {
		let mut errors = ValidationErrors::new();

		if self.user_id <= 0 {
			errors.add(
				"user_id",
				ValidationError::new(
						"User ID cannot be empty"
					)
					.with_message(Cow::Borrowed("User ID cannot be empty.")
				)
			);
		}

		if self.description.is_empty() {
			errors.add(
				"description",
				ValidationError::new(
						"Description cannot be empty"
					)
					.with_message(Cow::Borrowed("Description cannot be empty.")
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