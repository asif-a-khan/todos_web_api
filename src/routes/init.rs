use axum::{
    middleware, routing::get, Extension, Router
};
use tower_cookies::CookieManagerLayer;

use crate::routes::{users, todos};

use super::middlewares::main_response_mapper;

pub async fn run() -> Result<Router, Box<dyn std::error::Error>> {
    // Database Init
    let pool = crate::database::init::run().await?;
    // Web Server Routes Init
    let app = Router::new()
        .route("/api", get(|| async { "Hello" }))
        .merge(users::routes())
        .merge(todos::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .layer(Extension(pool));

    Ok(app)
}