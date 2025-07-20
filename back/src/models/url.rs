use sqlx::prelude::FromRow;
use time::PrimitiveDateTime;

#[derive(FromRow)]
pub struct Url {
    pub id: i32,
    pub short_value: String,
    pub original_value: String,
    pub user_id: Option<i32>,
    pub created_at: PrimitiveDateTime
}
