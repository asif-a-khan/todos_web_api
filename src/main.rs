use std::env;

use axum::{
    routing::get, 
    Extension, 
    Router
};

use sqlx::{
    mysql::MySqlPoolOptions,
    Error
};

use dotenv::dotenv;

use todos_web_api::controllers::todos_controller::{
    todos_create, 
    todos_delete, 
    todos_find, 
    todos_index, 
    todos_update
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    // Database Init
    let db_url = env::var("DATABASE_URL").expect("Database URL Not Found");
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
    .route("/todos", 
        get(todos_index)
        .post(todos_create)
    )
    .route("/todos/:id", 
        get(todos_find)
        .patch(todos_update)
        .delete(todos_delete)
    )
    .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}