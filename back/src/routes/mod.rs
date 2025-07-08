use axum::Router;

use crate::handlers::users::{self, register::handle_register};

pub fn add_routes(app: &Router, pool: &sqlx::PgPool) {
    handle_register(app, pool);
}
