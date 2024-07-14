use axum::{
    routing::get, 
    Router
};

use sqlx::{
    mysql::MySqlPoolOptions,
    Error
};

use std::env;
use dotenv::dotenv;

use todos_web_api::todos_index;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database URL Not Found");
    // Database Init
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.unwrap();

    // Run Migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    // Web Server Routes Init
    let app = Router::new()
    .route("/", get(|| async { "Hello" }))
    .route("/todos", get(todos_index));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}