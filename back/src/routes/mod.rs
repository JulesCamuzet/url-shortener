use axum::{Router, routing::post};

use crate::{
    handlers::users::{register::handle_register, login::handle_login},
    AppState
};

pub fn get_router_with_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handle_register))
        .route("/login", post(handle_login))
}
