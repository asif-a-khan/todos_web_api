use axum::{routing::get, Router};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::Error;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database URL Not Found");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url) 
        .await?;

    let row: (String,) = sqlx::query_as("SELECT VERSION()")
        .fetch_one(&pool)
        .await?;
    
    println!("Connected to MySQL server version: {}", row.0);

    let app = Router::new()
    .route("/", get(|| async { "Hello" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}