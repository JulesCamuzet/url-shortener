use sqlx::prelude::FromRow;
use time::PrimitiveDateTime;

#[derive(FromRow)]
pub struct RedirectionDb {
    pub id: i32,
    pub link: String,
    pub probability_score: i64,
    pub url_id: i32,
    pub created_at: PrimitiveDateTime
}

pub struct CreateRedirectionDb {
    pub link: String,
    pub probability_score: i64,
    pub url_id: i32
}
