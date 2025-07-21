use axum::http::{HeaderMap, StatusCode};
use sqlx::PgPool;

use crate::{
    db::users::get_one_by_email::get_one_user_by_email,
    errors::HandlerError,
    helpers::{cookies::get_cookie, jwt::verify_jwt},
    models::user::User
};

pub async fn check_auth(
    headers: &HeaderMap,
    pool: &PgPool
) -> Result<User, HandlerError> {
    let auth_handler_error = HandlerError {
        status: StatusCode::UNAUTHORIZED,
        message: "Authentication error.".to_string(),
        code: "AUTH_ERROR".to_string()
    };
    
    let private_key = match std::env::var("PRIVATE_KEY") {
        Err(_) => return Err(auth_handler_error),
        Ok(key) => key
    };
    
    let token = match get_cookie(headers, "auth") {
        None => return Err(auth_handler_error),
        Some(token) => token
    };
        
    let claims = match verify_jwt(&token, private_key) {
        Err(_) => return Err(auth_handler_error),
        Ok(claims) => claims
    };
    
    let get_user_result = match get_one_user_by_email(&claims.email, pool).await {
        Err(_) => return Err(auth_handler_error),
        Ok(result) => result
    };
        
    let user = match get_user_result {
        None => return Err(auth_handler_error),
        Some(user) => user
    };
    
    Ok(user)
}
