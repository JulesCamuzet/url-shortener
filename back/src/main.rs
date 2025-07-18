use axum::Router;
use dotenv::dotenv;
use sqlx::PgPool;
use std::{fs};

mod db;
mod routes;
mod handlers;
mod models;
mod errors;
mod modules;
mod helpers;
mod middlewares;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub private_key: String
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();

    let postres_connection_string = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be provided.");

    println!("{}", postres_connection_string);

    println!("Creating database connection...");

    let pool = sqlx::postgres::PgPool::connect(postres_connection_string.as_str())
        .await
        .expect("Error while creating postgres connection.");

    println!("Database connected.");

    let init_db = std::env::var("INIT_DB").expect("INIT_DB must be provided.").as_str() == "1";

    if init_db {
        println!("Initializing database...");
        let sql_file = fs::read_to_string("./db/init.sql").expect("Error while reading sql init file.");
        let splitted_sql_file = sql_file.split(";");
        for command in splitted_sql_file {
            sqlx::query(command).execute(&pool)
                .await
                .expect(
                    format!("Error while running init sql file. Command : {}", command).as_str()
                );
        }
        println!("Database initialized.")
    }

    let private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be provided.");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    
    let app = Router::new()
        .merge(routes::get_router_with_routes())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(AppState { pool, private_key });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind listener.");

    println!("Starting the server...");

    axum::serve(listener, app).await.expect("Error while serve.");
}
