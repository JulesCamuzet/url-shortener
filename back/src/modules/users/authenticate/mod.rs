use sqlx::PgPool;
use crate::{
    db::users::get_one_by_email::get_one_user_by_email,
    helpers::{hash::verify_password, jwt::{get_jwt, Claim}}
};

pub struct AuthenticateUserInput {
    pub email: String,
    pub password: String,
    pub private_key: String,
    pub pool: PgPool
}

pub struct AuthenticateUserOutput {
    pub jwt: String
}

pub enum AuthenticateUserError {
    InvalidCredentials,
    Unknown
}

pub async fn authenticate_user(
    AuthenticateUserInput {
        email,
        password,
        private_key,
        pool
    }: AuthenticateUserInput
) -> Result<AuthenticateUserOutput, AuthenticateUserError> {
    let get_user_result = match get_one_user_by_email(email.as_str(), &pool).await {
        Err(_) => return Err(AuthenticateUserError::Unknown),
        Ok(result) => result
    };

    let user = match get_user_result {
        None => return Err(AuthenticateUserError::InvalidCredentials),
        Some(user) => user
    };

    let is_password_valid = match verify_password(password, user.password) {
        Err(_) => return Err(AuthenticateUserError::Unknown),
        Ok(result) => result
    };

    if !is_password_valid {
        return Err(AuthenticateUserError::InvalidCredentials);
    }

    let jwt = match get_jwt(Claim { email: user.email }, private_key) {
        Err(e) => return Err(AuthenticateUserError::Unknown),
        Ok(token) => token
    };

    let output = AuthenticateUserOutput {
        jwt
    };

    Ok(output)
}
