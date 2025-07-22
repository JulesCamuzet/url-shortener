use sqlx::PgPool;

use crate::{
    db::urls::insert_one::{insert_one_url, InsertOneUrlInput},
    helpers::{
        check_format::check_url_format,
        generate::{generate_random_string, GenerateRandomStringOptions}
    }
};

pub struct CreateUrlInput {
    pub user_id: Option<i32>,
    pub url: String,
    pub pool: PgPool
}

pub struct CreateUrlOutput {
    pub id: i32,
    pub short_value: String,
    pub original_value: String 
}

pub enum CreateUrlError  {
    WrongFormat,
    Unknown
}

pub async fn create_url<'a>(CreateUrlInput
    { user_id, url, pool }: CreateUrlInput
) -> Result<CreateUrlOutput, CreateUrlError> {
    if !check_url_format(url.as_str()) {
        return Err(CreateUrlError::WrongFormat)
    }
    
    let shortened_url = generate_random_string(5, GenerateRandomStringOptions {
        include_numbers: true,
        include_specials: false,
        include_uppercases: true
    });
    
    let result = match insert_one_url(InsertOneUrlInput {
        original_value: url,
        short_value: shortened_url,
        user_id,
        pool: &pool
    }).await {
        Err(_) => return Err(CreateUrlError::Unknown),
        Ok(result) => result
    };
    
    let output = CreateUrlOutput {
        id: result.id,
        original_value: result.original_value,
        short_value: result.short_value
    };

    Ok(output)
}
