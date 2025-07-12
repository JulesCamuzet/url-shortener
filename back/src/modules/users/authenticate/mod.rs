use sqlx::PgPool;
use crate::{db::users::get_one_by_email::get_one_user_by_email, helpers::hash::verify_password};

pub struct AuthenticateUserInput {
    pub email: String,
    pub password: String,
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
        pool
    }: AuthenticateUserInput
) -> Result<AuthenticateUserOutput, AuthenticateUserError> {
    let result = match get_one_user_by_email(email.as_str(), &pool).await {
        Err(_) => return Err(AuthenticateUserError::Unknown),
        Ok(result) => result
    };

    let user = match result {
        None => return Err(AuthenticateUserError::InvalidCredentials),
        Some(user) => user
    };

    if !verify_password(password, user.password) {
        return Err(AuthenticateUserError::InvalidCredentials);
    }


}
