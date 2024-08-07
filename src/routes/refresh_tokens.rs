use axum::{routing::get, Router};

use crate::controllers::refresh_tokens_controller::{
    refresh_tokens_create, 
    refresh_tokens_delete, 
    refresh_tokens_find, 
    refresh_tokens_index, 
    refresh_tokens_update
};

// Create user routes
pub fn routes() -> Router {
    Router::new()
        .route(
            "/api/refresh_tokens",
            get(refresh_tokens_index)
            .post(refresh_tokens_create)
        )
        .route(
            "/api/refresh_tokens/:id",
            get(refresh_tokens_find)
            .patch(refresh_tokens_update)
            .delete(refresh_tokens_delete)
        )
}