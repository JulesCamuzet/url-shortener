use sqlx::PgPool;

use crate::db::urls::get_by_short_value::{
    get_url_by_short_value,
    GetUrlByShortValueInput
};

pub struct GetUrlOriginalValueInput {
    pub short_value: String,
    pub pool: PgPool
}

pub enum GetUrlOriginalValueError {
    NotFound,
    Unknown
}

pub async fn get_url_original_value(GetUrlOriginalValueInput {
    short_value, pool
}: GetUrlOriginalValueInput) -> Result<String, GetUrlOriginalValueError> {
    match get_url_by_short_value(GetUrlByShortValueInput {
        short_value,
        pool: &pool
    }).await {
        Err(_) => Err(GetUrlOriginalValueError::Unknown),
        Ok(maybe_url) => match maybe_url {
            None => Err(GetUrlOriginalValueError::NotFound),
            Some(url) => Ok(url.original_value)
        }
    }
}
