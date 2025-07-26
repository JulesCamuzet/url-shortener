use serde::Serialize;
use sqlx::{prelude::FromRow, Error, PgPool};

use crate::models::url::CreateUrlDb;

#[derive(Serialize, FromRow)]
pub struct Output {
    pub id: i32,
    pub short_value: String
}

pub async fn insert_one_url<'a>(
    CreateUrlDb {
        short_value,
        user_id,
        name
    }: CreateUrlDb,
    pool: &PgPool
) -> Result<Output, Error> {
    let query = sqlx::query_as::<_, Output>("INSERT INTO url (
            name,
            short_value,
            user_id
        ) VALUES ($1, $2, $3) RETURNING id, short_value"
    ).bind(short_value.as_str())
    .bind(user_id);
    
    let result = query.fetch_one(pool).await;
    
    match result {
        Err(e) => {
            eprintln!("Error while inserting a url.
                short_value : {}
                name : {}
                user_id : {:#?}
                Error: {}",
                short_value, name, user_id, e);
            Err(e)
        },
        Ok(result) => Ok(result)
    }
}
