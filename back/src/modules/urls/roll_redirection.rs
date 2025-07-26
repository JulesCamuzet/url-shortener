use sqlx::PgPool;

use crate::{db::{redirections::get_many_by_url_id::get_many_redirections_by_url_id, urls::get_by_short_value::{
    get_url_by_short_value,
    GetUrlByShortValueInput
}}, models::redirection::RedirectionDb};

pub enum RollRedirectionLinkError {
    NotFound,
    Unknown
}

fn process_roll(redirections: Vec<RedirectionDb>) -> String {
    let mut sum: i64 = 0;
    
    for i in 0..redirections.len() {
        sum += redirections[i].probability_score;
    }
    
    let random_number = rand::random_range(0..=sum);
    
    sum = 0;
    
    for i in 0..redirections.len() {
        sum += redirections[i].probability_score;
        
        if sum <= random_number {
            return redirections[i].link.clone();
        }
    }
    
    return redirections[redirections.len() - 1].link.clone();
}

pub async fn roll_redirection_link(
    short_value: String, pool: PgPool
) -> Result<String, RollRedirectionLinkError> {
    let url = match get_url_by_short_value(GetUrlByShortValueInput
        { short_value: short_value, pool: &pool
    }).await {
        Err(_) => return Err(RollRedirectionLinkError::Unknown),
        Ok(maybe_url) => match maybe_url {
            None => return Err(RollRedirectionLinkError::NotFound),
            Some(url) => url
        }
    };
    
    let redirections = match get_many_redirections_by_url_id(url.id, &pool).await {
        Err(_) => return Err(RollRedirectionLinkError::Unknown),
        Ok(redirections) => redirections
    };
    
    if redirections.len() == 0 {
        return Err(RollRedirectionLinkError::NotFound);
    }
    
    Ok(String::new())
}
