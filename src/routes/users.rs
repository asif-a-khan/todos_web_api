use axum::{routing::{get, post}, Router};

use crate::controllers::{auth_controller::{login, logout}, users_controller::{
    users_create, 
    users_delete, 
    users_find, 
    users_index, 
    users_update
}};

// Create user routes
pub fn routes() -> Router {
    Router::new()
        .route(
            "/api/users",
            get(users_index)
            .post(users_create)
        )
        .route(
            "/api/users/:id",
            get(users_find)
            .patch(users_update)
            .delete(users_delete)
        )
        .nest(
            "/auth", 
            Router::new()
                .route("/login", post(login))
                // .route("/refresh", post(refresh)) 
                .route("/logout", post(logout)) 
        )
}