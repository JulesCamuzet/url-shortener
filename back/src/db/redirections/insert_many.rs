use sqlx::{prelude::FromRow, PgPool};

use crate::models::redirection::{CreateRedirectionDb};

#[derive(FromRow)]
pub struct OutputItem {
    pub id: i32
}

pub async fn insert_many_redirections(
    redirections: Vec<CreateRedirectionDb>,
    pool: &PgPool
) -> Result<Vec<OutputItem>, sqlx::Error> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "INSERT INTO redirections (link, probability_score, url_id)"
    );
    
    query_builder.push_values(redirections.iter(), |mut b, redirection| {
        b.push_bind(&redirection.link);
        b.push_bind(redirection.probability_score);
        b.push_bind(redirection.url_id);
    }).push(" RETURNING id");
    
    let result = query_builder.build_query_as::<OutputItem>().fetch_all(pool).await;
    
    match result {
        Err(e) => {
          eprintln!("
              Error while inserting many redirections.
              Error : {e}
              ");
          Err(e)
        },
        Ok(result) => Ok(result)
    }
}
