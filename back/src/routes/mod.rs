use axum::{routing::{get, post}, Router};

use crate::{
    handlers::{
        urls::{create::handle_create_url, redirection::handle_redirection},
        users::{login::handle_login, logout::handle_logout, register::handle_register}
    },
    AppState
};

pub fn get_router_with_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handle_register))
        .route("/login", post(handle_login))
        .route("/logout", post(handle_logout))
        .route("/urls", post(handle_create_url))
        .route("/{url}", get(handle_redirection))
}
