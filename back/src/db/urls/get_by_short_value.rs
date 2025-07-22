use sqlx::{Error, PgPool};

use crate::models::url::Url;

pub struct GetUrlByShortValueInput<'a> {
    pub short_value: String,
    pub pool: &'a PgPool
}

pub async fn get_url_by_short_value<'a>(GetUrlByShortValueInput {
    short_value,
    pool
}: GetUrlByShortValueInput<'a>) -> Result<Option<Url>, Error> {
    let query = sqlx::query_as::<_, Url>(
        "SELECT
            id,
            short_value,
            original_value,
            user_id,
            created_at
        FROM Url WHERE short_value = $1"
    ).bind(short_value.as_str());
    
    let result = query.fetch_optional(pool).await;
    
    match result {
        Err(e) => {
            eprint!("Error while getting url from short_value
                Short value : {short_value}
                Error : {e}");
            Err(e)
        },
        Ok(result) => Ok(result)
    }
}
