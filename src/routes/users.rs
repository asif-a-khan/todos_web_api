use axum::{middleware, routing::{get, post}, Router};

use crate::controllers::{auth_controller::{login, logout, refresh}, users_controller::{
    users_create, 
    users_delete, 
    users_find, 
    users_index, 
    users_update
}};

use super::middlewares::{api_key_auth, check_token_auth};

// Create user routes
pub fn routes() -> Router {
    Router::new()
        .nest(
        "/api/users",
        Router::new()
            .route("/", get(users_index).post(users_create))
            .route("/:id", get(users_find).patch(users_update).delete(users_delete))
            .route_layer(middleware::from_fn(check_token_auth))
        )
        .nest(
        "/api/external/users",
        Router::new()
            .route("/", get(users_index))
            .route("/:id", get(users_find))
            .route_layer(middleware::from_fn(api_key_auth))
        )
        .nest(
            "/api/auth", 
            Router::new()
                .route("/login", post(login))
                .route("/refresh", post(refresh)) 
                .route("/logout", post(logout)) 
        )
}