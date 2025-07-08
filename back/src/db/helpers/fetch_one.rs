use sqlx::{postgres::PgRow, PgPool, Error};

pub async fn fetch_one_from_db(sql: &str, params: Vec<&str>, pool: &PgPool) -> Result<PgRow, Error> {
    let result = sqlx::query(sql)
        .bind(params)
        .fetch_one(pool)
        .await;

    return result;
}
