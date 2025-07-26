use serde::Deserialize;
use time::PrimitiveDateTime;

#[derive(Deserialize)]
pub struct RedirectionDto {
    pub link: String,
    pub probability_score: i64 
}

pub struct UrlDto {
    pub id: i32,
    pub name: String,
    pub short_value: String,
    pub redirections: Vec<RedirectionDto>,
    pub user_id: Option<i32>,
    pub created_at: PrimitiveDateTime
}

#[derive(Deserialize)]
pub struct CreateUrlDto {
    pub user_id: Option<i32>,
    pub name: String,
    pub redirections: Vec<RedirectionDto>
}
