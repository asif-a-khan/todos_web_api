use axum::{
	extract::Path, 
	http::StatusCode, 
	response::IntoResponse, 
	Json,
	Extension
};

use super::super::models::todo::{
	Todo, 
	CreateTodo,
	UpdateTodo
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

	match todo {
		Ok(_) => Ok((StatusCode::OK, Json(todo.unwrap()))),
		Err(e) => Err(e)
	}

}

pub async fn todos_create(
	Extension(pool): Extension<MySqlPool>,
	Json(input): Json<CreateTodo>
) -> Result<impl IntoResponse, (StatusCode, String)>  {
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

	match todo {
		Ok(_) => Ok((StatusCode::OK, Json(todo.unwrap()))),
		Err(e) => Err(e)
	}
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

// Find_todo helper function
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

// todos_update helper function
pub fn build_update_query_string(query: &mut String, params: &mut Vec<String>, updates: &UpdateTodo) {
    if let Some(description) = &updates.description {
        query.push_str(&format!("username = {}, ", description));
        params.push(description.to_string());
    }
    if let Some(done) = &updates.done {
        query.push_str(&format!("email = {}, ", done));
        params.push(done.to_string());
    }
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

	Ok(StatusCode::OK)
}

// Don't know why this won't work
// pub async fn todos_index() -> Result<(StatusCode, Json<Vec<Todo>>), Box<dyn Error>> {
//     dotenv().ok();
//     let db_url = env::var("DATABASE_URL").expect("Database URL Not Found");
//     let pool = sqlx::mysql::MySqlPoolOptions::new()
//         .max_connections(5)
//         .connect(&db_url)
//         .await?;

//     let q = "SELECT * FROM todos";

//     let todos = sqlx::query_as::<_, Todo>(q)
//         .fetch_all(&pool)
//         .await?;
	
//     Ok((StatusCode::OK, Json(todos)))
// }

