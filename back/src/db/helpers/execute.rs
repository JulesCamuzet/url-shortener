use sqlx::{self, postgres::PgQueryResult, Error, PgPool};

pub async fn execute_db(query: &str, pool: &PgPool) -> Result<PgQueryResult, Error> {
    let result = sqlx::query(query).execute(pool).await;

    return result;
}
