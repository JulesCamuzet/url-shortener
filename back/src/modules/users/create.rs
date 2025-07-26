use sqlx::{PgPool};

use crate::{
    db::users::{get_one_by_email::get_one_user_by_email, insert_one::{insert_one_user}},
    dtos::{user::CreateUserDto},
    helpers::{
        check_format::{check_email_format, check_password_format},
        generate::{generate_random_string, GenerateRandomStringOptions},
        hash::hash_password
    },
    models::user::CreateUserDb
};

pub struct CreateUserOutput {
    pub id: i32
}

pub enum CreateUserError {
    InvalidEmailFormat,
    InvalidPasswordFormat,
    EmailAlreadyExists,
    Unknown
}

pub async fn create_user(CreateUserDto {
    email, password
}: CreateUserDto, pool: PgPool) -> Result<CreateUserOutput, CreateUserError> {
    match check_email_format(email.as_str()) {
        Ok(is_valid) => {
            if !is_valid {
                return Err(CreateUserError::InvalidEmailFormat);
            }
        },
        Err(_) => return Err(CreateUserError::Unknown)
    };

    let is_password_valid = check_password_format(password.as_str());

    if !is_password_valid {
        return Err(CreateUserError::InvalidPasswordFormat);
    }

    match get_one_user_by_email(email.as_str(), &pool).await {
        // TODO add 'maybe_' prefix to variables that are Options
        Ok(result) => {
            match result {
                Some(_) => return Err(CreateUserError::EmailAlreadyExists),
                None => {}
            }
        },
        Err(_) => return Err(CreateUserError::Unknown)
    }

    let verification_token = generate_random_string(16, GenerateRandomStringOptions {
        include_uppercases: true,
        include_specials: false,
        include_numbers: true
    });

    let hashed_password = match hash_password(password) {
        Ok(hashed_password) => hashed_password,
        Err(_) => return Err(CreateUserError::Unknown)
    };

    let result = match insert_one_user(
        CreateUserDb {
            email,
            password: hashed_password,
            verification_token: verification_token
        },
        &pool
    ).await {
        Ok(output) => output,
        Err(_) => return Err(CreateUserError::Unknown)
    };

    let output = CreateUserOutput {
        id: result.id
    };

    Ok(output)
}
