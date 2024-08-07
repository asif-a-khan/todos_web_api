use axum::{routing::get, Router};

use crate::controllers::access_tokens_controller::{
    access_tokens_create, 
    access_tokens_delete, 
    access_tokens_find, 
    access_tokens_index, 
    access_tokens_update
};

// Create user routes
pub fn routes() -> Router {
    Router::new()
        .route(
            "/api/access_tokens",
            get(access_tokens_index)
            .post(access_tokens_create)
        )
        .route(
            "/api/access_tokens/:id",
            get(access_tokens_find)
            .patch(access_tokens_update)
            .delete(access_tokens_delete)
        )
}