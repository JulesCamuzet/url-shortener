use serde::Serialize;
use sqlx::{prelude::FromRow, Error, PgPool};

pub struct InsertOneUrlInput<'a> {
    pub short_value: String,
    pub original_value: String,
    pub user_id: Option<i32>,
    pub pool: &'a PgPool
}

#[derive(Serialize, FromRow)]
pub struct Output {
    pub id: i32,
    pub original_value: String,
    pub short_value: String
}

pub async fn insert_one_url<'a>(InsertOneUrlInput {
        short_value,
        original_value,
        user_id,
        pool
    }: InsertOneUrlInput<'a>
) -> Result<Output, Error> {
    let query = sqlx::query_as::<_, Output>("INSERT INTO url (
            short_value,
            original_value,
            user_id
        ) VALUES ($1, $2, $3) RETURNING id, original_value, short_value"
    ).bind(short_value.as_str())
    .bind(original_value.as_str())
    .bind(user_id);
    
    let result = query.fetch_one(pool).await;
    
    match result {
        Err(e) => {
            eprintln!("Error while inserting a url.
                short_value : {}
                original_value : {}
                user_id : {:#?}
                Error: {}",
                short_value, original_value, user_id, e);
            Err(e)
        },
        Ok(result) => Ok(result)
    }
}
