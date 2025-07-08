use axum::{
    Router
};
use dotenv::dotenv;
use std::fs;

mod db;
mod routes;
mod handlers;

use crate::{db::helpers::execute::execute_db, routes::add_routes};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();

    let app = Router::new();

    let postres_connection_string = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be provided.");

    let pool = sqlx::postgres::PgPool::connect(postres_connection_string.as_str())
        .await
        .expect("Error while creating postgres connection.");

    let init_db = std::env::var("INIT_DB").expect("INIT_DB must be provided.").as_str() == "1";

    if init_db {
        let sql_file = fs::read_to_string("./db/init.sql").expect("Error while reading sql init file.");
        let splitted_sql_file = sql_file.split(";");
        for command in splitted_sql_file {
            execute_db(command, &pool)
                .await
                .expect(
                    format!("Error while running init sql file. Command : {}", command).as_str()
                );
        }
    }

    add_routes(&app, &pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind listener.");

    axum::serve(listener, app).await.unwrap();
}
