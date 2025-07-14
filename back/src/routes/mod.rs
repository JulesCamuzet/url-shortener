use axum::{Router, routing::post};

use crate::{
    AppState,
    handlers::users::{login::handle_login, logout::handle_logout, register::handle_register},
};

pub fn get_router_with_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handle_register))
        .route("/login", post(handle_login))
        .route("/logout", post(handle_logout))
}
