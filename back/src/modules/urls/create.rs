use sqlx::PgPool;

use crate::{
    constants::MAX_PROBABILITY_SCORE, db::{redirections::insert_many::insert_many_redirections, urls::insert_one::insert_one_url}, dtos::url::CreateUrlDto, helpers::{
        check_format::check_url_format,
        generate::{generate_random_string, GenerateRandomStringOptions}
    }, models::{redirection::CreateRedirectionDb, url::CreateUrlDb}
};

pub struct CreateUrlOutput {
    pub id: i32,
    pub short_value: String,
    pub redirections_ids: Vec<i32>
}

pub enum CreateUrlError  {
    WrongProbabilityScore,
    WrongFormat,
    Unknown
}

pub async fn create_url(
    create_url_dto: CreateUrlDto, pool: PgPool
) -> Result<CreateUrlOutput, CreateUrlError> {
    for i in 0..create_url_dto.redirections.len() {
        if !check_url_format(&create_url_dto.redirections[i].link) {
            return Err(CreateUrlError::WrongFormat)
        }
        
        if create_url_dto.redirections[i].probability_score > MAX_PROBABILITY_SCORE || 
            create_url_dto.redirections[i].probability_score < 1 {
            return Err(CreateUrlError::WrongProbabilityScore)
        }
    }
    
    let shortened_url = generate_random_string(5, GenerateRandomStringOptions {
        include_numbers: true,
        include_specials: false,
        include_uppercases: true
    });
    
    let insert_url_result = match insert_one_url(
        CreateUrlDb {
            user_id: create_url_dto.user_id,
            name: create_url_dto.name,
            short_value: shortened_url
        },
        &pool
    ).await {
        Err(_) => return Err(CreateUrlError::Unknown),
        Ok(result) => result
    };
    
    let create_redirections_db: Vec<CreateRedirectionDb> = create_url_dto.redirections
        .iter()
        .map(|redirection| CreateRedirectionDb {
            link: redirection.link.to_string(),
            probability_score: redirection.probability_score,
            url_id: insert_url_result.id
        })
        .collect();
    
    let insert_redirections_result = match insert_many_redirections(
        create_redirections_db, &pool
    ).await {
        Err(_) => return Err(CreateUrlError::Unknown),
        Ok(result) => result
    };
    
    let output = CreateUrlOutput {
        id: insert_url_result.id,
        short_value: insert_url_result.short_value,
        redirections_ids: insert_redirections_result
            .iter()
            .map(|item| item.id)
            .collect()
    };

    Ok(output)
}
