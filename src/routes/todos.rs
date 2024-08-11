use axum::{routing::get, Router};

use crate::controllers::todos_controller::{
    todos_create, 
    todos_delete, 
    todos_find, 
    todos_index, 
    todos_update
};

// Create todo routes
pub fn routes() -> Router {
    Router::new()
        .route(
            "/api/todos", 
            get(todos_index)
            .post(todos_create)
        )
        .route(
            "/api/todos/:id", 
            get(todos_find)
            .patch(todos_update)
            .delete(todos_delete)
        )
}