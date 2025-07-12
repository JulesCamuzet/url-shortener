use sqlx::{Error, PgPool};
use sqlx::prelude::FromRow;

pub struct InsertOneUserInput<'a> {
    pub email: String,
    pub password: String,
    pub verification_token: String,
    pub pool: &'a PgPool
}

#[derive(FromRow)]
pub struct Output {
    pub id: i32
}

pub async fn insert_one_user<'a>(InsertOneUserInput {
    email,
    password,
    verification_token,
    pool
}: InsertOneUserInput<'a>) -> Result<Output, Error> {
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
