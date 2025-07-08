use sqlx::{postgres::PgRow, Error, PgPool};

pub async fn fetch_all_from_db(sql: &str, params: Vec<&str>, pool: &PgPool) -> Result<Vec<PgRow>, Error>  {
    let result = sqlx::query(sql)
        .bind(params)
        .fetch_all(pool)
        .await;

    return result;
}
