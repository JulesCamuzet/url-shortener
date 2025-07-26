use sqlx::PgPool;
use crate::{
    db::users::get_one_by_email::get_one_user_by_email,
    dtos::user::AuthenticateUserDto,
    helpers::{hash::verify_password, jwt::{get_jwt, Claim}}
};

pub struct AuthenticateUserInput {
    pub auth_user_dto: AuthenticateUserDto,
    pub private_key: String,
    pub pool: PgPool,
    pub exp: i64
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
        auth_user_dto,
        private_key,
        pool,
        exp
    }: AuthenticateUserInput
) -> Result<AuthenticateUserOutput, AuthenticateUserError> {
    let get_user_result = match get_one_user_by_email(
        auth_user_dto.email.as_str(), &pool
    ).await {
        Err(_) => return Err(AuthenticateUserError::Unknown),
        Ok(result) => result
    };

    let user = match get_user_result {
        None => return Err(AuthenticateUserError::InvalidCredentials),
        Some(user) => user
    };

    let is_password_valid = match verify_password(
        auth_user_dto.password, user.password
    ) {
        Err(_) => return Err(AuthenticateUserError::Unknown),
        Ok(result) => result
    };

    if !is_password_valid {
        return Err(AuthenticateUserError::InvalidCredentials);
    }

    let jwt = match get_jwt(Claim { email: user.email, exp }, private_key) {
        Err(_) => return Err(AuthenticateUserError::Unknown),
        Ok(token) => token
    };

    let output = AuthenticateUserOutput {
        jwt
    };

    Ok(output)
}
