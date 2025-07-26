use sqlx::prelude::FromRow;
use time::PrimitiveDateTime;

#[derive(FromRow)]
pub struct UrlDb {
    pub id: i32,
    pub short_value: String,
    pub user_id: Option<i32>,
    pub created_at: PrimitiveDateTime
}

pub struct CreateUrlDb {
    pub short_value: String,
    pub name: String,
    pub user_id: Option<i32> 
}
