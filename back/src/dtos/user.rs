use serde::Deserialize;

pub struct UserDto {
    pub id: i32,
    pub email: String,
    pub created_at: String
}

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct AuthenticateUserDto {
    pub email: String,
    pub password: String
}
