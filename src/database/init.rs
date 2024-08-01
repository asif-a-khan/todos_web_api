use dotenv::dotenv;

use std::env;

use sqlx::{
    mysql::MySqlPoolOptions, 
    MySql, 
    Pool,
    Error
};

pub async fn run() -> Result<Pool<MySql>, Error> {
    dotenv().ok();
    // Database Init
    let db_url = env::var("DATABASE_URL").expect("Database URL Not Found");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    // Run Migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}