use axum::{
	extract::Path, 
	http::StatusCode, 
	response::IntoResponse, 
	Json,
	Extension
};

use validator::Validate;

use crate::{
	models::todo::{
		Todo, 
		CreateTodo,
		UpdateTodo,
		FieldValue
	},
	utils::input_validation::handle_validation_errors
};

use sqlx::MySqlPool;

pub async fn todos_index(
	Extension(pool): Extension<MySqlPool>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
	let q = "SELECT * FROM todos";

	let todos = sqlx::query_as::<_, Todo>(q)
		.fetch_all(&pool)
		.await;

	if let Err(e) = todos {
		return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error fetching todos from databasse: {}", e)))
	}

	Ok((StatusCode::OK, Json(todos.unwrap())))
}

pub async fn todos_find(
	Extension(pool): Extension<MySqlPool>, 
	Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
	let todo = fetch_todo(&id, &pool).await;

	if let Err(e) = todo {
		return Err(e)
	}

	Ok((StatusCode::OK, Json(todo.unwrap())))
}

pub async fn todos_create(
	Extension(pool): Extension<MySqlPool>,
	Json(input): Json<CreateTodo>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
	// Validation
	let validation = input.validate();

	if let Err(e) = validation {
		let error_string = handle_validation_errors(e);
		return Err((StatusCode::BAD_REQUEST, format!("Validation failed: {}", error_string)))
	}

	let q = "INSERT INTO todos (description, done, user_id) VALUES (?, ?, ?)";

	let todo_id = sqlx::query(q)
		.bind(input.description)
		.bind(input.done)
		.bind(input.user_id)
		.execute(&pool)
		.await;

	if let Err(e) = todo_id {
		return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to insert todo into database: {}", e)))
	}

	let id = todo_id.unwrap().last_insert_id() as i32;
	let todo = fetch_todo(&id, &pool).await;

	if let Err(e) = todo {
		return Err(e)
	}

	Ok((StatusCode::OK, Json(todo.unwrap())))
}

pub async fn todos_update(
    Path(id): Path<i32>,
    Extension(pool): Extension<MySqlPool>,
    Json(updates): Json<UpdateTodo>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
    let mut query_string = "UPDATE todos SET ".to_string();
    let mut params = Vec::new();

    // Use a helper function for query string building
    build_update_query_string(&mut query_string, &mut params, &updates);
    
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
		return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update todo: {}", e)))
	}

	// Fetch the updated todo
	let todo = fetch_todo(&id, &pool).await;	
	if let Err(e) = todo {
		return Err(e)
	}

	Ok((StatusCode::OK, Json(todo.unwrap())))
	// Ok((StatusCode::OK, "test".to_string()))
}

// todos_update helper function
pub fn build_update_query_string(query: &mut String, params: &mut Vec<String>, updates: &UpdateTodo) {
	for (field, item) in updates.clone().into_iter() {
		match item {
			FieldValue::Description(value) => {
				if let Some(val) = value {
					query.push_str(&format!("{} = '{}', ", field, val));
					params.push(val);
				}
			},
			FieldValue::Done(value) => {
				if let Some(val) = value {
					query.push_str(&format!("{} = '{}', ", field, val as i32));
					params.push(val.to_string());
				}
			}
		}
	}
}

pub async fn fetch_todo(id: &i32, pool: &MySqlPool) -> Result<Todo, (StatusCode, String)> {
	let q = &format!("SELECT * FROM todos WHERE id = {}", id).to_string();

	let todo = sqlx::query_as::<_, Todo>(q)
		.fetch_one(pool)
		.await;

	if let Err(e) = todo {
		return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch todo from database: {}", e)))
	}

	Ok(todo.unwrap())
}

pub async fn todos_delete(
	Extension(pool): Extension<MySqlPool>,
	Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)> {
	let q = "DELETE FROM todos WHERE id = ?";

	let delete = sqlx::query(q)
		.bind(id)
		.execute(&pool)
		.await;

	if let Err(e) = delete {
		return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete todo from database: {}", e)))
	}

	Ok((StatusCode::OK, "Todo deleted".to_string()))
}

pub async fn fetch_user_todo(user_id: &i32, todo_id: &i32, pool: &MySqlPool) -> Result<Todo, (StatusCode, String)> {
	let q = &format!("SELECT * FROM todos WHERE user_id = {} AND id = {}", user_id, todo_id).to_string();

	let todo = sqlx::query_as::<_, Todo>(q)
		.fetch_one(pool)
		.await;

	if let Err(e) = todo {
		return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch todo from database: {}", e)))
	}

	Ok(todo.unwrap())
}

