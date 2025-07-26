use sqlx::{PgPool, Error};

use crate::models::redirection::RedirectionDb;

pub async fn get_many_redirections_by_url_id (
    url_id: i32, pool: &PgPool
) -> Result<Vec<RedirectionDb>, Error> {
    let query = sqlx::query_as::<_, RedirectionDb>(
        "SELECT 
            id,
            link,
            probability_score,
            url_id,
            created_at
        FROM redirections
        WHERE url_id = $1"
    ).bind(url_id);
    
    let result = query.fetch_all(pool).await;
    
    match result {
        Ok(result) => Ok(result),
        Err(e) => {
            eprintln!("Error while getting many redirection by url_id
                url_id : {url_id},
                Error : {e}
                ");
            Err(e)
        }
    }
}
