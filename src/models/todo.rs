use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Todo {
	pub id: i32,
	pub description: String,
	pub done: bool,
    pub user_id: i32
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct CreateTodo {
	pub description: String,
	pub done: bool,
    pub user_id: i32
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub description: Option<String>,
    pub done: Option<bool>
}