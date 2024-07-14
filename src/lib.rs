use std::env;

use axum::{
    extract::Path, 
    http::StatusCode, 
    response::IntoResponse, 
    Json
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct CreateTodo {
    pub description: String,
    pub done: bool
}

pub async fn todos_index() -> impl IntoResponse  {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database URL Not Found");
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.unwrap();

    let q = "SELECT * FROM todos";

    let todos = sqlx::query_as::<_, Todo>(q)
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to get todos: {}", e);
            let test = Todo {
                id: 0,
                description: "Error".to_string(),
                done: false
            };
            vec![test]
        });

    (StatusCode::OK, Json(todos))
}

pub async fn todos_find(Path(id): Path<i32>) -> impl IntoResponse  {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database URL Not Found");
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.unwrap();

    let q = format!("SELECT * FROM todos WHERE id = {}", id).to_string();

    let todo = sqlx::query_as::<_, Todo>(&q)
        .fetch_one(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(todo))
}

pub async fn todos_create(Json(input): Json<CreateTodo>) -> impl IntoResponse  {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database URL Not Found");
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.unwrap();

    let q = "INSERT INTO todos (description, done) VALUES (?, ?)";

    let todo = sqlx::query(q)
        .bind(input.description)
        .bind(input.done)
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_id();

    (StatusCode::OK, Json(todo))
}

pub async fn todos_update(Path(id): Path<i32>, Json(input): Json<CreateTodo>) -> impl IntoResponse  {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database URL Not Found");
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.unwrap();

    let q = "UPDATE todos SET description = ?, done = ? WHERE id = ?";

    let todo = sqlx::query(q)
        .bind(input.description)
        .bind(input.done)
        .bind(id)
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_id();

    (StatusCode::OK, Json(todo))
}

pub async fn todos_delete(Path(id): Path<i32>) -> impl IntoResponse  {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database URL Not Found");
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.unwrap();

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

