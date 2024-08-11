use axum::{routing::get, Router};

use crate::controllers::api_keys_controller::{
    api_keys_create, 
    api_keys_delete, 
    api_keys_find, 
    api_keys_index, 
    api_keys_update
};

// Create user routes
pub fn routes() -> Router {
    Router::new()
        .route(
            "/api/api_keys",
            get(api_keys_index)
            .post(api_keys_create)
        )
        .route(
            "/api/api_keys/:id",
            get(api_keys_find)
            .patch(api_keys_update)
            .delete(api_keys_delete)
        )
}