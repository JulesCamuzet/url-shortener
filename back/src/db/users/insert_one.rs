use sqlx::{Error, PgPool};
use sqlx::prelude::FromRow;

use crate::models::user::CreateUserDb;

#[derive(FromRow)]
pub struct Output {
    pub id: i32
}

pub async fn insert_one_user(CreateUserDb {
    email,
    password,
    verification_token
}: CreateUserDb, pool: &PgPool) -> Result<Output, Error> {
    let query = sqlx::query_as::<_, Output>(
        "INSERT INTO users (
            email,
            password,
            verification_token
        ) VALUES ($1, $2, $3) RETURNING id"
    ).bind(email.as_str())
    .bind(password)
    .bind(verification_token);

    let result = query.fetch_one(pool).await;

    match result {
        Ok(output) => Ok(output),
        Err(e) => {
            eprintln!("Error while inserting a user. Email : {}. Error: {}", email, e);
            Err(e)
        }
    }
}
