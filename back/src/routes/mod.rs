use axum::{Router, routing::post};

use crate::{handlers::users::register::handle_register, AppState};

pub fn get_router_with_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handle_register))
}
