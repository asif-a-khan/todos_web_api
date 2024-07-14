use std::env;

use axum::{http::StatusCode, response::IntoResponse, Json};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
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
                title: "Error".to_string(),
                done: false
            };
            vec![test]
        });

    (StatusCode::OK, Json(todos))
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

