use axum::{
	extract::Path, 
	http::StatusCode, 
	response::IntoResponse, 
	Json,
	Extension
};

use super::super::models::todo::{
	Todo, 
	CreateTodo
};

use sqlx::MySqlPool;

pub async fn todos_index(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse  {
	let q = "SELECT * FROM todos";

	let todos = sqlx::query_as::<_, Todo>(q)
		.fetch_all(&pool)
		.await
		.unwrap_or_else(|e| {
			eprintln!("Failed to get todos: {}", e);
			let test = Todo {
				id: 0,
				description: "Error".to_string(),
				done: false,
				user_id: 1
			};
			vec![test]
		});

	(StatusCode::OK, Json(todos))
}

pub async fn todos_find(
	Extension(pool): Extension<MySqlPool>, 
	Path(id): Path<i32>
) -> impl IntoResponse  {
	let q = format!("SELECT * FROM todos WHERE id = {}", id).to_string();

	let todo = sqlx::query_as::<_, Todo>(&q)
		.fetch_one(&pool)
		.await
		.unwrap();

	(StatusCode::OK, Json(todo))
}

pub async fn todos_create(
	Extension(pool): Extension<MySqlPool>,
	Json(input): Json<CreateTodo>
) -> impl IntoResponse  {
	let q = "INSERT INTO todos (description, done, user_id) VALUES (?, ?, ?)";

	let todo = sqlx::query(q)
		.bind(input.description)
		.bind(input.done)
		.bind(input.user_id)
		.execute(&pool)
		.await
		.unwrap()
		.last_insert_id();

	(StatusCode::OK, Json(todo))
}

pub async fn todos_update(
	Extension(pool): Extension<MySqlPool>,
	Path(id): Path<i32>, Json(input): Json<CreateTodo>
) -> impl IntoResponse  {
	let q = "UPDATE todos SET description = ?, done = ?, user_id = ? WHERE id = ?";

	let todo = sqlx::query(q)
		.bind(input.description)
		.bind(input.done)
		.bind(input.user_id)
		.bind(id)
		.execute(&pool)
		.await
		.unwrap()
		.last_insert_id();

	(StatusCode::OK, Json(todo))
}

pub async fn todos_delete(
	Extension(pool): Extension<MySqlPool>,
	Path(id): Path<i32>
) -> impl IntoResponse  {
	let q = "DELETE FROM todos WHERE id = ?";

	let _delete = sqlx::query(q)
		.bind(id)
		.execute(&pool)
		.await
		.unwrap();

	StatusCode::OK
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

